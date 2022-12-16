use sqlparser::ast::{Expr, TableWithJoins};

use crate::ts_generator::types::{TsFieldType, TsQuery};
/*
pub fn translate_single_expr(left: &Box<Expr>, right: &Box<Expr>) -> Some<TsFieldType> {
    match *left.clone() {
        Expr::Identifier(field_name) => {
            Some(TsFieldType::Any)
        },
        Expr::CompoundIdentifier(_) => todo!(),
        () => {
            return None;
        }
    }
}
*/

pub fn translate_where_stmt(ts_query: &mut TsQuery, expr: &Expr, table_with_joins: &Vec<TableWithJoins>) {
    // todos
    // if exp is identifier + a placeholder, we should record it
    // otherwise keep looping
    match expr {
        Expr::BinaryOp { left, op, right } => {
            // let mut table_alias = None;
            println!("checking left {left} {right}");
            println!("checking left {}", left);
            match *left.clone() {
                Expr::CompoundIdentifier(identifiers) => {
                    let left = identifiers.get(0);
                    println!("checking left {:?}", left);
                },
                _ => unimplemented!()
            }

            // Loop right expression until there is nothing left
            let right = *right.clone();
            println!("checking left {:?} right {:?}", left, right);
            match right {
                Expr::BinaryOp { left, op, right } => match *left.clone() {
                    Expr::Identifier(ident) => {
                        let field_name = ident.to_string();
                        println!("checking identifier {:?}", field_name);
                        ts_query.params.push(TsFieldType::Any);
                    }
                    Expr::CompoundIdentifier(identifiers) => {
                        let table_name = identifiers[0].to_string();
                        let field_name = identifiers[1].to_string();
                        println!("checking compound identifier {:?} , {:?}", table_name, field_name);
                        ts_query.params.push(TsFieldType::Any);
                    }
                    _ => unimplemented!()
                },
                Expr::Value(v) => {
                    match v {
                        sqlparser::ast::Value::Placeholder(placeholder) => {
                            println!("checking placeholder {placeholder}");
                        },
                        _ => unimplemented!()
                    }
                },
                _ => unimplemented!()
            }

            // Finally if left is just an identifier, translate it as well
            match *left.clone() {
                Expr::Identifier(ident) => {
                    let field_name = ident.to_string();
                    ts_query.params.push(TsFieldType::Any);
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
