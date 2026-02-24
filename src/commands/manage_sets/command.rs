use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    commands::manage_sets::containers::{ManageSetsType, SetStyle},
    containers::{
        cleansweep_file_paths::CleansweepFilePaths, sets_read_write_type::SetsReadWriteType,
    },
    systems::json_io::{read_file_to_struct, write_json_file_from_struct},
    utils::{
        get_home_dir::get_cleansweep_dir,
        run_time_user_input::{get_number_input, get_number_input_in_range},
    },
};

pub fn manage_sets() -> Result<(), String> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|e| format!("{e}"))?;

    let scanned_sets: Vec<SetsReadWriteType> =
        read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::FoundSets.name()))
            .map_err(|e| format!("{e}"))?;

    let mut dir_set_scan_was_run_from = String::new();

    let mut managed_sets: Vec<ManageSetsType> = scanned_sets.iter().try_fold(
        Vec::<ManageSetsType>::new(),
        |mut acc, set| -> Result<Vec<ManageSetsType>, String> {
            let label = set
                .files
                .get(0)
                .ok_or(|| "No File")
                .map_err(|_| format!("No File"))?
                .clone();

            if dir_set_scan_was_run_from.is_empty() {
                dir_set_scan_was_run_from = label.clone();
            }

            while label.find(&dir_set_scan_was_run_from).is_none() {
                dir_set_scan_was_run_from =
                    dir_set_scan_was_run_from[0..dir_set_scan_was_run_from.len() - 1].to_string()
            }

            let new_set = ManageSetsType {
                full_set: set.files.clone(),
                label,
                chosen_style: None,
            };

            acc.push(new_set);

            Ok(acc)
        },
    )?;

    let len_to_strip_away: usize = dir_set_scan_was_run_from.len();
    let mut first_in_sets: Vec<String>;

    println!(
        "References to $PATH represent {}",
        dir_set_scan_was_run_from
    );

    loop {
        first_in_sets = vec![
            "Exit Manage Sets".to_string(),
            "Select a default management style".to_string(),
        ];
        let length_initial_first_in_sets = first_in_sets.len();

        first_in_sets.append(
            &mut managed_sets
                .iter()
                .map(|item| {
                    format!(
                        "{} : $PATH/{}",
                        item.style_to_string(),
                        item.label_truncated(len_to_strip_away)
                    )
                })
                .collect(),
        );

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&first_in_sets)
            .default(0)
            .interact()
            .map_err(|e| format!("Failed to create select instance, {:?}", e))?;

        if selection == 0 {
            // Hardcoded exit option
            break;
        } else if selection == 1 {
            // Hardcoded default setting
            println!(
                "Any sets where a provided 'N-Value' exceeds its length will not have a default applied"
            );
            select_default_style(&mut managed_sets).map_err(|e| format!("{e}"))?;
        } else {
            select_management_style_for_set(
                managed_sets
                    .get_mut(selection - length_initial_first_in_sets)
                    .ok_or_else(|| ())
                    .map_err(|_| format!("Bad index to managed_sets!"))?,
                len_to_strip_away,
            )
            .map_err(|e| format!("{e}"))?;
        }
    }

    let mut files_for_keep: Vec<String> = Vec::new();
    let mut files_for_delete: Vec<String> = Vec::new();

    for set in &mut managed_sets {
        match separate_files_based_on_style(set, &mut files_for_keep, &mut files_for_delete) {
            Ok(_) => {}
            Err(_) => println!("A set didn't have a method specified, skipping.."),
        }
    }

    println!("Overriding Keep list with:");
    files_for_keep.iter().for_each(|item| println!("- {item}"));

    println!("Overriding Delete list with:");
    files_for_delete
        .iter()
        .for_each(|item| println!("- {item}"));

    write_json_file_from_struct(
        &files_for_keep,
        cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()),
    )
    .map_err(|e| format!("{e}"))?;

    write_json_file_from_struct(
        &files_for_delete,
        cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()),
    )
    .map_err(|e| format!("{e}"))?;

    Ok(())
}

fn select_default_style(managed_sets: &mut Vec<ManageSetsType>) -> Result<(), String> {
    let selection = select_management_style().map_err(|e| format!("{e}"))?;

    let value = match selection {
        SetStyle::EveryN(_) => {
            let n_value: usize = get_number_input(
                "Enter the number of how often to save a file when interating over the set: ",
                true,
            )
            .map_err(|e| format!("{}", e))?;

            SetStyle::EveryN(n_value)
        }
        SetStyle::EvenlySpacedN(_) => {
            println!(
                "This will, on average save exactly N files. There will be a margin of error if N > len / 2"
            );
            let n_value: usize =
                get_number_input("Enter how many files do you want to save: ", true)
                    .map_err(|e| format!("{}", e))?;
            SetStyle::EvenlySpacedN(n_value)
        }
        other => other,
    };

    for set in managed_sets.iter_mut() {
        set.chosen_style = match &value {
            SetStyle::EveryN(n_value) => {
                if *n_value > set.full_set.len() {
                    None
                } else {
                    Some(SetStyle::EveryN(*n_value))
                }
            }
            SetStyle::EvenlySpacedN(n_value) => {
                if *n_value > set.full_set.len() {
                    None
                } else {
                    Some(SetStyle::EvenlySpacedN(*n_value))
                }
            }
            other => Some(other.clone()),
        };
    }

    Ok(())
}

fn select_management_style() -> Result<SetStyle, String> {
    let sub_options: Vec<SetStyle> = vec![
        SetStyle::First,
        SetStyle::Last,
        SetStyle::FirstAndLast,
        SetStyle::EveryN(0),
        SetStyle::EvenlySpacedN(0),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Submenu")
        .items(&sub_options)
        .interact()
        .map_err(|e| format!("Failed to create select instance, {:?}", e))?;

    match sub_options.get(selection).ok_or(|| ()) {
        Ok(val) => return Ok(val.clone()),
        Err(_) => return Err("Failed to choose set style due to bad indexing".to_string()),
    }
}

fn select_management_style_for_set(
    chosen_set: &mut ManageSetsType,
    len_to_strip: usize,
) -> Result<(), String> {
    chosen_set.full_set.iter().for_each(|elem| {
        println!(
            "- $PATH/{}",
            elem.clone()
                .drain(len_to_strip..elem.len())
                .fold(String::new(), |mut string, char| {
                    string = format!("{}{}", string, char);
                    string
                })
        )
    });

    let selection = select_management_style().map_err(|e| format!("{e}"))?;
    let len_of_set_sub_one = chosen_set.full_set.len() - 1;

    chosen_set.chosen_style = match selection {
        SetStyle::EveryN(_) => {
            let n_value: usize = get_number_input_in_range(
                "Enter the number of how often to save a file when interating over the set: ",
                1,
                len_of_set_sub_one + 1,
            )
            .map_err(|e| format!("{}", e))?;

            Some(SetStyle::EveryN(n_value))
        }
        SetStyle::EvenlySpacedN(_) => {
            println!(
                "This will, on average save exactly N files. There will be a margin of error if N > len / 2"
            );

            let n_value: usize = get_number_input_in_range(
                "Enter how many files do you want to save: ",
                1,
                len_of_set_sub_one + 1,
            )
            .map_err(|e| format!("{}", e))?;
            Some(SetStyle::EvenlySpacedN(n_value))
        }
        other => Some(other.clone()),
    };

    Ok(())
}

fn separate_files_based_on_style(
    chosen_set: &ManageSetsType,
    keep_list: &mut Vec<String>,
    delete_list: &mut Vec<String>,
) -> Result<(), ()> {
    let len_of_set_sub_one = chosen_set.full_set.len() - 1;

    match &chosen_set.chosen_style {
        None => return Err(()),
        Some(chosen_style) => match chosen_style {
            SetStyle::First => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    match index {
                        0 => keep_list.push(value.clone()),
                        _ => delete_list.push(value.clone()),
                    }
                }
            }
            SetStyle::Last => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index == len_of_set_sub_one {
                        keep_list.push(value.clone());
                    } else {
                        delete_list.push(value.clone());
                    }
                }
            }
            SetStyle::FirstAndLast => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index == len_of_set_sub_one {
                        keep_list.push(value.clone());
                    } else {
                        match index {
                            0 => keep_list.push(value.clone()),
                            _ => delete_list.push(value.clone()),
                        }
                    }
                }
            }
            SetStyle::EveryN(n_value) => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index == len_of_set_sub_one || (len_of_set_sub_one - index) % n_value == 0 {
                        keep_list.push(value.clone());
                    } else {
                        delete_list.push(value.clone());
                    }
                }
            }
            SetStyle::EvenlySpacedN(n_value) => {
                let chunk_size =
                    (chosen_set.full_set.len() as f64 / *n_value as f64).round() as usize;

                for (chunk_num, chunk) in chosen_set.full_set.chunks(chunk_size).enumerate() {
                    if chunk_num == n_value - 1 {
                        let len_chunk = chunk.len() - 1;

                        for (index, value) in chunk.iter().enumerate() {
                            if index == len_chunk {
                                keep_list.push(value.clone());
                            } else {
                                delete_list.push(value.clone());
                            }
                        }
                    } else {
                        for (index, value) in chunk.iter().enumerate() {
                            match index {
                                0 => keep_list.push(value.clone()),
                                _ => delete_list.push(value.clone()),
                            }
                        }
                    }
                }
            }
        },
    }

    Ok(())
}
