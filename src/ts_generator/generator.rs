use crate::common::config::DbConnectionConfig;
use crate::common::string_cases::ConvertCase;
use crate::common::{config::Config, SQL};
use crate::ts_generator::types::{TsDataType, TsQuery};
use regex::Regex;
use sqlparser::ast::{ObjectName, TableWithJoins};
use sqlparser::ast::{
    SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr, Wildcard},
    SetExpr, Statement,
};
use sqlparser::{dialect::GenericDialect, parser::Parser};
use std::borrow::BorrowMut;
use std::collections::HashMap;

use super::errors::TsGeneratorError;
use super::information_schema::{DBSchema, MySQLSchema};
use super::types::DBConn;

fn get_query_name(sql: &SQL) -> Result<String, TsGeneratorError> {
    let re = Regex::new(r"@name:(.+)").unwrap();
    let var_decl_name = &sql.var_decl_name;
    let captures = re.captures(&sql.query.as_str());

    if let Some(captures) = captures {
        let query_name = captures
            .get(0)
            .unwrap()
            .as_str()
            .split(":")
            .last()
            .unwrap()
            .to_string();

        if query_name.is_empty() {
            return Err(TsGeneratorError::EmptyQueryNameFromAnnotation(
                sql.query.to_string(),
            ));
        }
        return Ok(query_name.to_pascal_case());
    }

    let var_decl_name = var_decl_name.clone();

    if let Some(var_decl_name) = var_decl_name {
        return Ok(var_decl_name.to_pascal_case());
    }

    Err(TsGeneratorError::EmptyQueryNameFromVarDecl)
}

fn get_table_name(table_with_join: &TableWithJoins) -> Option<String> {
    match &table_with_join.relation {
        sqlparser::ast::TableFactor::Table {
            name,
            alias,
            args,
            with_hints,
        } => match name {
            ObjectName(val) => {
                let alias = alias
                    .clone()
                    .and_then(|alias| Some(alias.clone().name.to_string()));
                let name = val.get(0).and_then(|val| Some(val.value.to_string()));

                if alias.is_some() {
                    return alias;
                } else if name.is_some() {
                    return name;
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}

pub fn generate_ts_interface(
    sql: &SQL,
    db_connection_config: &DbConnectionConfig,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let sql_ast = Parser::parse_sql(&dialect, &sql.query).unwrap();
    let mut ts_query = TsQuery {
        name: get_query_name(&sql)?,
        params: HashMap::new(),
        result: HashMap::new(),
    };
    let query_name = get_query_name(&sql);

    let mut result: HashMap<String, TsDataType> = HashMap::new();
    let mut params: HashMap<String, TsDataType> = HashMap::new();

    let db_name = db_connection_config
        .db_name
        .clone()
        .expect("DB_NAME is required to generate Typescript type definitions");
    let db_schema = match *db_conn.clone() {
        DBConn::MySQLPooledConn(_) => MySQLSchema::new(),
    };

    for sql in &sql_ast {
        match sql {
            Statement::Query(query) => {
                let body = &query.body;
                match body {
                    SetExpr::Select(select) => {
                        let projection = select.clone().projection;
                        let table_with_joins = select.clone().from;
                        // then fetch information schema to figure out each field's details
                        for select_item in projection {
                            match select_item {
                                UnnamedExpr(unnamed_expr) => {
                                    let default_table = table_with_joins.get(0)
                                    .expect(format!("Default FROM table is not found from the query {query}").as_str());
                                    let table_name = get_table_name(default_table)
                                    .expect(format!("Default FROM table is not found from the query {query}").as_str());

                                    match &db_conn {
                                        DBConn::MySQLPooledConn(conn) => {
                                            // TODO: update the method to use Result
                                            // TODO: We can also memoize this method
                                            let result = &db_schema.fetch_table(
                                                &db_name,
                                                &table_name,
                                                &conn,
                                            );
                                            println!("unnamed expr {:?}", unnamed_expr);
                                            println!("result {:?}", result);
                                        }
                                    }
                                }
                                ExprWithAlias { expr, alias } => {
                                    // println!("ExprWithAlias {:#?}", expr);
                                    // println!("ExprWithAlias {:?}", alias);
                                    match expr {
                                        sqlparser::ast::Expr::Exists(_) => {
                                            result.insert(alias.value, TsDataType::Boolean);
                                        }
                                        sqlparser::ast::Expr::Identifier(_) => todo!(),
                                        sqlparser::ast::Expr::CompoundIdentifier(_) => todo!(),
                                        sqlparser::ast::Expr::JsonAccess {
                                            left,
                                            operator,
                                            right,
                                        } => todo!(),
                                        sqlparser::ast::Expr::CompositeAccess { expr, key } => {
                                            todo!()
                                        }
                                        sqlparser::ast::Expr::IsFalse(_) => todo!(),
                                        sqlparser::ast::Expr::IsTrue(_) => todo!(),
                                        sqlparser::ast::Expr::IsNull(_) => todo!(),
                                        sqlparser::ast::Expr::IsNotNull(_) => todo!(),
                                        sqlparser::ast::Expr::IsDistinctFrom(_, _) => todo!(),
                                        sqlparser::ast::Expr::IsNotDistinctFrom(_, _) => todo!(),
                                        sqlparser::ast::Expr::InList {
                                            expr,
                                            list,
                                            negated,
                                        } => todo!(),
                                        sqlparser::ast::Expr::InSubquery {
                                            expr,
                                            subquery,
                                            negated,
                                        } => todo!(),
                                        sqlparser::ast::Expr::InUnnest {
                                            expr,
                                            array_expr,
                                            negated,
                                        } => todo!(),
                                        sqlparser::ast::Expr::Between {
                                            expr,
                                            negated,
                                            low,
                                            high,
                                        } => todo!(),
                                        sqlparser::ast::Expr::BinaryOp { left, op, right } => {
                                            todo!()
                                        }
                                        sqlparser::ast::Expr::AnyOp(_) => todo!(),
                                        sqlparser::ast::Expr::AllOp(_) => todo!(),
                                        sqlparser::ast::Expr::UnaryOp { op, expr } => todo!(),
                                        sqlparser::ast::Expr::Cast { expr, data_type } => todo!(),
                                        sqlparser::ast::Expr::TryCast { expr, data_type } => {
                                            todo!()
                                        }
                                        sqlparser::ast::Expr::Extract { field, expr } => todo!(),
                                        sqlparser::ast::Expr::Position { expr, r#in } => todo!(),
                                        sqlparser::ast::Expr::Substring {
                                            expr,
                                            substring_from,
                                            substring_for,
                                        } => todo!(),
                                        sqlparser::ast::Expr::Trim { expr, trim_where } => todo!(),
                                        sqlparser::ast::Expr::Collate { expr, collation } => {
                                            todo!()
                                        }
                                        sqlparser::ast::Expr::Nested(_) => todo!(),
                                        sqlparser::ast::Expr::Value(_) => todo!(),
                                        sqlparser::ast::Expr::TypedString { data_type, value } => {
                                            todo!()
                                        }
                                        sqlparser::ast::Expr::MapAccess { column, keys } => todo!(),
                                        sqlparser::ast::Expr::Function(_) => todo!(),
                                        sqlparser::ast::Expr::Case {
                                            operand,
                                            conditions,
                                            results,
                                            else_result,
                                        } => todo!(),
                                        sqlparser::ast::Expr::Subquery(_) => todo!(),
                                        sqlparser::ast::Expr::ListAgg(_) => todo!(),
                                        sqlparser::ast::Expr::GroupingSets(_) => todo!(),
                                        sqlparser::ast::Expr::Cube(_) => todo!(),
                                        sqlparser::ast::Expr::Rollup(_) => todo!(),
                                        sqlparser::ast::Expr::Tuple(_) => todo!(),
                                        sqlparser::ast::Expr::ArrayIndex { obj, indexes } => {
                                            todo!()
                                        }
                                        sqlparser::ast::Expr::Array(_) => todo!(),
                                    }
                                }
                                QualifiedWildcard(obj_name) => {
                                    println!("obj name {}", obj_name);
                                }
                                Wildcard => {
                                    println!("wild card!!!");
                                }
                            }
                        }
                    }
                    _ => println!("hmm"),
                }
            }
            _ => println!("not sure"),
        }
    }

    println!("Checking results {:?}", result);

    Ok(())
}
