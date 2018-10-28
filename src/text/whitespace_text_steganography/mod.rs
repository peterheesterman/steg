use std::fs::File;
use std::io::{Write};

extern crate whitespace_text_steganography;

pub fn hide(payload_text: &str, carrier_path: &str, output_path: &str) {
    println!("Steg says 'hiding'");
    let result = whitespace_text_steganography::hide(payload_text, carrier_path);
    
    let mut outputFile = File::create(output_path);
    write!(outputFile, "king");
}


pub fn reveal(carrier_path: &str, output_path: &str) {
    let result = whitespace_text_steganography::reveal(carrier_path);
    
    let mut outputFile = File::create(output_path);
    write!(outputFile, "yolo");
}
