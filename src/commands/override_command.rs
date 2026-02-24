use std::{
    path::Path,
    time::{Duration, SystemTime},
};

use crate::{
    cli::KeepAndDelete,
    containers::{
        cleansweep_file_paths::CleansweepFilePaths,
        file_container::FileContainer,
        file_date_data::{FileDateData, days_since_now_as_str_to_system_time},
    },
    systems::{
        filter_system::filter_category_info::FilterCategory,
        json_io::{read_file_to_struct, write_json_file_from_struct},
    },
    utils::get_home_dir::get_cleansweep_dir,
};

pub fn override_command(
    list_to_filter: &KeepAndDelete,
    filter_choice: &String,
    values: Vec<String>,
) -> Result<(), String> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|e| format!("{e}"))?;

    let list_of_keepers: Vec<String> =
        read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()))
            .map_err(|e| format!("{e}"))?;

    let list_of_deleters: Vec<String> =
        read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()))
            .map_err(|e| format!("{e}"))?;

    let mut scanned_keepers =
        get_list_file_containers_from_strings(&list_of_keepers).map_err(|e| format!("{e}"))?;

    let mut scanned_deleters =
        get_list_file_containers_from_strings(&list_of_deleters).map_err(|e| format!("{e}"))?;

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
        .map_err(|_| format!("Could not match input to any Filter Category"))?;

    println!(
        "Chosen Category: {}, {}",
        chosen_filter_generic.get_choice(),
        chosen_filter_generic.get_choice()
    );

    let filter_choice = chosen_filter_generic.get_filter();

    let filter_to_use = match filter_choice {
        FilterCategory::Name(_) => Ok(FilterCategory::Name(values)),
        FilterCategory::NameContains(_) => Ok(FilterCategory::NameContains(values)),
        FilterCategory::Size(_) => {
            if values.len() != 1 {
                Err("Can't accept more than one value for the size filter".to_string())
            } else {
                Ok(FilterCategory::Size(
                    values
                        .get(0)
                        .ok_or_else(|| ())
                        .map_err(|_| format!("Index somehow out of range?"))?
                        .parse::<u64>()
                        .map_err(|e| format!("Failed to parse number to an unsigned int, {e}"))?,
                ))
            }
        }
        FilterCategory::Extension(_) => Ok(FilterCategory::Extension(values)),
        FilterCategory::LastAccessed(_) => {
            if values.len() != 1 {
                Err("Can't accept more than one value for the size filter".to_string())
            } else {
                let num_days_as_seconds = values
                    .get(0)
                    .ok_or_else(|| ())
                    .map_err(|_| format!("Index somehow out of range?"))?;

                let days_since_now = days_since_now_as_str_to_system_time(num_days_as_seconds)
                    .map_err(|e| format!("{e}"))?;

                Ok(FilterCategory::LastAccessed(FileDateData::new(
                    days_since_now,
                )))
            }
        }
        FilterCategory::LastModified(_) => {
            if values.len() != 1 {
                Err("Can't accept more than one value for the size filter".to_string())
            } else {
                let num_days_as_seconds = values
                    .get(0)
                    .ok_or_else(|| ())
                    .map_err(|_| format!("Index somehow out of range?"))?;

                let days_since_now = days_since_now_as_str_to_system_time(num_days_as_seconds)
                    .map_err(|e| format!("{e}"))?;

                let _delete = FileDateData::new(days_since_now);

                Ok(FilterCategory::LastModified(FileDateData::new(
                    days_since_now,
                )))
            }
        }
        FilterCategory::DirectoryContains(_) => Ok(FilterCategory::DirectoryContains(values)),
    }
    .map_err(|e| format!("{e}"))?;

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
            |mut to_keep, file| -> Result<Vec<String>, String> {
                to_keep.push(
                    FileContainer::full_path_as_string(file.get_path())
                        .map_err(|e| format!("{e}"))?,
                );

                Ok(to_keep)
            },
        )
        .map_err(|e| format!("{e}"))?;

    let other_as_string: Vec<String> = other_list
        .iter()
        .try_fold(
            Vec::<String>::new(),
            |mut to_keep, file| -> Result<Vec<String>, String> {
                to_keep.push(
                    FileContainer::full_path_as_string(file.get_path())
                        .map_err(|e| format!("{e}"))?,
                );

                Ok(to_keep)
            },
        )
        .map_err(|e| format!("{e}"))?;

    write_json_file_from_struct(
        &chosen_as_string,
        cleansweep_dir.join(chosen_list_path.name()),
    )
    .map_err(|e| format!("Failed to save list of files - {:?}", e))?;
    write_json_file_from_struct(
        &other_as_string,
        cleansweep_dir.join(other_list_path.name()),
    )
    .map_err(|e| format!("Failed to save list of files - {:?}", e))?;
    Ok(())
}

fn get_list_file_containers_from_strings(list: &Vec<String>) -> Result<Vec<FileContainer>, String> {
    Ok(list
        .iter()
        .try_fold(
            Vec::<FileContainer>::new(),
            |mut scanned_files, path_as_str| -> Result<Vec<FileContainer>, String> {
                scanned_files
                    .push(FileContainer::new(Path::new(path_as_str)).map_err(|e| format!("{e}"))?);
                Ok(scanned_files)
            },
        )
        .map_err(|e| format!("{e}"))?)
}
