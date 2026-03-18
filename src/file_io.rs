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
