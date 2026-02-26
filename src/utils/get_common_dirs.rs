use std::{env::home_dir, path::PathBuf};

use thiserror::Error;

use crate::containers::cleansweep_file_paths::CleansweepFilePaths;

#[derive(Debug, Error)]
pub enum FilePathsError {
    #[error("env::home_dir() found no home directory")]
    HomeDirectoryNotFound,
}

pub fn get_home_directory() -> Result<PathBuf, FilePathsError> {
    home_dir().ok_or(FilePathsError::HomeDirectoryNotFound)
}

pub fn get_cleansweep_dir() -> Result<PathBuf, FilePathsError> {
    let cleansweep_dir: PathBuf = get_home_directory()?;

    Ok(cleansweep_dir.join(CleansweepFilePaths::MainDirectoryName.name()))
}
