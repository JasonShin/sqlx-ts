use crate::shared::JsExtension;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_folder<'a>(folder: &'a PathBuf, js_extension: JsExtension) -> Vec<PathBuf> {
    let path = Path::new(folder);
    let result: Vec<_> = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
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
