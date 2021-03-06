use super::Track;
use crate::playlist::Playlist;
use crate::Config;
use crate::converter::convert_to_mp3;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::path::{Path,PathBuf};

use anyhow::Result;
use walkdir::WalkDir;
use std::default::Default;


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
        let rel_path = abs_path.strip_prefix(&self.path).unwrap().to_path_buf();
        let base_path = rel_path.with_extension("");
        if let Some(extension) = abs_path.extension(){
            match extension.to_str() {
                //Some("m3u") => {
                //    self.playlists.insert(base_path,Playlist::new());
                //},
                Some("flac") | Some("mp3") => {
                    self.tracks.insert(base_path, Track::from_file(&self.path, &abs_path).unwrap());
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
        let src_keys: HashSet<PathBuf> = self.tracks.keys().cloned().into_iter().collect();
        let dst_keys: HashSet<PathBuf> = other.tracks.keys().cloned().into_iter().collect();
        let mut whole_keys: Vec<PathBuf> = src_keys.union(&dst_keys).cloned().collect();
        whole_keys.sort();
        for key in whole_keys.iter() {

            let src_track: &Track;
            let dst_track: &Track;
            match (self.tracks.get(key), other.tracks.get(key)) {
                (None, None) => panic!(),
                (None, Some(d)) => {
                    println!("Delete {:?}(unimplimented)", key);
                    continue;
                },
                (Some(s), None) => {
                    src_track = s;
                    other.add_track_with_rel_path(&src_track.rel_path.with_extension("mp3"));
                    dst_track = other.tracks.get(key).unwrap();
                },
                (Some(s), Some(d)) => {
                    src_track = s;
                    dst_track = d;
                }
            };

            src_track.sync_to(&dst_track);
        }
    }
    pub fn add_track_with_rel_path(&mut self, path: &impl AsRef<Path>) {
        let path: PathBuf = path.as_ref().to_path_buf();
        assert!(path.is_relative());
        self.tracks.insert(path.with_extension(""), Track{
            rel_path: path.clone(),
            abs_path: self.path.join(path),
            ..Track::default()
        });
    }
    pub fn get_track_with_rel_path(&self, path: &impl AsRef<Path>) -> Track {
        let path: PathBuf = path.as_ref().to_path_buf();
        assert!(path.is_relative());
        Track{
            rel_path: path.clone(),
            abs_path: self.path.join(path),
            ..Track::default()
        }
    }
    pub fn get_relative_path(path: &impl AsRef<Path>) -> PathBuf {
        todo!();
    }
    pub fn get_absolute_path(path: &impl AsRef<Path>) -> PathBuf {
        todo!();
    }
    pub fn get_base_path(path: &impl AsRef<Path>) -> PathBuf {
        todo!();
    }
    pub fn is_valid_path(path: &impl AsRef<Path>) -> bool {
        todo!();
    }
    
}