use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
};

use dialoguer::{Input, Select, theme::ColorfulTheme};
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    commands::manage_sets::{
        containers::{ManageSetsType, SetStyle},
        utils::{
            choose_style_and_m_n_values, filter_files_from_styles, styles_to_string,
            vec_style_to_string,
        },
    },
    containers::cleansweep_file_paths::CleansweepFilePaths,
    systems::json_io::{JsonReadError, read_file_to_struct, write_json_file_from_struct},
    utils::run_time_user_input::get_number_input,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PreciseManagementStyle {
    // Percentages must sum to 100. Its length must be exactly equal to the length of rulesets
    Percentage(Vec<u8>),
    // Each number should be bigger than the previous
    // length must be exactly one less than the length of the ruleset (ruleset must account for
    // 'other')
    UpUntil(Vec<f32>),
    // length must be exactly one less than the length of the ruleset
    NumFiles(Vec<u32>),
}

pub fn vec_to_string<T: Display>(vec: &Vec<T>) -> String {
    vec.iter()
        .enumerate()
        .map(|(index, num)| {
            let maybe_comma = if index == vec.len() - 1 { "" } else { "," };
            format!("{}{}", num, maybe_comma)
        })
        .fold(String::new(), |mut string, str| {
            string = format!("{string}{str}");

            string
        })
}
impl PreciseManagementStyle {
    pub fn len(&self) -> usize {
        match self {
            PreciseManagementStyle::Percentage(v) => v.len(),
            PreciseManagementStyle::UpUntil(v) => v.len(),
            PreciseManagementStyle::NumFiles(v) => v.len(),
        }
    }
    pub fn as_string(&self) -> String {
        match self {
            PreciseManagementStyle::Percentage(v) => vec_to_string(v),
            PreciseManagementStyle::NumFiles(v) => vec_to_string(v),
            PreciseManagementStyle::UpUntil(v) => vec_to_string(v),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreciseManagement {
    pub style: PreciseManagementStyle,
    pub managements: Vec<Vec<Vec<SetStyle>>>,
}

impl PreciseManagement {
    pub fn new(
        style: PreciseManagementStyle,
        managements: Vec<Vec<Vec<SetStyle>>>,
    ) -> Option<Self> {
        match &style {
            PreciseManagementStyle::Percentage(_) => {
                if style.len() == managements.len() {
                    Some(Self { style, managements })
                } else {
                    None
                }
            }
            _ => {
                if style.len() + 1 == managements.len() {
                    Some(Self { style, managements })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ManageSetsPrecisionModeError {
    #[error("Failed to read the provided precise management file into the expected type - {0}")]
    ReadFileToStructError(JsonReadError),

    #[error("Failed to index the first item in a set")]
    GetFirstItemInSetFailure,

    #[error(
        "Error trying to turn the full list of files in a set into its split ones based on the percentages"
    )]
    TurnSetIntoPreciseListFailure,

    #[error("Error trying to filter the files of a set based on the provided styles")]
    FilterFilesFromStylesFailure,

    #[error("Error trying to write a json file from a struct")]
    WriteJsonFileFromStructFailure,
}

pub fn apply_precision_mode(
    cleansweep_dir: &PathBuf,
    managed_sets: &mut Vec<ManageSetsType>,
    file_name: &str,
) -> Result<(), ManageSetsPrecisionModeError> {
    let config_file: HashMap<String, PreciseManagement> = read_file_to_struct(Path::new(file_name))
        .map_err(|e| ManageSetsPrecisionModeError::ReadFileToStructError(e))?;

    let mut keep_list: Vec<String> = Vec::new();
    let mut delete_list: Vec<String> = Vec::new();

    while let Some(mut set) = managed_sets.pop() {
        let first_in_set = set
            .full_set
            .get(0)
            .ok_or_else(|| ())
            .map_err(|_| ManageSetsPrecisionModeError::GetFirstItemInSetFailure)?
            .clone();

        let mut filtered = false;
        // INFO: Served on a first come first serve basis to encourage high precision when naming
        // keys
        // TODO: Document that
        for key in config_file.keys() {
            // default is applied as a last resort at the end
            if key != "default" {
                if let Some((key, value)) = config_file.get_key_value(key) {
                    if !first_in_set.contains(key) {
                        continue;
                    }

                    let style: &PreciseManagementStyle = &value.style;
                    let managements: &Vec<Vec<Vec<SetStyle>>> = &value.managements;

                    let mut separated_lists = turn_set_into_precise_list(style, &mut set.full_set)
                        .map_err(|_| ManageSetsPrecisionModeError::TurnSetIntoPreciseListFailure)?;

                    for (index, mut list) in separated_lists.iter_mut().enumerate() {
                        let empty = &vec![];
                        let management_style: &Vec<Vec<SetStyle>> =
                            managements.get(index).map_or(empty, |v| v);

                        filter_files_from_styles(
                            &mut list,
                            management_style,
                            &mut keep_list,
                            &mut delete_list,
                        )
                        .map_err(|_| ManageSetsPrecisionModeError::FilterFilesFromStylesFailure)?;
                    }
                    filtered = true;
                    break;
                }
            }
        }

        if let Some(default) = config_file.get("default")
            && !filtered
        {
            let style: &PreciseManagementStyle = &default.style;
            let managements: &Vec<Vec<Vec<SetStyle>>> = &default.managements;

            let mut separated_lists = turn_set_into_precise_list(style, &mut set.full_set)
                .map_err(|_| ManageSetsPrecisionModeError::TurnSetIntoPreciseListFailure)?;

            for (index, mut list) in separated_lists.iter_mut().enumerate() {
                let empty = &vec![];
                let management_style: &Vec<Vec<SetStyle>> =
                    managements.get(index).map_or(empty, |v| v);

                filter_files_from_styles(
                    &mut list,
                    management_style,
                    &mut keep_list,
                    &mut delete_list,
                )
                .map_err(|_| ManageSetsPrecisionModeError::FilterFilesFromStylesFailure)?;
            }
        }
    }

    println!("Overriding Keep list with:");
    keep_list.iter().for_each(|item| println!("- {item}"));

    println!("Overriding Delete list with:");
    delete_list.iter().for_each(|item| println!("- {item}"));

    write_json_file_from_struct(
        &keep_list,
        cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()),
    )
    .map_err(|_| ManageSetsPrecisionModeError::WriteJsonFileFromStructFailure)?;

    write_json_file_from_struct(
        &delete_list,
        cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()),
    )
    .map_err(|_| ManageSetsPrecisionModeError::WriteJsonFileFromStructFailure)?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum BuildManagementConfigError {
    #[error(
        "Failed trying to create the outer-most selection menu when building a management config"
    )]
    SelectionFailure,
    #[error("Failure trying to create a new PreciseManagement object")]
    InsertNewPreciseManagementFailure,
    #[error("Failure attempting to get user input")]
    GetUserInputFailure,
    #[error("Failure attempting to replace an existing PreciseManagement object")]
    ReplaceExistingPreciseManagementFailure,
    #[error("Failure trying to write a json file from a structure")]
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
            .with_prompt("Choose whether to add/modify a precise management style")
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
            5 => {
                println!("Printing the currently stored management config:");

                management_config.iter_mut().for_each(|(key, value)| {
                    let perc_string: String = value.style.as_string();
                    let managements: Vec<String> = value
                        .managements
                        .iter()
                        .map(|rule_set| format!("{}", styles_to_string(rule_set)))
                        .collect();

                    println!("{} : {{", key);
                    println!("  Percentages : {}", perc_string);
                    println!("  Managements : {{");
                    managements
                        .iter()
                        .for_each(|string| println!("    {}", string));
                    println!("  }}");
                    println!("}}");
                });
            }
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
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose what style of precise management you would like for this key:")
        .items(vec![
            "Percentages",
            "Number Of Files",
            "File IDs Up Until N",
        ])
        .default(0)
        .interact()
        .map_err(|_| ())?;

    let chosen_style = match selection {
        0 => Ok(PreciseManagementStyle::Percentage(vec![])),
        1 => Ok(PreciseManagementStyle::NumFiles(vec![])),
        2 => Ok(PreciseManagementStyle::UpUntil(vec![])),
        _ => Err(()),
    }
    .map_err(|_| ())?;

    let style: PreciseManagementStyle = match chosen_style {
        PreciseManagementStyle::Percentage(_) => {
            PreciseManagementStyle::Percentage(get_percentages().map_err(|_| ())?)
        }
        PreciseManagementStyle::NumFiles(_) => {
            PreciseManagementStyle::NumFiles(get_num_files_vector().map_err(|_| ())?)
        }
        PreciseManagementStyle::UpUntil(_) => {
            PreciseManagementStyle::UpUntil(get_up_until_vector().map_err(|_| ())?)
        }
    };

    let num_to_match = match chosen_style {
        PreciseManagementStyle::Percentage(_) => 0,
        _ => 1,
    };

    while managements.len() != style.len() + num_to_match {
        match &style {
            PreciseManagementStyle::Percentage(percentages) => {
                if let Some(value) = percentages.get(managements.len()) {
                    println!(
                        "Entering a session to define the rules for {}% of relevant sets",
                        value
                    );
                }
            }
            PreciseManagementStyle::UpUntil(values) => {
                if let Some(val) = values.get(managements.len()) {
                    println!(
                        "Entering a session to define the rules for files with an ID up to {}",
                        val
                    );
                }
            }
            PreciseManagementStyle::NumFiles(values) => {
                let keyword = if managements.len() == 0 {
                    "first"
                } else {
                    "next"
                };

                if let Some(val) = values.get(managements.len()) {
                    println!("Entering a session to define the {} {} files", keyword, val);
                }
            }
        }
        managements.push(get_management_ruleset_for_key().map_err(|_| ())?);
    }

    let new_management = PreciseManagement::new(style, managements)
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
pub fn get_management_ruleset_for_key() -> Result<Vec<Vec<SetStyle>>, ()> {
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
            .with_prompt("Choose how to implement the new ruleset:")
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
        .map(|element| vec_style_to_string(element))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the ruleset to affect:")
        .items(&options)
        .default(0)
        .interact()
        .map_err(|_| ())?;

    styles.get_mut(selection).ok_or_else(|| ())
}

pub fn get_num_files_vector() -> Result<Vec<u32>, ()> {
    println!(
        "You will now choose the vector of file numbers on which the different styles will apply. \n\nNote that you will also specify a ruleset for any leftover files at the end"
    );

    let mut nums: Vec<u32> = Vec::new();

    let number_of_fields: usize =
        get_number_input("Input how many rulesets you want: ", true).map_err(|_| ())?;

    while nums.len() != number_of_fields {
        let new_number: u32 = get_number_input("Insert new value: ", true).map_err(|_| ())?;

        nums.push(new_number);
    }
    Ok(nums)
}

pub fn get_up_until_vector() -> Result<Vec<f32>, ()> {
    println!(
        "You will now choose the vector of maximum IDs on which the different styles will apply. \n\nEnsure they are ascending order and are unique. You will also specify a ruleset for any leftover files"
    );

    let mut nums: Vec<f32> = Vec::new();

    let number_of_fields: usize =
        get_number_input("Input how many rulesets you want: ", true).map_err(|_| ())?;

    while nums.len() != number_of_fields {
        let new_number: f32 = get_number_input("Insert new value: ", true).map_err(|_| ())?;

        if let Some(last) = nums.last() {
            if new_number <= *last {
                println!("Please ensure that every value is unique and is bigger than the last",);
            } else {
                nums.push(new_number);
            }
        } else {
            nums.push(new_number);
        }
    }
    Ok(nums)
}

pub fn get_percentages() -> Result<Vec<u8>, ()> {
    println!(
        "You will now choose the percentages on which the different styles will apply. \n\nOnce your percentages sum to exactly 100, the session will automatically finish"
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

fn turn_set_into_precise_list(
    style: &PreciseManagementStyle,
    list: &mut Vec<String>,
) -> Result<Vec<Vec<String>>, String> {
    // confirm if the style is valid
    match style {
        PreciseManagementStyle::Percentage(percentages) => {
            let percentages_sum: u8 = percentages.iter().sum();
            if percentages_sum != 100 {
                return Err("Percentages don't sum to 100".to_string());
            }
        }
        PreciseManagementStyle::UpUntil(values) => {
            let _: Option<&f32> = values
                .iter()
                .try_fold(
                    None,
                    |mut _prev_value, curr_val| -> Result<Option<&f32>, ()> {
                        if let Some(val) = _prev_value {
                            if val >= curr_val {
                                return Err(());
                            } else {
                                _prev_value = Some(curr_val);
                            }
                        } else {
                            _prev_value = Some(curr_val);
                        }
                        Ok(Some(curr_val))
                    },
                )
                .map_err(|_| "Values are not unique and/or in ascending order".to_string())?;
        }
        PreciseManagementStyle::NumFiles(_) => {}
    }

    let original_list_length = list.len();

    // Allows the usage of pop
    // Treat the list as a stack
    list.reverse();

    match style {
        PreciseManagementStyle::Percentage(percentages) => {
            let precise_list: Vec<Vec<String>> = percentages.into_iter().fold(
                Vec::<Vec<String>>::new(),
                |mut precise, curr_percentage| {
                    let num_files_to_extract: usize = ((original_list_length as f32)
                        * (*curr_percentage as f32 / 100.0))
                        as usize;

                    let mut new_list: Vec<String> = vec![];

                    for _ in 0..=num_files_to_extract {
                        if let Some(item) = list.pop() {
                            new_list.push(item);
                        }
                    }
                    precise.push(new_list);

                    precise
                },
            );

            Ok(precise_list)
        }
        PreciseManagementStyle::UpUntil(values) => {
            // First need to remove an extension if there is one
            let full_number_regex = match Regex::new(r"(\d+(\.\d+)?)\.[^\.]*$") {
                Ok(new) => new,
                Err(_) => return Err("Failed to create regex".to_string()),
            };

            let mut list_clone = list.clone();

            let mut precise_list: Vec<Vec<String>> = values
                .into_iter()
                .try_fold(
                    Vec::<Vec<String>>::new(),
                    |mut precise, curr_value| -> Result<Vec<Vec<String>>, ()> {
                        let mut curr: Vec<String> = Vec::new();
                        let mut leftovers: Vec<String> = Vec::new();
                        while list_clone.len() != 0 {
                            if let Some(extracted) = list_clone.pop() {
                                if !full_number_regex.is_match(&extracted) {
                                    return Err(());
                                }
                                let capture = match full_number_regex
                                    .captures(&extracted)
                                    .ok_or_else(|| ())
                                {
                                    Ok(new) => new,
                                    Err(_) => return Err(()),
                                };
                                let number_portion = match capture.get(1).ok_or_else(|| ()) {
                                    Ok(new) => new.as_str(),
                                    Err(_) => return Err(()),
                                };
                                let extracted_number = match number_portion.parse::<f32>() {
                                    Ok(result) => result,
                                    Err(_) => return Err(()),
                                };

                                if extracted_number <= *curr_value {
                                    curr.push(extracted);
                                } else {
                                    leftovers.push(extracted);
                                }
                            }
                        }
                        leftovers.reverse();
                        list_clone = leftovers.clone();
                        precise.push(curr);

                        Ok(precise)
                    },
                )
                .map_err(|_| "")?;
            // Undo the last reverse
            list_clone.reverse();
            precise_list.push(list_clone);

            Ok(precise_list)
        }
        PreciseManagementStyle::NumFiles(nums) => {
            let mut precise_list: Vec<Vec<String>> =
                nums.into_iter()
                    .fold(Vec::<Vec<String>>::new(), |mut precise, curr_value| {
                        let mut curr: Vec<String> = Vec::new();
                        let mut leftovers: Vec<String> = Vec::new();
                        let mut index = 0;
                        while list.len() != 0 {
                            if let Some(extracted) = list.pop() {
                                if index < *curr_value {
                                    curr.push(extracted);
                                } else {
                                    list.push(extracted);
                                    break;
                                }
                            }
                            index = index + 1;
                        }
                        leftovers.reverse();
                        precise.push(curr);
                        precise
                    });
            precise_list.push(list.clone());

            Ok(precise_list)
        }
    }
}
