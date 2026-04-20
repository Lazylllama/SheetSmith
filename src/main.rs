use anyhow::{Context, Ok, Result};
use clap::Parser;
use guillotiere::*;
use image::*;

//* Bigboy functions */
mod utils;
fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    } else {
        println!("----------------------------------");
        println!("Packing completed successfully.");
    }
}

fn run() -> Result<()> {
    let args = utils::Args::parse();

    // Validate args
    utils::check_args(&args).context("Argument validation failed")?;

    // Set input dir again
    let input_dir = std::path::Path::new(&args.input_dir);

    // Read image files from input directory
    let mut image_count = 0;
    let mut image_files = Vec::<(String, RgbaImage)>::new();
    input_dir
        .read_dir()
        .expect("Failed to read input directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file()
                && entry.path().extension().map_or(false, |ext| {
                    ext.eq_ignore_ascii_case("png") || ext.eq_ignore_ascii_case("jpg")
                })
        })
        .for_each(|entry| {
            if args.debug {
                println!("Found image file: {}", entry.path().display());
            }

            image_files.push((
                entry.file_name().to_string_lossy().to_string(),
                image::open(entry.path())
                    .expect("Failed to open image")
                    .to_rgba8(),
            ));

            image_count += 1;
        });

    if args.debug {
        println!("----------------------------------");
    }

    // Check if any images were found
    if image_count == 0 {
        eprintln!(
            "No image files found in input directory '{}'.",
            args.input_dir
        );
        std::process::exit(1);
    } else {
        println!("Total image files found: {}", image_count);
    }

    // Remove transparent pixels from images and print dimensions if requested
    if args.trim_transparent {
        println!("Trimming transparent pixels from images...");
        for (name, image) in image_files.iter_mut() {
            *image = utils::trim_image(name, image, args.debug);
        }
        println!("----------------------------------");
    }

    // Print dimensions of images if debug mode is on
    if args.debug {
        for (filename, image) in &image_files {
            println!(
                "Image: {}, Dimensions: {}x{}",
                filename,
                image.width(),
                image.height()
            );
        }
        println!("----------------------------------");
    }

    // Pack images using Guillotiere
    let mut atlas = AtlasAllocator::new(size2(args.max_size, args.max_size));
    let mut placements: Vec<(String, Allocation)> = Vec::new();
    for (filename, image) in &image_files {
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
        let image = image_files
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

    // Save output image
    output_image
        .save(&args.output)
        .with_context(|| "Failed to save output image")?;

    return Ok(());
}
