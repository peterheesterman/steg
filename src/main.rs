
extern crate clap;
use clap::{App, Arg};

extern crate steg;
use steg::Config;

fn main() {
    let matches = App::new("Steg")
        .version("0.1.0")
        .author("Peter Heesterman <heesterman.peter@gmail.com>")
        .about("Hides things for you")

        .arg(Arg::with_name("payload")
            .short("p")
            .long("payload")
            .value_name("FILE")
            .help("The secret message or content to be hidden")
            .required(true)
            .takes_value(true))

        .arg(Arg::with_name("carrier")
            .short("c")
            .long("carrier")
            .value_name("FILE")
            .help("The entity in which you are trying to hide a payload")
            .required(true)
            .takes_value(true))

        .arg(Arg::with_name("strategy")
            .short("s")
            .long("strategy")
            .value_name("FILE")
            .help("The specific way in which you want to hide the payload in the carrier")
            .takes_value(true))

        .get_matches();


    // Print out the values of the inputs
    let carrier_path = match matches.value_of("carrier") {
        Some(path) => path,
        None => panic!("There is no carrier supplied"),
    };

    let payload_path = match matches.value_of("payload") {
        Some(path) => path,
        None => panic!("There is no payload supplied"),
    };

    let strategy_path = match matches.value_of("strategy") {
        Some(path) => path,
        None => panic!("There is no strategy supplied"),
    };

    let config = Config {
        carrier_path,
        payload_path,
        strategy_path
    };

    println!("Configuration: {:?}", config);

    steg::run(config)
}
