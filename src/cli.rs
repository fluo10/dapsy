mod args;
mod cmd;

use clap::{Args,ArgGroup,Parser, Subcommand};
use std::path::PathBuf;
pub use args::GlobalArgs;
use cmd::MainSubcommand;
use crate::Library;
use crate::Config;


#[derive(Parser)]
#[clap(about, version, author)]
//#[clap(group(
//    ArgGroup::new("files").arg("source").arg("destination").required(true).multiple(true),
//))]
//#[clap(group(
//    ArgGroup::new("mode").arg("preset").conflicts_with("files"),
//))]
pub struct MainCommand {
    #[clap(short, group = "test")]
    preset: bool,
    //#[clap(short, long)]
    //config: Option<PathBuf>,
    #[clap(short='n', long)]
    dry_run: bool,
    #[clap()]
    source: Option<PathBuf>,
    #[clap()]
    destination: Option<PathBuf>,
    #[clap(subcommand)]
    pub subcommand: Option<MainSubcommand>,

}


impl MainCommand {
    pub fn run(&self) {
        if let Some(x) = &self.subcommand {
            x.run();
            ()
        };
        Config{
            dry_run: self.dry_run,
            ..Config::default()
        }.globalize();
        let src = self.source.as_ref().unwrap();
        let dst = self.destination.as_ref().unwrap();
        let mut src_library = Library::from_dir(src).unwrap();
        let mut dst_library = Library::from_dir(dst).unwrap();
        src_library.sync_tracks_with(&mut dst_library);
        println!("Convert {} to {}", src.to_str().unwrap(), dst.to_str().unwrap());
    }
}

#[derive(Args)]
pub struct FileArgs {
    pub source: PathBuf,
    pub destination: PathBuf,
}