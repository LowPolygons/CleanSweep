use thiserror::Error;

use crate::containers::file_container::FileContainer;
use crate::containers::file_date_data::FileDateData;
use crate::filter_codes::filter_codes::FilterCodes;

#[derive(Debug)]
pub enum FilterCategory {
    Name(Vec<String>),
    // TODO: Name Contains
    Size(u64),
    Extension(Vec<String>),
    LastAccessed(FileDateData),
    LastModified(FileDateData),
    // TODO: Directory Contains
}

#[derive(Debug, Error)]
pub enum FilterCategoryError {
    #[error("Failed to initialise FilterCategory: {0}")]
    InititialisationError(String),

    #[error("Attempted to utilise FilterCategory before initialising it")]
    DidntInitBeforeUse,
}

pub trait FilterForCategory {
    fn init(
        &mut self,
        filter_keep: FilterCategory,
        filter_delete: FilterCategory,
    ) -> Result<(), FilterCategoryError>;
    fn categorise_file(&self, file: &FileContainer) -> Result<FilterCodes, FilterCategoryError>;
}
