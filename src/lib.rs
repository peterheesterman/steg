
#[derive(Debug)] 
pub struct Config<'a> {
    pub strategy_path: &'a str,
    pub payload_path: &'a str,
    pub carrier_path: &'a str,
}

mod text;
mod images;
mod videos;

pub fn run (config: Config) {
    // cargo run -- -p dfkjl.png -c dsjfkl.png -s kdsjf
    if config.payload_path.ends_with(".png") && config.carrier_path.ends_with(".png") {
        println!("We are going to default to lsb_png_steganography");
        images::lsb_png_steganography::run();
    } else {
        panic!("Nothing happened!")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn lsb_png_steganography_does_not_panic() {
        let config = Config {
            strategy_path: "",
            payload_path: ".png",
            carrier_path: ".png",
        };

        super::run(config);
    }
}
