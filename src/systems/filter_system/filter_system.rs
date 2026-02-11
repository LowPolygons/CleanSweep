use crate::{
    containers::file_container::FileContainer,
    filter_codes::filter_codes::FilterCodes,
    systems::filter_system::filter_category_info::{FilterCategoryError, FilterForCategory},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilterSystemError {
    #[error("Error when attempting to filter provided file: {0}")]
    CategoriseFileError(#[from] FilterCategoryError),
}

pub struct FilterSystem {
    filterers: Vec<Box<dyn FilterForCategory>>,
}

impl FilterSystem {
    pub fn new(filterers: Vec<Box<dyn FilterForCategory>>) -> Self {
        Self { filterers }
    }

    pub fn filter_file(&self, file: &FileContainer) -> Result<FilterCodes, FilterSystemError> {
        let mut filter_code_results: Vec<FilterCodes> = Vec::new();

        for filter_category in &self.filterers {
            match filter_category.categorise_file(&file) {
                Ok(code) => filter_code_results.push(code),
                Err(e) => Err(FilterSystemError::CategoriseFileError(e))?,
            };
        }

        if filter_code_results.contains(&FilterCodes::ToKeep) {
            return Ok(FilterCodes::ToKeep);
        }
        if filter_code_results.contains(&FilterCodes::ToDelete) {
            return Ok(FilterCodes::ToDelete);
        }

        Ok(FilterCodes::NonSpecial)
    }
}
