use anyhow::{Ok, Result, bail};
use image::*;

// Trim transparency of image
pub fn trim_image(image: &RgbaImage, debug: bool) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut trimmed_image = image.clone();
    let (width, height) = trimmed_image.dimensions();
    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0;
    let mut max_y = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = trimmed_image.get_pixel(x, y);
            if pixel[3] > 0 {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    if min_x <= max_x && min_y <= max_y {
        trimmed_image = image::imageops::crop_imm(
            &trimmed_image,
            min_x,
            min_y,
            max_x - min_x + 1,
            max_y - min_y + 1,
        )
        .to_image();
    }

    if debug {
        println!(
            "Trimmed Image - Original Dimensions: {}x{}, Trimmed Dimensions: {}x{}",
            width,
            height,
            trimmed_image.width(),
            trimmed_image.height()
        );
    }

    return Ok(trimmed_image);
}

pub fn find_optimal_size(
    image_files: Vec<(String, RgbaImage)>,
    padding: u32,
) -> Result<(u32, u32)> {
    if image_files.is_empty() {
        bail!("No image files provided.");
    }

    let mut new_image_files = image_files.clone();
    let mut total_area = 0 as f64;

    // Keep square-ish layout assumption
    while (new_image_files.len() as f32).sqrt().fract() != 0.0 {
        new_image_files.push((new_image_files[0].0.clone(), new_image_files[0].1.clone()));
    }

    for (_, image) in &new_image_files {
        let height = image.height() + (padding * 2);
        let width = image.width() + (padding * 2);
        total_area += (height * width) as f64;
    }

    let sqrt_area = total_area.sqrt();
    Ok((sqrt_area as u32, sqrt_area as u32))
}
