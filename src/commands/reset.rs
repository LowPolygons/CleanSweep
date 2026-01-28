use crate::cli::ListAndResetArgs;

pub fn reset(args: &ListAndResetArgs) -> Result<(), String> {
    println!("Hello from reset {:?}", args);
    Ok(())
}
