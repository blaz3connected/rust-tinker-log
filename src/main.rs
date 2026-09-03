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
    let start_time = std::time::Instant::now();


    let re = Regex::new(&args.pattern).expect("Invalid regex pattern");


    let file = File::open(&args.path)?;
    let reader = io::BufReader::new(file);


    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;


    let matches_found = lines
        .par_iter()
        .enumerate()
        .filter(|(_, line)| re.is_match(line))
        .inspect(|(index, line)| {
            println!("{}: {}", index + 1, line);
    })
    .count();


    let duration = start_time.elapsed();
    eprintln!("\nFound {} matches in {:?}", matches_found, duration);


    Ok(())
}
