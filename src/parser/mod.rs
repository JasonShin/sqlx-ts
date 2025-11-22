mod decl;
mod import;
mod js_parser;
mod sql_parser;
mod tag;

use crate::common::SQL;
use crate::parser::js_parser::parse_js_file;
use crate::parser::sql_parser::parse_sql_file;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use swc_common::errors::Handler;

pub fn parse_source(path: &PathBuf) -> Result<(HashMap<PathBuf, Vec<SQL>>, Handler)> {
  let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
  let result = match extension {
    "ts" | "js" | "mts" | "cts" | "mjs" | "cjs" => parse_js_file(path),
    "sql" => parse_sql_file(path),
    _ => {
      return Err(color_eyre::eyre::eyre!("Unsupported file extension: {}", extension));
    }
  };
  result
}
