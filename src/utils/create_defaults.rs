use crate::containers::user_settings::*;

pub fn create_default_user_settings() -> UserSettings {
    UserSettings::new(
        PerFilterOptions::new(
            vec!["z".to_string(), "exe".to_string(), "d".to_string()], //with_extension,
            vec!["cleansweep".to_string()],                            //name_contains,
            vec!["cleansweep".to_string()],                            //path_contains,
            vec![".".to_string()],                                     //name_starts_with,
            10000000000, // ~10 GB                                     //larger_than,
            0,           // modify_date_cutoff,
            0,           // access_date_cutoff,
        ), //to_keep_list,
        PerFilterOptions::new(
            vec!["out".to_string()], //with_extension,
            vec![
                "OUTPUT".to_string(),
                "HISTORY".to_string(),
                "slurm-".to_string(),
            ], //name_contains,
            vec!["deleteme".to_string()], //path_contains,
            Vec::new(),              //name_starts_with,
            1001,                    //larger_than,
            0,                       // modify_date_cutoff,
            0,                       // access_date_cutoff,
        ), // to_delete_list,
        SetScanOptions::new(
            vec!["h5".to_string()], // with_extension,
            Vec::new(),             //name_contains
        ), // set_scan_options,
    )
}

pub fn get_default_filter_category_list() -> Vec<String> {
    return vec![
        "name".to_string(),
        "extension".to_string(),
        "size".to_string(),
        "modify".to_string(),
    ];
}
