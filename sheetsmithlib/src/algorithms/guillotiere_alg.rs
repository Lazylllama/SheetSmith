use anyhow::{Result, bail};
use image::RgbaImage;

pub struct GuillotiereArgs {
    pub size: (u32, u32),
    pub padding: i32,
    pub image_files: Vec<(String, RgbaImage)>,
}

pub fn pack_images_guillotiere(args: GuillotiereArgs) -> Result<RgbaImage> {
    let canvas_width = args.size.0 as i64;
    let pad = args.padding as i64;

    let mut placements: Vec<(String, i64, i64)> = Vec::new();
    let mut cursor_x: i64 = 0;
    let mut cursor_y: i64 = 0;
    let mut row_height: i64 = 0;

    for (filename, image) in &args.image_files {
        let w = image.width() as i64 + pad * 2;
        let h = image.height() as i64 + pad * 2;

        if cursor_x + w > canvas_width {
            if cursor_x == 0 {
                bail!(
                    "Image '{}' ({}x{}) is too wide to fit in the canvas (width={}, padding={}).",
                    filename,
                    image.width(),
                    image.height(),
                    args.size.0,
                    args.padding
                );
            }
            cursor_x = 0;
            cursor_y += row_height;
            row_height = 0;
        }

        placements.push((filename.clone(), cursor_x, cursor_y));
        cursor_x += w;
        row_height = row_height.max(h);
    }

    let total_height = (cursor_y + row_height) as u32;
    if total_height > args.size.1 {
        bail!(
            "Images overflow the canvas height ({} > {}). Try increasing max_size or reducing padding.",
            total_height,
            args.size.1
        );
    }

    let mut output_image = RgbaImage::new(args.size.0, total_height);

    for (filename, x, y) in &placements {
        let image = args
            .image_files
            .iter()
            .find(|(name, _)| name == filename)
            .unwrap()
            .1
            .clone();

        image::imageops::overlay(&mut output_image, &image, x + pad, y + pad);
    }

    Ok(output_image)
}
