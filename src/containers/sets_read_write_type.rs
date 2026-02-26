use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SetsReadWriteType {
    pub files: Vec<String>,
}
