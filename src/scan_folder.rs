use std::path::{Path, PathBuf};

use crate::common::lazy::CONFIG;
use crate::common::types::JsExtension;
use regex::{Regex, Error as RegexError};
use walkdir::WalkDir;

fn pattern_to_regex(pattern: &str) -> Result<Regex, RegexError> {
  let pattern = pattern.replace('.', "\\.");
  let pattern = pattern.replace('*', ".*");
  let pattern = format!("^{}$", pattern);
  Regex::new(&pattern)
}

fn is_match(pattern: &str, path: &Path) -> bool {
  let regex = pattern_to_regex(pattern);

  if regex.is_err() {
      let invalid_pattern = format!("Invalid ignore path pattern found in the ignore file - pattern: ${:?}, path: ${:?}", pattern, path);
      panic!("{}", invalid_pattern);
    }

    let regex = regex.unwrap();

    if pattern.starts_with('!') {
        !regex.is_match(path.to_str().unwrap())
    } else {
        regex.is_match(path.to_str().unwrap())
    }
}

pub fn scan_folder<'a>(folder: &'a PathBuf, js_extension: &'a JsExtension) -> Vec<PathBuf> {
  let ignore_paths = &CONFIG.ignore_patterns;
  let node_modules_path = folder.join(Path::new("node_modules"));
  let path = Path::new(folder);
  let result: Vec<_> = WalkDir::new(path)
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|entry| {
      // 1. ignore node modules
      if entry.path().starts_with(node_modules_path.as_path()) {
        return false;
      }

      // 2. any custom ignore paths set by user should be ignored
      let should_ignore = ignore_paths
        .iter()
        .any(|ignore| is_match(ignore.as_str(), entry.path()));
      if should_ignore {
        return false;
      }

      let f_name = entry.file_name().to_string_lossy();

      f_name.ends_with(js_extension.to_string().as_str())
    })
    .map(|entry| entry.path().to_owned())
    .collect();

  result
}
