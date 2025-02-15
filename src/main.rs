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

use std::cell::LazyCell;
use crate::core::execute::execute;

use std::env;
use std::sync::LazyLock;
use crate::common::lazy::*;
use crate::common::logger::*;
use crate::ts_generator::generator::clear_single_ts_file_if_exists;
use crate::{parser::parse_source, scan_folder::scan_folder};
use color_eyre::eyre::Result;

fn set_default_env_var() {
  if env::var("SQLX_TS_LOG").is_err() {
    env::set_var("SQLX_TS_LOG", "info");
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  LazyLock::force(&CLI_ARGS);
  LazyLock::force(&CONFIG);
  LazyLock::force(&DB_SCHEMA);
  LazyLock::force(&ERR_DB_CONNECTION_ISSUE);
  LazyLock::force(&DB_CONN_CACHE);
  LazyLock::force(&DB_CONNECTIONS);

  println!("forced all lazy vars");
  std::panic::set_hook(Box::new(|info| {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
      error!("{}\n", s);
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
      error!("{}\n", s);
    } else {
      error!("unknown error\n");
    }
    std::process::exit(1)
  }));

  set_default_env_var();

  println!("set default env vars");
  let source_folder = &CLI_ARGS.path;
  let ext = &CLI_ARGS.ext;
  println!("source folder and ext {:?} - {:?}", source_folder, ext);

  info!("Scanning {:?} for SQLs with extension {}", source_folder, ext);

  // If CLI_ARGS.generate_types is true, it will clear the single TS file so `execute` will generate a new one from scratch
  clear_single_ts_file_if_exists()?;

  let files = scan_folder(source_folder, ext);

  println!("scanned folder");
  if files.is_empty() {
    info!(
      "No targets detected, is it an empty folder? - source_folder: {:?}, ext: {}",
      source_folder, ext
    );
    std::process::exit(0);
  }

  for file_path in files.iter() {
    println!("before scanning source {:?}", file_path);
    let (sqls, handler) = parse_source(file_path)?;
    println!("checking sqls parsed {:?}", file_path);
    let failed = execute(&sqls, &handler).await?;
    #[allow(clippy::print_stderr)]
    if failed {
      error!("SQLs failed to compile!\n");
      std::process::exit(1)
    }
  }

  info!("No SQL errors detected!\n");
  // NOTE: There are different exit code depending on the platform https://doc.rust-lang.org/std/process/fn.exit.html#platform-specific-behavior
  // Make sure to consider exit code all major platforms
  std::process::exit(0);
}
