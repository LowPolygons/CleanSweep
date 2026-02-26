use thiserror::Error;

use crate::cli::ListAndResetArgs;

use std::path::PathBuf;

use crate::containers::cleansweep_file_paths::CleansweepFilePaths;
use crate::utils::get_common_dirs::get_cleansweep_dir;

use crate::systems::json_io::*;

#[derive(Debug, Error)]
pub enum ResetError {
    #[error("Failure trying to get the cleansweep directory")]
    GetCleansweepDirectoryFailure,

    #[error("Failure trying to write json file from internal struct")]
    WriteJsonFileFromStructFailure,
}

pub fn reset(args: &ListAndResetArgs) -> Result<(), ResetError> {
    let cleansweep_dir: PathBuf =
        get_cleansweep_dir().map_err(|_| ResetError::GetCleansweepDirectoryFailure)?;

    let path_to_open = match args {
        ListAndResetArgs::ToDelete => CleansweepFilePaths::ToDelete,
        ListAndResetArgs::ToKeep => CleansweepFilePaths::ToKeep,
        ListAndResetArgs::Sets => CleansweepFilePaths::FoundSets,
    };

    write_json_file_from_struct(&Empty::new(), cleansweep_dir.join(&path_to_open.name()))
        .map_err(|_| ResetError::WriteJsonFileFromStructFailure)?;

    println!(
        "Reset the directory $HOME/.cleansweep/{}",
        path_to_open.name()
    );
    Ok(())
}
