use anyhow::Result;

// mod calldata;
mod export;
mod forge;
pub mod target;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Args {
    pub fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Export(args) => args.run(),
            Subcommand::Forge(args) => args.run(),
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    Export(export::Args),
    Forge(forge::Args),
}
