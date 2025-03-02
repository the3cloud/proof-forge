use anyhow::Result;
use clap::Parser;

mod commands;
mod exporter;
mod forge;

fn main() -> Result<()> {
    env_logger::init();

    let args = commands::Args::parse();

    args.run()
}
