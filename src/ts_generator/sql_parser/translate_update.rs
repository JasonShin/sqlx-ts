use sqlparser::ast::{Assignment, Expr, TableWithJoins};

use crate::ts_generator::{
    errors::TsGeneratorError,
    types::{db_conn::DBConn, ts_query::TsQuery},
};

use super::expressions::{
    translate_expr::{translate_assignment, translate_expr},
    translate_table_with_joins::{translate_table_from_assignments, get_default_table},
};

fn translate_assignments(
    ts_query: &mut TsQuery,
    table_with_joins: &TableWithJoins,
    assignments: &Vec<Assignment>,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    for assignment in assignments {
        let table = translate_table_from_assignments(&vec![table_with_joins.to_owned()], assignment).expect(
            "Failed to find the table based on assignment {assignment} from table with joins {table_with_joins}",
        );
        translate_assignment(assignment, table.as_str(), ts_query, db_conn).unwrap();
    }
    Ok(())
}

pub fn translate_update(
    ts_query: &mut TsQuery,
    table_with_joins: &TableWithJoins,
    assignments: &Vec<Assignment>,
    from: &Option<TableWithJoins>,
    selection: &Option<Expr>,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    translate_assignments(ts_query, table_with_joins, assignments, db_conn)?;

    if selection.is_some() {
        // let mut binding = from.clone().map(|x| vec![x]);
        // let from = binding;
        let table_with_joins = vec![table_with_joins.clone()];
        let current_scope_table = get_default_table(&table_with_joins);
        let current_scope_table = current_scope_table.as_str();
        translate_expr(
            &selection.to_owned().unwrap(),
            &Some(current_scope_table),
            &Some(table_with_joins),
            None,
            ts_query,
            db_conn,
            false,
        )?;
    }
    Ok(())
}
