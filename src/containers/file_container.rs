use std::path::{Path, PathBuf};
use std::{fs::Metadata, time::SystemTime};

use std::ffi::OsStr;

use clap::error;
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

    #[error("Error when attempting to get the parent (path only) of the provided file")]
    PathParentNotAvailable,

    #[error("Error converting the provided path to a string slice")]
    PathToStringError,
}

#[derive(Debug, Error)]
pub enum PathToStringError {
    #[error("Error converting path to string")]
    ToStrError,
}

#[derive(Debug)]
pub struct FileContainer {
    path: PathBuf,
    statistics: FileStatistics,
}

impl FileContainer {
    pub fn full_path_as_string(path: &Path) -> Result<String, PathToStringError> {
        let path_string = path
            .to_str()
            .ok_or(PathToStringError::ToStrError)?
            .to_string();

        Ok(path_string)
    }

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

        let file_path_only: String = path
            .parent()
            .ok_or_else(|| FileContainerInitError::PathParentNotAvailable)?
            .to_str()
            .ok_or_else(|| FileContainerInitError::PathToStringError)?
            .to_string();

        let file_statistics: FileStatistics = FileStatistics::new(
            file_stem,
            path_metadata.len(),
            file_extension,
            FileDateData::new(modifed_date),
            FileDateData::new(access_date),
            file_path_only,
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
