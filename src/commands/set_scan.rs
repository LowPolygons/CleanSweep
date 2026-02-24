use std::{collections::HashMap, env::current_dir};

use regex::Regex;

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
    utils::get_home_dir::get_cleansweep_dir,
};

pub fn set_scan(optional_subpath: &String) -> Result<(), String> {
    // Initial path validation
    let mut path = current_dir().map_err(|err| format!("Error getting current dir {}", err))?;
    path = path.join(optional_subpath);

    if !std::fs::exists(&path)
        .map_err(|_| format!("Could not verify if the full directory {:?} exists", &path))?
    {
        return Err(format!("The provided path does not exist"));
    }
    let cleansweep_dir = get_cleansweep_dir()
        .map_err(|e| format!("Failed to load the cleansweep directory - {:?}", e))?;

    // Get the data structures needed for the scan
    let user_settings = json_io::read_file_to_struct::<UserSettings, _>(
        cleansweep_dir.join(CleansweepFilePaths::UserSettings.name()),
    )
    .map_err(|e| format!("Failed to load user settings, does it exist? {}", e))?;
    let user_set_scans: &SetScanOptions = user_settings.get_set_scan_option();

    let filters: Vec<FilterCategory> = vec![
        FilterCategory::Extension(user_set_scans.get_with_extension().clone()),
        FilterCategory::Name(user_set_scans.get_name_contains().clone()),
    ];

    // Perform scan
    let scanned_files: Vec<FileContainer> = FileScanner::scan(path, FileScannerScanMode::Recursive)
        .map_err(|err| format!("Failed to perform scan - {:?}", err))?;

    // Load the SetDetector object
    let found_sets: Vec<SetsReadWriteType> =
        SetScannerSystem::get_found_sets(&scanned_files, &filters).map_err(|e| format!("{e}"))?;

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
    .map_err(|e| format!("Failed to save list of file-sets: {}", e))?;

    Ok(())
}
