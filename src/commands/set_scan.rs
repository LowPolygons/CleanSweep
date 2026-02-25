use std::env::current_dir;

use thiserror::Error;

use crate::{
    containers::{
        cleansweep_file_paths::CleansweepFilePaths,
        file_container::FileContainer,
        sets_read_write_type::SetsReadWriteType,
        user_settings::{SetScanOptions, UserSettings},
    },
    systems::{
        file_scanner::{FileScanner, FileScannerScanMode},
        filter_system::filter_category_info::FilterCategory,
        json_io::{self, write_json_file_from_struct},
        set_scanner_system::SetScannerSystem,
    },
    utils::get_common_dirs::get_cleansweep_dir,
};

#[derive(Debug, Error)]
pub enum SetScanError {
    #[error("Failed to get the users current directory")]
    GetCurrentDirectoryFailure,

    #[error("Could not verify whether the full path exists")]
    VerifyIfPathExistsFailure,

    #[error("The path you have provided relative to your current directory does not exist")]
    ProvidedPathDoesNotExist,

    #[error("Failed to get the cleansweep directory")]
    GetCleansweepDirectoryFailure,

    #[error("Failed to read the user_settings.json file into a UserSettings object")]
    ReadUserSettingsFileToObjectFailure,

    #[error("Failed to scan and format the files found at the provided path")]
    FileScanAndFormatFailure,

    #[error("Failed to create a list of found sets from the list of files")]
    CovertFileListToFoundSetsFailure,

    #[error("Failed to write the sets json file from the sets structure")]
    WriteJsonFileToStruct,
}

pub fn set_scan(optional_subpath: &String, ignore_dirs: &Vec<String>) -> Result<(), SetScanError> {
    // Initial path validation
    let mut path = current_dir().map_err(|_| SetScanError::GetCurrentDirectoryFailure)?;
    path = path.join(optional_subpath);

    if !std::fs::exists(&path).map_err(|_| SetScanError::VerifyIfPathExistsFailure)? {
        return Err(SetScanError::ProvidedPathDoesNotExist);
    }
    let cleansweep_dir =
        get_cleansweep_dir().map_err(|_| SetScanError::GetCleansweepDirectoryFailure)?;

    // Get the data structures needed for the scan
    let user_settings = json_io::read_file_to_struct::<UserSettings, _>(
        cleansweep_dir.join(CleansweepFilePaths::UserSettings.name()),
    )
    .map_err(|_| SetScanError::ReadUserSettingsFileToObjectFailure)?;

    let user_set_scans: &SetScanOptions = user_settings.get_set_scan_option();

    let filters: Vec<FilterCategory> = vec![
        FilterCategory::Extension(user_set_scans.get_with_extension().clone()),
        FilterCategory::Name(user_set_scans.get_name_contains().clone()),
    ];

    // Perform scan
    let scanned_files: Vec<FileContainer> =
        FileScanner::scan(path, FileScannerScanMode::Recursive, ignore_dirs)
            .map_err(|_| SetScanError::FileScanAndFormatFailure)?;

    // Load the SetDetector object
    let found_sets: Vec<SetsReadWriteType> =
        SetScannerSystem::get_found_sets(&scanned_files, &filters)
            .map_err(|_| SetScanError::CovertFileListToFoundSetsFailure)?;

    for set in &found_sets {
        println!("Set:");
        for file in &set.files {
            println!("- {}", file);
        }
    }

    // Save the found sets object
    write_json_file_from_struct(
        &found_sets,
        cleansweep_dir.join(CleansweepFilePaths::FoundSets.name()),
    )
    .map_err(|_| SetScanError::WriteJsonFileToStruct)?;

    Ok(())
}
