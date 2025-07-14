#![deny(
  clippy::correctness,
  clippy::style,
  clippy::perf,
  clippy::unnecessary_unwrap,
  clippy::bool_comparison,
  clippy::useless_asref,
  clippy::borrow_deref_ref,
  clippy::clone_on_copy,
  clippy::extra_unused_lifetimes,
  clippy::explicit_auto_deref,
  clippy::print_stdout,
  clippy::print_stderr,
  clippy::println_empty_string
)]
#![allow(clippy::ptr_arg)]
mod common;
mod core;
mod parser;
mod scan_folder;
mod ts_generator;

extern crate clap;
extern crate dotenv;

use crate::core::execute::execute;

use crate::common::lazy::*;
use crate::common::logger::*;
use crate::common::types::FileExtension;
use crate::ts_generator::generator::clear_single_ts_file_if_exists;
use crate::{parser::parse_source, scan_folder::scan_folder};
use color_eyre::eyre::Result;
use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

unsafe fn set_default_env_var() {
  if env::var("SQLX_TS_LOG").is_err() {
    env::set_var("SQLX_TS_LOG", "info");
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  std::env::set_var("RUST_BACKTRACE", "0");
  LazyLock::force(&CLI_ARGS);
  LazyLock::force(&CONFIG);
  LazyLock::force(&DB_SCHEMA);
  LazyLock::force(&ERR_DB_CONNECTION_ISSUE);
  LazyLock::force(&DB_CONN_CACHE);
  LazyLock::force(&DB_CONNECTIONS);

  std::panic::set_hook(Box::new(|info| {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
      error!("{}\n", s);
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
      error!("{}\n", s);
    } else {
      error!("unknown error\n");
    }
  }));

  unsafe {
    set_default_env_var();
  }

  let source_folder = &CLI_ARGS.path;
  // If no file extensions were provided
  let exts = if CLI_ARGS.ext.is_empty() {
    vec![FileExtension::Ts, FileExtension::Sql]
  } else {
    CLI_ARGS.ext.clone()
  };

  let exts_str = exts
    .iter()
    .map(|ext| ext.to_string())
    .collect::<Vec<String>>()
    .join(",");

  info!("Scanning {:?} for SQLs with extensions [{}]", source_folder, exts_str);

  // If CLI_ARGS.generate_types is true, it will clear the single TS file so `execute` will generate a new one from scratch
  clear_single_ts_file_if_exists()?;

  let files: Vec<PathBuf> = exts
    .iter()
    .map(|ext| scan_folder(source_folder, ext))
    .into_iter()
    .flatten()
    .collect();

  if files.is_empty() {
    info!(
      "No targets detected, is it an empty folder? - source_folder: {:?}, file extensions: [{}]",
      source_folder, exts_str,
    );
    std::process::exit(0);
  }

  let mut num_sqls = 0;
  for file_path in files.iter() {
    let (sqls, handler) = parse_source(file_path)?;
    let failed = execute(&sqls, &handler).await?;

    for sql in sqls {
      num_sqls += sql.1.iter().len();
    }

    #[allow(clippy::print_stderr)]
    if failed {
      error!("SQLs failed to compile!\n");
      std::process::exit(1)
    }
  }

  if num_sqls == 0 {
    info!("No SQL queries found");
  } else {
    let num_sqls_msg = format!("Found {num_sqls} SQL queries");
    info!(num_sqls_msg);
  }

  info!("No SQL errors detected!\n");
  // NOTE: There are different exit code depending on the platform https://doc.rust-lang.org/std/process/fn.exit.html#platform-specific-behavior
  // Make sure to consider exit code all major platforms
  std::process::exit(0);
}
