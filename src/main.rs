use std::error::Error;
use std::fs::File;
use std::io::{BufWriter};
use icns::{IconFamily, Image, PixelFormat};
use image::{DynamicImage, GenericImage, RgbaImage};
use image::imageops::FilterType;

fn main() -> Result<(), Box<dyn Error>> {

    let mut args = std::env::args().skip(1);
    assert_eq!(args.len(), 1, "Arguments must be: file_path");

    let file_path = args.next().unwrap();

    let size = image::image_dimensions(&file_path)?;

    let src_img = if size.0 != size.1 {
        // resize to square image
        let max = std::cmp::max(size.0, size.1);
        let mut buf = RgbaImage::new(max, max);
        let src_img = image::open(&file_path)?.to_rgba8();
        let x = if size.0 < size.1 { (max - size.0) / 2 } else { 0 };
        let y = if size.0 < size.1 { (max - size.1) / 2 } else { 0 };
        buf.copy_from(&src_img, x, y)?;
        image::DynamicImage::ImageRgba8(buf)
    } else {
        image::open(&file_path)?
    };

    println!("src : [{}] {} x {}", file_path, src_img.width(), src_img.height());

    write_icns(&src_img)?;
    write_ico(&src_img)?;

    Ok(())
}

/// Write icon as the apple icon image format.
fn write_icns(src_img: &DynamicImage) -> Result<(), std::io::Error> {
    let mut icons = IconFamily::new();
    for size in vec![16, 32, 48, 64, 128, 256, 512, 1024] {
        let bytes = src_img
            .resize(size, size, FilterType::Lanczos3)
            .to_rgba8()
            .to_vec();
        let icon = Image::from_data(PixelFormat::RGBA, size, size, bytes)?;
        icons.add_icon(&icon)?;
    }

    let out = BufWriter::new(File::create("out.icns")?);
    icons.write(out)
}


/// Write icon as the ico format.
fn write_ico(src_img: &DynamicImage) -> Result<(), std::io::Error> {

    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for size in vec![16, 32, 48, 256] {
        let bytes = src_img
            .resize(size, size, FilterType::Lanczos3)
            .to_rgba8()
            .to_vec();
        let image = ico::IconImage::from_rgba_data(size, size, bytes);
        icon_dir.add_entry(ico::IconDirEntry::encode(&image).unwrap());
    }

    let file = std::fs::File::create("out.ico")?;
    icon_dir.write(file)
}
