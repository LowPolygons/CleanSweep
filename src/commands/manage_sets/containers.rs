use core::fmt;

use dialoguer::{Select, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};

use crate::{
    commands::manage_sets::command::extract_number_from_string,
    utils::run_time_user_input::get_number_input,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetStyle {
    First,
    Last,
    FirstAndLast,
    FirstN(usize),
    LastN(usize),
    FirstNandLastM(usize, usize),
    EveryNIndexed(usize, ZeroOrOne),
    EvenlySpacedN(usize),
    IDisDivisibleByN(usize),
    NumberDivisibleByN(f64),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZeroOrOne {
    Zero,
    One,
}

impl fmt::Display for SetStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SetStyle::First => "First",
            SetStyle::Last => "Last",
            SetStyle::FirstAndLast => "First And Last",
            SetStyle::FirstN(_) => "First N",
            SetStyle::LastN(_) => "Last N",
            SetStyle::FirstNandLastM(_, _) => "First N and Last M",
            SetStyle::EveryNIndexed(_, zero_or_one) => {
                let addition: String = match zero_or_one {
                    ZeroOrOne::Zero => String::from("0"),
                    ZeroOrOne::One => String::from("1"),
                };

                &format!("Position in set is divisible by N ({addition} Indexed)")
            }
            SetStyle::EvenlySpacedN(_) => "N Evenly Spaced",
            SetStyle::IDisDivisibleByN(_) => "ID is divisible by N",
            SetStyle::NumberDivisibleByN(_) => "Number is divisible by N",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone)]
pub struct ManageSetsType {
    pub full_set: Vec<String>,
    pub label: String,
    pub chosen_styles: Vec<Vec<SetStyle>>,
}

impl ManageSetsType {
    pub fn vec_style_to_string(styles: &Vec<SetStyle>) -> String {
        let mut per_set_list: String = String::from("(");
        styles.iter().for_each(|item| {
            per_set_list = format!("{} {:?} + ", per_set_list, item);
        });

        per_set_list = format!("{} )", per_set_list);

        per_set_list
    }
    pub fn styles_to_string(&self) -> String {
        let mut string = String::from("[");
        for styles in &self.chosen_styles {
            string = format!(
                "{} -> {:?}",
                string,
                ManageSetsType::vec_style_to_string(styles)
            )
        }
        string = format!("{}]", string);

        string
    }
    pub fn label_truncated(&self, length_to_strip: usize) -> String {
        let length = self.label.len();

        let mut clone_of_label = self.label.clone();

        clone_of_label.drain(length_to_strip..length).collect()
    }
}

// INFO:
// These enums are here to make return types of the helper methods more clear
pub enum AppendOrOverride {
    Append,
    Override,
}
pub enum ChoiceInGettingStyle {
    AffectStoredStyles(NewStyleBehaviour),
    NotAffectingStyles(NotAffectingStyles),
}
pub enum NotAffectingStyles {
    Back,
    FullTable,
    Preview,
}
pub enum NewStyleBehaviour {
    Append,
    Reset,
    Copy,
    Set,
}

pub fn choose_style_and_m_n_values() -> Result<SetStyle, ()> {
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

pub fn filter_files_from_styles(
    chosen_set: &mut Vec<String>,
    chosen_styles: &Vec<Vec<SetStyle>>,
    keep_list: &mut Vec<String>,
    delete_list: &mut Vec<String>,
) -> Result<(), ()> {
    for current_style_list in chosen_styles {
        // Apply Sets in order
        let len_of_set_sub_one = chosen_set.len() - 1;
        let mut new_set_list: Vec<String> = Vec::new();

        // These should apply simultaneously
        for current_style in current_style_list {
            match current_style {
                SetStyle::First => {
                    for (index, value) in chosen_set.iter().enumerate() {
                        match index {
                            0 => push_if_new(keep_list, value.clone()),
                            _ => {}
                        }
                    }
                }
                SetStyle::Last => {
                    for (index, value) in chosen_set.iter().enumerate() {
                        if index == len_of_set_sub_one {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::FirstAndLast => {
                    for (index, value) in chosen_set.iter().enumerate() {
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
                    for (index, value) in chosen_set.iter().enumerate() {
                        if index < *n_value {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::LastN(n_value) => {
                    for (index, value) in chosen_set.iter().enumerate() {
                        if index > len_of_set_sub_one.saturating_sub(*n_value) {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::FirstNandLastM(n_value, m_value) => {
                    for (index, value) in chosen_set.iter().enumerate() {
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

                    for (index, value) in chosen_set.iter().enumerate() {
                        if index != 0 && (index + index_addition) % n_value == 0 {
                            push_if_new(keep_list, value.clone());
                        }
                    }
                }
                SetStyle::EvenlySpacedN(n_value) => {
                    let chunk_size = (chosen_set.len() as f64 / *n_value as f64).round() as usize;

                    for (chunk_num, chunk) in chosen_set.chunks(chunk_size).enumerate() {
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
                    for value in chosen_set.iter() {
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
                    for value in chosen_set.iter() {
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
        new_set_list = chosen_set.iter().fold(new_set_list, |mut list, item| {
            if !keep_list.contains(item) {
                list.push(item.clone())
            }
            list
        });
        *chosen_set = new_set_list;
    }

    // Lastly, if there were styles applied, move all the remaining to delete list
    if chosen_styles.len() != 0 {
        for file in chosen_set {
            delete_list.push(file.clone());
        }
    }

    Ok(())
}
