use clap::Parser;
use super::GlobalArgs;

#[derive(Parser)]
pub struct InitCmd {
    #[clap(flatten)]
    pub global: GlobalArgs,
}