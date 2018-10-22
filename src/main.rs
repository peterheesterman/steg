
extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate steg;
use steg:: { HideConfig, RevealConfig };

fn main() {
    let matches = App::new("Steg")
        .version("0.1.0")
        .author("Peter Heesterman <heesterman.peter@gmail.com>")
        .about("Command Line Steganography")
        .subcommand(SubCommand::with_name("hide")
            .about("Hide things with steganography")
            .version("0.1.0")
            .author("Peter Heesterman <heesterman.peter@gmail.com>")
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

                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Where to put the result of hiding the payload in the carrier")
                    .takes_value(true))

                .arg(Arg::with_name("strategy")
                    .short("s")
                    .long("strategy")
                    .value_name("STRING")
                    .help("The specific way in which you want to hide the payload in the carrier")
                    .takes_value(true)))

        .subcommand(SubCommand::with_name("reveal")
            .about("Reveal things that have been hidden using steganography")
            .version("0.1.0")
            .author("Peter Heesterman <heesterman.peter@gmail.com>")                    
                .arg(Arg::with_name("carrier")
                    .short("c")
                    .long("carrier")
                    .value_name("FILE")
                    .help("The entity in which you are trying to hide a payload")
                    .required(true)
                    .takes_value(true))

                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("The specific way in which you want to hide the payload in the carrier")
                    .takes_value(true))

                .arg(Arg::with_name("strategy")
                    .short("s")
                    .long("strategy")
                    .value_name("STRING")
                    .help("The specific way in which you want to hide the payload in the carrier")
                    .takes_value(true)))

        .get_matches();


    if let Some(hide) = matches.subcommand_matches("hide") {
        
        let carrier_path = match hide.value_of("carrier") {
            Some(path) => path,
            None => panic!("There is no carrier supplied"),
        };
        
        let payload_path = match hide.value_of("payload") {
            Some(path) => path,
            None => panic!("There is no payload supplied"),
        };

        let config = HideConfig {
            payload_path,
            carrier_path,
            output_path: hide.value_of("output"),
            strategy: hide.value_of("strategy")
        };

        steg::run_hide(config);
    } else if let Some(reveal) = matches.subcommand_matches("reveal") {
        let carrier_path = match reveal.value_of("carrier") {
            Some(path) => path,
            None => panic!("There is no carrier supplied"),
        };

        let config = RevealConfig {
            carrier_path,
            output_path: reveal.value_of("output"),
            strategy: reveal.value_of("strategy")
        };

        println!("Configuration: {:?}", config);

        steg::run_reveal(config);
    } else {
        panic!("No valid command found")
    }
}
