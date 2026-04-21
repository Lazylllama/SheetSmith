use anyhow::{Context, Ok, Result};
use clap::Parser;
use image::*;

//* Bigboy functions */
mod algorithms;
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
    println!("{}", utils::ascii_text());

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

    let mut max_size = args.max_size;
    if args.auto_size {
        max_size = utils::find_optimal_size(image_files.clone(), args.padding as u32);
    }

    // Pack images using selected alg
    let mut output_image = RgbaImage::new(0, 0);
    if args.algorithm == "guillotiere" {
        output_image =
            algorithms::pack_images_guillotiere(algorithms::guillotiere_alg::GuillotiereArgs {
                max_size: max_size,
                padding: args.padding,
                image_files,
            })
            .context("Failed to pack images using Guillotiere algorithm. Try another algorithm or add \"--debug\" to your command to find the problematic image.")?;
    }

    // Save output image
    output_image
        .save(&args.output)
        .with_context(|| "Failed to save output image")?;

    return Ok(());
}
