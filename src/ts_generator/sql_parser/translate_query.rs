use sqlparser::ast::{Query, SelectItem, SetExpr, TableWithJoins};

use crate::ts_generator::{errors::TsGeneratorError, types::db_conn::DBConn, types::ts_query::TsQuery};

use super::expressions::{
    translate_expr::translate_expr, translate_table_with_joins::translate_table_with_joins,
    translate_wildcard_expr::translate_wildcard_expr,
};

/// Translates a query and workout ts_query's results and params
pub fn translate_query(
    ts_query: &mut TsQuery,
    // this parameter is used to stack table_with_joins while recursing through subqueries
    // If there is only 1 entry of table_with_joins, it means it's processing the top level entity
    table_with_joins: &Option<Vec<TableWithJoins>>,
    query: &Query,
    db_conn: &DBConn,
    alias: Option<&str>,
    is_selection: bool,
) -> Result<(), TsGeneratorError> {
    let body = *query.body.clone();
    match body {
        SetExpr::Select(select) => {
            let projection = select.clone().projection;
            
            // We create a new table with joins within the scope of this select (it could be within a subquery)
            // full_table_with_joins should contain all tables from the parent and keeping it within this query's scope
            let mut full_table_with_joins: Vec<TableWithJoins> = vec![];
            let child_table_with_joins = select.clone().from;
            
            // The most inner table should be extended first as it will be the default table within the subquery
            full_table_with_joins.extend(child_table_with_joins);

            if table_with_joins.is_some() {
                let table_with_joins = table_with_joins.as_ref().unwrap();
                full_table_with_joins.extend(table_with_joins.clone());
            }

            let full_table_with_joins = &Some(full_table_with_joins.clone());

            // Handle all select projects and figure out each field's type
            for select_item in projection {
                match &select_item {
                    SelectItem::UnnamedExpr(unnamed_expr) => {
                        let table_name = translate_table_with_joins(full_table_with_joins, &select_item)
                            .expect("Default FROM table is not found from the query {query}");

                        // Handles SQL Expression and appends result
                        translate_expr(
                            unnamed_expr,
                            &Some(&table_name.as_str()),
                            full_table_with_joins,
                            alias,
                            ts_query,
                            db_conn,
                            is_selection,
                        )?;
                    }
                    SelectItem::ExprWithAlias { expr, alias } => {
                        let alias = alias.to_string();
                        let table_name = translate_table_with_joins(full_table_with_joins, &select_item);
                        let table_name = table_name.expect("Unknown table name");

                        translate_expr(
                            expr,
                            &Some(table_name.as_str()),
                            full_table_with_joins,
                            Some(alias.as_str()),
                            ts_query,
                            db_conn,
                            is_selection,
                        )?;
                    }
                    SelectItem::QualifiedWildcard(_, _) => {
                        // TODO: If there's are two tables and two qualifieid wildcards are provided
                        // It will simply generate types for both tables' columns
                        // Should we namespace each field based on the table alias? e.g. table1_field1, table2_field1
                        if is_selection {
                            translate_wildcard_expr(query, ts_query, db_conn)?;
                        }
                    }
                    SelectItem::Wildcard(_) => {
                        if is_selection {
                            translate_wildcard_expr(query, ts_query, db_conn)?;
                        }
                    }
                }
            }

            // If there's any WHERE statements, process it
            if let Some(selection) = select.selection {
                translate_expr(
                    &selection,
                    &None,
                    full_table_with_joins,
                    None,
                    ts_query,
                    db_conn,
                    false,
                )?;
            }
            Ok(())
        }
        _ => Err(TsGeneratorError::Unknown(format!(
            "Unknown query type while processing query: {:#?}",
            query
        ))),
    }
}
