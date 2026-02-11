use std::path::{Path, PathBuf};
use std::{fs::Metadata, time::SystemTime};

use std::ffi::OsStr;

use thiserror::Error;

use crate::containers::file_date_data::FileDateData;
use crate::containers::file_statistics::FileStatistics;

#[derive(Debug, Error)]
pub enum FileContainerInitError {
    #[error("Error when attempting to read the metadata of a file")]
    MetadataNotFound,

    #[error("Error when attempting to access modify date from file metadata")]
    ModifyDateNotAvailable,

    #[error("Error when attempting to access the access date from the file metadata")]
    AccessDateNotAvailable,

    #[error("Error when attempting to read the stem of the provided file")]
    FileStemNotAvailable,
}

#[derive(Debug)]
pub struct FileContainer {
    path: PathBuf,
    statistics: FileStatistics,
}

impl FileContainer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FileContainerInitError> {
        let path = path.as_ref().to_path_buf();

        let path_metadata: Metadata =
            std::fs::metadata(&path).map_err(|_| FileContainerInitError::MetadataNotFound)?;

        let modifed_date: SystemTime = path_metadata
            .modified()
            .map_err(|_| FileContainerInitError::ModifyDateNotAvailable)?;
        let access_date: SystemTime = path_metadata
            .accessed()
            .map_err(|_| FileContainerInitError::AccessDateNotAvailable)?;

        let file_stem: String = path
            .file_stem()
            .ok_or_else(|| FileContainerInitError::FileStemNotAvailable)?
            .to_string_lossy()
            .to_string();

        let file_extension: String = path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .to_string();

        let file_statistics: FileStatistics = FileStatistics::new(
            file_stem,
            path_metadata.len(),
            file_extension,
            FileDateData::new(modifed_date),
            FileDateData::new(access_date),
        );

        Ok(Self {
            path: path,
            statistics: file_statistics,
        })
    }
    pub fn get_statistics(&self) -> &FileStatistics {
        &self.statistics
    }
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}
