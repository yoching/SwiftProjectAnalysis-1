use clap::Parser;
use glob::*;
use std::fs::File;
use std::io::{self, BufRead};

/// Simple project analyzation
#[derive(Parser, Debug)]
struct Args {
    /// Path of root directory to analyze
    #[clap(short, long, value_parser, default_value = "./")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let root_path = args.path.to_string_lossy().to_string();
    let search_pattern = root_path.clone() + "**/*.swift";

    let entries: Vec<_> = glob(&search_pattern)
        .expect("Error when searching files")
        .filter_map(Result::ok)
        .collect();
    let count = entries.len();

    println!("{} files found", count);

    for entry in entries {
        let file = File::open(entry.clone()).unwrap();
        let lines = io::BufReader::new(file).lines();
        let relative_path = entry.strip_prefix(&root_path).unwrap();
        println!("- {}, {}", relative_path.display(), lines.count());
    }
}
