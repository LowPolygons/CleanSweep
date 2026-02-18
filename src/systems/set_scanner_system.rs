use crate::{
    containers::file_container::FileContainer,
    systems::filter_system::filter_category_info::{FilterCategory, FilterForCategory},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetScannerError {
    #[error("Error converting Path to string")]
    ConvertPathToName,

    #[error("Error when creating a regex")]
    CreatingRegexError,

    #[error("Failed to capture number portion after it was deemed to have exist")]
    CaptureNumberAfterExistanceConfirmationError,

    #[error("Failed to extract number portion after it was deemed to have exist")]
    ExtractNumberAfterExistanceConfirmationError,

    #[error("Failed to convert what was supposed to be a stringy number into a f64")]
    ConvertStringToError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoundSet {
    pub files: Vec<String>,
}

pub struct SetScannerSystem {}

impl SetScannerSystem {
    pub fn get_found_sets(
        scanned_files: &Vec<FileContainer>,
        filters: &Vec<FilterCategory>,
    ) -> Result<Vec<FoundSet>, SetScannerError> {
        // The final datastructure
        let mut found_sets: Vec<FoundSet> = Vec::new();
        let mut path_maybe_in_set: Vec<FileContainer> = Vec::new();

        // Filter files that meet the minimum requirements to be in a set
        for file in scanned_files {
            if SetScannerSystem::maybe_in_set(file, filters) {
                path_maybe_in_set.push(file.clone());
            }
        }
        // Split files into a path to stem suffix
        let mut path_to_stem_suffix_map: HashMap<String, Vec<(String, String)>> = HashMap::new();

        for file in path_maybe_in_set {
            let dir = file.get_statistics().get_directory();
            let stem = file.get_statistics().get_name();
            let extension = file.get_statistics().get_extension();

            path_to_stem_suffix_map
                .entry(dir.clone())
                .and_modify(|vec| vec.push((stem.clone(), extension.clone())))
                .or_insert(Vec::new());
        }

        // Begin iterating over each directory and create the USPs
        for (path, stem_suffix) in &path_to_stem_suffix_map {
            // Attempt to split any number from the end of the string
            // Suffix is not modified so it can just be a slice
            let mut split_file_names: Vec<(String, &str, f64)> = Vec::new();

            // \d is a shorthand for [0-9]
            // \d+ => Any number of any digits, (\.\d+)? followed by an option decimal number with any number of any
            // digits, $ end of line
            let string_num_separator =
                Regex::new(r"\d+(\.\d+)?$").map_err(|_| SetScannerError::CreatingRegexError)?;

            for (stem, suffix) in stem_suffix {
                if string_num_separator.is_match(stem) {
                    let captures = string_num_separator.captures(stem).ok_or_else(|| {
                        SetScannerError::CaptureNumberAfterExistanceConfirmationError
                    })?;
                    let number_portion = captures
                        .get(0)
                        .ok_or_else(|| {
                            SetScannerError::ExtractNumberAfterExistanceConfirmationError
                        })?
                        .as_str();
                    split_file_names.push((
                        string_num_separator.replace_all(stem, "").to_string(),
                        suffix,
                        number_portion
                            .parse::<f64>()
                            .map_err(|_| SetScannerError::ConvertStringToError)?,
                    ));
                }
            }
            // unique string portion
            let mut usps: HashMap<String, Vec<(&str, &str, &f64)>> = HashMap::new();

            // Split into USP and sort
            for (stem, suffix, number) in &split_file_names {
                usps.entry(stem.clone())
                    .and_modify(|vec| {
                        vec.push((stem, suffix, number));
                        vec.sort_by(|tup_a, tup_b| tup_a.2.total_cmp(tup_b.2));
                    })
                    .or_insert(Vec::new());
            }

            for (_, usp_tup) in &usps {
                let mut files_in_usp: Vec<String> = Vec::new();

                // Can consume
                for (stem, suffix, number) in usp_tup {
                    files_in_usp.push(format!("{}/{}{}.{}", path, stem, number, suffix));
                }

                found_sets.push(FoundSet {
                    files: files_in_usp,
                });
            }
        }
        Ok(found_sets)
    }

    // If the files match either one OR the other
    fn maybe_in_set(file: &FileContainer, filters: &Vec<FilterCategory>) -> bool {
        for filter in filters {
            if filter.is_file_flagged(file) {
                return true;
            }
        }
        return false;
    }
}
