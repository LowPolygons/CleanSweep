use std::env::current_dir;

use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    containers::file_container::FileContainer,
    systems::file_scanner::{FileScanner, FileScannerScanMode},
    utils::path_types_to_string::path_to_string,
};

pub fn print_hidden_stats(
    optional_subpath: &String,
    recursive: &bool,
    ignore_dirs: &Vec<String>,
) -> Result<(), String> {
    // Read files in immediate directory and format them into FileContainers
    // Open an interactive terminal and let the user fiddle about and choose them
    // Inline the formatted data of the selected file
    let path = current_dir()
        .map_err(|e| format!("Error getting the current dir {}", e))?
        .join(optional_subpath);

    match path.try_exists() {
        Ok(status) => {
            if !status {
                return Err(format!(
                    "The provided path was verified to have not existed"
                ));
            }
        }
        Err(e) => {
            return Err(format!(
                "Could not verify whether the path {:?} exists, {}",
                path, e
            ));
        }
    }

    let scan_mode = if *recursive {
        FileScannerScanMode::Recursive
    } else {
        FileScannerScanMode::Immediate
    };

    let scanned_files: Vec<FileContainer> = FileScanner::scan(path, scan_mode, ignore_dirs)
        .map_err(|e| format!("Failed to perform scan in immediate scan, {:?}", e))?;

    let mut file_labels: Vec<String> = vec![String::from("Exit Menu")];
    let index_offset = file_labels.len();

    file_labels = scanned_files.iter().try_fold(
        file_labels,
        |mut file_labels, curr_container| -> Result<Vec<String>, String> {
            let stringy_path =
                path_to_string(curr_container.get_path()).map_err(|e| format!("{e}"))?;
            file_labels.push(stringy_path);

            Ok(file_labels)
        },
    )?;

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a file to view it's stats")
            .items(&file_labels)
            .default(0)
            .interact()
            .map_err(|e| format!("Failed to create select instance, {}", e))?;

        if selection == 0 {
            break;
        }

        let chosen_file: &FileContainer = scanned_files
            .get(selection - index_offset)
            .ok_or_else(|| ())
            .map_err(|_| format!("Failed to index item from list of file containers"))?;

        print_file_data(chosen_file).map_err(|e| format!("{e}"))?;
    }

    Ok(())
}

fn print_file_data(chosen_file: &FileContainer) -> Result<(), String> {
    let stats = chosen_file.get_statistics();

    println!("Hidden Statistics: ");
    println!("- Size (bytes): {}", stats.get_size());
    println!("- Last Accessed : {}", stats.get_last_accessed().format());
    println!("- Last Modified : {}", stats.get_last_modified().format());

    let exit_indicator: Vec<String> = vec!["Finish".to_string()];

    let _ = Select::with_theme(&ColorfulTheme::default())
        .items(exit_indicator)
        .default(0)
        .interact()
        .map_err(|e| {
            format!(
                "Failed to create select instance for exiting print state {}",
                e
            )
        })?;

    Ok(())
}
