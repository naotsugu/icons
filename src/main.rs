use std::error::Error;
use std::fs::File;
use std::io::{BufWriter};
use icns::{IconFamily, Image, PixelFormat};
use image::imageops::FilterType;
fn main() -> Result<(), Box<dyn Error>> {

    let mut args = std::env::args().skip(1);
    assert_eq!(args.len(), 1, "Arguments must be: file_path");

    let file_path = args.next().unwrap();
    let src_img = image::open(&file_path)?;

    println!(" [{}] {} x {}", file_path, src_img.width(), src_img.height());

    let mut icons = IconFamily::new();
    let bytes = src_img
        .resize(64, 64, FilterType::Lanczos3)
        .to_rgba8()
        .to_vec();
    let icon = Image::from_data(PixelFormat::RGBA, 64, 64, bytes)?;
    icons.add_icon(&icon)?;

    let out = BufWriter::new(File::create("out.icns")?);
    icons.write(out)?;

    Ok(())
}
