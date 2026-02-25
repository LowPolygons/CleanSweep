use thiserror::Error;

#[derive(Debug, Error)]
pub enum SizeInBytesToOtherError {
    #[error("You are seemingly handling more than 1000 Exabytes, something has gone wrong")]
    HandlingMoreThan1000Exas,
}

pub fn size_in_bytes_to_formatted_string(size: u64) -> Result<String, SizeInBytesToOtherError> {
    // Cast to f64 for better division precision
    let mut size_divisible = size as f64;
    let original_size = size as f64;

    let mut num_divisibles_by_1024 = 0;

    while size_divisible / 1024.0 > 1.0 {
        size_divisible = size_divisible / 1024.0;

        num_divisibles_by_1024 = num_divisibles_by_1024 + 1;
    }

    match num_divisibles_by_1024 {
        0 => return Ok(format!("{} Bytes", original_size)),
        1 => return Ok(format!("{:.4} Kilobytes", size_divisible)),
        2 => return Ok(format!("{:.4} Megabytes", size_divisible)),
        3 => return Ok(format!("{:.4} Gigabytes", size_divisible)),
        4 => return Ok(format!("{:.4} Terabytes", size_divisible)),
        5 => return Ok(format!("{:.4} PetaBytes", size_divisible)),
        6 => {
            return Ok(format!("{:.4} ExaBytes... Yikes..", size_divisible));
        }
        _ => return Err(SizeInBytesToOtherError::HandlingMoreThan1000Exas),
    }
}
