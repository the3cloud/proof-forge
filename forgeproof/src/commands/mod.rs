use anyhow::Result;

// mod calldata;
mod export;
mod target;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Args {
    pub fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Export(args) => args.run(),
            // Subcommand::Calldata(args) => args.run(),
            Subcommand::Forge => todo!(),
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    Export(export::Args),
    // Calldata(calldata::Args),
    Forge,
}
