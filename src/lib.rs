use clap::Parser;
use glob::*;
// use std::io::Result;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

use tabled::{object::Columns, Alignment, Modify, Style, Table, TableIteratorExt, Tabled};

use serde::Serialize;

mod web;

/// Simple project analyzation
#[derive(Parser, Debug)]
pub struct Args {
    /// Path of root directory to analyze
    #[clap(short, long, value_parser, default_value = "./")]
    pub path: std::path::PathBuf,
}

#[derive(Tabled, Serialize)]
struct FileAnalysis {
    name: String,
    length: usize,
}

pub fn get_ast(file: &str) -> io::Result<Vec<u8>> {
    Command::new("swiftc")
        .arg("-dump-parse")
        .arg(file)
        .output()
        .map(|i| i.stdout)
}

pub fn run(args: Args) -> io::Result<()> {
    let root_path = args.path.to_string_lossy().to_string();

    let swift_file_paths = search_swift_files(&root_path);

    // number of files
    println!("## Summary");
    println!("{} files found", swift_file_paths.len());

    // File stats
    let mut file_stats: Vec<FileAnalysis> = swift_file_paths
        .iter()
        .map(|path| make_file_stats(path, &root_path))
        .collect();

    serde_json::to_writer_pretty(&fs::File::create("web/file_stats.json")?, &file_stats)?;

    file_stats.sort_by(|a, b| b.length.cmp(&a.length));

    let file_stats_table = Table::new(&file_stats)
        .with(Style::psql())
        .with(Modify::new(Columns::first()).with(Alignment::left()))
        .with(Modify::new(Columns::last()).with(Alignment::right()))
        .to_string();

    println!("\n## File stats");
    println!("{}", file_stats_table);

    // Analysis
    for entry in &swift_file_paths {}

    web::start_http_server()
}

fn search_swift_files(root_path: &str) -> Vec<PathBuf> {
    let search_pattern = (*root_path).to_string() + "**/*.swift";
    search_entries(&search_pattern)
}

fn search_entries(search_pattern: &str) -> Vec<PathBuf> {
    glob(search_pattern)
        .expect("Error when searching files")
        .filter_map(Result::ok)
        .collect()
}

fn make_file_stats(file_path: &PathBuf, root_path: &str) -> FileAnalysis {
    let mut contents = String::new();
    fs::File::open(file_path.clone())
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    FileAnalysis {
        name: file_path
            .strip_prefix(root_path)
            .unwrap()
            .to_string_lossy()
            .to_string(),
        length: contents.lines().count(),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn strip_one_line_comment() {
//         let swift = "\
//         // comment
//         let a = 1
//         ";

//         assert_eq!(
//             "\
//         let a = 1
//         ",
//             strip(swift)
//         );
//     }
// }
