use std::{fs, path::PathBuf};

use clap::StructOpt;
use dqk_parser::{Parser, Stmt, StringAllocator};

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to source
    source: PathBuf,
}

fn main() {
    let args = Args::parse();

    let source = fs::read_to_string(&args.source).unwrap();

    let mut string_allocator = StringAllocator::new();

    let mut parser = Parser::new(
        &source,
        string_allocator.get_path("path"),
        &mut string_allocator,
    );

    println!("{:#?}", parser.parse::<Stmt>());
}
