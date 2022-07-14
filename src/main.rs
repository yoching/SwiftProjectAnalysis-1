use clap::Parser;

use project_analysis::*;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    run(args)
}
