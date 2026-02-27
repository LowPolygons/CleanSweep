use crate::containers::file_container::FileContainer;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct DirectoryContainsFilter {
    keep_dir: FilterCategory,
    delete_dir: FilterCategory,
    init: bool,
}

impl DirectoryContainsFilter {
    pub fn new() -> Self {
        Self {
            keep_dir: FilterCategory::NameContains(Vec::new()),
            delete_dir: FilterCategory::NameContains(Vec::new()),
            init: false,
        }
    }
}

impl FilterForCategory for DirectoryContainsFilter {
    fn init(
        &mut self,
        to_keep_category: FilterCategory,
        to_delete_category: FilterCategory,
    ) -> Result<(), FilterCategoryError> {
        self.keep_dir = match &to_keep_category {
            FilterCategory::DirectoryContains(_) => to_keep_category,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to NameFilter Keep list",
                to_keep_category
            )))?,
        };
        self.delete_dir = match &to_delete_category {
            FilterCategory::DirectoryContains(_) => to_delete_category,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to NameFilter delete list",
                to_delete_category
            )))?,
        };
        self.init = true;
        Ok(())
    }

    fn categorise_file(&self, file: &FileContainer) -> Result<FilterCodes, FilterCategoryError> {
        if !self.init {
            Err(FilterCategoryError::DidntInitBeforeUse)?
        }

        if self.keep_dir.is_file_flagged(file) {
            return Ok(FilterCodes::ToKeep);
        }

        if self.delete_dir.is_file_flagged(file) {
            return Ok(FilterCodes::ToDelete);
        }

        Ok(FilterCodes::ToKeep)
    }
}
