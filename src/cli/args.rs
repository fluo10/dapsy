use clap::Args;

#[derive(Args)]
pub struct GlobalArgs {
    #[clap(short, long)]
    verbose: bool,
}