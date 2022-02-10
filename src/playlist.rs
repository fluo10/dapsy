use chrono::{DateTime, Utc};
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use super::track::Track;

pub struct Playlist {
    path: PathBuf,
    last_updated: DateTime<Utc>,
    tracks: Vec<Track>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


impl Playlist{
    pub fn new() -> Self {
        Playlist{
            path: PathBuf::new(),
            last_updated: Utc::now(),
            tracks: Vec::new(),
        }
    }
    pub fn from_file(path: &impl AsRef<Path>) -> Self {
        todo!();
    }
    pub fn sync_with(&mut self, other: &mut Self) -> Result<()>{
        //Compare date

        //newer.pull(older);
        todo!();
    }
    pub fn pull(&mut self, other: &Self){
        self.tracks = other.tracks.clone();
    }
    pub fn fix_extention(&mut self, tracks: &HashMap<PathBuf, Track>) -> Result<()> {
        todo!();
    }
    pub fn check(&self) -> Result<()>{
        todo!();
    }

}