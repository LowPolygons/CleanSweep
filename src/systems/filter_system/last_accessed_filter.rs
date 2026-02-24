use crate::containers::file_container::FileContainer;
use crate::containers::file_date_data::FileDateData;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct LastAccessedFilter {
    lower_last_accessed: FilterCategory,
    upper_last_accessed: FilterCategory,
    init: bool,
}

impl LastAccessedFilter {
    pub fn new() -> Self {
        Self {
            lower_last_accessed: FilterCategory::LastAccessed(FileDateData::new(
                std::time::UNIX_EPOCH,
            )),
            upper_last_accessed: FilterCategory::LastAccessed(FileDateData::new(
                std::time::UNIX_EPOCH,
            )),
            init: false,
        }
    }
}

impl FilterForCategory for LastAccessedFilter {
    fn init(
        &mut self,
        lower_last_accessed: FilterCategory,
        upper_last_accessed: FilterCategory,
    ) -> Result<(), FilterCategoryError> {
        self.lower_last_accessed = match &lower_last_accessed {
            FilterCategory::LastAccessed(_) => lower_last_accessed,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to LastAccessedFilter lower bound",
                lower_last_accessed
            )))?,
        };
        self.upper_last_accessed = match &upper_last_accessed {
            FilterCategory::LastAccessed(_) => upper_last_accessed,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to LastAccessedFilter upper bound",
                upper_last_accessed
            )))?,
        };
        self.init = true;
        Ok(())
    }

    fn categorise_file(&self, file: &FileContainer) -> Result<FilterCodes, FilterCategoryError> {
        if !self.init {
            Err(FilterCategoryError::DidntInitBeforeUse)?
        }

        let is_before_lower_flagged = self.lower_last_accessed.is_file_flagged(file);
        let is_before_upper_flagged = self.lower_last_accessed.is_file_flagged(file);

        if is_before_lower_flagged || !is_before_upper_flagged {
            return Ok(FilterCodes::ToKeep);
        }
        if !is_before_lower_flagged || is_before_upper_flagged {
            return Ok(FilterCodes::ToDelete);
        }

        // Should never reach
        Ok(FilterCodes::NonSpecial)
    }
}
