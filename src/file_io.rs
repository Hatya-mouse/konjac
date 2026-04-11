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

use std::{fmt::Display, path::PathBuf};

pub(super) enum FileValidationError {
    NotFound(String),
    NotAFile(String),
}

impl Display for FileValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileValidationError::NotFound(path) => write!(f, "File not found: {}", path),
            FileValidationError::NotAFile(path) => write!(f, "Not a file: {}", path),
        }
    }
}

pub(super) fn validate_file_path(
    path_str: &str,
    should_exist: bool,
) -> Result<PathBuf, FileValidationError> {
    let path = PathBuf::from(path_str);
    if should_exist && !path.exists() {
        return Err(FileValidationError::NotFound(path_str.to_string()));
    }
    if path.is_dir() {
        return Err(FileValidationError::NotAFile(path_str.to_string()));
    }
    Ok(path)
}
