use crate::common::config::DbConnectionConfig;
use crate::common::string_cases::ConvertCase;
use crate::common::{config::Config, SQL};
use crate::ts_generator::types::{TsFieldType, TsQuery};
use regex::Regex;
use sqlparser::ast::{ObjectName, TableWithJoins, Expr};
use sqlparser::ast::{
    SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr, Wildcard},
    SetExpr, Statement,
};
use sqlparser::{dialect::GenericDialect, parser::Parser};
use std::borrow::BorrowMut;
use std::collections::HashMap;

use super::errors::TsGeneratorError;
use super::information_schema::{MySQLSchema};
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

fn handle_sql_expr(
    expr: &Expr,
    db_name: &str,
    table_name: &str,
    result: &mut HashMap<String, TsFieldType>,
    db_conn: &DBConn,
) {
    let mysql_schema = MySQLSchema::new();

    match expr {
        Expr::Identifier(ident) => {
            let column_name = ident.value.to_string();

            match &db_conn {
                DBConn::MySQLPooledConn(conn) => {
                    // TODO: update the method to use Result
                    // TODO: We can also memoize this method
                    let table_details = &mysql_schema.fetch_table(
                        &db_name,
                        &table_name,
                        &conn,
                    );
                    if let Some(table_details) = table_details {
                        let field = table_details.get(&column_name).unwrap();
                        result.insert(column_name.clone(), field.field_type.clone());
                    }
                    println!("column_name {:?}", column_name);
                    println!("result {:?}", result);
                }
            }
        
        },
        _ => todo!(),
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

    let mut result: HashMap<String, TsFieldType> = HashMap::new();
    let mut params: HashMap<String, TsFieldType> = HashMap::new();

    let db_name = db_connection_config
        .db_name
        .clone()
        .expect("DB_NAME is required to generate Typescript type definitions");
    
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
                                    
                                    // Handles SQL Expression and appends result
                                    handle_sql_expr(&unnamed_expr, &db_name, &table_name, &mut result, &db_conn);
                                }
                                _ => todo!(),
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
