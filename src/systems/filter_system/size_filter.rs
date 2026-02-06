use crate::containers::file_container::FileContainer;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct SizeFilter {
    lower_size: u64,
    upper_size: u64,
    init: bool,
}

impl SizeFilter {
    pub fn new() -> Self {
        Self {
            lower_size: 0,
            upper_size: 0,
            init: false,
        }
    }
}

impl FilterForCategory for SizeFilter {
    fn init(
        &mut self,
        lower_category: FilterCategory,
        upper_category: FilterCategory,
    ) -> Result<(), FilterCategoryError> {
        self.lower_size = match lower_category {
            FilterCategory::Size(value) => value,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to SizeFilter Lower end",
                lower_category
            )))?,
        };
        self.upper_size = match upper_category {
            FilterCategory::Size(value) => value,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to SizeFilter Upper end",
                upper_category
            )))?,
        };
        self.init = true;
        Ok(())
    }

    fn categorise_file(&self, file: &FileContainer) -> Result<FilterCodes, FilterCategoryError> {
        if !self.init {
            Err(FilterCategoryError::DidntInitBeforeUse)?
        }

        let file_size: &u64 = file.get_statistics().get_size();

        if file_size < &self.lower_size || file_size > &self.upper_size {
            return Ok(FilterCodes::ToKeep);
        }
        if file_size >= &self.lower_size || file_size <= &self.upper_size {
            return Ok(FilterCodes::ToDelete);
        }
        // Should never be reached
        Ok(FilterCodes::NonSpecial)
    }
}
