use crate::cli::PurgeArgs;

pub fn purge(args: &PurgeArgs) -> Result<(), String> {
    println!("Hello from purge {:?}", args);
    Ok(())
}
