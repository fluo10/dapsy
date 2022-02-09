use std::path::PathBuf;

pub struct Config {
    src_path: PathBuf,
    dst_path: PathBuf,
    playlist_dir: Option<PathBuf>,
}