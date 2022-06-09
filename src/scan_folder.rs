use std::path::{Path, PathBuf};

use crate::common::types::JsExtension;
use walkdir::WalkDir;

pub fn scan_folder<'a>(
    folder: &'a PathBuf,
    js_extension: &'a JsExtension,
    ignore_paths: &'a Vec<PathBuf>,
) -> Vec<PathBuf> {
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
                .any(|ignore| entry.path().starts_with(ignore));
            if should_ignore {
                return false;
            }

            let f_name = entry.file_name().to_string_lossy();
            if f_name.ends_with(js_extension.to_string().as_str()) {
                true
            } else {
                false
            }
        })
        .map(|entry| entry.path().to_owned())
        .collect();

    result
}
