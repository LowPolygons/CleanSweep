use std::path::{Path, PathBuf};

use crate::containers::file_container::{FileContainer, FileContainerInitError};

pub struct FileScanner {}

impl FileScanner {
    pub fn new() -> Self {
        return Self {};
    }

    // Utilise a stack to safely perform a breadth first search
    pub fn scan<P: AsRef<Path>>(path: P) -> Result<Vec<FileContainer>, String> {
        let path: PathBuf = path.as_ref().into();

        if !path.is_dir() {
            return Err("Passed path is not a directory".to_string());
        }

        let mut file_containers: Vec<FileContainer> = Vec::new();
        let mut directories_to_search: Vec<PathBuf> = Vec::new();

        directories_to_search.push(path);

        while let Some(path) = directories_to_search.pop() {
            if !path.is_dir() {
                return Err(format!("Target path {:?} is not a directory", path));
            }
            let elements_in_dir = path
                .read_dir()
                .map_err(|err| format!("Failed to read directory {}", err))?;

            for entry in elements_in_dir {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        directories_to_search.push(entry.path());
                    }
                    if entry.path().is_file() {
                        file_containers.push(
                            FileContainer::new(entry.path())
                                .map_err(|err| format!("Error initing FileContainer: {:?}", err))?,
                        )
                    }
                } else {
                    return Err(format!("Failed to read entry {:?}", entry));
                }
            }
        }
        Ok(file_containers)
    }
}
