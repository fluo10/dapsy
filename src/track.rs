use std::fs::metadata;
use std::path::{Path,PathBuf};
use std::time::SystemTime;
use crate::converter::convert_to_mp3;
use crate::config::Config;
use chrono::{DateTime, Utc};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Clone)]
pub struct Track {
    abs_path: PathBuf,
    rel_path: PathBuf,
    modified: SystemTime,
}

impl Track {
    pub fn new() -> Self {
        Self{
            abs_path: PathBuf::new(),
            rel_path: PathBuf::new(),
            modified: SystemTime::now(),
        }
    }
    pub fn from_file(library: &impl AsRef<Path>, track: &impl AsRef<Path>) -> Result<Self> {
        let abs_path= track.as_ref().canonicalize()?;
        let rel_path= abs_path.clone().strip_prefix(library.as_ref().canonicalize()?)?.to_path_buf();
        let metadata= metadata(track)?;
        let modified= metadata.modified()?;
        Ok(Self{
            abs_path: abs_path,
            rel_path: rel_path,
            modified: modified,
        })
    }
    pub fn sync_to( &self, other: &Self) -> Result<()> {
        let config = Config::global();
        if other.abs_path.is_file() {
            if self.modified > other.modified {
                return convert_to_mp3(&self.abs_path, &other.abs_path)
            } else {
                Ok(())
            }
        } else if !other.abs_path.exists() {
            convert_to_mp3(&self.abs_path, &other.abs_path)
        } else {
            Ok(())
        }
    }
}

