use std::env::VarError;

use thiserror::Error;

use crate::{
    cli::SettingsArgs,
    containers::{
        cleansweep_file_paths::CleansweepFilePaths,
        user_settings::{UserSettings, get_user_setting_lines},
    },
    systems::json_io::{read_file_to_struct, write_json_file_from_struct},
    utils::{create_defaults::create_default_user_settings, get_home_dir::get_cleansweep_dir},
};

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Failure trying to get the cleansweep directory")]
    GetCleansweepDirectoryFailure,

    #[error("Failure trying to overwrite the existing user settings with the defaults")]
    OverwriteUserSettingsWithDefaultFailure,

    #[error("Failed to verify whether the target editor exists")]
    VerifyIfAnEditorExistsFailure,

    #[error("Failed to access the users Editor variable")]
    AccessEditorVarFailure,

    #[error("Failed to prompt an open-editor command")]
    OpenEditorCommandFailure,

    #[error("Failed to read the user_settings.json file into a UserSettings object")]
    ReadUserSettingsFileToObjectFailure,
}

pub fn settings(args: &SettingsArgs) -> Result<(), String> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|e| format!("{}", e))?;

    match args {
        SettingsArgs::Reset => {
            let defalt_user_settings = create_default_user_settings();

            write_json_file_from_struct(
                &defalt_user_settings,
                cleansweep_dir.join(CleansweepFilePaths::UserSettings.name()),
            )
            .map_err(|e| format!("{}", e))?;

            return Ok(());
        }
        SettingsArgs::Modify => {
            let editor = match std::env::var("EDITOR") {
                Ok(val) => Ok(val),
                Err(err) => {
                    let result = match err {
                        VarError::NotPresent => {
                            if std::fs::exists("/usr/bin/nano").map_err(|e| {
                                format!("Couldnt validate /usr/bin/nano directory, {e}")
                            })? {
                                println!("No EDITOR var set, defaulting to nano");
                                Ok("/usr/bin/nano".to_string())
                            } else if std::fs::exists("/usr/bin/vi").map_err(|e| {
                                format!("Couldnt validate /usr/bin/vi directory, {e}")
                            })? {
                                println!("No EDITOR var set, couldn't find nano, using vi");
                                Ok("/usr/bin/vi".to_string())
                            } else {
                                Err("Could not find backup editor nano or vi".to_string())
                            }
                        }
                        VarError::NotUnicode(e) => Err(format!(
                            "Couldnt get EDITOR var due to it not being Unicode, {e:?}"
                        )),
                    };
                    result
                }
            }
            .map_err(|e| format!("Failed to access the $EDITOR var, {e:?}"))?;

            std::process::Command::new(editor)
                .arg(&cleansweep_dir.join(CleansweepFilePaths::UserSettings.name()))
                .status()
                .map_err(|e| format!("Failed to edit file {e}"))?;

            println!("Whatever changes you made directly affected the user_settings file.");
            println!(
                "If a new command attempts to open it and fails, consider running 'cleansweep settings reset"
            );

            return Ok(());
        }

        SettingsArgs::Display => {
            let user_settings: UserSettings =
                read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::UserSettings.name()))
                    .map_err(|e| format!("{e}"))?;

            for line in get_user_setting_lines(user_settings) {
                println!("{}", line);
            }

            Ok(())
        }
    }
}
