use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    abbr: String,
    desc: String,
}

impl Entry {
    #[cfg(test)]
    pub fn new(abbr: &str, desc: &str) -> Self {
        Self {
            abbr: abbr.to_string(),
            desc: desc.to_string(),
        }
    }
    pub fn abbreviation(&self) -> &str {
        &self.abbr
    }

    pub fn description(&self) -> &str {
        &self.desc
    }
}

pub static ABBREVIATIONS: Lazy<Vec<Entry>> = Lazy::new(|| {
    let json = include_str!("abbreviations.json");
    serde_json::from_str(json).expect("Failed to parse abbreviations from JSON")
});
