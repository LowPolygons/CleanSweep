use std::fs::File;
use std::path::PathBuf;

use crate::containers::user_settings::UserSettings;
use crate::utils::create_default_user_settings::create_default_user_settings;
use crate::{
    containers::cleansweep_file_paths::CleansweepFilePaths, utils::get_home_dir::get_home_directory,
};

use crate::systems::json_io::*;

pub fn setup() -> Result<(), String> {
    let cleansweep_dir: PathBuf = get_home_directory()
        .map_err(|e| format!("Failed to get home directory in setup: {:?}", e))?
        .join(CleansweepFilePaths::MainDirectoryName.name());

    if !cleansweep_dir.is_dir() {
        return Err(format!("Folder '$HOME/.cleansweep' not found"));
    }

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

    for file in txt_files_to_write_with_nothing {
        let _ = File::create(cleansweep_dir.join(file.name()))
            .map_err(|_| "Failed to create a file".to_string())?;
    }

    for file in json_files_to_write_with_nothing {
        write_json_file_from_struct(&Empty::new(), cleansweep_dir.join(&file.name())).map_err(
            |err| match &err {
                JsonWriteError::FileCreateFromPathError => {
                    format!(
                        "Failed to create the file from the given path - {}",
                        &file.name()
                    )
                }
                JsonWriteError::SerdeJsonWritePrettyError => {
                    format!(
                        "Failed to write the json string into the file - {}",
                        &file.name()
                    )
                }
            },
        )?;
    }

    let default_user_settings: UserSettings = create_default_user_settings();

    for file in user_settings_files {
        write_json_file_from_struct(&default_user_settings, cleansweep_dir.join(&file.name()))
            .map_err(|err| match &err {
                JsonWriteError::FileCreateFromPathError => {
                    format!(
                        "Failed to create the file from the given path - {}",
                        &file.name()
                    )
                }
                JsonWriteError::SerdeJsonWritePrettyError => {
                    format!(
                        "Failed to write the json string into the file - {}",
                        &file.name()
                    )
                }
            })?;
    }

    Ok(())
}
