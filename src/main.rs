use clap::Parser;
use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(author, version, about = "Fast and concurrent log analyzer")]
struct Args {
    /// The pattern to search for
    #[arg(short = 'n', long)]
    pattern: String,


    /// The path to the file to read
    #[arg(short = 'p', long)]
    path: PathBuf,
}


fn main() -> io::Result<()> {
    let args = Args::parse();


    let re = Regex::new(&args.pattern).expect("Invalid regex pattern");


    let file = File::open(&args.path)?;
    let reader = io::BufReader::new(file);


    // Read all lines into a vector so Rayon can parallelize them
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;


    // Search concurrently across all CPU cores using Rayon
    lines.par_iter().enumerate().for_each(|(index, line)| {
        if re.is_match(line) {
            println!("{}: {}", index + 1, line);
        }
    });


    Ok(())
}
