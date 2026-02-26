use crate::containers::file_container::FileContainer;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct ExtensionFilter {
    keep_extension: FilterCategory,   //Vec<String>,
    delete_extension: FilterCategory, //Vec<String>,
    init: bool,
}

impl ExtensionFilter {
    pub fn new() -> Self {
        Self {
            keep_extension: FilterCategory::Extension(Vec::new()),
            delete_extension: FilterCategory::Extension(Vec::new()),
            init: false,
        }
    }
}

impl FilterForCategory for ExtensionFilter {
    fn init(
        &mut self,
        to_keep_category: FilterCategory,
        to_delete_category: FilterCategory,
    ) -> Result<(), FilterCategoryError> {
        self.keep_extension = match &to_keep_category {
            FilterCategory::Extension(_) => to_keep_category,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to ExtensionFilter Keep list",
                to_keep_category
            )))?,
        };

        self.delete_extension = match &to_delete_category {
            FilterCategory::Extension(_) => to_delete_category,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to ExtensionFilter Delete list",
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

        if self.keep_extension.is_file_flagged(file) {
            return Ok(FilterCodes::ToKeep);
        }
        if self.delete_extension.is_file_flagged(file) {
            return Ok(FilterCodes::ToDelete);
        }

        Ok(FilterCodes::NonSpecial)
    }
}
