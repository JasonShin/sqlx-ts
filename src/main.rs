mod parser;
mod scan_folder;
mod shared;

use crate::parser::parse_source;
use crate::scan_folder::scan_folder;
use crate::shared::JsExtension;
use sqlx_ts_core::execute::execute;

fn main() {
    let source_folder = "./tests/postgres";

    let files = scan_folder(&source_folder, JsExtension::Ts);

    for file_path in files {
        let sqls = parse_source(&file_path);
        let sqls = sqls.iter().map(String::as_str).collect();
        println!("explain sqls {:?}", sqls);
        execute(&sqls)
    }
}
