mod convert;

use convert::TrackConvertCommand;
use clap::{Parser,Subcommand};
use super::GlobalArgs;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct TrackCmd {
    #[clap(subcommand)]
    pub sub_command: TrackSubCommand,
}

#[derive(Subcommand)]
pub enum TrackSubCommand {
    Convert(TrackConvertCommand),
}