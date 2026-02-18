use crate::{
    containers::file_container::FileContainer,
    systems::filter_system::filter_category_info::{FilterCategory, FilterForCategory},
};

use thiserror::Error;

pub struct SetDetector {
    filters: Vec<FilterCategory>,
}

#[derive(Debug, Error)]
pub enum SetDetectorError {
    #[error("Error converting Path to string")]
    ConvertPathToName,
}

impl SetDetector {
    pub fn new(filters: Vec<FilterCategory>) -> Self {
        Self { filters }
    }
    // If the files match either one OR the other
    pub fn maybe_in_set(&self, file: &FileContainer) -> bool {
        for filter in &self.filters {
            if filter.is_file_flagged(file) {
                return true;
            }
        }
        return false;
    }
}
