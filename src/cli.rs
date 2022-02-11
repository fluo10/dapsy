mod args;
mod cmd;

use clap::{Args,ArgGroup,Parser, Subcommand};
use std::path::PathBuf;
pub use args::GlobalArgs;
use cmd::MainSubcommand;


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
        let src = self.source.as_ref().unwrap();
        let dst = self.destination.as_ref().unwrap().clone();
        println!("Convert {} to {}", src.to_str().unwrap(), dst.to_str().unwrap());

    }
}

#[derive(Args)]
pub struct FileArgs {
    pub source: PathBuf,
    pub destination: PathBuf,
}