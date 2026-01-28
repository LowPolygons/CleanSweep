use crate::cli::SettingsArgs;

pub fn settings(args: &SettingsArgs) -> Result<(), String> {
    println!("hello from settings {:?}", args);
    Ok(())
}
