extern crate reqwest;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate colored;

use chrono::prelude::*;
use colored::*;

mod cli;
mod error;
mod sl;

fn main() {
    env_logger::init();

    let matches = cli::build_cli().get_matches();

    let from_str = matches.value_of("from").unwrap();
    let to_str = matches.value_of("to").unwrap();
    let num_results = value_t!(matches, "number", usize).unwrap_or(1);
    debug!("Use number of result: {}", num_results);

    let sl = sl::SlApi::new();

    let find_from = match sl.find(&from_str) {
        Ok(r) => r,
        Err(e) => panic!("Unable to retrieve result for from '{}': {:?}", from_str, e),
    };
    let from = &find_from.candidates()[0];

    let find_to = match sl.find(&to_str) {
        Ok(r) => r,
        Err(e) => panic!("Unable to retrieve result for to '{}': {:?}", to_str, e),
    };
    let to = &find_to.candidates()[0];

    let mut topt = sl::TravelOption::Now;

    if let Some(a) = matches.value_of("arrive") {
        if let Some(dt) = get_date_time(a) {
            topt = sl::TravelOption::Arrival(dt);
        }
    }
    if let Some(d) = matches.value_of("depart") {
        if let Some(dt) = get_date_time(d) {
            topt = sl::TravelOption::Departure(dt);
        }
    }

    debug!("Use TravelOption: {:?}", &topt);

    let find_travel = match sl.travel(from, to, &topt) {
        Ok(r) => r,
        Err(e) => {
            error!(
                "Unable to find trips from '{}' to '{}': {:?}",
                from_str, to_str, e
            );
            std::process::exit(11);
        }
    };

    let travel = find_travel.directions();

    for trip in &travel.Trips[0..num_results] {
        print_trip(trip);
    }
}

//------------------------------------------------------------------------------

fn get_date_time(dt: &str) -> Option<NaiveDateTime> {
    debug!("Parse date time: {}", dt);

    let input = format!("{}-{}", Utc::today().year(), dt);
    if let Ok(r) = NaiveDateTime::parse_from_str(&input, "%Y-%m-%d %H:%M") {
        return Some(r);
    }

    if let Ok(t) = NaiveTime::parse_from_str(dt, "%H:%M") {
        let dt = Utc::today().naive_utc().and_time(t);
        return Some(dt);
    }

    None
}

//------------------------------------------------------------------------------

fn print_trip(trip: &sl::Trip) {
    println!("\n* {}", trip.Origin.green());
    println!("|  Duration: {}", trip.Duration.on_red());
    println!(
        "|  {} - {} ({})",
        trip.DepartureTime.format("%H:%M").to_string().yellow(),
        trip.ArrivalTime.format("%H:%M").to_string().yellow(),
        trip.DepartureDate.format("%Y-%m-%d").to_string().yellow()
    );
    let lines: Vec<String> = trip
        .Transports
        .iter()
        .map(|ref t| {
            if t.LineNumber.is_empty() {
                "walk".to_string()
            } else {
                t.LineNumber.clone()
            }
        })
        .collect();

    println!("|  Summary: {}", lines.join(" - "));
    println!("|");
    for sub in &trip.SubTrips {
        println!(
            "|  {}   {} {}",
            "*".blue(),
            sub.DepartureTime.format("%H:%M").to_string().yellow(),
            sub.Origin
        );
        //println!(" {}", "|".blue());
        println!("|  {}       {}", "|".blue(), sub.TransportText);
        println!(
            "|  {}       Line:     {} {}",
            "|".blue(),
            sub.transport_string(),
            sub.LineNumber
        );
        println!(
            "|  {}       Duration: {}",
            "|".blue(),
            format!("{} min", sub.duration().num_minutes()).on_red()
        );
        //println!(" {}", "|".blue());
        println!(
            "|  {}   {} {}",
            "*".blue(),
            sub.ArrivalTime.format("%H:%M").to_string().yellow(),
            sub.Destination
        );
    }

    println!("|\n* {}\n\n{}", trip.Destination.green(), "-".repeat(80));
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use chrono::{Datelike, NaiveDateTime, NaiveTime, UTC};

    #[test]
    fn test_get_date_time_time_only() {
        let t = NaiveTime::from_hms(21, 00, 00);
        let d = UTC::today();

        let dt = NaiveDateTime::new(d.naive_utc(), t);
        assert_eq!(super::get_date_time("21:00"), Some(dt));
    }

    #[test]
    fn test_get_date_time_dt() {
        let input = format!("{}-08-30 21:30", UTC::today().year());
        let dt = NaiveDateTime::parse_from_str(&input, "%Y-%m-%d %H:%M").unwrap();
        assert_eq!(super::get_date_time("08-30 21:30"), Some(dt));
    }
}
