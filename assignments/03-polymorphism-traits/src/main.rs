mod args;
mod input_reader;
mod tests;
use args::Args;
use input_reader::InputReader;

use anyhow::Result;
use clap::Parser;
use std::fs::File;

fn main() -> Result<()> {
    let args = Args::parse();
    if args.file.is_none() {
        let stdin = std::io::stdin();
        stdin.search(&args);
    } else {
        let file = File::open(args.file.clone().unwrap())?;
        file.search(&args);
    }
    Ok(())
}
