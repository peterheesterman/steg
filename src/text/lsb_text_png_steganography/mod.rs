extern crate lsb_text_png_steganography;

use std::fs;

pub fn hide(payload_path: &str, carrier_path: &str, output_path: &str) {
    let img = lsb_text_png_steganography::hide(payload_path, carrier_path);

    img.save(output_path).unwrap();
}

pub fn reveal(carrier_path: &str, output_path: &str) {
    let result = lsb_text_png_steganography::reveal(carrier_path);
    // todo (sdv) this does not create the file if the directory does not exist
    fs::write(output_path, result).expect("Unable to write hidden text file");
}
