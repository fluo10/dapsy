use std::path::{Path,PathBuf};
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum ConvertOption {
    OverWrite,
    DryRun,
}
pub fn convert_to_mp3(src: &impl AsRef<Path>, dst: &impl AsRef<Path>) -> Result<()> {
    let src_path: &Path = src.as_ref();
    let dst_path: &Path = dst.as_ref();
    
    let output = Command::new("ffmpeg").args(["-i", src_path.to_str().unwrap(),
        "-f", "mp3",
        "-b:a", "256k",
        dst_path.to_str().unwrap()]).output()?;
    if !output.status.success() {
    }
    Ok(())
}