use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use rand::prelude::*;

use crate::{
    cli::PurgeArgs,
    containers::cleansweep_file_paths::CleansweepFilePaths,
    systems::json_io::read_file_to_struct,
    utils::{file_to_string_vec::file_to_string_vec, get_home_dir::get_cleansweep_dir},
};

pub fn purge(args: &PurgeArgs) -> Result<(), String> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|e| format!("{}", e))?;

    match &args {
        PurgeArgs::Stage => {
            let keep_list: Vec<String> =
                read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()))
                    .map_err(|e| format!("{}", e))?;
            let delete_list: Vec<String> =
                read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()))
                    .map_err(|e| format!("{}", e))?;

            println!("Staging {} files for deletion..", delete_list.len());

            let mut temp_keep_list_file =
                File::create(Path::new(CleansweepFilePaths::ToKeepLocalTemp.name()))
                    .map_err(|e| format!("Failed to create the temp file: {}", e))?;
            let mut temp_delete_list_file =
                File::create(Path::new(CleansweepFilePaths::ToDeleteLocalTemp.name()))
                    .map_err(|e| format!("Failed to create the temp file: {}", e))?;

            for str_path in &delete_list {
                temp_delete_list_file
                    .write_all(format!("{}\n", str_path).as_bytes())
                    .map_err(|e| format!("Failed to write line to temp delete file: {}", e))?;
            }
            for str_path in &keep_list {
                temp_keep_list_file
                    .write_all(format!("{}\n", str_path).as_bytes())
                    .map_err(|e| format!("Failed to write line to temp delete file: {}", e))?;
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
            let delete_these_files =
                file_to_string_vec(Path::new(CleansweepFilePaths::ToDeleteLocalTemp.name()))
                    .map_err(|e| format!("{:?}", e))?;

            for path in &delete_these_files {
                println!("Deleting {}..", path);
                // TODO: fs::remove_file(path)
                MOCK_REMOVE_FILE(path)
                    .map_err(|e| format!("  ..Failed to remove file {}, {}", path, e))?
            }

            fs::remove_file(CleansweepFilePaths::ToDeleteLocalTemp.name())
                .map_err(|_| String::new())?;
            fs::remove_file(CleansweepFilePaths::ToKeepLocalTemp.name())
                .map_err(|_| String::new())?;

            Ok(())
        }
    }
}

// TODO: cargo remove rand
fn MOCK_REMOVE_FILE(_: &String) -> Result<(), String> {
    let mut rng = rand::rng();
    let number = rng.random_range(1..=100);

    if number > 99 {
        return Err("Pretend fail".to_string());
    } else {
        return Ok(());
    }
}
