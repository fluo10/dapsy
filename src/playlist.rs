mod entry;
mod entries;

pub use entry::PlaylistEntry;
pub use entries::PlaylistEntries;

use chrono::{DateTime, Utc};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use super::track::Track;


pub struct Playlist {
    pub path: PathBuf,
    pub last_updated: DateTime<Utc>,
    pub raw: String,
    pub tracks: Vec<PathBuf>,
    pub entries: PlaylistEntries,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Default for Playlist {
    fn default() -> Self {
        Playlist{
            path: PathBuf::new(),
            last_updated: Utc::now(),
            raw: String::new(),
            tracks: Vec::new(),
            entries: PlaylistEntries::default(),
        }
    }
}

impl Playlist{
    pub fn new() -> Self {
        Default::default()
    }
    fn from_file(path: &impl AsRef<Path>) -> Result<Self> {
        let mut playlist: Playlist = Playlist::new();
        playlist.path = path.as_ref().to_path_buf();
        playlist.read();
        Ok(playlist)
    }
    pub fn read(&mut self){
     
        let f = File::open(&self.path).unwrap();
        
        self.entries = PlaylistEntries::read(f);
    }
    pub fn write(&self) {
        let file;
        if self.path.is_file() {
            file = OpenOptions::new().write(true).open(self.path.clone()).unwrap();
        } else if !self.path.exists() {
            file = File::create(self.path.clone()).unwrap();
        } else {
            panic!();
        }
        let mut buf = BufWriter::new(file);
        buf.write(self.raw.as_bytes()).unwrap();
    }
    pub fn sync_with(&mut self, other: &mut Self) -> Result<()>{
        //Compare date

        //newer.pull(older);
        
        todo!();
    }
    pub fn pull(mut self, other: &Self){
        self.tracks = other.tracks.clone();
    }
    pub fn fix_extention(&mut self, tracks: &HashMap<PathBuf, Track>) -> Result<()> {
        todo!();
    }
    pub fn check(&self) -> Result<()>{
        todo!();
    }
}

pub struct PlaylistContent {

}

#[cfg(test)]
mod tests {

    #[test]
    fn parser(){}

}
