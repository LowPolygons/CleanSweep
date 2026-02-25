use core::time;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use thiserror::Error;

use crate::{
    cli::PurgeArgs,
    containers::cleansweep_file_paths::CleansweepFilePaths,
    systems::json_io::read_file_to_struct,
    utils::{
        file_to_string_vec::file_to_string_vec, get_common_dirs::get_cleansweep_dir,
        run_time_user_input::get_string_input_matching_provided_string,
    },
};

#[derive(Debug, Error)]
pub enum PurgeError {
    #[error("Failure getting the cleansweep directory")]
    GetCleansweepDirectoryFailure,

    #[error("Failure attempting to read list json into struct")]
    ReadListJsonToStructFailure,

    #[error("Failure attempting to create temporary file")]
    CreateTemporaryFileFailure,

    #[error("Failure writing data to the temporary file")]
    WriteToTemporaryFileFailure,

    #[error("Failed to create an input field")]
    CreateInputFieldFailure,

    #[error("Failure trying to read list of files into an internal string vector")]
    CreateListOfStringPathsFromFileFailure,

    #[error("Failure trying to delete a listed file")]
    DeleteFileFailure,

    #[error("Failure trying to delete temporary file")]
    DeleteTemporaryFileFailure,

    #[error("You did not correctly confirm if you wish to continue with the purge")]
    StringInputDoesNotMatchExpected,
}

pub fn purge(args: &PurgeArgs) -> Result<(), PurgeError> {
    let cleansweep_dir =
        get_cleansweep_dir().map_err(|_| PurgeError::GetCleansweepDirectoryFailure)?;

    match &args {
        PurgeArgs::Stage => {
            let keep_list: Vec<String> =
                read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()))
                    .map_err(|_| PurgeError::ReadListJsonToStructFailure)?;
            let delete_list: Vec<String> =
                read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()))
                    .map_err(|_| PurgeError::ReadListJsonToStructFailure)?;

            println!("Staging {} files for deletion..", delete_list.len());

            let mut temp_keep_list_file =
                File::create(Path::new(CleansweepFilePaths::ToKeepLocalTemp.name()))
                    .map_err(|_| PurgeError::CreateTemporaryFileFailure)?;
            let mut temp_delete_list_file =
                File::create(Path::new(CleansweepFilePaths::ToDeleteLocalTemp.name()))
                    .map_err(|_| PurgeError::CreateTemporaryFileFailure)?;

            for str_path in &delete_list {
                temp_delete_list_file
                    .write_all(format!("{}\n", str_path).as_bytes())
                    .map_err(|_| PurgeError::WriteToTemporaryFileFailure)?;
            }
            for str_path in &keep_list {
                temp_keep_list_file
                    .write_all(format!("{}\n", str_path).as_bytes())
                    .map_err(|_| PurgeError::WriteToTemporaryFileFailure)?;
            }

            println!(
                "The staged files for deletion have been written to {} in your current directory",
                CleansweepFilePaths::ToDeleteLocalTemp.name()
            );
            println!(
                "For a sanity check, the files to keep have been written to {} in your current directory",
                CleansweepFilePaths::ToKeepLocalTemp.name()
            );
            println!("Consult these files to ensure you are happy with what will be deleted");

            Ok(())
        }
        PurgeArgs::Continue => {
            println!("Please enter 'confirm' for the following statements (space/case sensitive)");

            let confirm_irreverability = get_string_input_matching_provided_string(
                "I confirm that i want these files to be deleted and that this is an irreversable action",
                "confirm",
            ).map_err(|_| PurgeError::CreateInputFieldFailure)?;

            let confirm_user_settings_validity = get_string_input_matching_provided_string(
                "I confirm that these files were scanned using my most up-to-date settings and therefore any deletions are expected",
                "confirm",
            ).map_err(|_| PurgeError::CreateInputFieldFailure)?;

            if !confirm_irreverability {
                println!("Your input for confirming the irreversability was incorrect, cancelling");
                return Err(PurgeError::StringInputDoesNotMatchExpected);
            }

            if !confirm_user_settings_validity {
                println!(
                    "Your input for confirming your settings were up to date was incorrect, cancelling"
                );
                return Err(PurgeError::StringInputDoesNotMatchExpected);
            }

            let mut files_which_failed_deletion: Vec<String> = Vec::new();

            if confirm_irreverability && confirm_user_settings_validity {
                println!("Waiting 5 seconds. Break out to cancel operation");

                std::thread::sleep(time::Duration::from_secs(5));

                println!("Deleting files..");

                let delete_these_files =
                    file_to_string_vec(Path::new(CleansweepFilePaths::ToDeleteLocalTemp.name()))
                        .map_err(|_| PurgeError::CreateListOfStringPathsFromFileFailure)?;

                for path in &delete_these_files {
                    println!("Deleting {}..", path);

                    match fs::remove_file(path) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Failed to delete {}", path);
                            files_which_failed_deletion.push(path.clone())
                        }
                    }
                }
            }

            if files_which_failed_deletion.len() != 0 {
                return Err(PurgeError::DeleteFileFailure);
            }

            fs::remove_file(CleansweepFilePaths::ToDeleteLocalTemp.name())
                .map_err(|_| PurgeError::DeleteTemporaryFileFailure)?;
            fs::remove_file(CleansweepFilePaths::ToKeepLocalTemp.name())
                .map_err(|_| PurgeError::DeleteTemporaryFileFailure)?;

            Ok(())
        }
    }
}
