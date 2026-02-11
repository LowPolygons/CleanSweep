use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TxtFileStringVecError<P: AsRef<Path>> {
    #[error("The file could not be verified to exist or not")]
    FileExistanceNotVerifiable,

    #[error("Could not find the provided file {0}")]
    FileNotFound(P),

    #[error("Could not read to string - path: {0}")]
    FileNotReadableToString(P),
}

pub fn txt_file_to_string_vec<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<String>, TxtFileStringVecError<P>> {
    if !std::fs::exists(&path).map_err(|_| TxtFileStringVecError::FileExistanceNotVerifiable)? {
        return Err(TxtFileStringVecError::FileNotFound(path));
    }

    let string_result: String = std::fs::read_to_string(&path)
        .map_err(|_| TxtFileStringVecError::FileNotReadableToString(path))?;

    let line_by_line: Vec<String> = string_result.lines().map(|x| x.to_string()).collect();

    Ok(line_by_line)
}

