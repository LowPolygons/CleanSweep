use thiserror::Error;

use crate::{
    cli::KeepAndDelete,
    containers::{
        cleansweep_file_paths::CleansweepFilePaths,
        file_container::{FileContainer, get_list_file_containers_from_strings},
        file_date_data::{
            DaysSinceNowToSystemTimeError, FileDateData, days_since_now_as_str_to_system_time,
        },
    },
    systems::{
        filter_system::filter_category_info::FilterCategory,
        json_io::{read_file_to_struct, write_json_file_from_struct},
    },
    utils::{get_common_dirs::get_cleansweep_dir, path_types_to_string::path_to_string},
};

#[derive(Debug, Error)]
pub enum OverrideError {
    #[error("Failed to get the cleansweep directory")]
    GetCleansweepDirectoryFailure,

    #[error("Failed to read the list json file into the internal structure")]
    ReadFileToStructFailure,

    #[error("Failed to turn list of paths as strings into file container objects")]
    GetFileContainerListFromStringsFailure,

    // WARN: May be a user input error
    #[error("Failed to turn chosen filter category into a usable object with user inputs")]
    MakeGenericFilterCategoryUsableFailure,

    #[error("Failed to turn Path into a String")]
    PathToStringFailure,

    #[error("Failed to write list json file from the internal structure")]
    WriteJsonFileFromStructFailure,

    /*
     * Errors as a result of user misuse
     */
    #[error("Could not match the input to any existing Filter Category")]
    MatchStringToCategoryFailure,

    #[error("Cannot accept more than one value for the selected filter.")]
    TooManyArgsPassedForFilter,

    #[error("Cannot parse your input into its expected type")]
    FailedToParseInputToCorrectType,

    #[error("You're input seemingly pre-dates the UNIX Epoch")]
    InputPredatesUnixEpoch,
}

pub fn override_command(
    list_to_filter: &KeepAndDelete,
    filter_choice: &String,
    values: Vec<String>,
) -> Result<(), OverrideError> {
    let cleansweep_dir =
        get_cleansweep_dir().map_err(|_| OverrideError::GetCleansweepDirectoryFailure)?;

    let list_of_keepers: Vec<String> =
        read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()))
            .map_err(|_| OverrideError::GetCleansweepDirectoryFailure)?;

    let list_of_deleters: Vec<String> =
        read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()))
            .map_err(|_| OverrideError::ReadFileToStructFailure)?;

    let mut scanned_keepers = get_list_file_containers_from_strings(&list_of_keepers)
        .map_err(|_| OverrideError::GetFileContainerListFromStringsFailure)?;

    let mut scanned_deleters = get_list_file_containers_from_strings(&list_of_deleters)
        .map_err(|_| OverrideError::GetFileContainerListFromStringsFailure)?;

    let chosen_list_to_filter: &mut Vec<FileContainer>;
    let other_list: &mut Vec<FileContainer>;
    let chosen_list_path: CleansweepFilePaths;
    let other_list_path: CleansweepFilePaths;

    match list_to_filter {
        KeepAndDelete::ToDelete => {
            chosen_list_to_filter = &mut scanned_deleters;
            other_list = &mut scanned_keepers;
            chosen_list_path = CleansweepFilePaths::ToDelete;
            other_list_path = CleansweepFilePaths::ToKeep;
        }
        KeepAndDelete::ToKeep => {
            chosen_list_to_filter = &mut scanned_keepers;
            other_list = &mut scanned_deleters;
            chosen_list_path = CleansweepFilePaths::ToKeep;
            other_list_path = CleansweepFilePaths::ToDelete;
        }
    }

    // Create the filter
    let chosen_filter_generic = FilterCategory::match_string_to_category(filter_choice)
        .ok_or_else(|| ())
        .map_err(|_| OverrideError::MatchStringToCategoryFailure)?;

    println!(
        "Chosen Category: {}, {}",
        chosen_filter_generic.get_choice(),
        chosen_filter_generic.get_choice()
    );

    let filter_choice = chosen_filter_generic.get_filter();

    let filter_to_use = match filter_choice {
        FilterCategory::Name(_) => Ok(FilterCategory::Name(values)),
        FilterCategory::NameContains(_) => Ok(FilterCategory::NameContains(values)),
        FilterCategory::NameStartsWith(_) => Ok(FilterCategory::NameStartsWith(values)),
        FilterCategory::Size(_) => {
            if values.len() != 1 {
                Err(OverrideError::TooManyArgsPassedForFilter)
            } else {
                Ok(FilterCategory::Size(
                    values
                        .get(0)
                        .ok_or_else(|| ())
                        .map_err(|_| OverrideError::MakeGenericFilterCategoryUsableFailure)?
                        .parse::<u64>()
                        .map_err(|_| OverrideError::FailedToParseInputToCorrectType)?,
                ))
            }
        }
        FilterCategory::Extension(_) => Ok(FilterCategory::Extension(values)),
        FilterCategory::LastAccessed(_) => {
            if values.len() != 1 {
                Err(OverrideError::TooManyArgsPassedForFilter)
            } else {
                let num_days_as_seconds = values
                    .get(0)
                    .ok_or_else(|| ())
                    .map_err(|_| OverrideError::MakeGenericFilterCategoryUsableFailure)?;

                let days_since_now = days_since_now_as_str_to_system_time(num_days_as_seconds)
                    .map_err(|e| match e {
                        DaysSinceNowToSystemTimeError::FailedToParseNumberToUInt => {
                            OverrideError::FailedToParseInputToCorrectType
                        }
                        DaysSinceNowToSystemTimeError::NumDaysExceedsExpectedBounds => {
                            OverrideError::InputPredatesUnixEpoch
                        }
                    })?;

                Ok(FilterCategory::LastAccessed(FileDateData::new(
                    days_since_now,
                )))
            }
        }
        FilterCategory::LastModified(_) => {
            if values.len() != 1 {
                Err(OverrideError::TooManyArgsPassedForFilter)
            } else {
                let num_days_as_seconds = values
                    .get(0)
                    .ok_or_else(|| ())
                    .map_err(|_| OverrideError::MakeGenericFilterCategoryUsableFailure)?;

                let days_since_now = days_since_now_as_str_to_system_time(num_days_as_seconds)
                    .map_err(|e| match e {
                        DaysSinceNowToSystemTimeError::FailedToParseNumberToUInt => {
                            OverrideError::FailedToParseInputToCorrectType
                        }
                        DaysSinceNowToSystemTimeError::NumDaysExceedsExpectedBounds => {
                            OverrideError::InputPredatesUnixEpoch
                        }
                    })?;

                let _delete = FileDateData::new(days_since_now);

                Ok(FilterCategory::LastModified(FileDateData::new(
                    days_since_now,
                )))
            }
        }
        FilterCategory::DirectoryContains(_) => Ok(FilterCategory::DirectoryContains(values)),
    }?;

    const KEEP_IN_LIST: bool = true;
    const REMOVE_FROM_LIST: bool = false;

    chosen_list_to_filter.retain(|file_container| {
        if !filter_to_use.is_file_flagged(file_container) {
            KEEP_IN_LIST
        } else {
            println!(
                "Moving {:?} to the opposing list",
                file_container.get_path()
            );
            other_list.push(file_container.clone());
            REMOVE_FROM_LIST
        }
    });

    let chosen_as_string: Vec<String> = chosen_list_to_filter
        .iter()
        .try_fold(
            Vec::<String>::new(),
            // Only Error that occurs here is PathToStringFailure
            |mut to_keep, file| -> Result<Vec<String>, ()> {
                to_keep.push(path_to_string(file.get_path()).map_err(|_| ())?);

                Ok(to_keep)
            },
        )
        .map_err(|_| OverrideError::PathToStringFailure)?;

    let other_as_string: Vec<String> = other_list
        .iter()
        .try_fold(
            Vec::<String>::new(),
            |mut to_keep, file| -> Result<Vec<String>, ()> {
                // Only Error that occurs here is PathToStringFailure
                to_keep.push(path_to_string(file.get_path()).map_err(|_| ())?);

                Ok(to_keep)
            },
        )
        .map_err(|_| OverrideError::PathToStringFailure)?;

    write_json_file_from_struct(
        &chosen_as_string,
        cleansweep_dir.join(chosen_list_path.name()),
    )
    .map_err(|_| OverrideError::WriteJsonFileFromStructFailure)?;
    write_json_file_from_struct(
        &other_as_string,
        cleansweep_dir.join(other_list_path.name()),
    )
    .map_err(|_| OverrideError::WriteJsonFileFromStructFailure)?;

    Ok(())
}
