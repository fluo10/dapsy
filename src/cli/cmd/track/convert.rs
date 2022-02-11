use clap::Parser;
use crate::Library;
use std::path::{Path,PathBuf};

#[derive(Parser)]
pub struct TrackConvertCommand {
    source: PathBuf,
    destination: PathBuf,
}

impl TrackConvertCommand{
    pub fn run(&self) {
        println!("Run track convert");
    }
}