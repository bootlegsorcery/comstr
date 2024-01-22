use std::fs;
use std::io::{self, BufReader, BufRead};

use clap::Parser;
use common_substrings::get_substrings;

/// CLI Arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file to process. Default is to read from stdin. 
    filename: Option<String>,

    // How many times the substring should show up before it appears in the output.
    #[arg(short = 'c', long, default_value_t = 2)]
    min_count: usize,

    // Minimum length of the substring.
    #[arg(short = 'l', long, default_value_t = 3)]
    min_length: usize,
}


fn main() {
    let args = Args::parse();

    let reader: Box<dyn BufRead> = match args.filename {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };

    let strings: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let strs: Vec<&str> = strings.iter().map(|s| &**s).collect();

    for substr in get_substrings(strs, args.min_count, args.min_length) {
        println!("{}", substr.name)
    }
}