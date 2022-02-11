use clap::Parser;
use crate::Library;
use std::path::{Path,PathBuf};

#[derive(Parser)]
pub struct TrackConvertCommand {
    #[clap(short='n',long)]
    dry_run: bool,
    source: PathBuf,
    destination: PathBuf,
}

impl TrackConvertCommand{
    pub fn run(&self) {
        println!("Run track convert");
        let mut source_files:Vec<PathBuf> = Vec::new();
        if self.source.is_file() {
            source_files.push(self.source.clone());
        } else if self.source.is_dir() {
            
        }
        if self.dry_run {
            println!("Convert from")
        }
    }
}