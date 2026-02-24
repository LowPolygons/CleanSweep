use std::{fs, path::Path};

use rand::RngExt;

use crate::utils::{
    get_home_dir::get_cleansweep_dir,
    run_time_user_input::get_string_input_matching_provided_string,
};

pub fn demolish() -> Result<(), String> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|e| format!("{e}"))?;

    let has_confirmed = get_string_input_matching_provided_string(
        "To carry out demolish, please write \"cleansweep demolish\"",
        "cleansweep demolish",
    )
    .map_err(|e| format!("{e}"))?;

    if has_confirmed {
        fs::remove_dir_all(cleansweep_dir)
            .map_err(|e| format!("Failed to remove everything from the directory, {}", e))?;

        println!("Deleted the .cleansweep directory");
    } else {
        println!("You did not provide the correct string to carry out the demolish");
    }

    Ok(())
}
