use crate::systems::file_scanner::FileScanner;
use std::env::current_dir;

// TODO: allow the user to pass in either a path to a list of filter options, if none provided it
// uses them all, or any number of arguments and it only uses those
// alteranatively, have a flag like --use-set-components that uses a defined file in the
// .cleansweep dir
pub fn scan(args: &String, use_custom_filters: &bool) -> Result<(), String> {
    let path = current_dir().map_err(|err| format!("Error getting current dir {}", err))?;

    println!("{:?}", path);

    let paths = FileScanner::scan(path).map_err(|err| {
        println!("{:?}", err);
        "Failure"
    })?;

    println!("{}", paths.len());

    Ok(())
}
