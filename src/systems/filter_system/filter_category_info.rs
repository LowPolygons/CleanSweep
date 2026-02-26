use thiserror::Error;

use crate::containers::file_container::FileContainer;
use crate::containers::file_date_data::{FileDateData, secs_since_epoch_to_time};
use crate::filter_codes::filter_codes::FilterCodes;

#[derive(Debug, Clone)]
pub enum FilterCategory {
    Name(Vec<String>),
    NameContains(Vec<String>),
    NameStartsWith(Vec<String>),
    Size(u64),
    Extension(Vec<String>),
    LastAccessed(FileDateData),
    LastModified(FileDateData),
    DirectoryContains(Vec<String>),
}

#[derive(Debug, Error)]
pub enum FilterCategoryError {
    #[error("Failed to initialise FilterCategory: {0}")]
    InititialisationError(String),

    #[error("Attempted to utilise FilterCategory before initialising it")]
    DidntInitBeforeUse,
}

pub struct FilterCategoryInputInterpretation {
    filter_choice: FilterCategory,
    choice_as_string: String,
    reasoning: String,
}

pub trait FilterForCategory {
    fn init(
        &mut self,
        filter_keep: FilterCategory,
        filter_delete: FilterCategory,
    ) -> Result<(), FilterCategoryError>;
    fn categorise_file(&self, file: &FileContainer) -> Result<FilterCodes, FilterCategoryError>;
}

impl FilterCategory {
    pub fn is_file_flagged(&self, target: &FileContainer) -> bool {
        match self {
            FilterCategory::Name(list) => list.contains(target.get_statistics().get_name()),
            FilterCategory::Size(size) => target.get_statistics().get_size() > size,
            FilterCategory::Extension(list) => {
                list.contains(target.get_statistics().get_extension())
            }
            FilterCategory::LastAccessed(date) => {
                target
                    .get_statistics()
                    .get_last_accessed()
                    .time_since_zero()
                    < date.time_since_zero()
            }
            FilterCategory::LastModified(date) => {
                target
                    .get_statistics()
                    .get_last_modified()
                    .time_since_zero()
                    < date.time_since_zero()
            }
            FilterCategory::DirectoryContains(list) => {
                for item in list {
                    if target.get_statistics().get_directory().contains(item) {
                        return true;
                    }
                }
                false
            }
            FilterCategory::NameContains(list) => {
                for item in list {
                    if target.get_statistics().get_name().contains(item) {
                        return true;
                    }
                }
                false
            }
            FilterCategory::NameStartsWith(list) => {
                for item in list {
                    if target.get_statistics().get_name().starts_with(item) {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn match_string_to_category(input: &String) -> Option<FilterCategoryInputInterpretation> {
        // TODO: This will initially be quite a crude implementation, consider a refactor
        let input: String = input.to_lowercase();

        if input.contains("name") {
            if input.contains("contains") {
                return Some(FilterCategoryInputInterpretation::new(
                    FilterCategory::NameContains(Vec::new()),
                    "Name Contains Filter".to_string(),
                    format!(
                        "The provided string {} contains the substring 'name' and the substring 'contains'",
                        input
                    ),
                ));
            } else if input.contains("start") {
                return Some(FilterCategoryInputInterpretation::new(
                    FilterCategory::NameStartsWith(Vec::new()),
                    "Name Starts with Filter".to_string(),
                    format!(
                        "The provided string {} contains the substring 'name' and the substring 'start'",
                        input
                    ),
                ));
            } else {
                return Some(FilterCategoryInputInterpretation::new(
                    FilterCategory::Name(Vec::new()),
                    "Name Filter".to_string(),
                    format!(
                        "The provided string {} contains the substring 'name'",
                        input
                    ),
                ));
            }
        }
        if input.contains("directory") || input.contains("path") {
            if input.contains("contains") {
                return Some(FilterCategoryInputInterpretation::new(
                    FilterCategory::DirectoryContains(Vec::new()),
                    "Directory Contains Filter".to_string(),
                    format!(
                        "The provided string {} contains some combination of 'path' or'directory', and 'contains'",
                        input
                    ),
                ));
            }
        }

        if input.contains("size") {
            return Some(FilterCategoryInputInterpretation::new(
                FilterCategory::Size(0),
                "Size Filter".to_string(),
                format!(
                    "The provided string {} contains the substring 'size'",
                    input
                ),
            ));
        }
        if input.contains("extension") {
            return Some(FilterCategoryInputInterpretation::new(
                FilterCategory::Extension(Vec::new()),
                "Extension Filter".to_string(),
                format!(
                    "The provided string {} contains the substring 'extension'",
                    input
                ),
            ));
        }
        if input.contains("access") {
            return Some(FilterCategoryInputInterpretation::new(
                FilterCategory::LastAccessed(FileDateData::new(secs_since_epoch_to_time(0))),
                "Last Access Filter".to_string(),
                format!(
                    "The provided string {} contains the substring 'access'",
                    input
                ),
            ));
        }
        if input.contains("modif") {
            // supports modify or modified
            return Some(FilterCategoryInputInterpretation::new(
                FilterCategory::LastModified(FileDateData::new(secs_since_epoch_to_time(0))),
                "Last Modified Filter".to_string(),
                format!(
                    "The provided string {} contains the substring 'modif'",
                    input
                ),
            ));
        }
        None
    }
}

impl FilterCategoryInputInterpretation {
    pub fn new(filter_choice: FilterCategory, choice_as_string: String, reasoning: String) -> Self {
        Self {
            filter_choice,
            choice_as_string,
            reasoning,
        }
    }
    pub fn get_filter(&self) -> FilterCategory {
        self.filter_choice.clone()
    }
    pub fn get_choice(&self) -> &String {
        &self.choice_as_string
    }
    pub fn get_reasoning(&self) -> &String {
        &self.reasoning
    }
}
