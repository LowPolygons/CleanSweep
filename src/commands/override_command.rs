use std::path::Path;

use crate::{
    cli::KeepAndDelete,
    containers::{cleansweep_file_paths::CleansweepFilePaths, file_container::FileContainer},
    systems::json_io::read_file_to_struct,
    utils::get_home_dir::get_cleansweep_dir,
};

pub fn override_command(
    list_to_filter: &KeepAndDelete,
    filter_choice: &String,
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

    let mut chosen_list_to_filter: &mut Vec<FileContainer>;
    let mut other_list: &mut Vec<FileContainer>;

    match list_to_filter {
        KeepAndDelete::ToDelete => {
            chosen_list_to_filter = &mut scanned_deleters;
            other_list = &mut scanned_keepers;
        }
        KeepAndDelete::ToKeep => {
            chosen_list_to_filter = &mut scanned_keepers;
            other_list = &mut scanned_deleters;
        }
    }

    // Create the filter

    chosen_list_to_filter.retain(|file_container| {
        // False means dont keep it
        // True means keep it
        true
    });

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
