mod args;
mod filter;
mod print_json;
mod tests;

use anyhow::{bail, Result};
use args::Args;
use clap::Parser;
use serde_json::Value;
use std::{fs::File, io::BufReader};

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.file)?;
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader)?;

    // Check for conflicting arguments
    if args.indent.is_some() && args.compact_output {
        bail!("Cannot use --indent and --compact-output together")
    }
    if args.color_output && args.monochrome_output {
        bail!("Cannot use --color-output and --monochrome-output together")
    }

    // Set the indent value and compact_output flags
    let mut compact_output = args.compact_output;
    let indent = args.indent.map_or(Ok(2), |i| {
        if i == 0 {
            compact_output = true;
        }
        if i > 7 {
            bail!("Indent value cannot be greater than 7")
        } else {
            Ok(i)
        }
    })?;

    // Filter the JSON
    let values = &filter::filter(&json, &args.filter);

    // Print the JSON
    if args.monochrome_output {
        if compact_output {
            print_json::print_compact_json(values);
            println!();
        } else {
            print_json::print_json(values, indent, indent);
        }
    } else if compact_output {
        print_json::print_compact_json_color(values);
        println!();
    } else {
        print_json::print_json_color(values, indent, indent);
    }

    Ok(())
}
