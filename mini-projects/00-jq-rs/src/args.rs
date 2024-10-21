use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[clap(short = 'C', long)]
    pub color_output: bool,

    #[clap(short = 'M', long)]
    pub monochrome_output: bool,

    #[clap(short = 'S', long)]
    pub sort_keys: bool,

    #[clap(short, long)]
    pub indent: Option<usize>,

    #[clap(short = 'c', long)]
    pub compact_output: bool,

    pub filter: String,

    pub file: PathBuf,
}
