use bmp::{Image, Pixel};
use std::process::Command;
use std::path::Path;
use std::fs;

/// Converts an uploaded image into a modifiable BMP using ImageMagick
pub fn convert_to_bitmap<P: AsRef<Path>>(input_path: P) {
    let path_str = input_path.as_ref().to_string_lossy().to_string();

    // Ensure parent dir exists
    if let Some(parent) = input_path.as_ref().parent() {
        if !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }

    let status = Command::new("convert")
        .args(&[
            path_str.clone(),
            "-depth".to_string(),
            "24".to_string(),
            "-colors".to_string(),
            "256".to_string(),
            "-compress".to_string(),
            "none".to_string(),
            path_str,
        ])
        .status()
        .expect("Failed to execute convert");

    if !status.success() {
        panic!("ImageMagick convert failed with status: {:?}", status);
    }
}

// Applies gresycal to a given bmp
pub fn apply_greyscale(path: String) 
{
    let img = bmp::open(&path).expect("Failed to open image");

    let mut buffer = Image::new(img.get_width(), img.get_height());

    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        let avg_val: u8 = ((pixel.r as u16 + pixel.g as u16 + pixel.b as u16 ) / 3).try_into().unwrap();
        buffer.set_pixel(x, y, Pixel::new(avg_val, avg_val, avg_val));
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}