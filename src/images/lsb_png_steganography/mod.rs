
extern crate lsb_png_steganography;

pub fn hide(payload_path: &str, carrier_path: &str, output_path: &str) {
    let img = lsb_png_steganography::hide(payload_path, carrier_path);
    img.save(output_path).unwrap();
}

pub fn reveal(carrier_path: &str, output_path: &str) {
    let img = lsb_png_steganography::reveal(carrier_path);
    img.save(output_path).unwrap();
}