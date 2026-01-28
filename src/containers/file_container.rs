use std::path::PathBuf;
use std::{fs::Metadata, time::SystemTime};

use std::ffi::OsStr;

use crate::containers::file_date_data::FileDateData;
use crate::containers::file_statistics::FileStatistics;

pub struct FileContainer {
    path: PathBuf,
    statistics: FileStatistics,
}

impl FileContainer {
    pub fn new(path: String) -> Result<Self, String> {
        // Just a syntaxic guard against non-existant paths
        let path_metadata: Metadata = std::fs::metadata(&path).map_err(|_| {
            format!(
                "Failed to get the metadata on path {}, likely doesn't exist",
                &path
            )
        })?;

        let modifed_date: SystemTime = path_metadata
            .modified()
            .map_err(|err| format!("Failed to get modify date on path {}; Err {}", &path, err))?;
        let access_date: SystemTime = path_metadata
            .accessed()
            .map_err(|err| format!("Failed to get access date on path {}; Err {}", &path, err))?;

        let pathy_path: PathBuf = path.into();

        let full_file_name: PathBuf = pathy_path.file_name().unwrap().into();

        let file_stem: String = full_file_name
            .file_stem()
            .ok_or_else(|| format!("Path has no file stem: {:?}", pathy_path))?
            .to_string_lossy()
            .to_string();

        let file_extension: String = full_file_name
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
            path: pathy_path,
            statistics: file_statistics,
        })
    }
}
