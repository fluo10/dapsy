use super::Track;
use crate::playlist::Playlist;
use std::collections::HashMap;
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
        let path = path.as_ref().to_path_buf();
        assert!(path.is_dir());
        let mut library = Library{
            path: path.clone(),
            ..Default::default()
        };

        for entry in WalkDir::new(path){
            library.parse_file(&entry?.path());
        }
        Ok(library)
    }

    pub fn read(&mut self) {

    }
    pub fn parse_file(&mut self, path: &impl AsRef<Path>) {
        todo!();
    }
    pub fn merge_with_convert_to(&mut self, dst: &mut Library) {
        Library::merge_with_convert(self, dst);
    }
    pub fn merge_with_convert(src: &mut Library, dst: &mut Library){

    }
}