use std::path::Path;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fs::File;
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Empty {}
impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Error)]
pub enum JsonReadError {
    #[error("When reading a path variable, it errored turning result to string")]
    FsReadToStringError,

    #[error("Serde Json errored when interpreting string as struct")]
    SerdeJsonFromStrError,
}

#[derive(Debug, Error)]
pub enum JsonWriteError {
    #[error("Failed when trying to create a File object from provided Path")]
    FileCreateFromPathError,

    #[error("Serde Json errored trying to convert the passed struct into pretty json")]
    SerdeJsonWritePrettyError,
}

pub fn read_file_to_struct<T: DeserializeOwned, P: AsRef<Path>>(
    path: P,
) -> Result<T, JsonReadError> {
    let stringy_json =
        std::fs::read_to_string(path).map_err(|_| JsonReadError::FsReadToStringError)?;
    let structy_value =
        serde_json::from_str(&stringy_json).map_err(|_| JsonReadError::SerdeJsonFromStrError)?;

    Ok(structy_value)
}

pub fn write_json_file_from_struct<T: Serialize, P: AsRef<Path>>(
    object: &T,
    path: P,
) -> Result<(), JsonWriteError> {
    let file = File::create(path).map_err(|_| JsonWriteError::FileCreateFromPathError)?;

    serde_json::to_writer_pretty(file, object)
        .map_err(|_| JsonWriteError::SerdeJsonWritePrettyError)?;

    Ok(())
}
