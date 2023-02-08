use sqlparser::ast::{Ident, Query};

use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::{errors::TsGeneratorError, types::{TsQuery, DBConn}};

pub fn translate_insert(
    ts_query: &mut TsQuery,
    columns: &Vec<Ident>,
    source: &Box<Query>,
    db_name: &str,
    table_name: &str,
    conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    let fetched_table = DB_SCHEMA
        .fetch_table(db_name, &vec![table_name], conn)
        // Nearly impossible to panic at this point as we've already validated queries with prepare statements
        .unwrap();

    println!("fetched table {:?}", fetched_table);

    // println!("translating columns and source {:#?} - {:#?} - {}", columns, source, db_name);
    Ok(())
}
