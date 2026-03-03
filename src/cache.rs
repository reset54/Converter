use serde::{Deserialize, Serialize};
use std::fs;


#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppCache {
    pub last_input: String,
    pub last_output: String,
}


impl AppCache {
    pub fn load() -> Self {
        fs::read_to_string("cache.json")
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }


    pub fn save(&self) {
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write("cache.json", content);
        }
    }
}
