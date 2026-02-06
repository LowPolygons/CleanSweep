use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PerFilterOptions {
    with_extension: Option<Vec<String>>,
    name_contains: Option<Vec<String>>,
    path_contains: Option<Vec<String>>,
    name_starts_with: Option<Vec<String>>,
    larger_than: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetScanOptions {
    with_extension: Option<Vec<String>>,
    name_contains: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSettings {
    to_keep_list: PerFilterOptions,
    keep_if_smaller_than: Option<u64>,
    //
    to_delete_list: PerFilterOptions,
    //
    set_scan_options: SetScanOptions,
    // Seconds since unix epoch
    modify_date_cutoff: Option<u64>,
    access_date_cutoff: Option<u64>,
}

impl UserSettings {
    pub fn new(
        to_keep_list: PerFilterOptions,
        keep_if_smaller_than: Option<u64>,
        to_delete_list: PerFilterOptions,
        set_scan_options: SetScanOptions,
        modify_date_cutoff: Option<u64>,
        access_date_cutoff: Option<u64>,
    ) -> Self {
        Self {
            to_keep_list,
            keep_if_smaller_than,
            to_delete_list,
            set_scan_options,
            modify_date_cutoff,
            access_date_cutoff,
        }
    }

    pub fn get_to_keep_list(&self) -> &PerFilterOptions {
        &self.to_keep_list
    }
    pub fn get_keep_if_smaller_than(&self) -> &Option<u64> {
        &self.keep_if_smaller_than
    }
    pub fn get_to_delete_list(&self) -> &PerFilterOptions {
        &self.to_delete_list
    }
    pub fn get_set_scan_option(&self) -> &SetScanOptions {
        &self.set_scan_options
    }
    pub fn get_modify_date_cutoff(&self) -> &Option<u64> {
        &self.modify_date_cutoff
    }
    pub fn get_access_date_cutoff(&self) -> &Option<u64> {
        &self.access_date_cutoff
    }
}

impl SetScanOptions {
    pub fn new(with_extension: Option<Vec<String>>, name_contains: Option<Vec<String>>) -> Self {
        Self {
            with_extension,
            name_contains,
        }
    }
    pub fn get_with_extension(&self) -> &Option<Vec<String>> {
        &self.with_extension
    }
    pub fn get_name_contains(&self) -> &Option<Vec<String>> {
        &self.name_contains
    }
}

impl PerFilterOptions {
    pub fn new(
        with_extension: Option<Vec<String>>,
        name_contains: Option<Vec<String>>,
        path_contains: Option<Vec<String>>,
        name_starts_with: Option<Vec<String>>,
        larger_than: Option<u64>,
    ) -> Self {
        Self {
            with_extension,
            name_contains,
            path_contains,
            name_starts_with,
            larger_than,
        }
    }
}
