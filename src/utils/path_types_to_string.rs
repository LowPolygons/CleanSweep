use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathToStringError {
    #[error("Error converting path to string")]
    ToStrError,
}

pub fn path_to_string(path: &Path) -> Result<String, PathToStringError> {
    let path_string = path
        .to_str()
        .ok_or(PathToStringError::ToStrError)?
        .to_string();

    Ok(path_string)
}

pub fn path_buf_to_string(path: &PathBuf) -> Result<String, PathToStringError> {
    let path_string = path
        .to_str()
        .ok_or(PathToStringError::ToStrError)?
        .to_string();

    Ok(path_string)
}
