use crate::{
    cli::ListAndResetArgs,
    containers::{
        cleansweep_file_paths::CleansweepFilePaths, sets_read_write_type::SetsReadWriteType,
    },
    systems::json_io::read_file_to_struct,
    utils::get_home_dir::get_cleansweep_dir,
};

pub fn list(args: &ListAndResetArgs) -> Result<(), String> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|e| format!("{}", e))?;

    let mut args_is_sets: bool = false;
    let label: String;
    let path_to_open = match args {
        ListAndResetArgs::ToDelete => {
            label = "To Delete".to_string();
            CleansweepFilePaths::ToDelete
        }
        ListAndResetArgs::ToKeep => {
            label = "To Keep".to_string();
            CleansweepFilePaths::ToKeep
        }
        ListAndResetArgs::Sets => {
            // The structure of the sets file is different
            label = "Found Sets".to_string();
            args_is_sets = true;
            CleansweepFilePaths::FoundSets
        }
    };

    if args_is_sets {
        let list_of_files: Vec<SetsReadWriteType> =
            read_file_to_struct(cleansweep_dir.join(path_to_open.name()))
                .map_err(|e| format!("{}", e))?;

        println!("{}:", label);
        for set in &list_of_files {
            println!("- {}:", set.get_prefix());

            for file in set.get_list() {
                println!("- - {}", file);
            }
        }
    } else {
        let list_of_files: Vec<String> =
            read_file_to_struct(cleansweep_dir.join(path_to_open.name()))
                .map_err(|e| format!("{}", e))?;

        println!("{}:", label);
        for file in list_of_files {
            println!("- {}", file);
        }
    }

    Ok(())
}
