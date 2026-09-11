use icns::IconFamily;
use image::imageops::FilterType;
use image::{GenericImageView, ImageBuffer, Rgba};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, ErrorKind};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    assert_eq!(args.len(), 1, "arguments must be: <png_file_path> or <dir_path>");

    let path = args.next().unwrap();
    let path = Path::new(&path);

    // gather png files
    let mut png_files: Vec<PathBuf> = Vec::new();
    if path.metadata()?.is_file() {
        if is_png(path) {
            png_files.push(path.to_path_buf());
        }
    } else {
        for entry in fs::read_dir(path)? {
            let entry_path = entry?.path();
            if entry_path.is_file() && is_png(&entry_path) {
                png_files.push(entry_path);
            }
        }
    }
    if png_files.is_empty() {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::NotFound, "pn files not found"),
        ));
    }

    let mut images: Vec<image::RgbaImage> = Vec::new();
    for file_path in &png_files {
        let img = image::open(file_path)?;
        let (width, height) = img.dimensions();
        let max_side = std::cmp::max(width, height);
        let mut square_img = ImageBuffer::from_pixel(max_side, max_side, Rgba([0, 0, 0, 0]));
        let x_offset = (max_side - width) / 2;
        let y_offset = (max_side - height) / 2;
        image::imageops::overlay(&mut square_img, &img, x_offset as i64, y_offset as i64);
        images.push(square_img);
    }

    write_ico(&images)?;
    write_icns(&images)?;

    Ok(())
}

/// Write icon as the ico format (for windows).
fn write_ico(images: &Vec<image::RgbaImage>) -> Result<(), std::io::Error> {

    let target_sizes = [16, 32, 48, 256];
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for &size in &target_sizes {

        // find the image with the closest size (minimum absolute difference in width)
        let closest_img = images.iter()
            .min_by_key(|img| (img.width() as i32 - size as i32).abs()).unwrap();

        let resized_img = if closest_img.width() == size {
            closest_img.clone()
        } else {
            // resize the image to the target size using a high-quality filter
            image::imageops::resize(closest_img, size, size, FilterType::Lanczos3)
        };

        // create an IconImage from the raw RGBA pixels
        let icon_image = ico::IconImage::from_rgba_data(size, size, resized_img.into_raw());

        // add the image entry to the ICO directory
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
    }

    let file = std::fs::File::create("out.ico")?;
    icon_dir.write(file)?;

    Ok(())
}

/// Write icon as the apple icon image format.
fn write_icns(images: &Vec<image::RgbaImage>) -> Result<(), std::io::Error> {

    // define standard target sizes required for macOS .icns files
    let target_sizes = [16, 32, 64, 128, 256, 512, 1024];

    let mut icons = IconFamily::new();

    for &size in &target_sizes {

        // apple icons usually leave about 10% to 20% margin around the content.
        let content_size = match size {
            16 => 14,
            32 => 28,
            64 => 56,
            128 => 114,
            256 => 210,
            512 => 448,
            1024 => 896,
            _ => (size as f32 * 0.85) as u32, // fallback
        };

        // select the best source image (retina-friendly).
        // prefer images that are equal to or larger than the required content_size
        // to prevent upscaling blur, falling back to the closest smaller image if none found.
        let closest_img = images.iter()
            .min_by_key(|img| {
                let diff = img.width() as i32 - content_size as i32;
                if diff >= 0 { diff } else { diff.abs() + 10000 } // penalty for smaller images
            }).unwrap();

        // resize the source image to fit the active content area
        let resized_content = if closest_img.width() == content_size {
            closest_img.clone()
        } else {
            image::imageops::resize(closest_img, content_size, content_size, FilterType::Lanczos3)
        };

        // create an actual icon size image
        let mut padded_img = ImageBuffer::from_pixel(size, size, Rgba([0, 0, 0, 0]));

        // place the resized content into the center of the canvas to form the margins
        let offset = (size - content_size) / 2;
        image::imageops::overlay(&mut padded_img, &resized_content, offset as i64, offset as i64);

        // convert RgbaImage data into raw RGBA bytes and register to icns
        let icns_image = icns::Image::from_data(
            icns::PixelFormat::RGBA, size, size, padded_img.into_raw())?;

        // add the image entry to the icons icon family
        icons.add_icon(&icns_image)?;

    }

    let out = BufWriter::new(File::create("out.icns")?);
    icons.write(out)?;

    Ok(())
}

fn is_png(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.eq_ignore_ascii_case("png"))
        .unwrap_or(false)
}
