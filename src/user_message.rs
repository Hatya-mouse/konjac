use owo_colors::OwoColorize;
use std::io::{Write, stdin, stdout};

pub enum UserCmd {
    Back,
    Skip,
    Quit,
    Kill,
    Category(String),
    Message(String),
    Retry,
}

pub fn ask_for_message(variant: &str) -> UserCmd {
    print!("{}: ", variant.bold());
    stdout().flush().unwrap();

    parse_command(&get_input(), variant)
}

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_command(input: &str, variant: &str) -> UserCmd {
    match input {
        ":q" => UserCmd::Quit,
        ":q!" => {
            // Confirm quit without saving
            print!(
                "* {}",
                "Are you sure you want to quit without saving? [y/N]: ".bold()
            );
            stdout().flush().unwrap();

            if get_input() == "y" {
                println!("{}", "Aborting without saving...".red().bold());
                UserCmd::Kill
            } else {
                UserCmd::Retry
            }
        }
        ":b" => UserCmd::Back,
        ":s" => {
            println!("{} {}", "Skipped".cyan().bold(), variant);
            UserCmd::Skip
        }
        ":c" => {
            print!("* Enter the new category name: ");
            stdout().flush().unwrap();
            UserCmd::Category(get_input())
        }
        _ => UserCmd::Message(input.to_string()),
    }
}
