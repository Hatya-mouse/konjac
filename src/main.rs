mod fancy_print;
mod file_io;
mod process;
mod user_message;

use crate::{fancy_print::print_err, file_io::validate_file_path};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// A file to perform localization.
    path: String,

    /// A variant text file to load variants from.
    #[arg(short, long, value_name = "FILE")]
    variants: String,

    /// A string to use as the message key.
    #[arg(long, value_name = "KEY", default_value = "message")]
    key: String,

    /// A string to use as the category name.
    /// Defaults to the variant file name.
    #[arg(long, value_name = "CATEGORY")]
    category: Option<String>,
}

fn main() {
    let cli = Cli::parse();

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
    if let Err(err) = process::process_file(&file_path, &variants_path, &category) {
        print_err(err);
    }
}
