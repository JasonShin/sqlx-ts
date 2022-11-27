use sqlparser::ast::{Expr, TableWithJoins};

use crate::ts_generator::sql_parser::translate_table_with_joins::translate_table_with_joins;
use crate::ts_generator::types::{TsFieldType, TsQuery};

pub fn translate_where_stmt(ts_query: &mut TsQuery, expr: &Expr, table_with_joins: &Vec<TableWithJoins>) {
    match expr {
        Expr::BinaryOp { left, op, right } => {
            // Loop right expression until there is nothing left
            let right = *right.clone();
            match right {
                Expr::BinaryOp { left, op, right } => match *left.clone() {
                    Expr::Identifier(ident) => {
                        let field_name = ident.to_string();
                        ts_query.params.insert(field_name, vec![TsFieldType::Any]);
                    }
                    Expr::CompoundIdentifier(identifiers) => {
                        let table_name = identifiers[0].to_string();
                        let field_name = identifiers[1].to_string();
                        ts_query.params.insert(field_name, vec![TsFieldType::Any]);
                    }
                    _ => {}
                },
                _ => {}
            }

            // Finally if left is just an identifier, translate it as well
            match *left.clone() {
                Expr::Identifier(ident) => {
                    let field_name = ident.to_string();
                    ts_query.params.insert(field_name, vec![TsFieldType::Any]);
                }
                _ => {}
            }
            translate_where_stmt(ts_query, &*left.clone(), &table_with_joins);
        }
        _ => {
            println!("Skipping");
        }
    }
}