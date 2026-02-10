use crate::containers::user_settings::*;

pub fn create_default_user_settings() -> UserSettings {
    UserSettings::new(
        PerFilterOptions::new(
            Some(vec!["z".to_string(), "exe".to_string(), "d".to_string()]), //with_extension,
            Some(vec!["cleansweep".to_string()]),                            //name_contains,
            Some(vec!["cleansweep".to_string()]),                            //path_contains,
            Some(vec![".".to_string()]),                                     //name_starts_with,
            Some(10000000000), // ~10 GB                                     //larger_than,
        ), //to_keep_list,
        Some(1000), //keep_if_smaller_than,
        PerFilterOptions::new(
            Some(vec!["out".to_string()]), //with_extension,
            Some(vec![
                "OUTPUT".to_string(),
                "HISTORY".to_string(),
                "slurm-".to_string(),
            ]), //name_contains,
            Some(vec!["deleteme".to_string()]), //path_contains,
            None,                          //name_starts_with,
            Some(1001),                    //larger_than,
        ), // to_delete_list,
        SetScanOptions::new(
            Some(vec!["h5".to_string()]), // with_extension,
            None,                         //name_contains
        ), // set_scan_options,
        None,       // modify_date_cutoff,
        None,       // access_date_cutoff,
    )
}
