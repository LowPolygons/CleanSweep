use crate::containers::file_container::FileContainer;
use crate::containers::file_date_data::FileDateData;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct LastAccessedFilter {
    lower_last_accessed: FileDateData,
    upper_last_accessed: FileDateData,
    init: bool,
}

impl LastAccessedFilter {
    pub fn new() -> Self {
        Self {
            lower_last_accessed: FileDateData::new(std::time::UNIX_EPOCH),
            upper_last_accessed: FileDateData::new(std::time::UNIX_EPOCH),
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
        self.lower_last_accessed = match lower_last_accessed {
            FilterCategory::LastAccessed(value) => value,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to LastAccessedFilter lower bound",
                lower_last_accessed
            )))?,
        };
        self.upper_last_accessed = match upper_last_accessed {
            FilterCategory::LastAccessed(value) => value,
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

        let file_access_date = file.get_statistics().get_last_accessed().time_since_zero();
        let lower_last_accessed_secs = self.lower_last_accessed.time_since_zero();
        let upper_last_accessed_secs = self.upper_last_accessed.time_since_zero();

        if file_access_date < lower_last_accessed_secs
            || file_access_date > upper_last_accessed_secs
        {
            return Ok(FilterCodes::ToKeep);
        }

        if file_access_date >= lower_last_accessed_secs
            || file_access_date >= upper_last_accessed_secs
        {
            return Ok(FilterCodes::ToDelete);
        }
        // Should never reach
        Ok(FilterCodes::NonSpecial)
    }
}
