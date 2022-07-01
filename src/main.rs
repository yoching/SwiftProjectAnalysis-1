use clap::Parser;
use glob::*;
use std::fs::File;
use std::io::Read;
use tabled::{object::Columns, Alignment, Modify, Style, Table, TableIteratorExt, Tabled};

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

#[derive(Tabled)]
struct ElementsCount<'a> {
    element: &'a str,
    count: usize,
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

    let mut number_of_structs: usize = 0;
    let mut number_of_classes: usize = 0;
    let mut number_of_enums: usize = 0;

    for entry in entries {
        let mut file = File::open(entry.clone()).unwrap();
        let relative_path = entry.strip_prefix(&root_path).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut number_of_lines: usize = 0;

        for line in contents.lines() {
            if line.contains("struct ") {
                number_of_structs += 1;
            }
            if line.contains("class ") {
                number_of_classes += 1;
            }
            if line.contains("enum ") {
                number_of_enums += 1;
            }
            number_of_lines += 1;
        }

        let analysis = FileAnalysis {
            name: relative_path.to_string_lossy().to_string(),
            length: number_of_lines,
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

    let analytics: Vec<ElementsCount> = vec![
        ElementsCount {
            element: "struct",
            count: number_of_structs,
        },
        ElementsCount {
            element: "class",
            count: number_of_classes,
        },
        ElementsCount {
            element: "enum",
            count: number_of_enums,
        },
    ];
    let analytics_table = analytics
        .table()
        .with(Style::psql())
        .with(Modify::new(Columns::first()).with(Alignment::left()))
        .with(Modify::new(Columns::last()).with(Alignment::right()));
    println!("\n## Analytics");
    println!("{}", analytics_table.to_string());
}
