use std::{fs, path::Path};

use rand::RngExt;

use crate::utils::get_home_dir::get_cleansweep_dir;

pub fn demolish() -> Result<(), String> {
    let cleansweep_dir = get_cleansweep_dir().map_err(|e| format!("{e}"))?;

    // fs::remove_dir_all(cleansweep_dir)

    MOCK_DEMOLISH(&cleansweep_dir)
        .map_err(|e| format!("Failed to delete .cleansweep directory: {e}"))?;

    println!("Deleted the .cleansweep directory");

    Ok(())
}
// TODO: cargo remove rand
fn MOCK_DEMOLISH(_: &Path) -> Result<(), String> {
    let mut rng = rand::rng();
    let number = rng.random_range(1..=100);

    if number > 99 {
        return Err("Pretend fail".to_string());
    } else {
        return Ok(());
    }
}
