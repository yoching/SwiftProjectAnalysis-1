use clap::Parser;
use glob::*;
use std::fs::File;
use std::io::{self, BufRead};
use tabled::{object::Columns, Alignment, Modify, Style, Table, Tabled};

/// Simple project analyzation
#[derive(Parser, Debug)]
struct Args {
    /// Path of root directory to analyze
    #[clap(short, long, value_parser, default_value = "./")]
    path: std::path::PathBuf,
}

#[derive(Tabled)]
struct FileAnalysis {
    name: String,
    length: usize,
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

    println!("## Summary");
    println!("{} files found", count);

    let mut files: Vec<FileAnalysis> = Vec::new();

    for entry in entries {
        let file = File::open(entry.clone()).unwrap();
        let lines = io::BufReader::new(file).lines();
        let relative_path = entry.strip_prefix(&root_path).unwrap();
        let analysis = FileAnalysis {
            name: relative_path.to_string_lossy().to_string(),
            length: lines.count(),
        };
        files.push(analysis);
    }

    files.sort_by(|a, b| b.length.cmp(&a.length));

    let table = Table::new(files)
        .with(Style::psql())
        .with(Modify::new(Columns::first()).with(Alignment::left()))
        .with(Modify::new(Columns::last()).with(Alignment::right()))
        .to_string();

    println!("\n## Files");
    println!("{}", table);
}
