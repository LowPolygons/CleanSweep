use crate::{
    containers::{file_container::FileContainer, sets_read_write_type::SetsReadWriteType},
    systems::filter_system::filter_category_info::FilterCategory,
};
use regex::Regex;
use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetScannerError {
    #[error("Error when creating a regex")]
    CreatingRegexError,

    #[error("Failed to capture number portion after it was deemed to have exist")]
    CaptureNumberAfterExistanceConfirmationError,

    #[error("Failed to extract number portion after it was deemed to have exist")]
    ExtractNumberAfterExistanceConfirmationError,

    #[error("Failed to convert what was supposed to be a stringy number into a f64")]
    ConvertStringToError,
}

pub struct SetScannerSystem {}

impl SetScannerSystem {
    pub fn get_found_sets(
        scanned_files: &Vec<FileContainer>,
        filters: &Vec<FilterCategory>,
    ) -> Result<Vec<SetsReadWriteType>, SetScannerError> {
        // The final datastructure
        let mut found_sets: Vec<SetsReadWriteType> = Vec::new();

        // For all FileContains, this will create a tuple mapping its directory
        // to a vector of tuple of the separated 'stem' and 'suffix'
        // EG: /home/user/hello.txt, hello2.txt
        //  -> "/home/user" : {
        //      ("hello", "txt"),
        //      ("hello2", "txt")
        //  }
        let path_to_stem_suffix_map = scanned_files.into_iter().fold(
            HashMap::<String, Vec<(String, String)>>::new(),
            |mut path_stem_suffix_map, curr_file| {
                if !SetScannerSystem::maybe_in_set(curr_file, filters) {
                    return path_stem_suffix_map;
                }

                let stats = curr_file.get_statistics();
                let dir = stats.get_directory();
                let stem = stats.get_name();
                let extension = stats.get_extension();

                path_stem_suffix_map
                    .entry(dir.clone())
                    .and_modify(|vec| vec.push((stem.clone(), extension.clone())))
                    .or_insert(vec![(stem.clone(), extension.clone())]);

                path_stem_suffix_map
            },
        );

        let string_num_separator =
            Regex::new(r"\d+(\.\d+)?$").map_err(|_| SetScannerError::CreatingRegexError)?;

        found_sets = path_to_stem_suffix_map
            .into_iter()
            .try_fold(found_sets, |mut found_sets, (path, stem_suffix)| {
                // Step 1:
                //  // -:iunmap <Tab> Iterate over the tuples and attempt to extract the number portion off the end of the
                //  // - stem
                //  // - If it cannot find one, it is not in a map
                //  // - This is then stored in a Vec::<(String, String, f64)>
                //  // - Eg: ("hello2", "txt") -> ("hello", "txt", 2)
                let split_file_names = stem_suffix
                    .into_iter()
                    .try_fold(
                        Vec::<(String, String, f64)>::new(),
                        |mut split_file_names, (stem, suffix)| {
                            if !string_num_separator.is_match(&stem) {
                                return Ok(split_file_names);
                            }
                            let captures =
                                string_num_separator.captures(&stem).ok_or_else(|| {
                                    SetScannerError::CaptureNumberAfterExistanceConfirmationError
                                })?;
                            let number_portion = captures
                                .get(0)
                                .ok_or_else(|| {
                                    SetScannerError::ExtractNumberAfterExistanceConfirmationError
                                })?
                                .as_str();
                            split_file_names.push((
                                string_num_separator.replace_all(&stem, "").to_string(),
                                suffix.to_string(),
                                number_portion
                                    .parse::<f64>()
                                    .map_err(|_| SetScannerError::ConvertStringToError)?,
                            ));
                            Ok(split_file_names)
                        },
                    )
                    .map_err(|e| e)?;

                // Step 2:
                //  // - Some directories can contain multiple sets
                //  // - This will separate them out based on the 'stem' portion
                //  // - This string portion is referred to as the Unique Stem Portion
                //  // - Creates a new map for this directory which maps usps to the list of files
                let usps = split_file_names.into_iter().fold(
                    HashMap::<String, Vec<(String, String, f64)>>::new(),
                    |mut usps, (stem, suffix, number)| {
                        // println!("{} {} {}", stem, suffix, number);
                        usps.entry(stem.clone())
                            .and_modify(|vec| {
                                vec.push((stem.clone(), suffix.clone(), number));
                                vec.sort_by(|tup_a, tup_b| tup_a.2.total_cmp(&tup_b.2));
                            })
                            .or_insert(vec![(stem.clone(), suffix.clone(), number)]);
                        usps
                    },
                );
                // Step 3:
                //  // - For each USP, it turns its Vec<(Tuple)> into a Vec<String>
                //  // - The new string is the formatted path again for each File
                //  // - This vector is converted into the SetsReadWriteType object and added to found_sets
                found_sets = usps
                    .into_iter()
                    .fold(found_sets, |mut found_sets, (_, usp_tup)| {
                        let files_in_usp = usp_tup
                            .into_iter()
                            .map(|(stem, suffix, number)| {
                                format!("{}/{}{}.{}", path, stem, number, suffix)
                            })
                            .collect();

                        found_sets.push(SetsReadWriteType {
                            files: files_in_usp,
                        });

                        found_sets
                    });

                Ok(found_sets)
            })
            .map_err(|e| e)?;

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
