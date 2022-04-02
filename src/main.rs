mod config;
mod parser;
mod scan_folder;
mod shared;

extern crate clap;
extern crate dotenv;

use std::path::PathBuf;

use clap::{ArgEnum, Args, Parser, Subcommand};
use dotenv::dotenv;
use sqlx_ts_core::execute::execute;

use crate::{config::Config, parser::parse_source, scan_folder::scan_folder, shared::JsExtension};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Path to the Typescript or Javascript project
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

    /// Javascript Extension
    #[clap(
        arg_enum,
        long,
        default_value_t=JsExtension::Ts
    )]
    ext: JsExtension,
}

fn main() {
    let args = Cli::parse();
    let source_folder: PathBuf = args.path;
    let ext: JsExtension = args.ext;
    println!(
        "Scanning {:?} for sqls with extension {:?}",
        source_folder, ext
    );

    let files = scan_folder(&source_folder, ext);

    let explain_results: Vec<bool> = files
        .into_iter()
        .map(|file_path| {
            let (sqls, handler) = parse_source(&file_path);

            execute(&sqls, &handler)
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
