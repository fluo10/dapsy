use super::Track;
use crate::playlist::Playlist;
use crate::Config;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path,PathBuf};
use walkdir::WalkDir;
use std::default::Default;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Default)]
pub struct Library {
    tracks: HashMap<PathBuf,Track>,
    playlists: HashMap<PathBuf,Playlist>,
    path: PathBuf,
}



impl Library {

    pub fn from_dir(path: &impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf().canonicalize().unwrap();
        assert!(path.is_dir());
        assert!(path.is_absolute());
        let mut library = Library{
            path: path.clone(),
            ..Default::default()
        };

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()){
            library.parse_file(&entry.path());
        }
        Ok(library)
    }

    pub fn read(&mut self) {

    }
    pub fn parse_file(&mut self, path: &impl AsRef<Path>) {
        let abs_path = path.as_ref().to_path_buf().canonicalize().unwrap();
        let rel_path = abs_path.strip_prefix(&self.path).unwrap();
        if let Some(extension) = abs_path.extension(){
            match extension.to_str() {
                Some("m3u") => {
                    self.playlists.insert(rel_path.to_path_buf(),Playlist::new());
                },
                Some("flac") | Some("mp3") => {
                    self.tracks.insert(rel_path.to_path_buf(), Track::from_file(&self.path, &path).unwrap());
                },
                _ => {},
            }
        }
    }
    pub fn merge_with_convert_to(&mut self, dst: &mut Library) {
        Library::merge_with_convert(self, dst);
    }
    pub fn merge_with_convert(src: &mut Library, dst: &mut Library){

    }
    pub fn sync_tracks_with(&self, other: &mut Library) {

        for (key, src_track) in self.tracks.iter() {
            let dst_track: &Track;
            if !other.tracks.contains_key(key) {
                other.add_track_with_rel_path(key);
                dst_track = other.tracks.get(key).unwrap();
                println!("Convert from {} to new {}", src_track.abs_path.to_str().unwrap(), dst_track.abs_path.to_str().unwrap());
            } else {
                dst_track = other.tracks.get(key).unwrap();
            println!("Convert from {} to {}", src_track.abs_path.to_str().unwrap(), dst_track.abs_path.to_str().unwrap());
            }
        }
        if Config::global().dry_run {
            println!("Dry run");
        }
    }
    pub fn add_track_with_rel_path(&mut self, path: &impl AsRef<Path>) {
        let path: PathBuf = path.as_ref().to_path_buf();
        assert!(path.is_relative());
        self.tracks.insert(path.clone(), Track{
            rel_path: path.clone(),
            abs_path: self.path.join(path),
            ..Track::default()
        });
    }
}