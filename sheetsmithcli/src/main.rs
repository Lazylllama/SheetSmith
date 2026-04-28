mod cli_args;
mod utils;

use anyhow::{Ok, Result, bail};
use clap::Parser;
use colored::Colorize;
use image::*;
use sheetsmithlib::{algorithms, parse_size_arg};

fn main() {
    if let Err(err) = run() {
        eprintln!("{} {}", "error:".red(), err);
    } else {
        println!("{}", "----------------------------------".green());
        println!("{}", "Packing completed successfully.".green());
    }
}

fn run() -> Result<()> {
    // Get args
    let args = cli_args::Args::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

    // Banner
    println!("{}", utils::ascii_text());

    // Validate args
    cli_args::check_args(&args)?;

    let (mut image_files, image_count) =
        sheetsmithlib::utils::walk_input_directory(&args.input_dir, args.debug)?;

    if args.debug {
        println!("{}", "----------------------------------".green());
    }

    // Check if any images were found
    if image_count == 0 {
        bail!(
            "No image files found in input directory '{}'.",
            args.input_dir
        );
    } else {
        println!("{} {}", "Total image files found:".green(), image_count);
    }

    // Remove transparent pixels from images and print dimensions if requested
    if args.trim_transparent {
        println!("{}", "Trimming transparent pixels from images...".green());
        for (_, image) in image_files.iter_mut() {
            *image = sheetsmithlib::trim_image(image, args.debug)?;
        }
        println!("{}", "----------------------------------".green());
    }

    // Print dimensions of images if debug mode is on
    if args.debug {
        for (filename, image) in &image_files {
            utils::debug_print(&format!(
                "Image: {}, Dimensions: {}x{}",
                filename,
                image.width(),
                image.height()
            ));
        }
    }

    let mut size = parse_size_arg(&args.size)?;
    if args.auto_size {
        println!("{}", "----------------------------------".green());
        println!(
        "{}",
        "!! USING HIGHLY EXPERIMENTAL FEATURE, REMOVE \"-a\" or \"--auto-size\" IF THIS GETS STUCK OR DOESNT WORK".red().bold()
    );
        println!(
            "{}",
            "!! Changing the padding might fix allocation issues when using auto size."
                .red()
                .bold()
        );
        size = sheetsmithlib::find_optimal_size(image_files.clone(), args.padding as u32)?;
        println!(
            "{} {}{}",
            "!! Total Area:".bold().red(),
            size.0 * size.1,
            "px".bold().red()
        );
    }

    // Pack images using selected alg
    let mut output_image = RgbaImage::new(0, 0);
    if args.algorithm == "guillotiere" {
        output_image =
            sheetsmithlib::pack_images_guillotiere(algorithms::guillotiere_alg::GuillotiereArgs {
                size,
                padding: args.padding,
                image_files,
            })?;
    }

    // Save output image
    output_image.save(&args.output)?;

    return Ok(());
}
