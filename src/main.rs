use std::collections::HashSet;
use icns::IconFamily;
use image::imageops::FilterType;
use image::{GenericImageView, ImageBuffer, Rgba};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {

    let mut args = std::env::args().skip(1);
    assert_eq!(args.len(), 1,
        "arguments must be: <png_file_path> or <dir_path> or <ico/icns_file_path>");

    let path = args.next().unwrap();
    let path = Path::new(&path);

    // if the input is an icon file, extract its images as PNG files
    if path.is_file() {
        if has_extension(path, "ico") {
            return extract_ico(path);
        }
        if has_extension(path, "icns") {
            return extract_icns(path);
        }
    }

    // gather png files
    let mut png_files: Vec<PathBuf> = Vec::new();
    if path.metadata()?.is_file() {
        if has_extension(path, "png") {
            png_files.push(path.to_path_buf());
        }
    } else {
        for entry in fs::read_dir(path)? {
            let entry_path = entry?.path();
            if entry_path.is_file() && has_extension(&entry_path, "png") {
                png_files.push(entry_path);
            }
        }
    }
    if png_files.is_empty() {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::NotFound, "png files not found"),
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

    let target_sizes = [16, 20, 24, 32, 48, 256];
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

/// Extract all images from an ico file and save them as "<width>x<height>.png".
fn extract_ico(path: &Path) -> Result<(), Box<dyn Error>> {
    let icon_dir = ico::IconDir::read(BufReader::new(File::open(path)?))?;
    let mut written: HashSet<(u32, u32)> = HashSet::new();

    for entry in icon_dir.entries() {
        // decode the entry (BMP or PNG encoded) into raw RGBA pixels
        let icon_image = entry.decode()?;
        let (width, height) = (icon_image.width(), icon_image.height());

        // skip entries that have the same size as an already written one
        if !written.insert((width, height)) {
            continue;
        }

        save_rgba_png(width, height, icon_image.rgba_data().to_vec())?;
    }

    if written.is_empty() {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::NotFound, "no images found in the ico file"),
        ));
    }
    Ok(())
}

/// Extract all images from an icns file and save them as "<width>x<height>.png".
fn extract_icns(path: &Path) -> Result<(), Box<dyn Error>> {
    let family = IconFamily::read(BufReader::new(File::open(path)?))?;
    let mut written: HashSet<(u32, u32)> = HashSet::new();

    for icon_type in family.available_icons() {
        // skip entries that cannot be decoded
        let image = match family.get_icon_with_type(icon_type) {
            Ok(image) => image,
            Err(_) => continue,
        };

        // skip standalone mask entries (alpha only, not a real picture)
        if image.pixel_format() == icns::PixelFormat::Alpha {
            continue;
        }

        // normalize to RGBA
        let image = image.convert_to(icns::PixelFormat::RGBA);
        let (width, height) = (image.width(), image.height());

        // skip entries that have the same pixel size (e.g. retina variants)
        if !written.insert((width, height)) {
            continue;
        }

        save_rgba_png(width, height, image.data().to_vec())?;
    }

    if written.is_empty() {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::NotFound, "no images found in the icns file"),
        ));
    }
    Ok(())
}

/// Save raw RGBA pixels as "<width>x<height>.png" in the current directory.
fn save_rgba_png(width: u32, height: u32, rgba: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let img = image::RgbaImage::from_raw(width, height, rgba)
        .ok_or("pixel data size does not match the image dimensions")?;
    let file_name = format!("{}x{}.png", width, height);
    img.save(&file_name)?;
    println!("wrote {}", file_name);
    Ok(())
}

/// Check the file extension case-insensitively.
fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.eq_ignore_ascii_case(expected))
        .unwrap_or(false)
}