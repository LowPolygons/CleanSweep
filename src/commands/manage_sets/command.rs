use dialoguer::{Select, theme::ColorfulTheme};
use regex::Regex;
use thiserror::Error;

use crate::{
    commands::manage_sets::{
        containers::{
            AppendOrOverride, ChoiceInGettingStyle, ManageSetsType, NewStyleBehaviour,
            NotAffectingStyles, SetStyle, ZeroOrOne,
        },
        print_table::{Column, PrintableTable},
    },
    containers::{
        cleansweep_file_paths::CleansweepFilePaths, sets_read_write_type::SetsReadWriteType,
    },
    systems::json_io::{read_file_to_struct, write_json_file_from_struct},
    utils::{get_common_dirs::get_cleansweep_dir, run_time_user_input::get_number_input},
};

#[derive(Debug, Error)]
pub enum ManageSetsError {
    #[error("Failed when attempting to get the cleansweep directory")]
    GetCleansweepDirectoryFailure,

    #[error("Failed when trying to turn the sets file into the internal structure")]
    ReadSetsFileToStructFailure,

    #[error("Failed to index the first item in a set")]
    GetFirstItemInSetFailure,

    #[error("Failed to create the interactive menu for listing the sets")]
    ListSetsToManageFailure,

    #[error("Failed to choose the management style for the chosen option correctly")]
    ChooseManagementStyleFailure,

    #[error("Failed when deciding how to get the new set list")]
    GetChoiceOfHowToGetStyleFailure,

    #[error("Failed when trying to apply a chosen set to the target list")]
    ApplyingNewStyleToSetFailure,

    #[error(
        "Failed when trying to preview how a set will be separated based on its current styles"
    )]
    PreviewSeparatedFilesBasedOnStylesFailure,

    #[error("Failed to get a mutable reference to the chosen set")]
    GetMutRefToChosenSetFailure,

    #[error("Failed to save struct of files to the corresponding json file")]
    WriteJsonFileFromStructFailure,
}

// TODO: Perhaps there has been a coupling? Split the logic of multiplying by the decimal portion
// off if necessary
pub fn extract_number_from_string(file: &str) -> Option<(i64, i64)> {
    // First need to remove an extension if there is one
    let full_number = match Regex::new(r"(\d+(\.\d+)?)\.[^\.]*$") {
        Ok(new) => new,
        Err(_) => return None,
    };
    let after_decimal = match Regex::new(r"\.(\d+?)\.[^\.]*$") {
        Ok(new) => new,
        Err(_) => return None,
    };

    if !full_number.is_match(&file) {
        return None;
    }

    // Capture the full number
    let captures = match full_number.captures(file).ok_or_else(|| ()) {
        Ok(new) => new,
        Err(_) => return None,
    };
    let number_portion = match captures.get(1).ok_or_else(|| ()) {
        Ok(new) => new.as_str(),
        Err(_) => return None,
    };
    let actual_number = match number_portion.parse::<f64>() {
        Ok(result) => result,
        Err(_) => return None,
    };

    // Capture the after-decimal portion
    // If it doesnt find a decimal then the number is just returnable as it is
    let decimal_captures = match after_decimal.captures(file).ok_or_else(|| ()) {
        Ok(new) => new,
        Err(_) => return Some((actual_number as i64, 1)),
    };
    let decimal_portion = match decimal_captures.get(1).ok_or_else(|| ()) {
        Ok(new) => new,
        Err(_) => return Some((actual_number as i64, 1)),
    };

    let multiplier: u32 = decimal_portion.as_str().chars().count() as u32;

    let ten: i64 = 10;

    Some((
        (actual_number * (ten.pow(multiplier) as f64)) as i64,
        (ten.pow(multiplier) as i64),
    ))
}

// WARN: The new table mode made the short-mode possibly unecessary
pub fn manage_sets(_: &bool) -> Result<(), ManageSetsError> {
    let cleansweep_dir =
        get_cleansweep_dir().map_err(|_| ManageSetsError::GetCleansweepDirectoryFailure)?;

    let scanned_sets: Vec<SetsReadWriteType> =
        read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::FoundSets.name()))
            .map_err(|_| ManageSetsError::ReadSetsFileToStructFailure)?;

    let mut dir_set_scan_was_run_from = String::new();

    /*
     * Map the SetsReadWriteType to a ManageSetsType, and determine the [PATH] variable in code
     */
    let mut managed_sets: Vec<ManageSetsType> = scanned_sets.iter().try_fold(
        Vec::<ManageSetsType>::new(),
        |mut acc, set| -> Result<Vec<ManageSetsType>, ManageSetsError> {
            let label = set
                .files
                .get(0)
                .ok_or(|| "No File")
                .map_err(|_| ManageSetsError::GetFirstItemInSetFailure)?
                .clone();

            if dir_set_scan_was_run_from.is_empty() && scanned_sets.len() != 1 {
                dir_set_scan_was_run_from = label.clone();
            }

            while label.find(&dir_set_scan_was_run_from).is_none() && scanned_sets.len() != 1 {
                dir_set_scan_was_run_from =
                    dir_set_scan_was_run_from[0..dir_set_scan_was_run_from.len() - 1].to_string()
            }

            let new_set = ManageSetsType {
                full_set: set.files.clone(),
                label,
                chosen_styles: Vec::new(),
            };

            acc.push(new_set);

            Ok(acc)
        },
    )?;

    /*
     * If the last char was a / dont store in the $PATH var
     */
    let maybe_last_char = dir_set_scan_was_run_from.chars().last();

    if let Some(last_char) = maybe_last_char {
        if last_char == '/' {
            dir_set_scan_was_run_from = dir_set_scan_was_run_from.chars().enumerate().fold(
                String::new(),
                |mut string, (index, character)| {
                    if index != dir_set_scan_was_run_from.len() - 1 {
                        string = format!("{}{}", string, character);
                    }
                    string
                },
            );
        }
    }

    let len_to_strip_away: usize = dir_set_scan_was_run_from.len();
    let mut first_in_sets: Vec<String>;

    println!(
        "References to PATH represent \"{}\"",
        dir_set_scan_was_run_from
    );

    /*
     * Each loop represents a full cycle of
     * - Picking a set
     * - Choosing a source of management style or previewing
     * - Choose a management style
     * (cycle)
     */
    loop {
        /*
         * Stage One : The user chooses the set to manage
         */
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
                        "{} : [PATH]{} - {} Files",
                        item.styles_to_string(),
                        item.label_truncated(len_to_strip_away),
                        item.full_set.len()
                    )
                })
                .collect(),
        );

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&first_in_sets)
            .default(0)
            .interact()
            .map_err(|_| ManageSetsError::ListSetsToManageFailure)?;

        if selection == 0 {
            // Chose to exit
            break;
        }
        if selection == 1 {
            println!(
                "Any sets where a provided 'N-Value' exceeds its length will not have a default applied"
            );
        }

        /*
         * Stage Two: Choose Whether to preview the set, or choose how to get the new management style
         */
        let is_default = selection == 1;

        let maybe_how_to_get_style = get_source_of_new_style(is_default)
            .map_err(|_| ManageSetsError::GetChoiceOfHowToGetStyleFailure)?;

        let how_to_get_style: NewStyleBehaviour;

        match maybe_how_to_get_style {
            ChoiceInGettingStyle::NotAffectingStyles(behaviour) => {
                match behaviour {
                    NotAffectingStyles::Preview => {
                        let mut keep_as_of_now: Vec<String> = Vec::new();
                        let mut delete_as_of_now: Vec<String> = Vec::new();

                        let mut ref_to_set = managed_sets
                            .get(selection - length_initial_first_in_sets)
                            .ok_or_else(|| ())
                            .map_err(|_| ManageSetsError::GetMutRefToChosenSetFailure)?
                            .clone();

                        filter_files_from_styles(
                            &mut ref_to_set,
                            &mut keep_as_of_now,
                            &mut delete_as_of_now,
                        )
                        .map_err(|_| ManageSetsError::PreviewSeparatedFilesBasedOnStylesFailure)?;

                        print_set_status_as_table(
                            &managed_sets[selection - length_initial_first_in_sets],
                            &keep_as_of_now,
                            &delete_as_of_now,
                        );
                    }
                    NotAffectingStyles::Back => { /* Just continue immediately */ }
                    NotAffectingStyles::FullTable => {
                        let ref_to_set = managed_sets
                            .get(selection - length_initial_first_in_sets)
                            .ok_or_else(|| ())
                            .map_err(|_| ManageSetsError::GetMutRefToChosenSetFailure)?
                            .clone();

                        print_full_set_as_table(&ref_to_set);
                    }
                }
                continue;
            }
            ChoiceInGettingStyle::AffectStoredStyles(behaviour) => {
                how_to_get_style = behaviour;
            }
        }
        /*
         * Stage Three : choose the management style given the choice of sourcing it
         */
        // Type must be representitive of the final style list per set
        // This allows the code for appending/resetting/overriding to be automatically compatible
        let new_styles: Vec<Vec<SetStyle>> = match &how_to_get_style {
            NewStyleBehaviour::Append => {
                vec![vec![choose_style_and_m_n_values().map_err(|_| {
                    ManageSetsError::ChooseManagementStyleFailure
                })?]]
            }
            NewStyleBehaviour::Set => {
                vec![vec![choose_style_and_m_n_values().map_err(|_| {
                    ManageSetsError::ChooseManagementStyleFailure
                })?]]
            }
            NewStyleBehaviour::Reset => vec![Vec::new()],
            NewStyleBehaviour::Copy => {
                copy_management_styles_from_set(&managed_sets, len_to_strip_away)
                    .map_err(|_| ManageSetsError::ChooseManagementStyleFailure)?
            }
        };

        let should_choose_index: bool = match &how_to_get_style {
            NewStyleBehaviour::Append => true,
            NewStyleBehaviour::Set => true,
            _ => false,
        };

        if selection == 1 {
            for mutable_ref_to_set in &mut managed_sets {
                if apply_style_to_set(
                    mutable_ref_to_set,
                    &how_to_get_style,
                    &new_styles,
                    selection != 1,
                )
                .map_err(|_| ManageSetsError::ApplyingNewStyleToSetFailure)?
                .contains(&false)
                {
                    println!(
                        "Some filters weren't applied due to 'N' values exceeding their length"
                    )
                }
            }
        } else {
            let mutable_ref_to_set = managed_sets
                .get_mut(selection - length_initial_first_in_sets)
                .ok_or_else(|| ())
                .map_err(|_| ManageSetsError::GetMutRefToChosenSetFailure)?;

            if apply_style_to_set(
                mutable_ref_to_set,
                &how_to_get_style,
                &new_styles,
                should_choose_index,
            )
            .map_err(|_| ManageSetsError::ApplyingNewStyleToSetFailure)?
            .contains(&false)
            {
                println!(
                    "Some filters weren't applied to a set due to 'N' values exceeding their length"
                )
            }
        }
    }
    /*
     * Stage Four : Apply the sets upon exit
     */
    let mut files_for_keep: Vec<String> = Vec::new();
    let mut files_for_delete: Vec<String> = Vec::new();

    for set in &mut managed_sets {
        match filter_files_from_styles(set, &mut files_for_keep, &mut files_for_delete) {
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
    .map_err(|_| ManageSetsError::WriteJsonFileFromStructFailure)?;

    write_json_file_from_struct(
        &files_for_delete,
        cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()),
    )
    .map_err(|_| ManageSetsError::WriteJsonFileFromStructFailure)?;

    Ok(())
}

pub fn print_full_set_as_table(chosen_set: &ManageSetsType) {
    let summed_widths: usize = chosen_set.full_set.iter().map(|file| file.len()).sum();
    let max_id_width: usize = chosen_set
        .full_set
        .iter()
        .map(|file| {
            let (id, _): (i64, i64) = extract_number_from_string(file).map_or((0, 1), |v| v);

            id.to_string().chars().count()
        })
        .max()
        .map_or(3, |v| v)
        + 3;

    let mut table = PrintableTable::new(Vec::new());

    table.new_column(Column {
        width: 12,
        lines: Vec::new(),
        title: "Pos in Set".to_string(),
    });
    table.new_column(Column {
        width: summed_widths / chosen_set.full_set.len(),
        lines: Vec::new(),
        title: "File Name".to_string(),
    });
    table.new_column(Column {
        width: max_id_width,
        lines: Vec::new(),
        title: "File ID".to_string(),
    });

    chosen_set.full_set.iter().for_each(|file_name| {
        if let Some(index_in_set) = chosen_set
            .full_set
            .iter()
            .position(|file| file == file_name)
        {
            let (id, _): (i64, i64) = extract_number_from_string(file_name).map_or((0, 1), |v| v);
            table.insert_row(vec![
                index_in_set.to_string(),
                file_name.clone(),
                id.to_string(),
            ]);
        }
    });
    let lines = table.get_printable_strings();

    lines.into_iter().for_each(|line| println!("{line}"));
}

pub fn print_set_status_as_table(
    chosen_set: &ManageSetsType,
    list_keep: &Vec<String>,
    list_delete: &Vec<String>,
) {
    let summed_widths: usize = chosen_set.full_set.iter().map(|file| file.len()).sum();
    let max_id_width: usize = chosen_set
        .full_set
        .iter()
        .map(|file| {
            let (id, _): (i64, i64) = extract_number_from_string(file).map_or((0, 1), |v| v);

            file.chars().count() - id.to_string().chars().count()
        })
        .max()
        .map_or(3, |v| v);

    let mut table = PrintableTable::new(Vec::new());

    table.new_column(Column {
        width: 12,
        lines: Vec::new(),
        title: "Pos in Set".to_string(),
    });
    table.new_column(Column {
        width: 8,
        lines: Vec::new(),
        title: "List".to_string(),
    });
    table.new_column(Column {
        width: summed_widths / chosen_set.full_set.len(),
        lines: Vec::new(),
        title: "File Name".to_string(),
    });
    table.new_column(Column {
        width: max_id_width,
        lines: Vec::new(),
        title: "File ID".to_string(),
    });

    list_keep.iter().for_each(|file_name| {
        if let Some(index_in_set) = chosen_set
            .full_set
            .iter()
            .position(|file| file == file_name)
        {
            let (id, _): (i64, i64) = extract_number_from_string(file_name).map_or((0, 1), |v| v);
            table.insert_row(vec![
                index_in_set.to_string(),
                "Keep".to_string(),
                file_name.clone(),
                id.to_string(),
            ]);
        }
    });

    let mut num_files_hidden: usize = 0;

    list_delete
        .iter()
        .enumerate()
        .for_each(|(index, file_name)| {
            if index <= 6 || index >= list_delete.len() - 6 {
                if let Some(index_in_set) = chosen_set
                    .full_set
                    .iter()
                    .position(|file| file == file_name)
                {
                    table.insert_row(vec![
                        index_in_set.to_string(),
                        "Delete".to_string(),
                        file_name.clone(),
                    ]);
                }
            } else {
                num_files_hidden = num_files_hidden + 1;
            }
        });

    table.insert_row(vec![
        "...".to_string(),
        "Delete".to_string(),
        format!("{} Files Hidden", num_files_hidden),
        "N/A".to_string(),
    ]);

    let lines = table.get_printable_strings();

    lines.into_iter().for_each(|line| println!("{line}"));
}

fn choose_which_style_to_affect(set_styles: &mut Vec<Vec<SetStyle>>) -> Result<usize, ()> {
    let mut list_items: Vec<String> =
        set_styles
            .iter()
            .fold(Vec::<String>::new(), |mut list_items, curr_style_list| {
                list_items.push(ManageSetsType::vec_style_to_string(curr_style_list));

                list_items
            });
    list_items.push(String::from("Add new"));

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose which Management Style to affect, or add a new one")
        .items(&list_items)
        .default(&list_items.len() - 1)
        .interact()
        .map_err(|_| ())?;

    if selection == &list_items.len() - 1 {
        set_styles.push(Vec::new())
    }

    Ok(selection)
}

fn apply_style_to_set(
    mutable_ref_to_set: &mut ManageSetsType,
    how_style_was_made: &NewStyleBehaviour,
    new_styles: &Vec<Vec<SetStyle>>,
    choose_index: bool,
) -> Result<Vec<bool>, ()> {
    let mut any_failed: Vec<bool> = Vec::new();

    match how_style_was_made {
        NewStyleBehaviour::Append => {
            let passed_index = if choose_index {
                Some(
                    choose_which_style_to_affect(&mut mutable_ref_to_set.chosen_styles)
                        .map_err(|_| ())?,
                )
            } else {
                None
            };

            for current_style in new_styles {
                if try_apply_style_to_set(
                    mutable_ref_to_set,
                    current_style,
                    AppendOrOverride::Append,
                    passed_index,
                )
                .map_err(|_| ())?
                .contains(&false)
                {
                    any_failed.push(false);
                }
            }
        }
        NewStyleBehaviour::Set => {
            let passed_index = if choose_index {
                Some(
                    choose_which_style_to_affect(&mut mutable_ref_to_set.chosen_styles)
                        .map_err(|_| ())?,
                )
            } else {
                None
            };

            for current_style in new_styles {
                if try_apply_style_to_set(
                    mutable_ref_to_set,
                    current_style,
                    AppendOrOverride::Override,
                    passed_index,
                )
                .map_err(|_| ())?
                .contains(&false)
                {
                    any_failed.push(false);
                }
            }
        }
        // Not affected by the should_choose_index
        NewStyleBehaviour::Copy => {
            mutable_ref_to_set.chosen_styles = Vec::new();

            for current_style in new_styles {
                if try_apply_style_to_set(
                    mutable_ref_to_set,
                    current_style,
                    AppendOrOverride::Append,
                    None,
                )
                .map_err(|_| ())?
                .contains(&false)
                {
                    any_failed.push(false);
                }
            }
        }
        NewStyleBehaviour::Reset => mutable_ref_to_set.chosen_styles = new_styles.clone(),
    }
    Ok(any_failed)
}

fn get_source_of_new_style(for_defaults: bool) -> Result<ChoiceInGettingStyle, ()> {
    let mut options = vec![
        "Back",
        "Append To List",
        "Reset Style List",
        "Override Set List",
        "Copy Other Set List",
    ];
    if !for_defaults {
        options.push("Preview Current Style Effects");
        options.push("View full set");
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose how to pick the management style")
        .items(options)
        .default(0)
        .interact()
        .map_err(|_| ())?;

    match selection {
        0 => Ok(ChoiceInGettingStyle::NotAffectingStyles(
            NotAffectingStyles::Back,
        )),
        1 => Ok(ChoiceInGettingStyle::AffectStoredStyles(
            NewStyleBehaviour::Append,
        )),
        2 => Ok(ChoiceInGettingStyle::AffectStoredStyles(
            NewStyleBehaviour::Reset,
        )),
        3 => Ok(ChoiceInGettingStyle::AffectStoredStyles(
            NewStyleBehaviour::Set,
        )),
        4 => Ok(ChoiceInGettingStyle::AffectStoredStyles(
            NewStyleBehaviour::Copy,
        )),
        5 => Ok(ChoiceInGettingStyle::NotAffectingStyles(
            NotAffectingStyles::Preview,
        )),
        6 => Ok(ChoiceInGettingStyle::NotAffectingStyles(
            NotAffectingStyles::FullTable,
        )),
        _ => Err(()),
    }
}

fn copy_management_styles_from_set(
    set_list: &Vec<ManageSetsType>,
    len_to_strip: usize,
) -> Result<Vec<Vec<SetStyle>>, ()> {
    let first_in_sets: Vec<String> = set_list
        .iter()
        .try_fold(
            Vec::<String>::new(),
            |mut first_in_sets, chosen_set| -> Result<Vec<String>, ()> {
                let first_item = chosen_set
                    .full_set
                    .get(0)
                    .ok_or_else(|| ())
                    .map_err(|_| ())?;

                first_in_sets.push(format!(
                    "{} - [PATH]{}",
                    chosen_set.styles_to_string(),
                    first_item
                        .clone()
                        .drain(len_to_strip..first_item.len())
                        .fold(String::new(), |mut string, char| {
                            string = format!("{}{}", string, char);
                            string
                        })
                ));

                Ok(first_in_sets)
            },
        )
        .map_err(|_| ())?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a set to clone the parameters of")
        .items(first_in_sets)
        .default(0)
        .interact()
        .map_err(|_| ())?;

    Ok(set_list
        .get(selection)
        .ok_or_else(|| ())
        .map_err(|_| ())?
        .chosen_styles
        .clone())
}

fn try_apply_style_to_set(
    set: &mut ManageSetsType,
    styles: &Vec<SetStyle>,
    append_or_override: AppendOrOverride,
    maybe_index_of_set: Option<usize>,
) -> Result<Vec<bool>, ()> {
    let mut can_corresponding_style_apply: Vec<bool> = Vec::new();

    // Check if any fail
    styles.iter().for_each(|style| {
        can_corresponding_style_apply.push(n_m_values_work_for_set(set, style));
    });

    if let Some(index_of_set) = maybe_index_of_set {
        match append_or_override {
            AppendOrOverride::Append => {}
            AppendOrOverride::Override => {
                *set.chosen_styles
                    .get_mut(index_of_set)
                    .ok_or_else(|| ())
                    .map_err(|_| ())? = Vec::new();
            }
        }
        for (index, style) in styles.iter().enumerate() {
            // The length of can_corresponding_style_apply is by definition the same as
            // index which is why there is no concern for an else branch
            if let Some(is_valid) = can_corresponding_style_apply.get(index) {
                if *is_valid {
                    set.chosen_styles
                        .get_mut(index_of_set)
                        .ok_or_else(|| ())
                        .map_err(|_| ())?
                        .push(style.clone())
                }
            }
        }
    } else {
        // They chose Copy
        // the list will have been reset
        let sorted_styles: Vec<SetStyle> = styles
            .into_iter()
            .enumerate()
            .filter(
                |(index, _)| match can_corresponding_style_apply.get(*index) {
                    Some(is_valid) => *is_valid,
                    _ => false,
                },
            )
            .fold(Vec::<SetStyle>::new(), |mut list, (_, style)| {
                list.push(style.clone());
                list
            });

        set.chosen_styles.push(sorted_styles)
    }

    Ok(can_corresponding_style_apply)
}

fn n_m_values_work_for_set(set: &ManageSetsType, style: &SetStyle) -> bool {
    match &style {
        SetStyle::FirstN(n_value) => *n_value <= set.full_set.len(),
        SetStyle::LastN(n_value) => *n_value <= set.full_set.len(),
        SetStyle::FirstNandLastM(n_value, m_value) => {
            *n_value <= set.full_set.len() || *m_value <= set.full_set.len()
        }
        SetStyle::EveryNIndexed(n_value, _) => *n_value <= set.full_set.len(),
        SetStyle::EvenlySpacedN(n_value) => *n_value <= set.full_set.len(),
        _ => true,
    }
}

fn choose_style_and_m_n_values() -> Result<SetStyle, ()> {
    let sub_options: Vec<SetStyle> = vec![
        SetStyle::First,
        SetStyle::Last,
        SetStyle::FirstAndLast,
        SetStyle::FirstN(0),
        SetStyle::LastN(0),
        SetStyle::FirstNandLastM(0, 0),
        SetStyle::EveryNIndexed(0, ZeroOrOne::Zero),
        SetStyle::EveryNIndexed(0, ZeroOrOne::One),
        SetStyle::EvenlySpacedN(0),
        SetStyle::IDisDivisibleByN(0),
        SetStyle::NumberDivisibleByN(0.0),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a management style:")
        .items(&sub_options)
        .interact()
        .map_err(|_| ())?;

    Ok(
        match sub_options
            .get(selection)
            .ok_or_else(|| ())
            .map_err(|_| ())?
            .clone()
        {
            SetStyle::FirstN(_) => {
                let n_value: usize =
                    get_number_input("Enter how many of the first files you wish to keep:", true)
                        .map_err(|_| ())?;

                SetStyle::FirstN(n_value)
            }
            SetStyle::LastN(_) => {
                let n_value: usize =
                    get_number_input("Enter how many of the last files you wish to keep:", true)
                        .map_err(|_| ())?;

                SetStyle::LastN(n_value)
            }
            SetStyle::FirstNandLastM(_, _) => {
                let n_value: usize =
                    get_number_input("Enter how many of the first files you wish to keep:", true)
                        .map_err(|_| ())?;

                let m_value: usize =
                    get_number_input("Enter how many of the last files you wish to keep:", true)
                        .map_err(|_| ())?;

                SetStyle::FirstNandLastM(n_value, m_value)
            }
            SetStyle::EveryNIndexed(_, zero_or_one) => {
                let n_value: usize = get_number_input(
                    "Enter the number of how often to save a file when interating over the set: ",
                    true,
                )
                .map_err(|_| ())?;

                SetStyle::EveryNIndexed(n_value, zero_or_one)
            }
            SetStyle::EvenlySpacedN(_) => {
                println!(
                    "This will, on average save exactly N files. There will be a margin of error if N > len / 2"
                );
                let n_value: usize =
                    get_number_input("Enter how many files do you want to save: ", true)
                        .map_err(|_| ())?;
                SetStyle::EvenlySpacedN(n_value)
            }
            SetStyle::IDisDivisibleByN(_) => {
                println!(
                    "The ID is calculated by removing the decimal place if it is present and tacking the decimal number portion onto the end (123.456 -> 123456)"
                );
                let n_value: usize = get_number_input(
                    "Enter the number a files ID should be divisible by to keep: ",
                    true,
                )
                .map_err(|_| ())?;
                SetStyle::IDisDivisibleByN(n_value)
            }
            SetStyle::NumberDivisibleByN(_) => {
                println!(
                    "The number is calculated by removing the decimal place if it is present and tacking the decimal number portion onto the end (123.456 -> 123456)"
                );
                let n_value: f64 = get_number_input(
                    "Enter the number a files ID should be divisible by to keep: ",
                    true,
                )
                .map_err(|_| ())?;
                SetStyle::NumberDivisibleByN(n_value)
            }
            other => other,
        },
    )
}

fn push_if_new(list: &mut Vec<String>, new_item: String) {
    if !list.contains(&new_item) {
        list.push(new_item)
    }
}

fn filter_files_from_styles(
    chosen_set: &mut ManageSetsType,
    keep_list: &mut Vec<String>,
    delete_list: &mut Vec<String>,
) -> Result<(), ()> {
    for current_style_list in &chosen_set.chosen_styles {
        // Apply Sets in order
        let len_of_set_sub_one = chosen_set.full_set.len() - 1;
        let mut new_set_list: Vec<String> = Vec::new();

        // These should apply simultaneously
        for current_style in current_style_list {
            match current_style {
                SetStyle::First => {
                    for (index, value) in chosen_set.full_set.iter().enumerate() {
                        match index {
                            0 => push_if_new(keep_list, value.clone()),
                            _ => {}
                        }
                    }
                }
                SetStyle::Last => {
                    for (index, value) in chosen_set.full_set.iter().enumerate() {
                        if index == len_of_set_sub_one {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::FirstAndLast => {
                    for (index, value) in chosen_set.full_set.iter().enumerate() {
                        if index == len_of_set_sub_one {
                            push_if_new(keep_list, value.clone());
                        } else {
                            if index == 0 {
                                push_if_new(keep_list, value.clone());
                            } else {
                                match index {
                                    0 => push_if_new(keep_list, value.clone()),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                SetStyle::FirstN(n_value) => {
                    for (index, value) in chosen_set.full_set.iter().enumerate() {
                        if index < *n_value {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::LastN(n_value) => {
                    for (index, value) in chosen_set.full_set.iter().enumerate() {
                        if index > len_of_set_sub_one.saturating_sub(*n_value) {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::FirstNandLastM(n_value, m_value) => {
                    for (index, value) in chosen_set.full_set.iter().enumerate() {
                        if index < *n_value {
                            push_if_new(keep_list, value.clone());
                        } else if index > (len_of_set_sub_one - m_value) {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::EveryNIndexed(n_value, zero_or_one) => {
                    let index_addition: usize = match zero_or_one {
                        ZeroOrOne::Zero => 0,
                        ZeroOrOne::One => 1,
                    };

                    for (index, value) in chosen_set.full_set.iter().enumerate() {
                        if index != 0 && (index + index_addition) % n_value == 0 {
                            push_if_new(keep_list, value.clone());
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
                                    push_if_new(keep_list, value.clone());
                                }
                            }
                        } else {
                            for (index, value) in chunk.iter().enumerate() {
                                match index {
                                    0 => push_if_new(keep_list, value.clone()),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                SetStyle::IDisDivisibleByN(n_value) => {
                    for value in chosen_set.full_set.iter() {
                        // By default, if it fails to get an ID it will be set to zero so it
                        // always gets flagged as keep
                        let (set_id, _): (i64, i64) =
                            extract_number_from_string(value).map_or((0, 1), |v| v);

                        if set_id % (*n_value as i64) == 0 {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::NumberDivisibleByN(n_value) => {
                    for value in chosen_set.full_set.iter() {
                        // By default, if it fails to get an ID it will be set to zero so it
                        // always gets flagged as keep
                        let (set_id, multiplier): (i64, i64) =
                            extract_number_from_string(value).map_or((0, 1), |v| v);

                        if set_id % (*n_value as f64 * multiplier as f64) as i64 == 0 {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
            }
        }
        new_set_list = chosen_set
            .full_set
            .iter()
            .fold(new_set_list, |mut list, item| {
                if !keep_list.contains(item) {
                    list.push(item.clone())
                }
                list
            });
        chosen_set.full_set = new_set_list;
    }

    // Lastly, if there were styles applied, move all the remaining to delete list
    if chosen_set.chosen_styles.len() != 0 {
        for file in &chosen_set.full_set {
            delete_list.push(file.clone());
        }
    }

    Ok(())
}
