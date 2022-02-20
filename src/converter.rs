use std::path::{Path,PathBuf};
use std::process::Command;
use std::fs::create_dir_all;
use std::io::{stdout, stderr, Write};
use crate::Config;

use anyhow::Result;

pub enum ConvertOption {
    OverWrite,
    DryRun,
}

pub fn convert_to_mp3(src: &impl AsRef<Path>, dst: &impl AsRef<Path>) -> Result<()> {
    let src_path: &Path = src.as_ref();
    let dst_path: &Path = dst.as_ref();
    let mut dst_dir: PathBuf = dst_path.to_path_buf();
    dst_dir.pop();
    if !dst_dir.exists() {
        create_dir_all(dst_dir).unwrap_or_else(|e| eprintln!("Failed to create dir {}", e));
    }
    if Config::global().dry_run {
        println!("Convert {} to {}", src_path.to_str().unwrap(), dst_path.to_str().unwrap());
        Ok(())
    } else {
        let output = Command::new("ffmpeg").args(["-i", src_path.to_str().unwrap(),
            "-f", "mp3",
            "-q:a", "6",
            dst_path.to_str().unwrap()]).output()?;
        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(&output.stderr).unwrap();
        Ok(())
    }
}