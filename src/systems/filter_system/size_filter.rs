
use crate::containers::file_container::FileContainer;
use crate::filter_codes::filter_codes::FilterCodes;
use crate::systems::filter_system::filter_category_info::{
    FilterCategory, FilterCategoryError, FilterForCategory,
};

pub struct SizeFilter {
    lower_size: FilterCategory,
    upper_size: FilterCategory,
    init: bool,
}

impl SizeFilter {
    pub fn new() -> Self {
        Self {
            lower_size: FilterCategory::Size(0),
            upper_size: FilterCategory::Size(0),
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
        self.lower_size = match &lower_category {
            FilterCategory::Size(_) => lower_category,
            _ => Err(FilterCategoryError::InititialisationError(format!(
                "Passed in wrong FilterCategory type of value {:?} to SizeFilter Lower end",
                lower_category
            )))?,
        };
        self.upper_size = match &upper_category {
            FilterCategory::Size(_) => upper_category,
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

        let is_greater_than_lower = self.lower_size.is_file_flagged(file);
        let is_greater_than_upper = self.upper_size.is_file_flagged(file);

        if !is_greater_than_lower || is_greater_than_upper {
            return Ok(FilterCodes::ToKeep);
        }
        if is_greater_than_lower || !is_greater_than_upper {
            return Ok(FilterCodes::ToDelete);
        }
        // Should never be reached
        Ok(FilterCodes::NonSpecial)
    }
}
