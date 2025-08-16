pub mod fetcher;
pub mod reader;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub author: String,
    pub name: String,
    pub version: String,
}

impl PackageInfo {
    pub fn from_full_name(full_name: &str, version: &str) -> Option<Self> {
        let parts: Vec<&str> = full_name.split('/').collect();
        if parts.len() == 2 {
            Some(PackageInfo {
                author: parts[0].to_string(),
                name: parts[1].to_string(),
                version: version.to_string(),
            })
        } else {
            None
        }
    }
}
