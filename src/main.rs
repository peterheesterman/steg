
extern crate clap;
use clap::{App, Arg};

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

        .arg(Arg::with_name("stratagy")
            .short("s")
            .long("stratagy")
            .value_name("FILE")
            .help("The specific way in which you want to hide the payload in the carrier")
            .takes_value(true))

        .get_matches();


        // Print out the values of the inputs
        if let Some(carrier_path) = matches.value_of("carrier") {
            println!("Value for stratagy: {}", &carrier_path);
        }

        if let Some(payload_path) = matches.value_of("payload") {
            println!("Value for payload: {}", &payload_path);
        }

        if let Some(strategy_path) = matches.value_of("stratagy") {
            println!("Value for stratagy: {}", &strategy_path);
        }
}

