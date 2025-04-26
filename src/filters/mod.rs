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

    // Run command to convert to usable format
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

// Applies gresycale to a given bmp
pub fn apply_greyscale(path: String) 
{
    // Check that image can be opened
    let img = bmp::open(&path).expect("Failed to open image");

    // Open the image
    let mut buffer = Image::new(img.get_width(), img.get_height());

    // Loop through image coordinates, and convert it to greyscale
    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        let avg_val: u8 = ((pixel.r as u16 + pixel.g as u16 + pixel.b as u16 ) / 3).try_into().unwrap();
        buffer.set_pixel(x, y, Pixel::new(avg_val, avg_val, avg_val));
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}

// Applies negative filter
pub fn apply_negative(path: String) 
{
    // Check that image can be opened
    let img = bmp::open(&path).expect("Failed to open image");

    // Open the image
    let mut buffer = Image::new(img.get_width(), img.get_height());

    // Loop through image coordinates, and convert it to its negative
    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        buffer.set_pixel(x, y, Pixel::new(u8::MAX - pixel.r, u8::MAX - pixel.g, u8::MAX - pixel.b));
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}

// Applies sepia filter (thanks for the numbers, https://github.com/abhijitnathwani/image-processing/blob/master/image_colortosepia.c)
pub fn apply_sepia(path:String)
{
    // Check that image can be opened
    let img = bmp::open(&path).expect("Failed to open image");

    // Open the image
    let mut buffer = Image::new(img.get_width(), img.get_height());

    // Loop through image coordinates, and convert it to its negative
    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        let r = (pixel.r as f32 * 0.393) + (pixel.g as f32 * 0.769) + (pixel.b as f32 * 0.189);
        let g = (pixel.r as f32 * 0.349) + (pixel.g as f32 * 0.686) + (pixel.b as f32 * 0.168);
        let b = (pixel.r as f32 * 0.272) + (pixel.g as f32 * 0.534) + (pixel.b as f32 * 0.131);
        
        // Clamp the values to 255 and convert to u8
        let r = r.min(255.0) as u8;
        let g = g.min(255.0) as u8;
        let b = b.min(255.0) as u8;
        buffer.set_pixel(x, y, Pixel::new(r as u8, g as u8, b as u8));
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}

// Flips the image horizontaally
pub fn flip_x(path: String)
{
    // Check that image can be opened
    let img = bmp::open(&path).expect("Failed to open image");

    // Open the image
    let mut buffer = Image::new(img.get_width(), img.get_height());

    let width = img.get_width() as u32;

    // Loop through image coordinates
    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        buffer.set_pixel((width - 1 - x) as u32, y, Pixel::new(pixel.r, pixel.g, pixel.b));
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}

// Flips it vertically
pub fn flip_y(path: String)
{
    // Check that image can be opened
    let img = bmp::open(&path).expect("Failed to open image");

    // Open the image
    let mut buffer = Image::new(img.get_width(), img.get_height());

    let height = img.get_height() as u32;

    // Loop through image coordinates
    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        buffer.set_pixel(x, (height - 1 - y) as u32, Pixel::new(pixel.r, pixel.g, pixel.b));
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}

// 3x3 box blur
pub fn apply_blur(path: String) 
{
    let img = bmp::open(&path).expect("Failed to open image");

    let mut buffer = Image::new(img.get_width(), img.get_height());

    // Loop through every pixel in the source image
    for y in 0..img.get_height() 
    {
        for x in 0..img.get_width() 
        {
            // Temp variables to store rgb values
            let mut r: i32 = 0;
            let mut g: i32 = 0;
            let mut b: i32 = 0;

            // Temp variable to store the number of pixels that are averaged (divisor)
            let mut count: i32 = 0;

            // Loop through the blur bounds (3x3 pixel)
            for i in -1..=1 
            {
                for j in -1..=1 
                {
                    // Get the color of the pixel at the surrounding coordinates
                    let nx = (x as isize + i) as usize;
                    let ny = (y as isize + j) as usize;

                    // Skip if the neighbor is out of bounds
                    if nx < img.get_width() as usize && ny < img.get_height() as usize 
                    {
                        // Get the pixel at the blur coords
                        let pixel = img.get_pixel(nx as u32, ny as u32);

                        // Add its values to rgb variables
                        r += pixel.r as i32;
                        g += pixel.g as i32;
                        b += pixel.b as i32;

                        // Increment count
                        count += 1;
                    }
                }
            }

            // Average the colors by the number of valid neighbors
            r /= count;
            g /= count;
            b /= count;

            // Set the new pixel value in the buffer
            buffer.set_pixel(x, y, Pixel::new(r as u8, g as u8, b as u8));
        }
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}

// Use sobel operator to perform edge detection
pub fn edge_detect(path: String)
{
    let img = bmp::open(&path).expect("Failed to open image");

    let mut buffer = Image::new(img.get_width(), img.get_height());

    // Sobel operators
    let vmask: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let hmask: [[i32; 3]; 3] = [[1, 2, 1], [0, 0, 0], [-1, -2, -1]];

    // Loop through pixels
    for y in 0..img.get_height()
    {
        for x in 0..img.get_width()
        {
            // Vars to keep track of colors
            let mut yr: i32 = 0;
            let mut xr: i32 = 0;
            let mut yg: i32 = 0;
            let mut xg: i32 = 0;
            let mut yb: i32 = 0;
            let mut xb: i32 = 0;

            // Loop through neighboring pixels
            for i in -1..=1
            {
                for j in -1..=1
                {
                    // Get the coordinate of neighboring pixel
                    let nx = (x as isize + i) as usize;
                    let ny = (y as isize + j) as usize;

                    // Process pixel if in bounds
                    if nx < img.get_width() as usize && ny < img.get_height() as usize
                    {
                        // Get the pixel at the coords
                        let pixel = img.get_pixel(nx as u32, ny as u32);

                        // Get the red for the pixels
                        xr += hmask[(i + 1) as usize][(j + 1) as usize] * pixel.r as i32;
                        yr += vmask[(i + 1) as usize][(j + 1) as usize] * pixel.r as i32;

                        // Get the green for the pixels
                        xg += hmask[(i + 1) as usize][(j + 1) as usize] * pixel.g as i32;
                        yg += vmask[(i + 1) as usize][(j + 1) as usize] * pixel.g as i32;

                        // Get the blue for the pixels
                        xb += hmask[(i + 1) as usize][(j + 1) as usize] * pixel.b as i32;
                        yb += vmask[(i + 1) as usize][(j + 1) as usize] * pixel.b as i32;
                    }
                }
            }

            // Total RGB value = sum of squares
            let r = ((xr * xr + yr * yr) as f64).sqrt().round().clamp(0.0, 255.0) as u8;
            let g = ((xg * xg + yg * yg) as f64).sqrt().round().clamp(0.0, 255.0) as u8;
            let b = ((xb * xb + yb * yb) as f64).sqrt().round().clamp(0.0, 255.0) as u8;

            // Set the new pixel value in the buffer
            buffer.set_pixel(x, y, Pixel::new(r, g, b));
        }
    }

    // Save img
    let _ = buffer.save("./uploads/filtered.bmp");
}

// Posterize (reduce colors to 6 bits per channel)
pub fn apply_posterize(path: String)
{
    // Check that image can be opened
    let img = bmp::open(&path).expect("Failed to open image");

    // Open the image
    let mut buffer = Image::new(img.get_width(), img.get_height());

    let levels = 6;

    // Loop through image coordinates, and convert it to its negative
    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        let r = posterize_channel(pixel.r, levels);
        let g = posterize_channel(pixel.g, levels);
        let b = posterize_channel(pixel.b, levels);
        buffer.set_pixel(x, y, Pixel::new(r, g, b));
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}

// Helper function
fn posterize_channel(value: u8, levels: u8) -> u8
{
    let scale = 255.0 / (levels as f32 - 1.0);
    let level = ((value as f32 / 255.0) * (levels as f32 - 1.0)).round();
    return (level * scale).round() as u8;
}

// I messed up pixelate and it made this. Pretty cool! Also reversable.
pub fn apply_popify(path: String)
{
    // Check that image can be opened
    let img = bmp::open(&path).expect("Failed to open image");

    // Open the image
    let mut buffer = Image::new(img.get_width(), img.get_height());

    let block_size = 10;

    let height = img.get_height();
    let width = img.get_width();

    // Loop through image coordinates, and convert it to its negative
    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        buffer.set_pixel(x, y, Pixel::new(pixel.r, u8::MAX - pixel.g, u8::MAX - pixel.b));
    }

    for y in (0..height).step_by(block_size)
    {
        for x in (0..width).step_by(block_size)
        {
            let pixel = img.get_pixel(x, y);

            // Loop inside block
            for dy in 0..block_size
            {
                for dx in 0..block_size
                {
                    if (x + dx as u32) < width && (y + dy as u32) < height
                    {
                        buffer.set_pixel(x, y, pixel);
                    }
                }
            }
        }
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}

// Pixelizes to 10x10 grid
pub fn apply_pixelize(path: String)
{
    // Check that image can be opened
    let img = bmp::open(&path).expect("Failed to open image");

    // Open the image
    let mut buffer = Image::new(img.get_width(), img.get_height());

    let block_size = 10;

    let height = img.get_height();
    let width = img.get_width();

    // Loop through image coordinates, and convert it to its negative
    for (x, y) in img.coordinates()
    {
        let pixel: Pixel = img.get_pixel(x, y);
        buffer.set_pixel(x, y, Pixel::new(pixel.r, u8::MAX - pixel.g, u8::MAX - pixel.b));
    }

    for y in (0..height).step_by(block_size)
    {
        for x in (0..width).step_by(block_size)
        {
            let pixel = img.get_pixel(x, y);

            // Loop inside block
            for dy in 0..block_size
            {
                for dx in 0..block_size
                {
                    if (x + dx as u32) < width && (y + dy as u32) < height
                    {
                        buffer.set_pixel(x + dx as u32, y + dy as u32, pixel);
                    }
                }
            }
        }
    }

    let _ = buffer.save("./uploads/filtered.bmp");
}