use core::fmt;

use chrono::round;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use thiserror::Error;

use crate::{
    containers::{
        cleansweep_file_paths::CleansweepFilePaths, sets_read_write_type::SetsReadWriteType,
    },
    systems::json_io::read_file_to_struct,
    utils::get_home_dir::get_cleansweep_dir,
};

#[derive(Debug)]
enum SetStyle {
    First,
    Last,
    FirstAndLast,
    EveryN(usize),
    EvenlySpacedN(usize),
}

struct ManageSetsType {
    pub full_set: Vec<String>,
    pub label: String,
    pub chosen_style: Option<SetStyle>,
}

impl fmt::Display for SetStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SetStyle::First => "First",
            SetStyle::Last => "Last",
            SetStyle::FirstAndLast => "First And Last",
            SetStyle::EveryN(_) => "Every N",
            SetStyle::EvenlySpacedN(_) => "N Evenly Spaced",
        };
        write!(f, "{s}")
    }
}

impl ManageSetsType {
    fn style_to_string(&self) -> String {
        return match &self.chosen_style {
            Some(v) => format!("{}", v),
            None => format!("None Chosen"),
        };
    }
}

pub fn manage_sets() -> Result<(), String> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|e| format!("{e}"))?;

    let scanned_sets: Vec<SetsReadWriteType> =
        read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::FoundSets.name()))
            .map_err(|e| format!("{e}"))?;

    // TODO: this logic needs to be moved inside of the loop {}
    let mut managed_sets: Vec<ManageSetsType> = scanned_sets.iter().try_fold(
        Vec::<ManageSetsType>::new(),
        |mut acc, set| -> Result<Vec<ManageSetsType>, String> {
            let new_set = ManageSetsType {
                label: set
                    .files
                    .get(0)
                    .ok_or(|| "No File")
                    .map_err(|_| format!("No File"))?
                    .clone(),
                full_set: set.files.clone(),
                chosen_style: None,
            };

            acc.push(new_set);

            Ok(acc)
        },
    )?;

    let mut files_for_keep: Vec<String> = Vec::new();
    let mut files_for_delete: Vec<String> = Vec::new();

    let mut first_in_sets: Vec<String>;
    // TODO: add a flag to indicate if its set mode has been chosen
    // Indicate how many files in it
    // Indicate roughly how big it is

    loop {
        first_in_sets = vec![
            "Exit Manage Sets".to_string(),
            "Select a default management style".to_string(),
        ];
        let length_initial_first_in_sets = first_in_sets.len();

        first_in_sets.append(
            &mut managed_sets
                .iter()
                .map(|item| format!("{} : {}", item.style_to_string(), item.label.clone()))
                .collect(),
        );

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&first_in_sets)
            .default(0)
            .interact()
            .map_err(|e| format!("Failed to create select instance, {:?}", e))?;

        if selection == 0 {
            break;
        } else if selection == 1 {
            println!("Setting default..");
            break;
        }
        select_management_style_for_set(
            managed_sets
                .get_mut(selection - length_initial_first_in_sets)
                .ok_or_else(|| ())
                .map_err(|_| format!("Bad index to managed_sets!"))?,
        )
        .map_err(|e| format!("{e}"))?;
    }

    for set in &mut managed_sets {
        match separate_files_based_on_style(set, &mut files_for_keep, &mut files_for_delete) {
            Ok(_) => {}
            Err(_) => println!("A set didn't have a method specified, skipping.."),
        }
    }

    println!("Keep these:");
    for file in files_for_keep {
        println!("- {}", file);
    }
    println!("Delete these:");
    for file in files_for_delete {
        println!("- {}", file);
    }
    Ok(())

    // TODO: please god clean up this file lol
}

fn select_management_style_for_set(chosen_set: &mut ManageSetsType) -> Result<(), String> {
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

    let len_of_set_sub_one = chosen_set.full_set.len() - 1;

    chosen_set.chosen_style = match sub_options
        .get(selection)
        .ok_or(|| ())
        .map_err(|_| format!("Index somehow out of range despite being restricted"))?
    {
        SetStyle::First => Some(SetStyle::First),
        SetStyle::Last => Some(SetStyle::Last),
        SetStyle::FirstAndLast => Some(SetStyle::FirstAndLast),
        SetStyle::EveryN(_) => {
            let n_value: usize = get_number_input_in_range(
                "Enter how many files do you want to save: ",
                1,
                len_of_set_sub_one + 1,
            )
            .map_err(|e| format!("{}", e))?;

            Some(SetStyle::EveryN(n_value))
        }
        SetStyle::EvenlySpacedN(_) => {
            let n_value: usize = get_number_input_in_range(
                "Enter the number of how often to save a file when interating over the set: ",
                1,
                len_of_set_sub_one + 1,
            )
            .map_err(|e| format!("{}", e))?;
            Some(SetStyle::EvenlySpacedN(n_value))
        }
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

fn get_number_input(label: &str) -> Result<usize, String> {
    let number: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            input
                .parse::<usize>()
                .map(|_| ()) // validate_with needs to return nothing
                .map_err(|_| "Please enter a valid number")
        })
        .interact_text()
        .map_err(|e| format!("Failed to validate numerical input, {:?}", e))?
        .parse()
        .map_err(|e| format!("Error formatting the parsed number, {:?}", e))?;

    Ok(number)
}

fn get_number_input_in_range(label: &str, lower: usize, upper: usize) -> Result<usize, String> {
    let number: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(label)
        .validate_with(|input: &String| -> Result<(), &str> {
            let num_input = input
                .parse::<usize>()
                .map_err(|_| "Please enter a valid number")?;

            if num_input < lower || num_input > upper {
                return Err("Please enter a number in the correct range");
            }
            Ok(())
        })
        .interact_text()
        .map_err(|e| format!("Failed to validate numerical input, {:?}", e))?
        .parse()
        .map_err(|e| format!("Error formatting the parsed number, {:?}", e))?;

    Ok(number)
}
