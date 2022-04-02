mod parser;
mod scan_folder;
mod shared;

extern crate clap;
extern crate core;
extern crate dotenv;

use std::path::PathBuf;

use clap::{ArgEnum, Args, Parser, Subcommand};
use dotenv::dotenv;
use sqlx_ts_common::cli::{DatabaseType, JsExtension};
use sqlx_ts_common::config::Config;
use sqlx_ts_core::execute::execute;

use crate::{parser::parse_source, scan_folder::scan_folder};

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

    /// Type of primary database to connect
    #[clap(
        arg_enum,
        long,
        default_value_t=DatabaseType::Postgres
    )]
    db_type: DatabaseType,

    /// Primary DB host
    #[clap(long)]
    db_host: Option<String>,

    /// Primary DB Port
    #[clap(long)]
    db_port: Option<i32>,

    /// Primary DB user
    #[clap(long)]
    db_user: Option<String>,

    /// Primary DB pass
    #[clap(long)]
    db_pass: Option<String>,
}

fn main() {
    dotenv().ok();

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
