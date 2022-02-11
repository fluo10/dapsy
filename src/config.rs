use std::collections::{HashMap,HashSet};
use std::path::PathBuf;
use once_cell::sync::OnceCell;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static INSTANCE: OnceCell<Config> = OnceCell::new();

#[derive(Debug)]
pub struct Config {
    lossy_formats: HashSet<String>,
    lossless_formats: HashSet<String>,
    audio_formats: HashSet<String>,
    presets: HashMap<String,Preset>,
}
#[derive(Debug)]
pub struct Preset {
    src_path: PathBuf,
    dst_path: PathBuf,
    playlist_dir: Option<PathBuf>,
}
impl Config {
    pub fn global() -> &'static Config {
        INSTANCE.get().expect("Config is not initialized")
    }
    pub fn globalize(self) {
        INSTANCE.set(self).unwrap();
    }
}
impl Default for Config {
    fn default() -> Config {
        const LOSSY_FORMATS: &str = "mp3,aac";
        const LOSSLESS_FORMATS: &str = "flac,wav";

        let lossy_formats: HashSet<String> =  LOSSY_FORMATS.split(',').map(|s| s.to_string()).collect();
        let lossless_formats: HashSet<String> = LOSSLESS_FORMATS.split(',').map(|s| s.to_string()).collect();
        let audio_formats: HashSet<String> = lossy_formats.union(&lossless_formats).map(|s| s.to_string()).collect();

        Config{
            lossy_formats: lossy_formats,
            lossless_formats: lossless_formats,
            audio_formats: audio_formats,
            presets: HashMap::new(),
        }
        
    }
    
}