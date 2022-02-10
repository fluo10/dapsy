use std::collections::{HashMap,HashSet};
use std::path::PathBuf;

pub struct Config {
    lossy_formats: HashSet<String>,
    lossless_formats: HashSet<String>,
    audio_formats: HashSet<String>,
    presets: HashMap<String,Preset>,

    
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Preset {
    src_path: PathBuf,
    dst_path: PathBuf,
    playlist_dir: Option<PathBuf>,
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