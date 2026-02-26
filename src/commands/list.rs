use thiserror::Error;

use crate::{
    cli::ListAndResetArgs,
    containers::{
        cleansweep_file_paths::CleansweepFilePaths,
        file_container::get_list_file_containers_from_strings,
        sets_read_write_type::SetsReadWriteType,
    },
    systems::json_io::read_file_to_struct,
    utils::{
        get_common_dirs::get_cleansweep_dir,
        size_in_bytes_to_other::size_in_bytes_to_formatted_string,
    },
};

#[derive(Debug, Error)]
pub enum ListError {
    /*
     * Errors as a result of code failure
     */
    #[error("Failed to get the Cleansweep Directory")]
    GetCleansweepDirError,

    #[error("Failed when trying to read the sets json file to the internal type")]
    ReadingJsonFileToSetStructFailure,

    #[error("Failed when trying to read the list json file to the internal type")]
    ReadingJsonFileToStructFailure,

    #[error("Failed trying to convert a size in bytes to a better value")]
    FormatSizeInBytesToBetterUnitFailure,

    #[error("Failed to get the list of file containers from the list of strings")]
    GetFileContainerListFromStringsFailure, /*
                                             * Errors as a result of bad user input
                                             */
}

pub fn list(args: &ListAndResetArgs, summarise: &bool) -> Result<(), ListError> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|_| ListError::GetCleansweepDirError)?;

    if *summarise {
        // Summarise prints info to do with all, so ignore args
        let list_of_keeps: Vec<String> =
            read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()))
                .map_err(|_| ListError::ReadingJsonFileToStructFailure)?;
        let list_of_deletes: Vec<String> =
            read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()))
                .map_err(|_| ListError::ReadingJsonFileToStructFailure)?;

        let scanned_keepers = get_list_file_containers_from_strings(&list_of_keeps)
            .map_err(|_| ListError::GetFileContainerListFromStringsFailure)?;

        let scanned_deleters = get_list_file_containers_from_strings(&list_of_deletes)
            .map_err(|_| ListError::GetFileContainerListFromStringsFailure)?;

        let size_of_keeps: u64 = scanned_keepers
            .iter()
            .fold(Vec::<u64>::new(), |mut list_sizes, file| {
                list_sizes.push(file.get_statistics().get_size().clone());

                list_sizes
            })
            .iter()
            .sum();

        let size_of_deletes: u64 = scanned_deleters
            .iter()
            .fold(Vec::<u64>::new(), |mut list_sizes, file| {
                list_sizes.push(file.get_statistics().get_size().clone());

                list_sizes
            })
            .iter()
            .sum();

        println!("Summary:");
        println!(" - To Keep:");
        println!(" - - {} Files", list_of_keeps.len());
        println!(
            " - - {}",
            size_in_bytes_to_formatted_string(size_of_keeps)
                .map_err(|_| ListError::FormatSizeInBytesToBetterUnitFailure)?
        );
        println!(" - To Delete:");
        println!(" - - {} Files", list_of_deletes.len());
        println!(
            " - - {}",
            size_in_bytes_to_formatted_string(size_of_deletes)
                .map_err(|_| ListError::FormatSizeInBytesToBetterUnitFailure)?
        );
    } else {
        let mut args_is_sets: bool = false;
        let label: String;
        let path_to_open = match args {
            ListAndResetArgs::ToDelete => {
                label = "To Delete".to_string();
                CleansweepFilePaths::ToDelete
            }
            ListAndResetArgs::ToKeep => {
                label = "To Keep".to_string();
                CleansweepFilePaths::ToKeep
            }
            ListAndResetArgs::Sets => {
                // The structure of the sets file is different
                label = "Found Sets".to_string();
                args_is_sets = true;
                CleansweepFilePaths::FoundSets
            }
        };

        if args_is_sets {
            let list_of_files: Vec<SetsReadWriteType> =
                read_file_to_struct(cleansweep_dir.join(path_to_open.name()))
                    .map_err(|_| ListError::ReadingJsonFileToSetStructFailure)?;

            println!("{}:", label);
            for set in &list_of_files {
                println!("Set:");
                for file in &set.files {
                    println!("- - {}", file);
                }
            }
        } else {
            let list_of_files: Vec<String> =
                read_file_to_struct(cleansweep_dir.join(path_to_open.name()))
                    .map_err(|_| ListError::ReadingJsonFileToStructFailure)?;

            println!("{}:", label);
            for file in list_of_files {
                println!("- {}", file);
            }
        }
    }

    Ok(())
}
