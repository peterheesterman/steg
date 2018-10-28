extern crate whitespace_text_steganography;

use std::fs;

pub fn hide(payload_path: &str, carrier_path: &str, output_path: &str) {
    let result = whitespace_text_steganography::hide(payload_path, carrier_path);

    fs::write(output_path, result).expect("Unable to write package file");
}

pub fn reveal(carrier_path: &str, output_path: &str) {
    let result = whitespace_text_steganography::reveal(carrier_path);
    // todo (sdv) this does not create the file if the directory does not exist
    fs::write(output_path, result).expect("Unable to write hidden text file");
}
