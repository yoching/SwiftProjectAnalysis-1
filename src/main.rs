use clap::Parser;
use glob::*;
use std::fs::File;
use std::io::Read;
use tabled::{object::Columns, Alignment, Modify, Style, Table, TableIteratorExt, Tabled};

use project_analysis::*;

// /// Simple project analyzation
// #[derive(Parser, Debug)]
// pub struct Args {
//     /// Path of root directory to analyze
//     #[clap(short, long, value_parser, default_value = "./")]
//     path: std::path::PathBuf,
// }

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

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // let root_path = args.path.to_string_lossy().to_string();
    // let search_pattern = root_path.clone() + "**/*.swift";

    // let entries: Vec<std::path::PathBuf> = glob(&search_pattern)
    //     .expect("Error when searching files")
    //     .filter_map(Result::ok)
    //     .collect();
    // let count = entries.len();

    // println!("## Summary");
    // println!("{} files found", count);

    // let mut files: Vec<FileAnalysis> = Vec::new();

    // let mut number_of_structs: usize = 0;
    // let mut number_of_classes: usize = 0;
    // let mut number_of_enums: usize = 0;
    // let mut number_of_functions: usize = 0;

    // for entry in entries {
    //     let mut file = File::open(entry.clone()).unwrap();
    //     let relative_path = entry.strip_prefix(&root_path).unwrap();

    //     let ast = project_analysis::get_ast(entry.to_str().expect("File path is not valid"))
    //         .map(String::from_utf8)
    //         .expect("invalid ast")
    //         .expect("invalid ast");
    //     // println!("{:?}", ast);

    //     let mut contents = String::new();
    //     file.read_to_string(&mut contents).unwrap();

    //     let number_of_lines: usize = contents.lines().count();

    //     for line in ast.lines() {
    //         if line.contains("struct_decl") {
    //             number_of_structs += 1;
    //         }
    //         if line.contains("class_decl") {
    //             number_of_classes += 1;
    //         }
    //         if line.contains("enum_decl") {
    //             number_of_enums += 1;
    //         }
    //         if line.contains("func_decl") {
    //             number_of_functions += 1;
    //         }
    //     }

    //     let analysis = FileAnalysis {
    //         name: relative_path.to_string_lossy().to_string(),
    //         length: number_of_lines,
    //     };
    //     files.push(analysis);
    // }

    // files.sort_by(|a, b| b.length.cmp(&a.length));

    // let table = Table::new(files)
    //     .with(Style::psql())
    //     .with(Modify::new(Columns::first()).with(Alignment::left()))
    //     .with(Modify::new(Columns::last()).with(Alignment::right()))
    //     .to_string();

    // println!("\n## Files");
    // println!("{}", table);

    // let analytics: Vec<ElementsCount> = vec![
    //     ElementsCount {
    //         element: "struct",
    //         count: number_of_structs,
    //     },
    //     ElementsCount {
    //         element: "class",
    //         count: number_of_classes,
    //     },
    //     ElementsCount {
    //         element: "enum",
    //         count: number_of_enums,
    //     },
    //     ElementsCount {
    //         element: "functions",
    //         count: number_of_functions,
    //     },
    // ];
    // let analytics_table = analytics
    //     .table()
    //     .with(Style::psql())
    //     .with(Modify::new(Columns::first()).with(Alignment::left()))
    //     .with(Modify::new(Columns::last()).with(Alignment::right()));
    // println!("\n## Analytics");
    // println!("{}", analytics_table.to_string());

    run(args)
}
