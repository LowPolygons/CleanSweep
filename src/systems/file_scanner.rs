use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::{
    containers::file_container::{FileContainer, FileContainerInitError},
    utils::path_types_to_string::path_buf_to_string,
};

#[derive(Debug, Error)]
pub enum FileScannerError {
    #[error("Failed when attempting to read direction {0}")]
    ReadingDirectoryFail(String),

    #[error("Failed when converting a directory path to string")]
    CovertDirectoryPathToStringForComparison,

    #[error("Failed when attempting to turn a provided path into a FileContainer {0}")]
    FormingFileContainerError(FileContainerInitError),

    #[error("Error attempting to reead initial provided path {0}")]
    PathIsNotDirectory(PathBuf),

    #[error("Error when attempting to read entries after reading a directory")]
    CantReadEntryAfterReadDir,
}

// Passed into scan command to indicate behaviour
pub enum FileScannerScanMode {
    Recursive,
    Immediate,
}

pub struct FileScanner {}
impl FileScanner {
    // Utilise a stack to safely perform a breadth first search
    pub fn scan<P: AsRef<Path>>(
        path: P,
        scan_mode: FileScannerScanMode,
        ignore_dirs: &Vec<String>,
    ) -> Result<Vec<FileContainer>, FileScannerError> {
        let path: PathBuf = path.as_ref().into();

        if !path.is_dir() {
            return Err(FileScannerError::PathIsNotDirectory(path));
        }

        let mut file_containers: Vec<FileContainer> = Vec::new();
        let mut directories_to_search: Vec<PathBuf> = Vec::new();

        directories_to_search.push(path);

        while let Some(path) = directories_to_search.pop() {
            if !path.is_dir() {
                return Err(FileScannerError::PathIsNotDirectory(path));
            }
            let elements_in_dir = path
                .read_dir()
                .map_err(|err| FileScannerError::ReadingDirectoryFail(format!("{}", err)))?;

            for entry in elements_in_dir {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        // Check if it shouldn't be ignored
                        let mut ignore_this = false;

                        for dir_str in ignore_dirs {
                            if path_buf_to_string(&entry.path())
                                .map_err(|_| {
                                    FileScannerError::CovertDirectoryPathToStringForComparison
                                })?
                                .contains(dir_str)
                            {
                                ignore_this = true;
                            }
                        }
                        if !ignore_this {
                            directories_to_search.push(entry.path());
                        }
                    }
                    if entry.path().is_file() {
                        file_containers.push(
                            FileContainer::new(entry.path())
                                .map_err(|err| FileScannerError::FormingFileContainerError(err))?,
                        )
                    }
                } else {
                    return Err(FileScannerError::CantReadEntryAfterReadDir);
                }
            }
            // If scan mode == Immediate, clear the list
            directories_to_search = match scan_mode {
                FileScannerScanMode::Recursive => directories_to_search,
                FileScannerScanMode::Immediate => Vec::new(),
            };
        }
        Ok(file_containers)
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::systems::file_scanner::{FileScanner, FileScannerError, FileScannerScanMode};

    #[test]
    fn scan_succeeds() {
        let failure_test_path: String = "testing/THIS_DOES_NOT_EXIST".to_string();

        let failure_test_path = current_dir().unwrap().join(failure_test_path);

        let no_ignore_dirs: Vec<String> = Vec::new();

        let should_fail = FileScanner::scan(
            failure_test_path,
            FileScannerScanMode::Recursive,
            &no_ignore_dirs,
        );

        assert!(matches!(
            should_fail,
            Err(FileScannerError::PathIsNotDirectory(_))
        ));
    }

    #[test]
    fn scan_fails() {
        let successful_test_path: String = "testing/scan_test".to_string();

        let full_successful_test_path = current_dir().unwrap().join(successful_test_path);

        let no_ignore_dirs: Vec<String> = Vec::new();

        let should_pass = FileScanner::scan(
            full_successful_test_path,
            FileScannerScanMode::Recursive,
            &no_ignore_dirs,
        );

        assert!(matches!(should_pass, Ok(_)));
    }
}
