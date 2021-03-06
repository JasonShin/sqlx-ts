mod common;
mod core;
mod parser;
mod scan_folder;

extern crate clap;
extern crate dotenv;

use crate::core::execute::execute;
use clap::{ArgEnum, Args, Parser, Subcommand};
use dotenv::dotenv;

use crate::common::cli::Cli;
use crate::{parser::parse_source, scan_folder::scan_folder};

fn main() {
    dotenv().ok();

    let cli_args = Cli::parse();
    let source_folder = &cli_args.path;
    let ext = &cli_args.ext;
    let ignore_paths = &cli_args.ignore;

    println!(
        "Scanning {:?} for sqls with extension {:?}",
        source_folder, ext
    );

    let files = scan_folder(&source_folder, ext, ignore_paths);

    if files.is_empty() {
        println!("No targets detected, is it an empty folder?");
        return;
    }

    let explain_results: Vec<bool> = files
        .into_iter()
        .map(|file_path| {
            let (sqls, handler) = parse_source(&file_path);
            execute(&sqls, &handler, &cli_args)
        })
        .collect();

    let failed_to_compile = explain_results.iter().any(|x| x == &true);

    if failed_to_compile == false {
        println!("No SQL errors detected!");
        // NOTE: There are different exit code depending on the platform https://doc.rust-lang.org/std/process/fn.exit.html#platform-specific-behavior
        // Make sure to consider exit code all major platforms
        std::process::exit(0)
    } else {
        println!("SQLs failed to compile!");
        std::process::exit(1)
    }
}
