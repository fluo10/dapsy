use std::path::PathBuf;

use chrono::{DateTime, Utc};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Clone)]
pub struct Track {
    path: PathBuf,
    last_updated: DateTime<Utc>,
}

impl Track {
    pub fn convert( path: &PathBuf) -> Result<()> {
        todo!();
    }
}