use crate::containers::file_container::FileContainer;
use crate::containers::file_date_data::FileDateData;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct LastModifiedFilter {
    lower_last_modified: FilterCategory,
    upper_last_modified: FilterCategory,
    init: bool,
}

impl LastModifiedFilter {
    pub fn new() -> Self {
        Self {
            lower_last_modified: FilterCategory::LastModified(FileDateData::new(
                std::time::UNIX_EPOCH,
            )),
            upper_last_modified: FilterCategory::LastModified(FileDateData::new(
                std::time::UNIX_EPOCH,
            )),
            init: false,
        }
    }
}

impl FilterForCategory for LastModifiedFilter {
    fn init(
        &mut self,
        lower_last_modified: FilterCategory,
        upper_last_modified: FilterCategory,
    ) -> Result<(), FilterCategoryError> {
        self.lower_last_modified = match &lower_last_modified {
            FilterCategory::LastModified(_) => lower_last_modified,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to LastModifiedFilter lower bound",
                lower_last_modified
            )))?,
        };
        self.upper_last_modified = match &upper_last_modified {
            FilterCategory::LastModified(_) => upper_last_modified,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to LastModifiedFilter upper bound",
                upper_last_modified
            )))?,
        };
        self.init = true;
        Ok(())
    }

    fn categorise_file(&self, file: &FileContainer) -> Result<FilterCodes, FilterCategoryError> {
        if !self.init {
            Err(FilterCategoryError::DidntInitBeforeUse)?
        }

        let is_greater_than_lower_flagged = self.lower_last_modified.is_file_flagged(file);
        let is_greater_than_upper_flagged = self.lower_last_modified.is_file_flagged(file);

        if !is_greater_than_lower_flagged || is_greater_than_upper_flagged {
            return Ok(FilterCodes::ToKeep);
        }
        if is_greater_than_lower_flagged || !is_greater_than_upper_flagged {
            return Ok(FilterCodes::ToDelete);
        }

        Ok(FilterCodes::NonSpecial)
    }
}
