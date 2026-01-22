pub mod themes;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub paper_size: String,
    pub theme: String,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paper_size: "a4".to_string(),
            theme: "default".to_string(),
            verbose: false,
        }
    }
}

impl Config {
    pub fn paper_dimensions(&self) -> (f64, f64) {
        match self.paper_size.to_lowercase().as_str() {
            "letter" => (8.5 * 72.0, 11.0 * 72.0),
            "legal" => (8.5 * 72.0, 14.0 * 72.0),
            _ => (210.0 * 2.83465, 297.0 * 2.83465), // A4 in points
        }
    }

    pub fn paper_typst(&self) -> &str {
        match self.paper_size.to_lowercase().as_str() {
            "letter" => "us-letter",
            "legal" => "us-legal",
            _ => "a4",
        }
    }
}
