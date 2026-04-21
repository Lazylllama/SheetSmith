use guillotiere::*;
use image::RgbaImage;

pub struct GuillotiereArgs {
    pub max_size: i32,
    pub padding: i32,
    pub image_files: Vec<(String, RgbaImage)>,
}

pub fn pack_images_guillotiere(args: GuillotiereArgs) -> Result<RgbaImage, anyhow::Error> {
    let mut atlas = AtlasAllocator::new(size2(args.max_size, args.max_size));
    let mut placements: Vec<(String, Allocation)> = Vec::new();
    for (filename, image) in &args.image_files {
        let allocation = atlas
            .allocate(size2(
                image.width() as i32 + args.padding * 2,
                image.height() as i32 + args.padding * 2,
            ))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Failed to allocate space for '{}' in atlas (max_size={}, padding={}). Try reducing the number of images, increasing max_size, or decreasing padding.",
                    filename,
                    args.max_size,
                    args.padding
                )
            })?;
        placements.push((filename.clone(), allocation));
    }

    // Create output image
    let mut output_image = RgbaImage::new(
        atlas.size().to_array()[0] as u32,
        atlas.size().to_array()[1] as u32,
    );

    // Place images onto output image
    for (filename, rect) in &placements {
        let image = args
            .image_files
            .iter()
            .find(|(name, _)| name == filename)
            .unwrap()
            .1
            .clone();

        image::imageops::overlay(
            &mut output_image,
            &image,
            rect.rectangle.min.to_array()[0] as i64 + args.padding as i64,
            rect.rectangle.min.to_array()[1] as i64 + args.padding as i64,
        );
    }

    Ok(output_image)
}
