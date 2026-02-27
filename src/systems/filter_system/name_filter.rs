use crate::containers::file_container::FileContainer;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct NameFilter {
    keep_name: FilterCategory,
    delete_name: FilterCategory,
    init: bool,
}

impl NameFilter {
    pub fn new() -> Self {
        Self {
            keep_name: FilterCategory::Name(Vec::new()),
            delete_name: FilterCategory::Name(Vec::new()),
            init: false,
        }
    }
}

impl FilterForCategory for NameFilter {
    fn init(
        &mut self,
        to_keep_category: FilterCategory,
        to_delete_category: FilterCategory,
    ) -> Result<(), FilterCategoryError> {
        self.keep_name = match &to_keep_category {
            FilterCategory::Name(_) => to_keep_category,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to NameFilter Keep list",
                to_keep_category
            )))?,
        };
        self.delete_name = match &to_delete_category {
            FilterCategory::Name(_) => to_delete_category,
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

        if self.keep_name.is_file_flagged(file) {
            return Ok(FilterCodes::ToKeep);
        }

        if self.delete_name.is_file_flagged(file) {
            return Ok(FilterCodes::ToDelete);
        }

        Ok(FilterCodes::ToKeep)
    }
}
