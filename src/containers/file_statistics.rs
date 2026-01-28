use crate::containers::file_date_data::FileDateData;

#[derive(Debug)]
pub struct FileStatistics {
    name: String,
    size: u64,
    extension: String,
    last_accessed: FileDateData,
    last_modified: FileDateData,
}

impl FileStatistics {
    pub fn new(
        name: String,
        size: u64,
        extension: String,
        last_accessed: FileDateData,
        last_modified: FileDateData,
    ) -> Self {
        FileStatistics {
            name,
            size,
            extension,
            last_accessed,
            last_modified,
        }
    }
}
