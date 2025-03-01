use anyhow::Result;
use clap::Parser;

mod commands;
mod forge;

fn main() -> Result<()> {
    let args = commands::Args::parse();

    args.run()
}
