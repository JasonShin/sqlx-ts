use sqlparser::ast::{Ident, Query, SetExpr, Values};

use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::{
    errors::TsGeneratorError,
    types::{DBConn, TsQuery},
};

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
    println!("source {:#?}", source);
    let values = &source.body;

    // c1, c2, c3, c4, c5
    // [$1, $2, $4,  ?,  ?]
    // [ ?,  1,  2,  ?,  ?]
    // ->
    //
    for col in columns {
        match values {
            SetExpr::Values(values) => {}
            _ => unimplemented!("This SetExpr of INSERT statement is not yet implemented"),
        }
    }

    // println!("translating columns and source {:#?} - {:#?} - {}", columns, source, db_name);
    Ok(())
}
