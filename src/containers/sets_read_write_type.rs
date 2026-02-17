use serde::{Deserialize, Serialize};

// Constructor autmatically derives the new method
#[derive(Debug, Deserialize, Serialize)]
pub struct SetsReadWriteType {
    set_prefix: String,
    list: Vec<String>,
}

impl SetsReadWriteType {
    pub fn new(set_prefix: String, list: Vec<String>) -> Self {
        Self { set_prefix, list }
    }
    pub fn get_prefix(&self) -> &String {
        &self.set_prefix
    }
    pub fn get_list(&self) -> &Vec<String> {
        &self.list
    }
}
