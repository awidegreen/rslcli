use std::io::{Read};
use hyper::Client;
use chrono::{NaiveTime, NaiveDate, Duration, NaiveDateTime};

use serde_json;
use error;

mod date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
        where D: Deserializer<'de>
        {
            let s = String::deserialize(deserializer)?;
            NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
}

 mod time_format {
    use chrono::{NaiveTime};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%H:%M";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
pub enum TravelOption {
    Arrival(NaiveDateTime),
    Departure(NaiveDateTime),
    Now
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Station {
    Name: String,
    SiteId: u32,
    Type: String,
}
impl Station {
}

#[derive(Deserialize, Debug)]
pub struct FindResponse {
    ms: u32,
    status: String,
    data: Vec<Station>,
}

impl FindResponse {
    pub fn candidates(&self) -> &Vec<Station> {
        &self.data
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct SubTrip {
    pub Origin : String,
    pub Destination: String,
    pub Direction: String,
    pub TransportSymbol: String,
    pub TransportText: String,
    #[serde(with = "date_format")]
    pub DepartureDate: NaiveDate,
    #[serde(with = "time_format")]
    pub DepartureTime: NaiveTime,
    #[serde(with = "date_format")]
    pub ArrivalDate: NaiveDate,
    #[serde(with = "time_format")]
    pub ArrivalTime: NaiveTime,
    pub LineNumber : String,
}

impl SubTrip {
    pub fn duration(&self) -> Duration {
        let dta = self.ArrivalDate.and_time(self.ArrivalTime);
        let dtd = self.DepartureDate.and_time(self.DepartureTime);
        dta.signed_duration_since(dtd)
    }

    pub fn transport_string(&self) -> String {
        if self.TransportSymbol == "Walk" {
            return "walk".to_string()
        }

        if self.TransportSymbol.starts_with("MET") {
            return format!("\u{1F687}")
        }

        if self.TransportSymbol.starts_with("BUS") {
            return format!("\u{1F68C}")
        }

        if self.TransportSymbol.starts_with("TRM") {
            return format!("\u{1F68B}")
        }

        self.TransportSymbol.clone()
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Transport {
    pub LineNumber : String,
    pub Symbol : String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Trip {
    pub Origin: String,
    pub Destination: String,
    #[serde(with = "date_format")]
    pub DepartureDate: NaiveDate,
    #[serde(with = "time_format")]
    pub DepartureTime: NaiveTime,
    #[serde(with = "date_format")]
    pub ArrivalDate: NaiveDate,
    #[serde(with = "time_format")]
    pub ArrivalTime: NaiveTime,
    Changes: String,
    pub Duration: String,
    pub Transports : Vec<Transport>,
    pub SubTrips : Vec<SubTrip>,
}

impl Trip {

}
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TravelError {
    pub Message: String,
    pub Code: String,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Travel {
    pub From: String,
    pub To: String,
    pub Trips: Vec<Trip>,
    pub LastUpdate: String,
    Error: Option<TravelError>,
}


#[derive(Deserialize, Debug)]
pub struct TravelResponse {
    ms: u32,
    status: String,
    data : Travel
}

impl TravelResponse {
    pub fn directions(&self) -> &Travel {
        &self.data
    }

    pub fn is_error(&self) -> Option<String> {
        match self.data.Error {
            Some(ref err) => Some(err.Message.clone()),
            _ => None
        }
    }
}

//------------------------------------------------------------------------------

pub struct SlApi {
    client: Client,
}

impl SlApi {
    pub fn new() -> SlApi {
        SlApi {
            client : Client::new()
        }
    }

    fn request(&self, url: &str) -> Result<String, error::Error> {
        debug!("Use request: '{}'", &url);

        let mut res = self.client.get(url).send()?;

        debug!("Response: {}", res.status);

        let mut s = String::new();
        let _ = res.read_to_string(&mut s);

        Ok(s)
    }

    pub fn find(&self, term : &str) -> Result<FindResponse, error::Error> {
        let url = format!("http://sl.se/api/TypeAhead/Find/{term}", term = term);

        let s = self.request(&url)?;
        let r : FindResponse = serde_json::from_str(&s)?;

        return Ok(r)
    }

    pub fn travel(&self, from : &Station, to: &Station, opt: &TravelOption)
        -> Result<TravelResponse, error::Error>
    {
        let base_url =
            format!("http://sl.se/api/sv/TravelPlanner/SearchTravelById/{from_name}/{to_name}/{from_id}/{to_id}",
                          from_name = from.Name, from_id = from.SiteId,
                          to_name = to.Name, to_id = to.SiteId);

        let url = match opt {
            &TravelOption::Arrival(dt)   => format!("{}/{}/arrive/", base_url, dt.format("%Y-%m-%d %H_%M")),
            &TravelOption::Departure(dt) => format!("{}/{}/depart/", base_url, dt.format("%Y-%m-%d %H_%M")),
            &TravelOption::Now => base_url,
        };

        let s = self.request(&url)?;
        let r : TravelResponse = serde_json::from_str(&s)?;

        if let Some(e) = r.is_error() {
            debug!("json response: {}", &s);
            return Err(error::Error::InputError(e))
        }

        return Ok(r)
    }
}

