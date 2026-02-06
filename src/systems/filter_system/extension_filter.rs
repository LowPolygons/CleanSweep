use crate::containers::file_container::FileContainer;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct ExtensionFilter {
    keep_extension: Vec<String>,
    delete_extension: Vec<String>,
    init: bool,
}

impl ExtensionFilter {
    pub fn new() -> Self {
        Self {
            keep_extension: Vec::new(),
            delete_extension: Vec::new(),
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
        self.keep_extension = match to_keep_category {
            FilterCategory::Extension(value) => value,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to ExtensionFilter Keep list",
                to_keep_category
            )))?,
        };

        self.delete_extension = match to_delete_category {
            FilterCategory::Extension(value) => value,
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

        let file_extension: &String = file.get_statistics().get_extension();

        if self.keep_extension.contains(&file_extension) {
            return Ok(FilterCodes::ToKeep);
        }

        if self.delete_extension.contains(&file_extension) {
            return Ok(FilterCodes::ToDelete);
        }

        Ok(FilterCodes::NonSpecial)
    }
}
