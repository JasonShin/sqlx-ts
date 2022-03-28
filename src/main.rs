mod parser;

use sqlx_ts_core::execute::execute;
use crate::parser::parse_source;

fn main() {

    let path = "../tests/postgres.ts";
    let sqls = parse_source(&path);
    let sqls = sqls.iter().map(String::as_str).collect();
    execute(&sqls)
}
