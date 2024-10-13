use anyhow::Result;
use clap::Parser;
use colored::{Color, Colorize};
use regex::Regex;
use std::{fs::File, io::BufRead, path::PathBuf};

use std::io::{BufReader, Lines, Stdin};

trait InputReader
where
    for<'a> &'a Self: std::io::Read,
{
    fn read_lines(&self) -> Lines<BufReader<&Self>> {
        BufReader::new(self).lines()
    }

    fn write_matches(&self, matches: Vec<String>, color: Option<Color>, needle: String) {
        for line in matches {
            if let Some(color) = color {
                println!(
                    "{}",
                    line.replace(&needle, &needle.color(color).to_string())
                );
            } else {
                println!("{}", line);
            }
        }
    }

    fn search(
        &self,
        needle: &String,
        ignore_case: bool,
        invert_match: bool,
        regex: bool,
    ) -> Vec<String> {
        let mut matches = Vec::<String>::new();
        let mut invert_matches = Vec::<String>::new();
        let pattern = if regex {
            Some(Regex::new(&needle).expect("Invalid regex pattern"))
        } else {
            None
        };
        let lines = self.read_lines();
        for line in lines {
            match line {
                Ok(line) => {
                    let is_match = if let Some(ref pattern) = pattern {
                        pattern.is_match(&line)
                    } else if ignore_case {
                        line.to_lowercase().contains(&needle.to_lowercase())
                    } else {
                        line.contains(needle)
                    };

                    if is_match {
                        matches.push(line);
                    } else {
                        invert_matches.push(line);
                    }
                }
                Err(_) => {
                    eprintln!("Error reading line");
                }
            }
        }
        if invert_match {
            return invert_matches;
        }
        matches
    }
}

impl InputReader for File {}

impl InputReader for Stdin {}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ignore_case: bool,

    #[clap(short = 'v', long)]
    invert_match: bool,

    #[clap(short, long)]
    regex: bool,

    #[clap(short, long)]
    color: Option<Color>,

    needle: String,

    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{:?}", args);
    if args.file.is_none() {
        let stdin = std::io::stdin();
        let matches = stdin.search(
            &args.needle,
            args.ignore_case,
            args.invert_match,
            args.regex,
        );
        stdin.write_matches(matches, args.color, args.needle);
    } else {
        let file = File::open(args.file.unwrap())?;
        let matches = file.search(
            &args.needle,
            args.ignore_case,
            args.invert_match,
            args.regex,
        );
        file.write_matches(matches, args.color, args.needle);
    }

    Ok(())
}
