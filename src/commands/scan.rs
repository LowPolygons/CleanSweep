use crate::{
    containers::{
        cleansweep_file_paths::CleansweepFilePaths,
        file_container::FileContainer,
        file_date_data::{FileDateData, secs_since_epoch_to_time},
        user_settings::UserSettings,
    },
    filter_codes::filter_codes::FilterCodes,
    systems::{
        file_scanner::FileScanner,
        filter_system::{
            extension_filter::ExtensionFilter,
            filter_category_info::{FilterCategory, FilterForCategory},
            filter_system::FilterSystem,
            last_accessed_filter::LastAccessedFilter,
            last_modified_filter::LastModifiedFilter,
            name_filter::NameFilter,
            size_filter::SizeFilter,
        },
        json_io::{self, write_json_file_from_struct},
    },
    utils::{
        create_defaults::get_default_filter_category_list, file_to_string_vec,
        get_home_dir::get_cleansweep_dir,
    },
};
use std::env::current_dir;

// optional_subpath -> default ""
// use_custom_filters -> default false

#[rustfmt::skip]
pub fn scan(optional_subpath: &String, use_custom_filters: &bool) -> Result<(), String> {
    let mut path = current_dir().map_err(|err| format!("Error getting current dir {}", err))?;
    // Veryify that the path including the optional subpath is real
    path = path.join(optional_subpath);
   
    if !std::fs::exists(&path).map_err(|_| format!("Could not verify if the full directory {:?} exists", &path))? {
        return Err(format!("The provided path does not exist"));
    }

    let cleansweep_dir = get_cleansweep_dir()
        .map_err(|e| format!("Failed to load the cleansweep directory - {:?}", e))?;

    // Load user settings
    let user_settings: UserSettings =
        json_io::read_file_to_struct(cleansweep_dir.join(CleansweepFilePaths::UserSettings.name()))
            .map_err(|e| format!("Failed to load user settings, does it exist? {}", e))?;


    // Turn the filters into filter objects

    let mut filters_to_use: Vec<Box<dyn FilterForCategory>> = Vec::new();

    let list_of_filters_as_strings: Vec<String> = if *use_custom_filters {
        file_to_string_vec::file_to_string_vec(
            cleansweep_dir.join(CleansweepFilePaths::FilterComponentList.name()),
        )
        .map_err(|e| format!("Failed to load list of custom filters - {:?}", e))?
    } else {
        get_default_filter_category_list()
    };

    for item in list_of_filters_as_strings {
        if let Some(filter_obj) = FilterCategory::match_string_to_category(&item) {
            println!(
                "Loaded object {}, reasoning: {}",
                filter_obj.get_choice(),
                filter_obj.get_reasoning()
            );

            let mut filter: Box<dyn FilterForCategory>;
            let keep_filter_item: FilterCategory;
            let delete_filter_item: FilterCategory;

            match &filter_obj.get_filter() {
                FilterCategory::Name(_) => {
                    filter = Box::new(NameFilter::new());
                    keep_filter_item =
                        FilterCategory::Name(user_settings.get_to_keep_list().get_name().clone());
                    delete_filter_item =
                        FilterCategory::Name(user_settings.get_to_delete_list().get_name().clone());
                }
                FilterCategory::Size(_) => {
                    filter = Box::new(SizeFilter::new());
                    keep_filter_item =
                        FilterCategory::Size(user_settings.get_to_keep_list().get_larger_than_size().clone());
                    delete_filter_item =
                        FilterCategory::Size(user_settings.get_to_delete_list().get_larger_than_size().clone());
                }
                FilterCategory::Extension(_) => {
                    filter = Box::new(ExtensionFilter::new());
                    keep_filter_item =
                        FilterCategory::Extension(user_settings.get_to_keep_list().get_extensions().clone());
                    delete_filter_item =
                        FilterCategory::Extension(user_settings.get_to_delete_list().get_extensions().clone());
                }
                FilterCategory::LastAccessed(_) => {
                    filter = Box::new(LastAccessedFilter::new());
                    keep_filter_item = 
                        FilterCategory::LastAccessed(
                            FileDateData::new(secs_since_epoch_to_time(user_settings.get_to_keep_list().get_accessed_after().clone())
                        ));
                    delete_filter_item =
                        FilterCategory::LastAccessed(
                            FileDateData::new(secs_since_epoch_to_time(user_settings.get_to_delete_list().get_accessed_after().clone())
                        ));
                }
                FilterCategory::LastModified(_) => {
                    filter = Box::new(LastModifiedFilter::new());
                    keep_filter_item = 
                        FilterCategory::LastModified(
                            FileDateData::new(secs_since_epoch_to_time(user_settings.get_to_keep_list().get_modified_after().clone())
                        ));
                    delete_filter_item =
                        FilterCategory::LastModified(
                            FileDateData::new(secs_since_epoch_to_time(user_settings.get_to_delete_list().get_modified_after().clone())
                        ));
                }
            }

            filter.init(keep_filter_item, delete_filter_item).map_err(|e| format!("{:?}", e))?;
            filters_to_use.push(filter);
        }
    }

    // Build the filter object
    let filter_system: FilterSystem = FilterSystem::new(filters_to_use);

    // Perform scan
    let scanned_files: Vec<FileContainer> = FileScanner::scan(path)
        .map_err(|err| format!("Failed to perform scan - {:?}", err))?;

    let mut to_keep: Vec<String> = Vec::new();
    let mut to_delete: Vec<String> = Vec::new();

    // Sort files into keep/delete lists
    for file_object in scanned_files {
        let result: FilterCodes = filter_system.filter_file(&file_object)
            .map_err(|e| format!("Failed to scan the file object {:?}, error: {:?}", file_object, e))?;

        match &result {
            FilterCodes::ToKeep => to_keep.push(
                file_object
                    .get_path()
                    .clone()
                    .into_os_string()
                    .into_string()
                .map_err(|_| format!("Failed to convert path {:?} to string", file_object.get_path()))?),
            FilterCodes::ToDelete => to_delete.push(
                file_object
                    .get_path()
                    .clone()
                    .into_os_string()
                    .into_string()
                .map_err(|_| format!("Failed to convert path {:?} to string", file_object.get_path()))?),
            FilterCodes::NonSpecial => {}
        }
    } 

    // TODO: maybe make an explicit struct representing a List of filecontainers
    write_json_file_from_struct(&to_keep, cleansweep_dir.join(CleansweepFilePaths::ToKeep.name()))
        .map_err(|e| format!("Failed to save list of files to keep - {:?}", e))?;

    write_json_file_from_struct(&to_delete, cleansweep_dir.join(CleansweepFilePaths::ToDelete.name()))
        .map_err(|e| format!("Failed to save list of files to delete - {:?}", e))?;
    Ok(())    
}
