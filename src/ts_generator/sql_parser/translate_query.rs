use sqlparser::ast::{Query, SelectItem, SetExpr};

use crate::ts_generator::{errors::TsGeneratorError, types::db_conn::DBConn, types::ts_query::TsQuery};

use super::expressions::{
    translate_expr::translate_expr,
    translate_table_with_joins::translate_table_with_joins,
    translate_wildcard_expr::translate_wildcard_expr,
};

pub fn translate_query(
    ts_query: &mut TsQuery,
    query: &Query,
    db_conn: &DBConn,
    alias: Option<&str>,
    is_subquery: bool,
) -> Result<(), TsGeneratorError> {
    let body = *query.body.clone();
    match body {
        SetExpr::Select(select) => {
            let projection = select.clone().projection;
            let table_with_joins = select.clone().from;
            // Handle all select projects and figure out each field's type
            for select_item in projection {
                match &select_item {
                    SelectItem::UnnamedExpr(unnamed_expr) => {
                        let table_name = translate_table_with_joins(&table_with_joins, &select_item)
                            .expect("Default FROM table is not found from the query {query}");

                        // Handles SQL Expression and appends result
                        translate_expr(unnamed_expr, &Some(&table_name.as_str()), &Some(&table_with_joins), alias, ts_query, db_conn, is_subquery)?;
                    }
                    SelectItem::ExprWithAlias { expr, alias } => {
                        let alias = alias.to_string();
                        let table_name = translate_table_with_joins(&table_with_joins, &select_item);
                        let table_name = table_name.expect("Unknown table name");

                        translate_expr(
                            expr,
                            &Some(table_name.as_str()),
                            &Some(&table_with_joins),
                            Some(alias.as_str()),
                            ts_query,
                            db_conn,
                            is_subquery,
                        )?;
                    }
                    SelectItem::QualifiedWildcard(_, _) => {
                        // TODO: If there's are two tables and two qualifieid wildcards are provided
                        // It will simply generate types for both tables' columns
                        // Should we namespace each field based on the table alias? e.g. table1_field1, table2_field1
                        if !is_subquery {
                            translate_wildcard_expr(query, ts_query, db_conn)?;
                        }
                    }
                    SelectItem::Wildcard(_) => {
                        if !is_subquery {
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
                    &Some(&table_with_joins),
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
