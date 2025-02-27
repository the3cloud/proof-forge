use anyhow::Result;
use clap::Parser;

mod commands;

fn main() -> Result<()> {
    let args = commands::Args::parse();

    args.run()
}
