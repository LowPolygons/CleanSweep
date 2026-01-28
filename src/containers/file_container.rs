use std::path::{Path, PathBuf};
use std::{fs::Metadata, time::SystemTime};

use std::ffi::OsStr;

use crate::containers::file_date_data::FileDateData;
use crate::containers::file_statistics::FileStatistics;

pub struct FileContainer {
    path: PathBuf,
    statistics: FileStatistics,
}

impl FileContainer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();

        let path_metadata: Metadata = std::fs::metadata(&path).map_err(|err| {
            format!(
                "Failed to get the metadata on path {:?}, likely doesn't exist; Err {}",
                &path, err
            )
        })?;

        let modifed_date: SystemTime = path_metadata
            .modified()
            .map_err(|err| format!("Failed to get modify date on path {:?}; Err {}", &path, err))?;
        let access_date: SystemTime = path_metadata
            .accessed()
            .map_err(|err| format!("Failed to get access date on path {:?}; Err {}", &path, err))?;

        let file_stem: String = path
            .file_stem()
            .ok_or_else(|| format!("Path has no file stem: {:?}", path))?
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
}
