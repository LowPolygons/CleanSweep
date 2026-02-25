use std::fs::File;
use std::path::PathBuf;

use thiserror::Error;

use crate::containers::cleansweep_file_paths::CleansweepFilePaths;
use crate::containers::user_settings::UserSettings;
use crate::utils::create_defaults::create_default_user_settings;
use crate::utils::get_common_dirs::get_cleansweep_dir;

use crate::systems::json_io::*;

#[derive(Debug, Error)]
pub enum SetupError {
    #[error("Failed to get the cleansweep directory")]
    GetCleansweepDirectoryFailure,

    #[error("Failed to validate whether the .cleansweep path already exists")]
    ValidateIfPathExistsFailure,

    #[error("Failed to create the cleansweep directory")]
    CreateCleansweepDirectoryFailure,

    #[error("Failed to create the target file in the cleansweep directory")]
    CreateFileInCleansweepDirectoryFailure,

    #[error("Failed to write the json file for the corresponding object")]
    WriteJsonFileFromStructFailure,

    #[error("Cleansweep already exists on this machine!")]
    CleansweepAlreadyExists,
}

pub fn setup() -> Result<(), SetupError> {
    let cleansweep_dir: PathBuf =
        get_cleansweep_dir().map_err(|_| SetupError::GetCleansweepDirectoryFailure)?;

    if std::fs::exists(&cleansweep_dir).map_err(|_| SetupError::ValidateIfPathExistsFailure)? {
        println!("The .cleansweep directory already exists ");
        return Err(SetupError::CleansweepAlreadyExists);
    }

    std::fs::create_dir(&cleansweep_dir)
        .map_err(|_| SetupError::CreateCleansweepDirectoryFailure)?;

    let txt_files_to_write_with_nothing: Vec<CleansweepFilePaths> = vec![
        CleansweepFilePaths::LogFile,
        CleansweepFilePaths::FilterComponentList,
    ];
    let json_files_to_write_with_nothing: Vec<CleansweepFilePaths> = vec![
        CleansweepFilePaths::ToDelete,
        CleansweepFilePaths::ToKeep,
        CleansweepFilePaths::FoundSets,
    ];
    let user_settings_files: Vec<CleansweepFilePaths> = vec![
        CleansweepFilePaths::UserSettings,
        CleansweepFilePaths::UserSettingsDefault,
    ];

    for file in &txt_files_to_write_with_nothing {
        let _ = File::create(cleansweep_dir.join(file.name()))
            .map_err(|_| SetupError::CreateFileInCleansweepDirectoryFailure)?;
    }

    for file in &json_files_to_write_with_nothing {
        write_json_file_from_struct(&Empty::new(), cleansweep_dir.join(&file.name()))
            .map_err(|_| SetupError::WriteJsonFileFromStructFailure)?;
    }

    let default_user_settings: UserSettings = create_default_user_settings();

    for file in &user_settings_files {
        write_json_file_from_struct(&default_user_settings, cleansweep_dir.join(&file.name()))
            .map_err(|_| SetupError::WriteJsonFileFromStructFailure)?;
    }

    println!("Setup completed - initialised the following at $HOME/.cleansweep");

    for path in [
        txt_files_to_write_with_nothing,
        json_files_to_write_with_nothing,
        user_settings_files,
    ]
    .into_iter()
    .flatten()
    {
        println!("- ..{}", path.name());
    }

    Ok(())
}
