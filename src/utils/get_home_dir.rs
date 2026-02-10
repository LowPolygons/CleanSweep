use std::{env::home_dir, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilePathsError {
    #[error("env::home_dir() found no home directory")]
    HomeDirectoryNotFound,
}

pub fn get_home_directory() -> Result<PathBuf, FilePathsError> {
    home_dir().ok_or(FilePathsError::HomeDirectoryNotFound)
}
