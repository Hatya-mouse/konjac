mod fancy_print;
mod file_io;
mod process;
mod user_message;

use crate::{fancy_print::print_err, file_io::validate_file_path};
use clap::Parser;
use owo_colors::OwoColorize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// A file to perform localization.
    path: String,

    /// A variant text file to load variants from.
    #[arg(long, short, value_name = "FILE")]
    variants: String,

    /// A string to use as the category name.
    /// Defaults to the variant file name.
    #[arg(long, value_name = "CATEGORY")]
    category: Option<String>,

    /// Whether to omit the existing variants.
    #[arg(long, short, default_value = "false")]
    omit_existing: bool,
}

fn main() {
    let cli = Cli::parse();
    println!("{} v{}", " Konjac ".on_black().bold(), VERSION);

    // Get the file path from the CLI argument
    let file_path = match validate_file_path(&cli.path, false) {
        Ok(path) => path,
        Err(err) => {
            print_err(format!("File path: {}", err));
            return;
        }
    };
    let variants_path = match validate_file_path(&cli.variants, true) {
        Ok(path) => path,
        Err(err) => {
            print_err(format!("Variants path: {}", err));
            return;
        }
    };

    // Get the category name from the CLI argument, or use the variant file name as a fallback
    let category = cli.category.unwrap_or_else(|| {
        variants_path
            .file_prefix()
            .and_then(|p| p.to_str())
            .map(|p| p.to_string())
            .unwrap_or_default()
    });

    // Start processing!
    println!(":b Back  :s Skip  :c Change category  :q Save & Quit  :q! Quit without Saving");
    println!();
    if let Err(err) = process::process_file(&file_path, &variants_path, &category) {
        print_err(err);
    }
}
