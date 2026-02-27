use thiserror::Error;

use crate::{
    containers::{
        cleansweep_file_paths::CleansweepFilePaths,
        file_container::FileContainer,
        file_date_data::{FileDateData, days_since_now_to_system_time},
        user_settings::UserSettings,
    },
    filter_codes::filter_codes::FilterCodes,
    systems::{
        file_scanner::{FileScanner, FileScannerScanMode},
        filter_system::{
            directory_contains_filter::DirectoryContainsFilter,
            extension_filter::ExtensionFilter,
            filter_category_info::{FilterCategory, FilterForCategory},
            filter_system::FilterSystem,
            last_accessed_filter::LastAccessedFilter,
            last_modified_filter::LastModifiedFilter,
            name_contains_filter::NameContainsFilter,
            name_filter::NameFilter,
            name_starts_with_filter::NameStartsWithFilter,
            size_filter::SizeFilter,
        },
        json_io::{self, write_json_file_from_struct},
    },
    utils::{
        create_defaults::get_default_filter_category_list, file_to_string_vec,
        get_common_dirs::get_cleansweep_dir, path_types_to_string::path_to_string,
    },
};
use std::env::current_dir;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Failure getting the users current directory")]
    GetCurrentDirectoryFailure,

    #[error("Could not verify whether the provided directory exists or not")]
    VerifyIfDirectoryExistsFailure,

    #[error("Failure trying to get the cleansweep directory")]
    GetCleansweepDirectoryFailure,

    #[error("Failure trying to scan the provided path and format the files")]
    FileScanAndFormatFailure,

    #[error("Failure attempting to turn path into a String")]
    PathToStringFailure,

    #[error("Failure trying to read the user_settings.json into internal structure")]
    ReadUserSettingsFileToStructFailure,

    #[error("Failure trying to turn the file containing a list of filters into a string vector")]
    ReadListOfFiltersToStringVectorFailure,

    #[error("Failure trying to turn the filters as strings into Filter objects")]
    ConvertStringyFiltersToFilterObjectsFailure,

    #[error("Failed to sort the scanned files into the Keep/Delete lists")]
    SortScannedFilesIntoListsFailure,

    #[error("Failed to write the list into its corresponding json file")]
    WriteJsonFileFromStructFailure,

    #[error("The path you have provided relative to your current directory does not exist")]
    ProvidedPathDoesNotExist,
}

pub fn scan(
    optional_subpath: &String,
    use_custom_filters: &bool,
    no_filter: &bool,
    ignore_dirs: &Vec<String>,
) -> Result<(), ScanError> {
    // Initial path validation
    let mut path = current_dir().map_err(|_| ScanError::GetCurrentDirectoryFailure)?;
    path = path.join(optional_subpath);

    if !std::fs::exists(&path).map_err(|_| ScanError::VerifyIfDirectoryExistsFailure)? {
        return Err(ScanError::ProvidedPathDoesNotExist);
    }
    let cleansweep_dir =
        get_cleansweep_dir().map_err(|_| ScanError::GetCleansweepDirectoryFailure)?;

    // Perform scan
    let scanned_files: Vec<FileContainer> =
        FileScanner::scan(path, FileScannerScanMode::Recursive, ignore_dirs)
            .map_err(|_| ScanError::FileScanAndFormatFailure)?;

    // Sort, get the paths as strings and save
    let mut to_keep: Vec<String> = Vec::new();
    let mut to_delete: Vec<String> = Vec::new();

    if *no_filter {
        println!("Scan command run with no-filter, all files being added to the keep list");

        for file_container in &scanned_files {
            to_keep.push(
                path_to_string(file_container.get_path())
                    .map_err(|_| ScanError::PathToStringFailure)?,
            );
        }
    } else {
        // Get the data structures needed for the scan
        let user_settings: UserSettings = json_io::read_file_to_struct(
            cleansweep_dir.join(CleansweepFilePaths::UserSettings.name()),
        )
        .map_err(|_| ScanError::ReadUserSettingsFileToStructFailure)?;

        let list_of_filters_as_strings: Vec<String> = if *use_custom_filters {
            file_to_string_vec::file_to_string_vec(
                cleansweep_dir.join(CleansweepFilePaths::FilterComponentList.name()),
            )
            .map_err(|_| ScanError::ReadListOfFiltersToStringVectorFailure)?
        } else {
            get_default_filter_category_list()
        };
        let filters_to_use: Vec<Box<dyn FilterForCategory>> =
            stringy_filters_to_filter_objects(&user_settings, &list_of_filters_as_strings)
                .map_err(|_| ScanError::ConvertStringyFiltersToFilterObjectsFailure)?;

        // Build the filter object
        let filter_system: FilterSystem = FilterSystem::new(filters_to_use);

        sort_files_into_provided_lists(
            &scanned_files,
            &filter_system,
            &mut to_keep,
            &mut to_delete,
        )
        .map_err(|_| ScanError::SortScannedFilesIntoListsFailure)?;
    }

    write_json_file_from_struct(
        &to_keep,
        cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()),
    )
    .map_err(|_| ScanError::WriteJsonFileFromStructFailure)?;
    write_json_file_from_struct(
        &to_delete,
        cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()),
    )
    .map_err(|_| ScanError::WriteJsonFileFromStructFailure)?;

    Ok(())
}

fn stringy_filters_to_filter_objects(
    user_settings: &UserSettings,
    list_of_filters_as_strings: &Vec<String>,
) -> Result<Vec<Box<dyn FilterForCategory>>, String> {
    // Turn the filters into filter objects
    let mut filters_to_use: Vec<Box<dyn FilterForCategory>> = Vec::new();

    for item in list_of_filters_as_strings {
        if let Some(filter_obj) = FilterCategory::match_string_to_category(&item) {
            println!(
                "Loaded object {}, reasoning: {}",
                filter_obj.get_choice(),
                filter_obj.get_reasoning()
            );

            let mut filter: Box<dyn FilterForCategory>;
            let keep_filter_item: FilterCategory;
            let delete_filter_item: FilterCategory;

            match &filter_obj.get_filter() {
                FilterCategory::Name(_) => {
                    filter = Box::new(NameFilter::new());
                    keep_filter_item =
                        FilterCategory::Name(user_settings.get_to_keep_list().get_name().clone());
                    delete_filter_item =
                        FilterCategory::Name(user_settings.get_to_delete_list().get_name().clone());
                }
                FilterCategory::NameContains(_) => {
                    filter = Box::new(NameContainsFilter::new());
                    keep_filter_item = FilterCategory::NameContains(
                        user_settings
                            .get_to_keep_list()
                            .get_name_starts_with()
                            .clone(),
                    );
                    delete_filter_item = FilterCategory::NameContains(
                        user_settings
                            .get_to_delete_list()
                            .get_name_starts_with()
                            .clone(),
                    );
                }
                FilterCategory::NameStartsWith(_) => {
                    filter = Box::new(NameStartsWithFilter::new());
                    keep_filter_item = FilterCategory::NameStartsWith(
                        user_settings.get_to_keep_list().get_name().clone(),
                    );
                    delete_filter_item = FilterCategory::NameStartsWith(
                        user_settings.get_to_delete_list().get_name().clone(),
                    );
                }
                FilterCategory::DirectoryContains(_) => {
                    filter = Box::new(DirectoryContainsFilter::new());
                    keep_filter_item = FilterCategory::DirectoryContains(
                        user_settings.get_to_keep_list().get_directory().clone(),
                    );
                    delete_filter_item = FilterCategory::DirectoryContains(
                        user_settings.get_to_delete_list().get_directory().clone(),
                    );
                }
                FilterCategory::Size(_) => {
                    filter = Box::new(SizeFilter::new());
                    keep_filter_item = FilterCategory::Size(
                        user_settings
                            .get_to_keep_list()
                            .get_larger_than_size()
                            .clone(),
                    );
                    delete_filter_item = FilterCategory::Size(
                        user_settings
                            .get_to_delete_list()
                            .get_larger_than_size()
                            .clone(),
                    );
                }
                FilterCategory::Extension(_) => {
                    filter = Box::new(ExtensionFilter::new());
                    keep_filter_item = FilterCategory::Extension(
                        user_settings.get_to_keep_list().get_extensions().clone(),
                    );
                    delete_filter_item = FilterCategory::Extension(
                        user_settings.get_to_delete_list().get_extensions().clone(),
                    );
                }
                FilterCategory::LastAccessed(_) => {
                    filter = Box::new(LastAccessedFilter::new());
                    keep_filter_item = FilterCategory::LastAccessed(FileDateData::new(
                        days_since_now_to_system_time(
                            user_settings
                                .get_to_keep_list()
                                .get_accessed_after()
                                .clone(),
                        )
                        .map_err(|e| format!("{e}"))?,
                    ));
                    delete_filter_item = FilterCategory::LastAccessed(FileDateData::new(
                        days_since_now_to_system_time(
                            user_settings
                                .get_to_keep_list()
                                .get_accessed_after()
                                .clone(),
                        )
                        .map_err(|e| format!("{e}"))?,
                    ));
                }
                FilterCategory::LastModified(_) => {
                    filter = Box::new(LastModifiedFilter::new());
                    keep_filter_item = FilterCategory::LastModified(FileDateData::new(
                        days_since_now_to_system_time(
                            user_settings
                                .get_to_keep_list()
                                .get_modified_after()
                                .clone(),
                        )
                        .map_err(|e| format!("{e}"))?,
                    ));
                    delete_filter_item = FilterCategory::LastModified(FileDateData::new(
                        days_since_now_to_system_time(
                            user_settings
                                .get_to_keep_list()
                                .get_modified_after()
                                .clone(),
                        )
                        .map_err(|e| format!("{e}"))?,
                    ));
                }
            }

            filter
                .init(keep_filter_item, delete_filter_item)
                .map_err(|e| format!("{:?}", e))?;
            filters_to_use.push(filter);
        }
    }

    Ok(filters_to_use)
}

fn sort_files_into_provided_lists(
    list_of_files: &Vec<FileContainer>,
    filter_system: &FilterSystem,
    to_keep: &mut Vec<String>,
    to_delete: &mut Vec<String>,
) -> Result<(), String> {
    for file_object in list_of_files {
        let result: FilterCodes = filter_system.filter_file(&file_object).map_err(|e| {
            format!(
                "Failed to scan the file object {:?}, error: {:?}",
                file_object, e
            )
        })?;

        match &result {
            FilterCodes::ToKeep => to_keep.push(
                file_object
                    .get_path()
                    .clone()
                    .into_os_string()
                    .into_string()
                    .map_err(|_| {
                        format!(
                            "Failed to convert path {:?} to string",
                            file_object.get_path()
                        )
                    })?,
            ),
            FilterCodes::ToDelete => to_delete.push(
                file_object
                    .get_path()
                    .clone()
                    .into_os_string()
                    .into_string()
                    .map_err(|_| {
                        format!(
                            "Failed to convert path {:?} to string",
                            file_object.get_path()
                        )
                    })?,
            ),
        }
    }
    Ok(())
}
