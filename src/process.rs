use crate::user_message::{UserCmd, ask_for_message};
use owo_colors::OwoColorize;
use std::{collections::HashMap, path::Path};

pub(super) fn process_file(
    file_path: &Path,
    variants_path: &Path,
    category_name: &str,
) -> Result<(), String> {
    // Get the file contents
    let file_contents = std::fs::read_to_string(file_path).unwrap_or_default();
    let variants_file = std::fs::read_to_string(variants_path).unwrap_or_default();

    // Parse the file contents and variants file
    let mut contents: HashMap<String, toml::Value> =
        toml::from_str(&file_contents).map_err(|e| e.to_string())?;

    // Get the category table or create a new one if it doesn't exist
    let mut current_category_name = category_name.to_string();
    let mut current_category = get_or_create_category(&contents, category_name);

    // Get the variants to process
    let variants: Vec<&str> = variants_file.lines().collect();

    // Create a new rustyline editor
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    // Iterate over the variants
    let mut i = 0;
    while i < variants.len() {
        let variant = variants[i];
        let initial_value = current_category
            .get(variant)
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        // Add the variant to the parsed file
        match ask_for_message(i, variant, initial_value, &mut rl)? {
            UserCmd::Back => {
                if i > 0 {
                    i -= 1;
                } else {
                    println!("Already at the first variant");
                }
            }
            UserCmd::Skip => {
                i += 1;
            }
            UserCmd::Quit => {
                println!();
                break;
            }
            UserCmd::Kill => {
                return Ok(());
            }
            UserCmd::Category(new_category_name) => {
                contents.insert(
                    current_category_name.to_string(),
                    toml::Value::Table(current_category),
                );
                current_category_name = new_category_name.to_string();
                current_category = get_or_create_category(&contents, &new_category_name);
            }
            UserCmd::Message(message) => {
                current_category.insert(variant.to_string(), toml::Value::String(message));
                i += 1;
            }
            UserCmd::Retry => {}
        }

        println!();
    }

    // Print the result
    println!(
        "{} {}/{}",
        "Completed".green().bold(),
        current_category.len(),
        variants.len()
    );

    // Put back the updated category table into the parsed file
    contents.insert(
        current_category_name.to_string(),
        toml::Value::Table(current_category),
    );

    // Save the updated file
    let updated_file = toml::to_string(&contents).map_err(|e| e.to_string())?;
    std::fs::write(file_path, updated_file).map_err(|e| e.to_string())?;

    Ok(())
}

fn get_or_create_category(
    contents: &HashMap<String, toml::Value>,
    category: &str,
) -> toml::map::Map<String, toml::Value> {
    contents
        .get(category)
        .and_then(|v| v.as_table())
        .cloned()
        .unwrap_or_default()
}
