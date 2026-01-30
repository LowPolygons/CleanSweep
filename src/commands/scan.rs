use crate::containers::file_container::FileContainer;
use crate::systems::file_scanner::FileScanner;
use std::env::current_dir;

pub fn scan(args: &String) -> Result<(), String> {
    let path = current_dir().map_err(|err| format!("Error getting current dir {}", err))?;

    println!("{:?}", path);

    let paths = FileScanner::scan(path).map_err(|err| {
        println!("{:?}", err);
        "Failure"
    })?;

    println!("{}", paths.len());

    Ok(())
}
