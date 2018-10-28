use std::fs;
extern crate whitespace_text_steganography;

pub fn hide(payload_text: &str, carrier_path: &str, output_path: &str) {
    let result = whitespace_text_steganography::hide(payload_text, carrier_path);
    // todo (sdv) this does not create the file if it does not exist
    fs::write(output_path, result).expect("Unable to write file");
}


pub fn reveal(carrier_path: &str, output_path: &str) {
    let result = whitespace_text_steganography::reveal(carrier_path);
    // todo (sdv) this does not create the file if it does not exist
    fs::write(output_path, result).expect("Unable to write file");
}
