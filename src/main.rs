mod common;
mod core;
mod parser;
mod scan_folder;
mod ts_generator;

extern crate clap;
extern crate dotenv;

use crate::core::execute::execute;

use dotenv::dotenv;
use sqlx_ts::ts_generator::generator::clear_single_ts_file_if_exists;

use crate::common::lazy::CLI_ARGS;
use crate::{parser::parse_source, scan_folder::scan_folder};
use color_eyre::{eyre::eyre, eyre::Result};

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().ok();

    let source_folder = &CLI_ARGS.path;
    let ext = &CLI_ARGS.ext;

    println!("Scanning {:?} for SQLs with extension {:?}", source_folder, ext);

    let files = scan_folder(source_folder, ext);
    if files.is_empty() {
        println!("No targets detected, is it an empty folder? - source_folder: {:?}, ext: {:?}", source_folder, ext);
        std::process::exit(0);
    }

    // If CLI_ARGS.generate_types is true, it will clear the single TS file so `execute` will generate a new one from scratch
    clear_single_ts_file_if_exists()?;

    for file_path in files.iter() {
        let (sqls, handler) = parse_source(&file_path)?;
        let failed = execute(&sqls, &handler)?;
        if failed {
            eprintln!("SQLs failed to compile!");
            std::process::exit(1)
        }
    }

    println!("No SQL errors detected!");
    // NOTE: There are different exit code depending on the platform https://doc.rust-lang.org/std/process/fn.exit.html#platform-specific-behavior
    // Make sure to consider exit code all major platforms
    std::process::exit(0);
}
