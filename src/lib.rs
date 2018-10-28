
#[derive(Debug)] 
pub struct HideConfig<'a> {
    pub payload_path: &'a str,
    pub carrier_path: &'a str,
    pub output_path: Option<&'a str>,
    pub strategy: Option<&'a str>,
}

#[derive(Debug)] 
pub struct RevealConfig<'a> {
    pub carrier_path: &'a str,
    pub output_path: Option<&'a str>,
    pub strategy: Option<&'a str>,
}

mod text;
mod images;
mod videos;

pub fn run_hide (config: HideConfig) {
    if config.payload_path.ends_with(".png") && config.carrier_path.ends_with(".png") {
        println!("Hide using - Least Significant Bit PNG Steganography (lsb_png_steganography)");

        if let Some(output_path) = config.output_path {
            images::lsb_png_steganography::hide(config.payload_path, config.carrier_path, output_path);
        } else {
            images::lsb_png_steganography::hide(config.payload_path, config.carrier_path, "./steg_hide_output.png");
        }
    } else if config.payload_path.ends_with(".txt") && config.carrier_path.ends_with(".txt") {
            println!("Hide using - whitespace text steganography");

        if let Some(output_path) = config.output_path {
            text::whitespace_text_steganography::hide(config.payload_path, config.carrier_path, output_path);
        } else {
            text::whitespace_text_steganography::hide(config.payload_path, config.carrier_path, "./steg_hide_output.png");
        }
    }
}

pub fn run_reveal (config: RevealConfig) {
    if config.carrier_path.ends_with(".png") {
        if let Some(output_path) = config.output_path {
            images::lsb_png_steganography::reveal(config.carrier_path, output_path);
        } else {
            images::lsb_png_steganography::reveal(config.carrier_path, "./steg_reveal_output.png");
        }
    } else if config.carrier_path.ends_with(".txt") {
        if let Some(output_path) = config.output_path {
            text::whitespace_text_steganography::reveal(config.carrier_path, output_path);
        } else {
            text::whitespace_text_steganography::reveal(config.carrier_path, "./steg_hide_output.txt");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ HideConfig, RevealConfig };

    #[test]
    fn lsb_png_steganography_hide_does_not_panic() {
        let config = HideConfig {
            strategy: Some(""),
            output_path: Some("images/output/lsb_png_steganography_hide_does_not_panic.png"),
            payload_path: "./images/payload.png",
            carrier_path: "./images/carrier.png",
        };

        super::run_hide(config);
    }

     #[test]
    fn lsb_png_steganography_reveal_does_not_panic() {
        let config = RevealConfig {
            strategy: Some(""),
            output_path: Some("images/output/whitespace_text_steganography_reveal_does_not_panic.png"),
            carrier_path: "./images/hidden.png",
        };

        super::run_reveal(config);
    }

    #[test]
    fn whitespace_text_steganography_hide_does_not_panic() {
        let config = HideConfig {
            strategy: Some(""),
            output_path: Some("text/output/whitespace_text_steganography_hide_does_not_panic.txt"),
            payload_path: "./text/payload.txt",
            carrier_path: "./text/carrier.txt",
        };

        super::run_hide(config);
    }

    #[test]
    fn whitespace_text_steganography_reveal_does_not_panic() {
        let config = RevealConfig {
            strategy: Some(""),
            output_path: Some("text/output/whitespace_text_steganography_hide_does_not_panic.txt"),
            carrier_path: "./text/carrier.txt",
        };

        super::run_reveal(config);
    }
}
