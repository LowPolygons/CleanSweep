use std::path::PathBuf;

use crate::containers::user_settings::UserSettings;
use crate::utils::create_default_user_settings::create_default_user_settings;
use crate::{
    containers::cleansweep_file_paths::CleansweepFilePaths, utils::get_home_dir::get_home_directory,
};

use crate::systems::json_io::*;

pub fn setup() -> Result<(), String> {
    let cleansweep_dir: PathBuf = get_home_directory()
        .map_err(|_| "Failed to get home directory".to_string())?
        .join(CleansweepFilePaths::MainDirectoryName.name());

    if !cleansweep_dir.is_dir() {
        return Err("Folder '$HOME/.cleansweep' not found".to_string());
    }

    let files_to_write_with_nothing: Vec<CleansweepFilePaths> = vec![
        CleansweepFilePaths::ToDelete,
        CleansweepFilePaths::ToKeep,
        CleansweepFilePaths::LogFile,
        CleansweepFilePaths::FoundSets,
        CleansweepFilePaths::FilterComponentList,
    ];

    for file in files_to_write_with_nothing {
        write_json_file_from_struct(&Empty::new(), cleansweep_dir.join(file.name()))
            .map_err(|_| "Failed to write a file".to_string())?;
    }

    let default_user_settings: UserSettings = create_default_user_settings();

    // On setup the user settings are just the defaults
    write_json_file_from_struct(
        &default_user_settings,
        cleansweep_dir.join(CleansweepFilePaths::UserSettingsDefault.name()),
    )
    .map_err(|_| "Failed to write the user settings default file".to_string())?;
    write_json_file_from_struct(
        &default_user_settings,
        cleansweep_dir.join(CleansweepFilePaths::UserSettings.name()),
    )
    .map_err(|_| "Failed to write the user settings file".to_string())?;

    Ok(())
}
