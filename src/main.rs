mod common;
mod core;
mod parser;
mod scan_folder;
mod ts_generator;

extern crate clap;
extern crate dotenv;

use crate::core::execute::execute;

use dotenv::dotenv;

use crate::common::lazy::CLI_ARGS;
use crate::{parser::parse_source, scan_folder::scan_folder};
use color_eyre::{eyre::eyre, eyre::Result};

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().ok();

    let source_folder = &CLI_ARGS.path;
    let ext = &CLI_ARGS.ext;
    let ignore_paths = &CLI_ARGS.ignore;

    println!("Scanning {:?} for SQLs with extension {:?}", source_folder, ext);

    let files = scan_folder(source_folder, ext, ignore_paths);

    if files.is_empty() {
        return Err(eyre!("No targets detected, is it an empty folder?"));
    }

    let explain_results: Vec<bool> = files
        .into_iter()
        .map(|file_path| {
            let (sqls, handler) = parse_source(&file_path);
            execute(&sqls, &handler).unwrap()
        })
        .collect();

    let failed_to_compile = explain_results.iter().any(|x| x == &true);

    if !failed_to_compile {
        println!("No SQL errors detected!");
        // NOTE: There are different exit code depending on the platform https://doc.rust-lang.org/std/process/fn.exit.html#platform-specific-behavior
        // Make sure to consider exit code all major platforms
        std::process::exit(0)
    } else {
        eprintln!("SQLs failed to compile!");
        std::process::exit(1)
    }
}
