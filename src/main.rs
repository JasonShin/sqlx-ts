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
