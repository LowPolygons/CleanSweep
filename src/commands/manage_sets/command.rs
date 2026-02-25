use dialoguer::{Select, theme::ColorfulTheme};
use thiserror::Error;

use crate::{
    commands::manage_sets::containers::{
        AppendOrOverride, ChoiceInGettingStyle, ManageSetsType, SetStyle,
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

    #[error("Failed when deciding how to get the new set list")] //get_choice_of_how_to_get_style
    GetChoiceOfHowToGetStyleFailure,

    #[error(
        "Dev Note: This error should never be reached, if you see this it is a written software problem"
    )]
    SomehowNoneDespiteCheckingFailure,

    #[error(
        "Failed when trying to preview how a set will be separated based on its current styles"
    )]
    PreviewSeparatedFilesBasedOnStylesFailure,
    // #[error("Failed to set the default management style for the sets")]
    // SetDefaultManagementStyleFailure,
    //
    #[error("Failed to get a mutable reference to the chosen set")]
    GetMutRefToChosenSetFailure,

    // #[error("Failed to set the management style for the chosen set")]
    // SetManagementStyleForChosenSetFailure,
    //
    #[error("Failed to save struct of files to the corresponding json file")]
    WriteJsonFileFromStructFailure,
}

pub fn manage_sets() -> Result<(), ManageSetsError> {
    let cleansweep_dir =
        get_cleansweep_dir().map_err(|_| ManageSetsError::GetCleansweepDirectoryFailure)?;

    let scanned_sets: Vec<SetsReadWriteType> =
        read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::FoundSets.name()))
            .map_err(|_| ManageSetsError::ReadSetsFileToStructFailure)?;

    let mut dir_set_scan_was_run_from = String::new();

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

    // If the last char was a /, dont store in the $PATH var
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
                        "{} : [PATH]{}",
                        item.styles_to_string(),
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
            .map_err(|_| ManageSetsError::ListSetsToManageFailure)?;

        if selection == 0 {
            break;
        } else {
            if selection == 1 {
                println!(
                    "Any sets where a provided 'N-Value' exceeds its length will not have a default applied"
                );
            }

            let is_default = selection == 1;

            let maybe_how_to_get_style = get_choice_of_how_to_get_style(is_default)
                .map_err(|_| ManageSetsError::GetChoiceOfHowToGetStyleFailure)?;

            if maybe_how_to_get_style.is_none() {
                // To have reached here, the function must NOT be in default
                // In the event of misuse, the function still won't error, though
                let mut keep_as_of_now: Vec<String> = Vec::new();
                let mut delete_as_of_now: Vec<String> = Vec::new();

                let mutable_ref_to_set = managed_sets
                    .get_mut(selection - length_initial_first_in_sets)
                    .ok_or_else(|| ())
                    .map_err(|_| ManageSetsError::GetMutRefToChosenSetFailure)?;

                separate_files_based_on_style(
                    mutable_ref_to_set,
                    &mut keep_as_of_now,
                    &mut delete_as_of_now,
                )
                .map_err(|_| ManageSetsError::PreviewSeparatedFilesBasedOnStylesFailure)?;

                println!("This is how the set will be handled if you exit now:");
                println!("To be added to Keep list:");
                keep_as_of_now.iter().for_each(|item| println!("- {item}"));

                println!("To Be added to Delete list:");
                delete_as_of_now
                    .iter()
                    .for_each(|item| println!("- {item}"));
                continue;
            }

            let how_to_get_style = maybe_how_to_get_style
                .ok_or_else(|| ())
                .map_err(|_| ManageSetsError::SomehowNoneDespiteCheckingFailure)?;

            // First match gets you the new Vec<>
            let new_style = match &how_to_get_style {
                ChoiceInGettingStyle::Append => vec![
                    choose_management_style()
                        .map_err(|_| ManageSetsError::ChooseManagementStyleFailure)?,
                ],
                ChoiceInGettingStyle::Reset => Vec::new(),
                ChoiceInGettingStyle::Set => vec![
                    choose_management_style()
                        .map_err(|_| ManageSetsError::ChooseManagementStyleFailure)?,
                ],
                ChoiceInGettingStyle::Copy => {
                    copy_management_style_from_set(&managed_sets, len_to_strip_away)
                        .map_err(|_| ManageSetsError::ChooseManagementStyleFailure)?
                }
            };

            if selection == 1 {
                for mutable_ref_to_set in &mut managed_sets {
                    if apply_style_to_set(mutable_ref_to_set, &how_to_get_style, &new_style)
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

                if apply_style_to_set(mutable_ref_to_set, &how_to_get_style, &new_style)
                    .contains(&false)
                {
                    println!(
                        "Some filters weren't applied to a set due to 'N' values exceeding their length"
                    )
                }
            }
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
    .map_err(|_| ManageSetsError::WriteJsonFileFromStructFailure)?;

    write_json_file_from_struct(
        &files_for_delete,
        cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()),
    )
    .map_err(|_| ManageSetsError::WriteJsonFileFromStructFailure)?;

    Ok(())
}

fn apply_style_to_set(
    mutable_ref_to_set: &mut ManageSetsType,
    how_style_was_made: &ChoiceInGettingStyle,
    new_style: &Vec<SetStyle>,
) -> Vec<bool> {
    let mut any_failed: Vec<bool> = Vec::new();

    match how_style_was_made {
        ChoiceInGettingStyle::Append => {
            for current_style in new_style {
                if !apply_management_style_for_set(
                    mutable_ref_to_set,
                    current_style,
                    AppendOrOverride::Append,
                ) {
                    any_failed.push(false);
                }
            }
        }
        ChoiceInGettingStyle::Reset => mutable_ref_to_set.chosen_styles = new_style.clone(),
        ChoiceInGettingStyle::Set => {
            for current_style in new_style {
                if !apply_management_style_for_set(
                    mutable_ref_to_set,
                    current_style,
                    AppendOrOverride::Override,
                ) {
                    any_failed.push(false);
                }
            }
        }
        ChoiceInGettingStyle::Copy => {
            mutable_ref_to_set.chosen_styles = Vec::new();

            for current_style in new_style {
                if !apply_management_style_for_set(
                    mutable_ref_to_set,
                    current_style,
                    AppendOrOverride::Append,
                ) {
                    any_failed.push(false);
                }
            }
        }
    }
    any_failed
}

// Ok(Some) => THey have chosen a management style
// Ok(None) => They wish to just preview what the current style would do
// Err => Error
fn get_choice_of_how_to_get_style(for_defaults: bool) -> Result<Option<ChoiceInGettingStyle>, ()> {
    let options = if for_defaults {
        vec![
            "Append To List",
            "Reset Style List",
            "Override Set List",
            "Copy Other Set List",
        ]
    } else {
        vec![
            "Append To List",
            "Reset Style List",
            "Override Set List",
            "Copy Other Set List",
            "Preview Current Style Effects",
        ]
    };

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose how to pick the management style")
        .items(options)
        .default(0)
        .interact()
        .map_err(|_| ())?;

    match selection {
        0 => Ok(Some(ChoiceInGettingStyle::Append)),
        1 => Ok(Some(ChoiceInGettingStyle::Reset)),
        2 => Ok(Some(ChoiceInGettingStyle::Set)),
        3 => Ok(Some(ChoiceInGettingStyle::Copy)),
        4 => Ok(None),
        _ => Err(()),
    }
}

fn copy_management_style_from_set(
    set_list: &Vec<ManageSetsType>,
    len_to_strip: usize,
) -> Result<Vec<SetStyle>, ()> {
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

fn apply_management_style_for_set(
    set: &mut ManageSetsType,
    style: &SetStyle,
    append_or_override: AppendOrOverride,
) -> bool {
    if !check_input_works_for_set(set, style) {
        return false;
    }
    match append_or_override {
        AppendOrOverride::Append => set.chosen_styles.push(style.clone()),
        AppendOrOverride::Override => set.chosen_styles = vec![style.clone()],
    }

    true
}

fn check_input_works_for_set(set: &ManageSetsType, style: &SetStyle) -> bool {
    match &style {
        SetStyle::FirstN(n_value) => *n_value <= set.full_set.len(),
        SetStyle::LastN(n_value) => *n_value <= set.full_set.len(),
        SetStyle::FirstNandLastM(n_value, m_value) => {
            *n_value <= set.full_set.len() || *m_value <= set.full_set.len()
        }
        SetStyle::EveryN(n_value) => *n_value <= set.full_set.len(),
        SetStyle::EvenlySpacedN(n_value) => *n_value <= set.full_set.len(),
        _ => true,
    }
}

fn choose_management_style() -> Result<SetStyle, ()> {
    let sub_options: Vec<SetStyle> = vec![
        SetStyle::First,
        SetStyle::Last,
        SetStyle::FirstN(0),
        SetStyle::LastN(0),
        SetStyle::FirstNandLastM(0, 0),
        SetStyle::FirstAndLast,
        SetStyle::EveryN(0),
        SetStyle::EvenlySpacedN(0),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Submenu")
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
            SetStyle::EveryN(_) => {
                let n_value: usize = get_number_input(
                    "Enter the number of how often to save a file when interating over the set: ",
                    true,
                )
                .map_err(|_| ())?;

                SetStyle::EveryN(n_value)
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
            other => other,
        },
    )
}
//
// fn select_default_style(managed_sets: &mut Vec<ManageSetsType>) -> Result<(), String> {
//     let selection = select_management_style().map_err(|e| format!("{e}"))?;
//
//     let value = match selection {
//         SetStyle::FirstN(_) => {
//             let n_value: usize =
//                 get_number_input("Enter how many of the first files you wish to keep:", true)
//                     .map_err(|e| format!("{e}"))?;
//
//             SetStyle::FirstN(n_value)
//         }
//         SetStyle::LastN(_) => {
//             let n_value: usize =
//                 get_number_input("Enter how many of the last files you wish to keep:", true)
//                     .map_err(|e| format!("{e}"))?;
//
//             SetStyle::LastN(n_value)
//         }
//         SetStyle::FirstNandLastM(_, _) => {
//             let n_value: usize =
//                 get_number_input("Enter how many of the first files you wish to keep:", true)
//                     .map_err(|e| format!("{e}"))?;
//
//             let m_value: usize =
//                 get_number_input("Enter how many of the last files you wish to keep:", true)
//                     .map_err(|e| format!("{e}"))?;
//
//             SetStyle::FirstNandLastM(n_value, m_value)
//         }
//         SetStyle::EveryN(_) => {
//             let n_value: usize = get_number_input(
//                 "Enter the number of how often to save a file when interating over the set: ",
//                 true,
//             )
//             .map_err(|e| format!("{}", e))?;
//
//             SetStyle::EveryN(n_value)
//         }
//         SetStyle::EvenlySpacedN(_) => {
//             println!(
//                 "This will, on average save exactly N files. There will be a margin of error if N > len / 2"
//             );
//             let n_value: usize =
//                 get_number_input("Enter how many files do you want to save: ", true)
//                     .map_err(|e| format!("{}", e))?;
//             SetStyle::EvenlySpacedN(n_value)
//         }
//         other => other,
//     };
//
//     for set in managed_sets.iter_mut() {
//         let new_style = match &value {
//             SetStyle::FirstN(n_value) => {
//                 if *n_value > set.full_set.len() {
//                     None
//                 } else {
//                     Some(SetStyle::FirstN(*n_value))
//                 }
//             }
//             SetStyle::LastN(n_value) => {
//                 if *n_value > set.full_set.len() {
//                     None
//                 } else {
//                     Some(SetStyle::LastN(*n_value))
//                 }
//             }
//             SetStyle::FirstNandLastM(n_value, m_value) => {
//                 if *n_value > set.full_set.len() || *m_value > set.full_set.len() {
//                     None
//                 } else {
//                     Some(SetStyle::FirstNandLastM(*n_value, *m_value))
//                 }
//             }
//             SetStyle::EveryN(n_value) => {
//                 if *n_value > set.full_set.len() {
//                     None
//                 } else {
//                     Some(SetStyle::EveryN(*n_value))
//                 }
//             }
//             SetStyle::EvenlySpacedN(n_value) => {
//                 if *n_value > set.full_set.len() {
//                     None
//                 } else {
//                     Some(SetStyle::EvenlySpacedN(*n_value))
//                 }
//             }
//             other => Some(other.clone()),
//         };
//
//         if let Some(style) = new_style {
//             set.chosen_styles.push(style);
//         }
//     }
//
//     Ok(())
// }
//
// fn select_management_style() -> Result<SetStyle, String> {
//     let sub_options: Vec<SetStyle> = vec![
//         SetStyle::First,
//         SetStyle::Last,
//         SetStyle::FirstN(0),
//         SetStyle::LastN(0),
//         SetStyle::FirstNandLastM(0, 0),
//         SetStyle::FirstAndLast,
//         SetStyle::EveryN(0),
//         SetStyle::EvenlySpacedN(0),
//     ];
//
//     let selection = Select::with_theme(&ColorfulTheme::default())
//         .with_prompt("Submenu")
//         .items(&sub_options)
//         .interact()
//         .map_err(|e| format!("Failed to create select instance, {:?}", e))?;
//
//     match sub_options.get(selection).ok_or(|| ()) {
//         Ok(val) => return Ok(val.clone()),
//         Err(_) => return Err("Failed to choose set style due to bad indexing".to_string()),
//     }
// }

// fn select_management_style_for_set(
//     chosen_set: &mut ManageSetsType,
//     len_to_strip: usize,
// ) -> Result<(), String> {
//     chosen_set.full_set.iter().for_each(|elem| {
//         println!(
//             "- [PATH]{}",
//             elem.clone()
//                 .drain(len_to_strip..elem.len())
//                 .fold(String::new(), |mut string, char| {
//                     string = format!("{}{}", string, char);
//                     string
//                 })
//         )
//     });
//
//     let selection = select_management_style().map_err(|e| format!("{e}"))?;
//     let len_of_set_sub_one = chosen_set.full_set.len() - 1;
//
//     let new_style = match selection {
//         SetStyle::FirstN(_) => {
//             let n_value: usize = get_number_input_in_range(
//                 "Enter how many of the first files you with to save",
//                 1,
//                 len_of_set_sub_one + 1,
//             )
//             .map_err(|e| format!("{e}"))?;
//
//             Some(SetStyle::FirstN(n_value))
//         }
//         SetStyle::LastN(_) => {
//             let n_value: usize = get_number_input_in_range(
//                 "Enter how many of the last files you with to save",
//                 1,
//                 len_of_set_sub_one + 1,
//             )
//             .map_err(|e| format!("{e}"))?;
//
//             Some(SetStyle::LastN(n_value))
//         }
//         SetStyle::FirstNandLastM(_, _) => {
//             let n_value: usize = get_number_input_in_range(
//                 "Enter how many of the first files you with to save",
//                 1,
//                 len_of_set_sub_one + 1,
//             )
//             .map_err(|e| format!("{e}"))?;
//
//             let m_value: usize = get_number_input_in_range(
//                 "Enter how many of the last files you with to save",
//                 1,
//                 len_of_set_sub_one + 1,
//             )
//             .map_err(|e| format!("{e}"))?;
//             Some(SetStyle::FirstNandLastM(n_value, m_value))
//         }
//         SetStyle::EveryN(_) => {
//             let n_value: usize = get_number_input_in_range(
//                 "Enter the number of how often to save a file when interating over the set: ",
//                 1,
//                 len_of_set_sub_one + 1,
//             )
//             .map_err(|e| format!("{}", e))?;
//
//             Some(SetStyle::EveryN(n_value))
//         }
//         SetStyle::EvenlySpacedN(_) => {
//             println!(
//                 "This will, on average save exactly N files. There will be a margin of error if N > len / 2"
//             );
//
//             let n_value: usize = get_number_input_in_range(
//                 "Enter how many files do you want to save: ",
//                 1,
//                 len_of_set_sub_one + 1,
//             )
//             .map_err(|e| format!("{}", e))?;
//             Some(SetStyle::EvenlySpacedN(n_value))
//         }
//         other => Some(other.clone()),
//     };
//
//     if let Some(style) = new_style {
//         chosen_set.chosen_styles.push(style);
//     }
//
//     Ok(())
// }

fn separate_files_based_on_style(
    chosen_set: &mut ManageSetsType,
    keep_list: &mut Vec<String>,
    delete_list: &mut Vec<String>,
) -> Result<(), ()> {
    for current_style in &chosen_set.chosen_styles {
        // Apply Sets in order
        let len_of_set_sub_one = chosen_set.full_set.len() - 1;
        let mut new_set_list: Vec<String> = Vec::new();

        match current_style {
            SetStyle::First => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    match index {
                        0 => keep_list.push(value.clone()),
                        _ => new_set_list.push(value.clone()),
                    }
                }
            }
            SetStyle::Last => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index == len_of_set_sub_one {
                        keep_list.push(value.clone());
                    } else {
                        new_set_list.push(value.clone());
                    }
                }
            }
            SetStyle::FirstAndLast => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index == len_of_set_sub_one {
                        keep_list.push(value.clone());
                    } else {
                        if index == 0 {
                            keep_list.push(value.clone());
                        } else {
                            match index {
                                0 => keep_list.push(value.clone()),
                                _ => new_set_list.push(value.clone()),
                            }
                        }
                    }
                }
            }
            SetStyle::FirstN(n_value) => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index < *n_value {
                        keep_list.push(value.clone());
                    } else {
                        new_set_list.push(value.clone());
                    }
                }
            }
            SetStyle::LastN(n_value) => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index > len_of_set_sub_one.saturating_sub(*n_value) {
                        keep_list.push(value.clone());
                    } else {
                        new_set_list.push(value.clone());
                    }
                }
            }
            SetStyle::FirstNandLastM(n_value, m_value) => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index < *n_value {
                        keep_list.push(value.clone());
                    } else if index > (len_of_set_sub_one - m_value) {
                        keep_list.push(value.clone());
                    } else {
                        new_set_list.push(value.clone());
                    }
                }
            }
            SetStyle::EveryN(n_value) => {
                for (index, value) in chosen_set.full_set.iter().enumerate() {
                    if index == len_of_set_sub_one || (len_of_set_sub_one - index) % n_value == 0 {
                        keep_list.push(value.clone());
                    } else {
                        new_set_list.push(value.clone());
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
                                new_set_list.push(value.clone());
                            }
                        }
                    } else {
                        for (index, value) in chunk.iter().enumerate() {
                            if index == 0 {
                                keep_list.push(value.clone());
                            }
                            match index {
                                0 => keep_list.push(value.clone()),
                                _ => new_set_list.push(value.clone()),
                            }
                        }
                    }
                }
            }
        }
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
