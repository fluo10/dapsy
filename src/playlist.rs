use chrono::{DateTime, Utc};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use super::track::Track;

static BLANK_RE: OnceCell<Regex> = OnceCell::new();
static EXT_RE: OnceCell<Regex> = OnceCell::new();

pub struct Playlist {
    pub path: PathBuf,
    pub last_updated: DateTime<Utc>,
    pub raw: String,
    pub tracks: Vec<PathBuf>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Default for Playlist {
    fn default() -> Self {
        Playlist{
            path: PathBuf::new(),
            last_updated: Utc::now(),
            raw: String::new(),
            tracks: Vec::new(),
        }
    }
}

impl Playlist{
    pub fn new() -> Self {
        Default::default()
    }
    pub fn from(path: &impl AsRef<Path>) -> Self {
        let mut playlist: Playlist = Playlist::new();
        playlist.path = path.as_ref().to_path_buf();
        playlist.read();
        playlist
    } 
    pub fn read(&mut self){
        const EXT_PATTERN:&str = r##"(?m)^[^# ].*$"##;
        EXT_RE.get_or_try_init(|| Regex::new(EXT_PATTERN)).unwrap();
        const BLANK_PATTERN:&str = r##"^\s.*"##;
        BLANK_RE.get_or_try_init( || Regex::new(BLANK_PATTERN)).unwrap();

        let f = File::open(&self.path).unwrap();
        let mut reader = BufReader::new(f);
        for line in reader.lines(){
            let line = line.unwrap();
            if EXT_RE.get().unwrap().is_match(&line) {
                self.tracks.push(PathBuf::from(&line));
            }
        }
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
