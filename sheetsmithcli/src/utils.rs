use colored::{ColoredString, Colorize};

/// Startup Ascii Text
pub fn ascii_text() -> ColoredString {
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
    text.push_str("\n");
    return text.red().bold();
}

/// Debug Prints
pub fn debug_print(message: &str) {
    println!("{} {}", "[DEBUG]".yellow(), message);
}

/// Parse size arg
pub fn parse_size_arg(size: &str) -> Result<(u32, u32), anyhow::Error> {
    let parts: Vec<&str> = size.split('x').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid size format. Expected 'WIDTHxHEIGHT'.");
    }
    let width = parts[0].parse::<u32>()?;
    let height = parts[1].parse::<u32>()?;
    Ok((width, height))
}
