use sqlparser::ast::{Ident, Query, SetExpr};

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
    let table_details = DB_SCHEMA
        .fetch_table(db_name, &vec![table_name], conn)
        // Nearly impossible to panic at this point as we've already validated queries with prepare statements
        .unwrap();

    let values = &source.body;

    // c1, c2, c3, c4, c5
    // [$1, $2, $4,  ?,  ?]
    // [ ?,  1,  2,  ?,  ?]
    // ->

    match values {
        SetExpr::Values(values) => {
            // Process the list of values list
            for (row, values) in values.0.iter().enumerate() {
                // Given a list of values
                // [ [?, 1, ?], [?, ?, ?] ]
                // Loop each value placeholder / actual values, if it finds the placeholder either `?` or `$n`
                // build the insert param in `types.rs`
                for (column, value) in values.iter().enumerate() {
                    if value.to_string() == "?" {
                        let match_col = columns
                            .get(column)
                            .expect(&format!(
                                "Matching column of idx {column} is not found while processing insert params"
                            ))
                            .to_string();

                        let field = table_details.get(match_col.as_str()).expect(&format!(
                            "Column {match_col} is not found while processing insert params"
                        ));

                        &ts_query.insert_value_params(&field.field_type, &(row, column), &Some(value.to_string()));
                    }
                    // TODO: work on postgres insert params handling logic
                }
            }
        }
        _ => unimplemented!(),
    }

    Ok(())
}
