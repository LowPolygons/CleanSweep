use std::{collections::HashMap, path::Path};

use dialoguer::{Input, Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    commands::manage_sets::containers::{ManageSetsType, SetStyle, choose_style_and_m_n_values},
    systems::json_io::{JsonReadError, read_file_to_struct, write_json_file_from_struct},
    utils::run_time_user_input::get_number_input,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreciseManagement {
    percentages: Vec<u8>,
    managements: Vec<Vec<Vec<SetStyle>>>,
}

impl PreciseManagement {
    pub fn new(percentages: Vec<u8>, managements: Vec<Vec<Vec<SetStyle>>>) -> Option<Self> {
        if percentages.len() == managements.len() {
            Some(Self {
                percentages,
                managements,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Error)]
pub enum ManageSetsPrecisionModeError {
    #[error("Failed to read the provided precise management file into the expected type - {0}")]
    ReadFileToStructError(JsonReadError),
}

pub fn apply_precision_mode(
    managed_sets: Vec<ManageSetsType>,
    file_name: &str,
) -> Result<(), ManageSetsPrecisionModeError> {
    let config_file: HashMap<String, PreciseManagement> = read_file_to_struct(Path::new(file_name))
        .map_err(|e| ManageSetsPrecisionModeError::ReadFileToStructError(e))?;

    Ok(())
}

pub enum BuildManagementConfigError {
    SelectionFailure,
    InsertNewPreciseManagementFailure,
    GetUserInputFailure,
    ReplaceExistingPreciseManagementFailure,
    WriteJsonFileFromStructFailure,
}

pub fn build_management_config(file_name: &str) -> Result<(), BuildManagementConfigError> {
    let mut management_config: HashMap<String, PreciseManagement> = HashMap::new();

    let outermost_options = vec![
        "Finish",
        "Set Default",
        "Create New",
        "Override Existing",
        "Delete Existing",
        "List Current",
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&outermost_options)
            .default(0)
            .interact()
            .map_err(|_| BuildManagementConfigError::SelectionFailure)?;

        match selection {
            0 => break,
            1 => insert_new_precise_management(
                "default".to_string(),
                &mut management_config,
                ReplaceOrNew::New,
            )
            .map_err(|_| BuildManagementConfigError::InsertNewPreciseManagementFailure)?,
            2 => {
                let new_key = get_user_input("Set a key to create: ")
                    .map_err(|_| BuildManagementConfigError::GetUserInputFailure)?;
                insert_new_precise_management(new_key, &mut management_config, ReplaceOrNew::New)
                    .map_err(|_| BuildManagementConfigError::InsertNewPreciseManagementFailure)?;
            }
            3 => {
                let existing_key = get_user_input("Choose a key to override: ")
                    .map_err(|_| BuildManagementConfigError::GetUserInputFailure)?;
                insert_new_precise_management(
                    existing_key,
                    &mut management_config,
                    ReplaceOrNew::Replace,
                )
                .map_err(|_| BuildManagementConfigError::ReplaceExistingPreciseManagementFailure)?;
            }
            4 => {
                let existing_key = get_user_input("Choose a key to override: ")
                    .map_err(|_| BuildManagementConfigError::GetUserInputFailure)?;
                if let Some(_) = management_config.get_mut(&existing_key) {
                    management_config.remove(&existing_key);
                }
            }
            5 => {}
            _ => {}
        }
    }

    write_json_file_from_struct(&management_config, Path::new(file_name))
        .map_err(|_| BuildManagementConfigError::WriteJsonFileFromStructFailure)?;
    Ok(())
}

pub fn insert_new_precise_management(
    key: String,
    config: &mut HashMap<String, PreciseManagement>,
    how_to_insert: ReplaceOrNew,
) -> Result<(), ()> {
    let mut managements: Vec<Vec<Vec<SetStyle>>> = Vec::new();
    let percentages: Vec<u8> = get_percentages().map_err(|_| ())?;

    while managements.len() != percentages.len() {
        if let Some(value) = percentages.get(managements.len()) {
            println!(
                "Entering a session to define the rules for {}% of relevant sets",
                value
            );
        }
        managements.push(get_management_style_for_percentage().map_err(|_| ())?);
    }

    let new_management = PreciseManagement::new(percentages, managements)
        .ok_or_else(|| ())
        .map_err(|_| ())?;

    match config.get_mut(&key) {
        Some(entry) => match how_to_insert {
            ReplaceOrNew::Replace => {
                *entry = new_management;
            }
            ReplaceOrNew::New => {
                println!("A management with that key already exists, ignoring..");
            }
        },
        None => {
            config.insert(key, new_management);
        }
    }
    Ok(())
}

pub fn get_user_input(label: &str) -> Result<String, ()> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(label)
        .interact_text()
        .map_err(|_| ())?;

    Ok(input)
}
pub fn get_management_style_for_percentage() -> Result<Vec<Vec<SetStyle>>, ()> {
    let mut styles: Vec<Vec<SetStyle>> = Vec::new();

    let choices = vec![
        "Finish",
        "New Rule",
        "Append to Existing Rule",
        "Reset Exisiting",
        "Override Existing",
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option:")
            .items(&choices)
            .default(0)
            .interact()
            .map_err(|_| ())?;

        match selection {
            0 => break,
            1 => {
                let mut new: Vec<SetStyle> = Vec::new();
                new.push(choose_style_and_m_n_values().map_err(|_| ())?);

                styles.push(new)
            }
            2 => {
                let mut_ref: &mut Vec<SetStyle> =
                    pick_which_existing_style_to_modify(&mut styles).map_err(|_| ())?;

                mut_ref.push(choose_style_and_m_n_values().map_err(|_| ())?);
            }
            3 => {
                let mut_ref: &mut Vec<SetStyle> =
                    pick_which_existing_style_to_modify(&mut styles).map_err(|_| ())?;

                *mut_ref = Vec::<SetStyle>::new();
            }
            4 => {
                let mut new: Vec<SetStyle> = Vec::new();
                new.push(choose_style_and_m_n_values().map_err(|_| ())?);

                let mut_ref: &mut Vec<SetStyle> =
                    pick_which_existing_style_to_modify(&mut styles).map_err(|_| ())?;

                *mut_ref = new;
            }
            _ => {}
        }
    }
    Ok(styles)
}

pub fn pick_which_existing_style_to_modify(
    styles: &mut Vec<Vec<SetStyle>>,
) -> Result<&mut Vec<SetStyle>, ()> {
    let options: Vec<String> = styles
        .iter()
        .map(|element| ManageSetsType::vec_style_to_string(element))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the ruleset to affect:")
        .items(&options)
        .default(0)
        .interact()
        .map_err(|_| ())?;

    styles.get_mut(selection).ok_or_else(|| ())
}

pub fn get_percentages() -> Result<Vec<u8>, ()> {
    println!(
        "You will now choose the percentages on which the different styles will apply. Once your percentages sum to exactly 100, the session will automatically finish"
    );

    let mut percentages: Vec<u8> = Vec::new();
    let mut sum_of_perctanges: u8 = 0;

    while sum_of_perctanges != 100 {
        let new_number: u8 = get_number_input("Insert new percentage: ", true).map_err(|_| ())?;

        if new_number > 100 {
            println!("Input number exceeds 100.");
            continue;
        }

        if sum_of_perctanges + new_number > 100 {
            println!("Total sum of all percentages exceeds 100")
        }

        percentages.push(new_number);

        sum_of_perctanges = percentages.iter().sum();
    }

    Ok(percentages)
}

pub enum ReplaceOrNew {
    Replace,
    New,
}
