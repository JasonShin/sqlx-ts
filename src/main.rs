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

use std::env;

use crate::common::lazy::CLI_ARGS;
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
  std::panic::set_hook(Box::new(|info| {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
      error!("encountered an error: {}\n", s);
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
      error!("encountered an error: {}\n", s);
    } else {
      error!("encountered an error: unknown error\n");
    }
  }));

  set_default_env_var();

  let source_folder = &CLI_ARGS.path;
  let ext = &CLI_ARGS.ext;

  info!("Scanning {:?} for SQLs with extension {}\n", source_folder, ext);

  // If CLI_ARGS.generate_types is true, it will clear the single TS file so `execute` will generate a new one from scratch
  clear_single_ts_file_if_exists()?;

  let files = scan_folder(source_folder, ext);
  if files.is_empty() {
    info!(
      "No targets detected, is it an empty folder? - source_folder: {:?}, ext: {}\n",
      source_folder, ext
    );
    std::process::exit(0);
  }

  for file_path in files.iter() {
    let (sqls, handler) = parse_source(file_path)?;
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
