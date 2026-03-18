use owo_colors::OwoColorize;
use std::fmt::Display;

pub fn print_err(e: impl Display) {
    println!("{}", " ERROR ".on_red().bold());
    eprintln!("{}", e);
}
