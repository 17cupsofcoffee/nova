mod generated;

use std::path::Path;

pub use generated::*;

impl LdtkJson {
    pub fn from_str(input: &str) -> LdtkJson {
        serde_json::from_str(input).unwrap()
    }

    pub fn from_file(path: impl AsRef<Path>) -> LdtkJson {
        let json = std::fs::read_to_string(path).unwrap();
        LdtkJson::from_str(&json)
    }
}
