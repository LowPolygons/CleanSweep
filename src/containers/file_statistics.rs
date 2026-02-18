use crate::containers::file_date_data::FileDateData;

#[derive(Debug)]
pub struct FileStatistics {
    name: String,
    size: u64,
    extension: String,
    last_accessed: FileDateData,
    last_modified: FileDateData,
    directory: String,
}

impl FileStatistics {
    pub fn new(
        name: String,
        size: u64,
        extension: String,
        last_accessed: FileDateData,
        last_modified: FileDateData,
        directory: String,
    ) -> Self {
        FileStatistics {
            name,
            size,
            extension,
            last_accessed,
            last_modified,
            directory,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_directory(&self) -> &String {
        &self.directory
    }
    pub fn get_size(&self) -> &u64 {
        &self.size
    }
    pub fn get_extension(&self) -> &String {
        &self.extension
    }
    pub fn get_last_accessed(&self) -> &FileDateData {
        &self.last_accessed
    }
    pub fn get_last_modified(&self) -> &FileDateData {
        &self.last_modified
    }
}
