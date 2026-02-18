use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PerFilterOptions {
    with_extension: Vec<String>,
    name_contains: Vec<String>,
    directory_contains: Vec<String>,
    name_starts_with: Vec<String>,
    larger_than: u64,
    modified_after: u64,
    accessed_after: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetScanOptions {
    with_extension: Vec<String>,
    name_contains: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSettings {
    to_keep_list: PerFilterOptions,
    //
    to_delete_list: PerFilterOptions,
    //
    set_scan_options: SetScanOptions,
}

impl UserSettings {
    pub fn new(
        to_keep_list: PerFilterOptions,
        to_delete_list: PerFilterOptions,
        set_scan_options: SetScanOptions,
    ) -> Self {
        Self {
            to_keep_list,
            to_delete_list,
            set_scan_options,
        }
    }

    pub fn get_to_keep_list(&self) -> &PerFilterOptions {
        &self.to_keep_list
    }
    pub fn get_to_delete_list(&self) -> &PerFilterOptions {
        &self.to_delete_list
    }
    pub fn get_set_scan_option(&self) -> &SetScanOptions {
        &self.set_scan_options
    }
}

impl SetScanOptions {
    pub fn new(with_extension: Vec<String>, name_contains: Vec<String>) -> Self {
        Self {
            with_extension,
            name_contains,
        }
    }
    pub fn get_with_extension(&self) -> &Vec<String> {
        &self.with_extension
    }
    pub fn get_name_contains(&self) -> &Vec<String> {
        &self.name_contains
    }
}

impl PerFilterOptions {
    pub fn new(
        with_extension: Vec<String>,
        name_contains: Vec<String>,
        directory_contains: Vec<String>,
        name_starts_with: Vec<String>,
        larger_than: u64,
        modified_after: u64,
        accessed_after: u64,
    ) -> Self {
        Self {
            with_extension,
            name_contains,
            directory_contains,
            name_starts_with,
            larger_than,
            modified_after,
            accessed_after,
        }
    }
    pub fn get_extensions(&self) -> &Vec<String> {
        &self.with_extension
    }
    pub fn get_name(&self) -> &Vec<String> {
        &self.name_contains
    }
    pub fn get_directory(&self) -> &Vec<String> {
        &self.directory_contains
    }
    pub fn get_name_starts_with(&self) -> &Vec<String> {
        &self.name_starts_with
    }
    pub fn get_larger_than_size(&self) -> &u64 {
        &self.larger_than
    }
    pub fn get_modified_after(&self) -> &u64 {
        &self.modified_after
    }
    pub fn get_accessed_after(&self) -> &u64 {
        &self.accessed_after
    }
}
