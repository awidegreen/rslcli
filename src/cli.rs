use clap::{App, SubCommand, Arg, AppSettings};

pub fn build_cli() -> App<'static, 'static> {
    App::new("rslci")
        .about("CLI client for SL trip planning")
        .author("Armin Widegreen <armin.widegreen@gmail.com>")
        .version(crate_version!())
        .arg(Arg::with_name("number")
             .short("n")
             .long("number")
             .takes_value(true)
             .help("Number of trips to show, max 5"))
        .arg(Arg::with_name("depart")
             .short("d")
             .long("depart")
             .takes_value(true)
             .conflicts_with("arrive")
             .help("Departure date time in format: '[mm-dd] HH:MM', if no day is given, current is assumed."))
        .arg(Arg::with_name("arrive")
             .short("a")
             .long("arrive")
             .takes_value(true)
             .conflicts_with("depart")
             .help("Arrival date time in format: '[mm-dd] HH:MM', if no day is given, current is assumed."))
        .arg(Arg::with_name("from")
             .required(true)
             .index(1))
        .arg(Arg::with_name("to")
             .required(true)
             .index(2))
        .subcommand(SubCommand::with_name("completions")
            .about("Generates completion scripts for your shell")
            .setting(AppSettings::Hidden)
            .arg(Arg::with_name("SHELL")
                .required(true)
                .possible_values(&["bash", "fish", "zsh"])
                .help("The shell to generate the script for")))
}
