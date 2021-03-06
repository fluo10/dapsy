use std::fs::metadata;
use std::path::{Path,PathBuf};
use std::time::SystemTime;
use crate::converter::convert_to_mp3;
use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::default::Default;



#[derive(Clone)]
pub struct Track {
    pub abs_path: PathBuf,
    pub rel_path: PathBuf,
    pub modified: SystemTime,
}

impl Default for Track {
    fn default() -> Self {
        Self{
            abs_path: PathBuf::new(),
            rel_path: PathBuf::new(),
            modified: SystemTime::now(),
        }
    }
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
                println!("Convert {:?} to existing file", &self.rel_path);
                return convert_to_mp3(&self.abs_path, &other.abs_path)
            } else {
                println!("Skip {:?} (Destination is newer)", &self.rel_path);
                Ok(())
            }
        } else if !other.abs_path.exists() {
            println!("Convert {:?} to new file", &self.rel_path);
            convert_to_mp3(&self.abs_path, &other.abs_path)
        } else {
            Ok(())
        }
    }
}

