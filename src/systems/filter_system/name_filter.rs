use crate::containers::file_container::FileContainer;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct NameFilter {
    keep_name: Vec<String>,
    delete_name: Vec<String>,
    init: bool,
}

impl NameFilter {
    pub fn new() -> Self {
        Self {
            keep_name: Vec::new(),
            delete_name: Vec::new(),
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
        self.keep_name = match to_keep_category {
            FilterCategory::Name(value) => value,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to NameFilter Keep list",
                to_keep_category
            )))?,
        };
        self.delete_name = match to_delete_category {
            FilterCategory::Name(value) => value,
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

        let file_name: &String = file.get_statistics().get_name();

        if self.keep_name.contains(&file_name) {
            return Ok(FilterCodes::ToKeep);
        }

        if self.delete_name.contains(&file_name) {
            return Ok(FilterCodes::ToDelete);
        }

        Ok(FilterCodes::NonSpecial)
    }
}
