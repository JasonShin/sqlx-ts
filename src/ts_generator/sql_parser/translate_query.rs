use sqlparser::ast::{Query, SelectItem, SetExpr};

use crate::ts_generator::{errors::TsGeneratorError, types::db_conn::DBConn, types::ts_query::TsQuery};

use super::expressions::{
    translate_expr::translate_expr, translate_table_with_joins::translate_table_with_joins,
    translate_where_stmt::translate_where_stmt, translate_wildcard_expr::translate_wildcard_expr,
};

/// translates query
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
                        translate_expr(unnamed_expr, &table_name, alias, ts_query, db_conn, is_subquery)?;
                    }
                    SelectItem::ExprWithAlias { expr, alias } => {
                        let alias = alias.to_string();
                        let table_name = translate_table_with_joins(&table_with_joins, &select_item);

                        translate_expr(
                            expr,
                            table_name.expect("Unknown table name").as_str(),
                            Some(alias.as_str()),
                            ts_query,
                            db_conn,
                            is_subquery,
                        )?;
                    }
                    SelectItem::QualifiedWildcard(a, b) => {
                        translate_wildcard_expr(query, ts_query, db_conn)?;
                    }
                    SelectItem::Wildcard(_) => {
                        translate_wildcard_expr(query, ts_query, db_conn)?;
                    }
                }
            }

            // If there's any WHERE statements, process it
            if let Some(selection) = select.selection {
                translate_where_stmt(ts_query, &selection, &None, &Some(&table_with_joins), db_conn)?;
            }
            Ok(())
        }
        SetExpr::Query(_) => todo!(),
        SetExpr::SetOperation {
            op: _,
            left: _,
            right: _,
            set_quantifier: _,
        } => todo!(),
        SetExpr::Values(_) => todo!(),
        SetExpr::Insert(_) => todo!(),
        SetExpr::Update(_) => todo!(),
        SetExpr::Table(_) => todo!(),
    }
}
