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

mod images;
mod text;
mod videos;

fn hide_default(payload: &str, carrier: &str, _o:&str) {
    panic!("No strategy can be used to hide {} in {}!", payload, carrier);
}

pub fn run_hide(config: HideConfig) {
    let mut hide: fn(&str, &str, &str) = hide_default;
    let mut output_file_name_if_note_specified = "./steg_package.txt";

    if config.payload_path.ends_with(".png") && config.carrier_path.ends_with(".png") {
        println!("Hide using - Least Significant Bit PNG Steganography (lsb_png_steganography)");
        hide = images::lsb_png_steganography::hide;
        output_file_name_if_note_specified = "./steg_package.png";
    } else if config.payload_path.ends_with(".txt") && config.carrier_path.ends_with(".txt") {
        println!("Hide using - whitespace text steganography (whitespace_text_steganography)");
        hide = text::whitespace_text_steganography::hide; 
        output_file_name_if_note_specified = "./steg_package.txt";
    } else if config.payload_path.ends_with(".txt") && config.carrier_path.ends_with(".png") {
        println!("Hide using - Least Significant Bit Text into PNG Steganography (lsb_text_png_steganography)");
        hide = text::lsb_text_png_steganography::hide;
        output_file_name_if_note_specified = "./steg_package.png";
    }

    if let Some(output_path) = config.output_path {
        hide(config.payload_path, config.carrier_path, output_path);
    } else {
        hide(config.payload_path, config.carrier_path, output_file_name_if_note_specified);
    }
}

fn reveal_default(carrier: &str, _o:&str) {
    panic!("No strategy was specified for getting a payload from {}!", carrier);
}

pub fn run_reveal(config: RevealConfig) {
    let mut reveal: fn(&str, &str) = reveal_default;
    let mut output_file_name_if_note_specified = "./hidden_message.txt";

    if config.carrier_path.ends_with(".png") {
        if let Some(strategy) = config.strategy {
            if strategy == "lsb_png_steganography" {
                reveal = images::lsb_png_steganography::reveal;
                output_file_name_if_note_specified = "./hidden_message.png";
            } else if strategy == "lsb_text_png_steganography" {
                reveal = text::lsb_text_png_steganography::reveal;
                output_file_name_if_note_specified = "./hidden_message.txt";
            } else {
                panic!("The strategy {} can't be used for .png files", strategy);
            }
        } else {
            panic!(".png files need a strategy specified to get messages out (e.g. -s lsb_text_png_steganography)");
        }
    } else if config.carrier_path.ends_with(".txt") {
        reveal = text::whitespace_text_steganography::reveal;
        output_file_name_if_note_specified = "./hidden_message.txt";
    }

    if let Some(output_path) = config.output_path {
        reveal(config.carrier_path, output_path);
    } else {
        reveal(config.carrier_path, output_file_name_if_note_specified);
    }
}

#[cfg(test)]
mod tests {
    use super::{HideConfig, RevealConfig};

    #[test]
    fn lsb_png_steganography_hide_does_not_panic() {
        let config = HideConfig {
            strategy: None,
            payload_path: "./images/payload.png",
            carrier_path: "./images/carrier.png",
            output_path: Some("images/output/lsb_png_steganography_hide_does_not_panic.png"),
        };

        super::run_hide(config);
    }

    #[test]
    fn lsb_png_steganography_reveal_does_not_panic() {
        let config = RevealConfig {
            strategy: Some("lsb_png_steganography"),
            carrier_path: "./images/contains_hidden_image.png",
            output_path: Some(
                "./images/output/whitespace_text_steganography_reveal_does_not_panic.png",
            ),
        };

        super::run_reveal(config);
    }

    #[test]
    fn whitespace_text_steganography_hide_does_not_panic() {
        let config = HideConfig {
            strategy: None,
            payload_path: "./texts/payload_haiku.txt",
            carrier_path: "./texts/carrier_sonnet.txt",
            output_path: Some(
                "./texts/output/whitespace_text_steganography_hide_does_not_panic.txt",
            ),
        };

        super::run_hide(config);
    }

    #[test]
    fn whitespace_text_steganography_reveal_does_not_panic() {
        let config = RevealConfig {
            strategy: None,
            carrier_path: "./texts/hidden.txt",
            output_path: Some(
                "./texts/output/whitespace_text_steganography_reveal_does_not_panic.txt",
            ),
        };

        super::run_reveal(config);
    }

     #[test]
    fn lsb_text_png_steganography_hide_does_not_panic() {
        let config = HideConfig {
            strategy: None,
            payload_path: "./texts/payload_haiku.txt",
            carrier_path: "./images/carrier.png",
            output_path: Some(
                "./images/output/lsb_text_png_steganography_hide_does_not_panic.png",
            ),
        };

        super::run_hide(config);
    }

    #[test]
    fn lsb_text_png_steganography_reveal_does_not_panic() {
        let config = RevealConfig {
            strategy: Some("lsb_text_png_steganography"),
            carrier_path: "./images/contains_hidden_text.png",
            output_path: Some(
                "./texts/output/lsb_text_png_steganography_reveal_does_not_panic.txt",
            ),
        };

        super::run_reveal(config);
    }
}
