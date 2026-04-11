//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::user_message::{UserCmd, ask_for_message};
use owo_colors::OwoColorize;
use std::{collections::HashMap, path::Path};

pub(super) fn process_file(
    file_path: &Path,
    variants_path: &Path,
    category_name: &str,
    omit_existing: bool,
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
    let variants: Vec<&str> = variants_file.lines().filter(|l| !l.is_empty()).collect();

    // Create a new rustyline editor
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    // Iterate over the variants
    let mut i = 0;
    // Used when jumping to a variant with omit_existing set to true
    let mut ignore_omitting = false;
    while i < variants.len() {
        let variant = variants[i];
        let initial_value = current_category
            .get(variant)
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        // If the omit_existing flag is set, skip variants that already exist
        if !ignore_omitting && omit_existing && !initial_value.is_empty() {
            i += 1;
            continue;
        }
        ignore_omitting = false;

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
            UserCmd::Jump(jump_index) => {
                if jump_index < variants.len() {
                    println!(
                        "{} to {}",
                        "Jumped".green().bold(),
                        jump_index.blue().bold()
                    );
                    i = jump_index;
                    ignore_omitting = true;
                } else {
                    println!(
                        "{} Number of variants: {}",
                        "Index out of bounds.".red().bold(),
                        variants.len()
                    );
                }
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
