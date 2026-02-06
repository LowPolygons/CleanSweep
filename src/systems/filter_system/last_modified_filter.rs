use crate::containers::file_container::FileContainer;
use crate::containers::file_date_data::FileDateData;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct LastModifiedFilter {
    lower_last_modified: FileDateData,
    upper_last_modified: FileDateData,
    init: bool,
}

impl LastModifiedFilter {
    pub fn new() -> Self {
        Self {
            lower_last_modified: FileDateData::new(std::time::UNIX_EPOCH),
            upper_last_modified: FileDateData::new(std::time::UNIX_EPOCH),
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
        self.lower_last_modified = match lower_last_modified {
            FilterCategory::LastAccessed(value) => value,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to LastModifiedFilter lower bound",
                lower_last_modified
            )))?,
        };
        self.upper_last_modified = match upper_last_modified {
            FilterCategory::LastAccessed(value) => value,
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

        let file_modified_date = file.get_statistics().get_last_accessed().time_since_zero();
        let lower_last_modified_secs = self.lower_last_modified.time_since_zero();
        let upper_last_modified_secs = self.upper_last_modified.time_since_zero();

        if file_modified_date < lower_last_modified_secs
            || file_modified_date > upper_last_modified_secs
        {
            return Ok(FilterCodes::ToKeep);
        }

        if file_modified_date >= lower_last_modified_secs
            || file_modified_date >= upper_last_modified_secs
        {
            return Ok(FilterCodes::ToDelete);
        }
        Ok(FilterCodes::NonSpecial)
    }
}
