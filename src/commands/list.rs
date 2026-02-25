use thiserror::Error;

use crate::{
    cli::ListAndResetArgs,
    containers::{
        cleansweep_file_paths::CleansweepFilePaths, sets_read_write_type::SetsReadWriteType,
    },
    systems::json_io::read_file_to_struct,
    utils::get_common_dirs::get_cleansweep_dir,
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
    /*
     * Errors as a result of bad user input
     */
}

pub fn list(args: &ListAndResetArgs) -> Result<(), ListError> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|_| ListError::GetCleansweepDirError)?;

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

    Ok(())
}
