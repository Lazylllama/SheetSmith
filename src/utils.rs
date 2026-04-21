use anyhow::{Ok, Result, bail};
use clap::Parser;
use image::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The input directory containing the images to pack
    #[arg(short, long, default_value = "input")]
    pub input_dir: String,

    /// The output file for the packed sprite sheet
    #[arg(short, long, default_value = "output.png")]
    pub output: String,

    /// The format of the output metadata (json or unity)
    // #[arg(short, long, default_value = "unity")]
    // TODO: Implement

    // pub format: String,
    /// The maximum size of the output sprite sheet
    #[arg(short, long, default_value_t = 2048)]
    pub max_size: i32,

    /// Padding between sprites in pixels
    #[arg(short, long, default_value_t = 2)]
    pub padding: i32,

    /// Algorithm to use for packing.
    /// Options: guillotiere
    // TODO: Add maxrect and skyline algorithms maybe maybe
    #[arg(long = "alg", default_value = "guillotiere")]
    pub algorithm: String,

    /// Trim transparent pixels from the edges of images before packing
    /// This can help GREATLY reduce the size of the output sprite sheet and improve packing efficiency.
    #[arg(short, long, default_value_t = false)]
    pub trim_transparent: bool,

    /// Automatically find a good sheet size
    #[arg(short, long, default_value_t = false)]
    pub auto_size: bool,

    /// Debug mode: Print more often to find problematic images
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

/// Bails if anything is wrong with the args, otherwise returns okie dokie
pub fn check_args(args: &Args) -> Result<()> {
    // TODO: Future update
    // let format = args.format.to_ascii_lowercase();
    // if format != "json" && format != "unity" {
    //     bail!(
    //         "Format '{}' is not supported. Use 'json' or 'unity'.",
    //         args.format
    //     );
    // }

    // Only guillotiere is supported for now
    let algorithm = args.algorithm.to_ascii_lowercase();
    if algorithm != "guillotiere" {
        bail!(
            "Algorithm '{}' is not supported. Use 'guillotiere'.",
            args.algorithm
        );
    }

    // File checks
    let input_dir = std::path::Path::new(&args.input_dir);
    if !input_dir.exists() {
        bail!("Input directory '{}' does not exist.", args.input_dir);
    }
    if !input_dir.is_dir() {
        bail!("Input path '{}' is not a directory.", args.input_dir);
    }

    if !args.output.ends_with(".png") && !args.output.ends_with(".json") {
        bail!(
            "Output file '{}' must have a .png or .json extension.",
            args.output
        );
    }

    // Make sure sizes arent stupid
    if args.max_size <= 0 {
        bail!("Max size must be greater than 0.");
    }

    if args.padding < 0 {
        bail!("Padding cannot be negative.");
    }

    if args.padding > args.max_size {
        bail!("Padding cannot be greater than max size.");
    }

    Ok(())
}

// Trim transparency of image
pub fn trim_image(name: &str, image: &RgbaImage, debug: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
            "Trimmed Image: {}, Original Dimensions: {}x{}, Trimmed Dimensions: {}x{}",
            name,
            width,
            height,
            trimmed_image.width(),
            trimmed_image.height()
        );
    }

    return trimmed_image;
}

pub fn find_optimal_size(image_files: Vec<(String, RgbaImage)>, padding: u32) -> i32 {
    println!("----------------------------------");
    println!(
        "!! USING HIGHLY EXPERIMENTAL FEATURE, REMOVE \"-a\" or \"--auto-size\" IF THIS GETS STUCK OR DOESNT WORK !!"
    );

    let mut total_area = 0;

    for (_filename, image) in &image_files {
        total_area += image.width() * image.height();
    }

    total_area += (image_files.len() as u32 - 1) * padding * padding; // Add padding for each image

    let mut new_image_files = image_files.clone();
    while (new_image_files.len() as f32).sqrt().fract() != 0.0 {
        new_image_files.push((new_image_files[0].0.clone(), new_image_files[0].1.clone()));
        total_area += new_image_files[0].1.width() * new_image_files[0].1.height(); // Add fake area
    }

    let sqrt_area = (total_area as f64).sqrt() as i32;
    sqrt_area + (padding * padding) as i32
}

/// Startup Ascii Text
pub fn ascii_text() -> String {
    let mut text = String::from("");
    text.push_str("================================================================\n");

    text.push_str("||   _____            _ _       _____           _ _   _       ||\n");
    text.push_str("||  /  ___|          (_) |     /  ___|         (_) | | |      ||\n");
    text.push_str("||  \\ `--. _ __  _ __ _| |_ ___\\ `--. _ __ ___  _| |_| |__    ||\n");
    text.push_str("||   `--. \\ '_ \\| '__| | __/ _ \\`--. \\ '_ ` _ \\| | __| '_ \\   ||\n");
    text.push_str("||  /\\__/ / |_) | |  | | ||  __/\\__/ / | | | | | | |_| | | |  ||\n");
    text.push_str("||  \\____/| .__/|_|  |_|\\__\\___\\____/|_| |_| |_|_|\\__|_| |_|  ||\n");
    text.push_str("||        | |                                                 ||\n");
    text.push_str("||        |_|                                                 ||\n");

    text.push_str("================================================================");
    return text;
}
