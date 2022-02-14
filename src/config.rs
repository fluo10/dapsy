use std::collections::{HashMap,HashSet};
use std::path::PathBuf;
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static INSTANCE: OnceCell<Config> = OnceCell::new();



#[serde(default)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub lossy_formats: HashSet<String>,
    pub lossless_formats: HashSet<String>,
    pub audio_formats: HashSet<String>,
    pub presets: HashMap<String,Preset>,
    #[serde(skip)]
    pub dry_run: bool,
    #[serde(skip)]
    pub verbose: bool,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Preset {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub playlist_dir: Option<PathBuf>,
}
impl Config {
    pub fn global() -> &'static Config {
        INSTANCE.get().expect("Config is not initialized")
    }
    pub fn globalize(self) {
        INSTANCE.set(self).unwrap();
    }

    pub fn from_toml(t: &str) -> Self {
        toml::from_str(t).unwrap()
    }
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }
    pub fn read(path: &impl AsRef<PathBuf>) {

    }
    pub fn write(&self, path: &impl AsRef<PathBuf>) {

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
            dry_run: false,
            verbose: false,
        }
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_toml() {
        const TOML: &str = r#"[presets.test_preset]
        source= "path/to/source/dir"
        destination = "path/to/destination/dir"
        "#;
        let config_from_toml = Config::from_toml(TOML);
        let mut config = Config::default();
        config.presets.insert( "test_preset".to_string(), Preset{
            source: PathBuf::from("path/to/source/dir"),
            destination : PathBuf::from("path/to/destination/dir"),
            playlist_dir: None,
        });
        assert_eq!(config_from_toml, config);
    }
}