use anyhow::{Ok, Result, bail};
use clap::Parser;
use sheetsmithlib::parse_size_arg;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The input directory containing the images to pack
    #[arg(short, long, default_value = "input")]
    pub input_dir: String,

    /// The output file for the packed sprite sheet
    #[arg(short, long, default_value = "output.png")]
    pub output: String,

    /// Disable color in prints
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    /// The size of the output sprite sheet
    #[arg(short, long, default_value = "1080x1080")]
    pub size: String,

    /// Padding between sprites in pixels
    #[arg(short, long, default_value_t = 0)]
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
    parse_size_arg(&args.size)?;

    if args.padding < 0 {
        bail!("Padding cannot be negative.");
    }

    if args.padding > parse_size_arg(&args.size).unwrap_or((0, 0)).0 as i32 {
        bail!("Padding cannot be greater than max size.");
    }

    Ok(())
}
