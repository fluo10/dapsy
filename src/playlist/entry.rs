use std::collections::HashMap;
use std::default::Default;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlaylistEntry {
    pub ext: HashMap<String, String>,
    pub path: PathBuf,
}

