use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Filters {
    filters: Vec<String>,
}

impl Filters {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let filters: Filters = serde_json::from_reader(reader)?;

        Ok(filters)
    }

    pub fn is_filtered(&self, line: &str) -> bool {
        self.filters.iter().any(|filter| line.contains(filter))
    }
}
