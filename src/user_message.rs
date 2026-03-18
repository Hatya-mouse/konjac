use owo_colors::OwoColorize;
use std::io::{Write, stdout};

pub enum UserCmd {
    Back,
    Skip,
    Quit,
    Kill,
    Category(String),
    Message(String),
    Retry,
}

pub fn ask_for_message(
    index: usize,
    variant: &str,
    initial_value: &str,
    rl: &mut rustyline::DefaultEditor,
) -> Result<UserCmd, String> {
    println!("[{}] {}", index, variant.bold());
    stdout().flush().unwrap();

    // Get the input
    let input = rl
        .readline_with_initial("> ", (initial_value, ""))
        .map(|s| s.trim().to_string())
        .map_err(|e| e.to_string())?;
    rl.add_history_entry(&input).map_err(|e| e.to_string())?;
    // Parse the input
    parse_command(&input, variant)
}

fn get_input() -> Result<String, rustyline::error::ReadlineError> {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    rl.readline("> ").map(|s| s.trim().to_string())
}

fn parse_command(input: &str, variant: &str) -> Result<UserCmd, String> {
    match input {
        ":q" => Ok(UserCmd::Quit),
        ":q!" => {
            // Confirm quit without saving
            println!(
                "* {}",
                "Are you sure you want to quit without saving? [y/N]"
                    .yellow()
                    .bold()
            );
            stdout().flush().unwrap();

            let input = get_input().map_err(|e| e.to_string())?;
            if input == "y" {
                println!("{}", "Aborting without saving...".red().bold());
                Ok(UserCmd::Kill)
            } else {
                Ok(UserCmd::Retry)
            }
        }
        ":b" => Ok(UserCmd::Back),
        ":s" => {
            println!("{} {}", "Skipped".cyan().bold(), variant);
            Ok(UserCmd::Skip)
        }
        ":c" => {
            println!("* Enter the new category name");
            stdout().flush().unwrap();

            let input = get_input().map_err(|e| e.to_string())?;
            println!(
                "{} the category name to {}",
                "Changed".green().bold(),
                input.blue().bold()
            );
            Ok(UserCmd::Category(input))
        }
        _ => {
            println!("{} \"{}\"", "✓".green(), input);
            Ok(UserCmd::Message(input.to_string()))
        }
    }
}
