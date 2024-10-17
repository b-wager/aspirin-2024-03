use colored::{Color, Colorize};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Stdin},
};

use crate::args::Args;

pub(crate) trait InputReader
where
    for<'a> &'a Self: std::io::Read,
{
    fn write_matches(&self, pattern: &Regex, line: &String, color: &Option<Color>) {
        if let Some(color) = color {
            let colored_line = pattern.replace_all(line, |caps: &regex::Captures| {
                caps[0].color(*color).to_string()
            });
            println!("{}", colored_line);
        } else {
            println!("{}", line);
        }
    }

    fn search(&self, args: &Args) {
        let mut pattern = if args.regex {
            args.needle.clone()
        } else {
            regex::escape(&args.needle)
        };
        if args.ignore_case {
            pattern = format!("(?i){}", pattern);
        }
        let pattern = Regex::new(&pattern).expect("Invalid regex pattern");

        for line in BufReader::new(self).lines() {
            match line {
                Ok(line) => {
                    if pattern.is_match(&line) != args.invert_match {
                        self.write_matches(&pattern, &line, &args.color);
                    }
                }
                Err(_) => {
                    eprintln!("Error reading line");
                }
            }
        }
    }
}

impl InputReader for File {}

impl InputReader for Stdin {}
