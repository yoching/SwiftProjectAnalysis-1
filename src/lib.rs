use clap::Parser;
use glob::*;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use tabled::{object::Columns, Alignment, Modify, Style, Table, Tabled};

use serde::Serialize;

use console::{style, Emoji};
use indicatif::ProgressIterator;

mod web;

/// Simple project analyzation
#[derive(Parser, Debug)]
pub struct Args {
    /// Path of root directory to analyze
    #[clap(short, long, value_parser, default_value = "./")]
    pub path: std::path::PathBuf,

    /// Without analysis. Only run server.
    #[clap(long, action)]
    pub without_analysis: bool,

    /// output to console
    #[clap(long, action)]
    pub console_output: bool,
}

#[derive(Tabled, Serialize)]
struct FileAnalysis {
    name: String,
    length: usize,
    struct_count: usize,
    class_count: usize,
    enum_count: usize,
}

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static MICROSCOPE: Emoji<'_, '_> = Emoji("🔬  ", "");
static SERVER: Emoji<'_, '_> = Emoji("💻  ", "");

pub fn run(args: Args) -> io::Result<()> {
    if args.without_analysis {
        println!(
            "{} {}Launching web server... Open http://127.0.0.1:8080/",
            style("[1/1]").bold().dim(),
            SERVER
        );
        return web::start_http_server();
    }

    let root_path = args.path.to_string_lossy().to_string();
    let root_path = if root_path.chars().last().unwrap() == '/' {
        root_path
    } else {
        root_path + "/"
    };

    println!(
        "{} {}Searching Swift files in {}",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS,
        root_path
    );

    let swift_file_paths = search_swift_files(&root_path);

    // number of files
    println!("      {} files found", swift_file_paths.len());

    println!(
        "{} {}Analyzing files...",
        style("[2/4]").bold().dim(),
        MICROSCOPE
    );

    // File stats
    let mut file_stats: Vec<FileAnalysis> = swift_file_paths
        .iter()
        .progress()
        .map(|path| make_file_stats(path, &root_path))
        .collect();

    serde_json::to_writer_pretty(&fs::File::create("web/file_stats.json")?, &file_stats)?;

    if args.console_output {
        file_stats.sort_by(|a, b| b.length.cmp(&a.length));

        let file_stats_table = Table::new(&file_stats)
            .with(Style::psql())
            .with(Modify::new(Columns::first()).with(Alignment::left()))
            .with(Modify::new(Columns::last()).with(Alignment::right()))
            .to_string();

        println!("{}", file_stats_table);
    }

    println!(
        "{} {}Launching web server... Open http://127.0.0.1:8080/",
        style("[3/4]").bold().dim(),
        SERVER
    );

    // _ = open::that("http://127.0.0.1:8080/");
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

    let name = file_path
        .strip_prefix(root_path)
        .unwrap()
        .to_string_lossy()
        .to_string();

    let ast = get_ast(file_path.to_str().expect("File path is not valid"))
        .map(String::from_utf8)
        .expect("invalid ast")
        .expect("invalid ast");

    let struct_count = ast.matches("struct_decl").count();
    let class_count = ast.matches("class_decl").count();
    let enum_count = ast.matches("enum_decl").count();

    FileAnalysis {
        name: name,
        length: contents.lines().count(),
        struct_count: struct_count,
        class_count: class_count,
        enum_count: enum_count,
    }
}

pub fn get_ast(file: &str) -> io::Result<Vec<u8>> {
    Command::new("swiftc")
        .arg("-dump-parse")
        .arg(file)
        .output()
        .map(|i| i.stdout)
}
