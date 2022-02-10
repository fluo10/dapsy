use super::Track;
use std::collections::HashMap;
use std::path::PathBuf;

pub Struct Library {
    track: HashMap<Track>;
    playlists: HashMap<Playlist>;
    root_dir: PathBuf;
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


impl Library {

    fn new() -> Self {
        Self{
            track: HashMap::new();
            root_dir: PathBuf::new();
        }
    }
    pub fn from_dir(&path: AsRef<PathBuf>) -> Result<Self> {
        let path = &path.as_ref(path);
        if path.is_file() {}
            for entry in WalkDir::new(path.as_ref(PathBuf)){
                match entry.extension {
                    Some("m3u") => {
                        playlists.add(entry, Playlist::from_file(entry));
                    },

                }
            }
        }
    }

    pub fn read(&mut self) {

    }
    pub fn parse_file(&mut self, AsRef<PathBuf>) {
        
    }
    pub fn merge_with_convert_to(&mut self, dst: &mut Library) {
        Library::merge_with_convert(&mut self, &mut dst);
    }
    pub fn merge_with_convert(src: &mut Library, dst: &mut Library){

    }
}