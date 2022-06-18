use crate::common::string_cases::ConvertCase;
use crate::common::{config::Config, SQL};
use crate::ts_generator::types::TsDataType;
use core::fmt;
use mysql::prelude::Queryable;
use regex::Regex;
use sqlparser::ast::{
    SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr, Wildcard},
    SetExpr, Statement,
};
use sqlparser::{dialect::GenericDialect, parser::Parser};
use std::collections::HashMap;

#[derive(Debug)]
pub enum GetQueryNameError {
    EmptyQueryNameFromAnnotation(String),
    EmptyQueryNameFromVarDecl,
}

impl fmt::Display for GetQueryNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetQueryNameError::EmptyQueryNameFromAnnotation(query) => writeln!(
                f,
                "Failed to fetch query name from DB name annotation - query: {}",
                query
            ),
            GetQueryNameError::EmptyQueryNameFromVarDecl => todo!(),
        }
    }
}

fn get_query_name(sql: &SQL) -> Result<String, GetQueryNameError> {
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
            return Err(GetQueryNameError::EmptyQueryNameFromAnnotation(
                sql.query.to_string(),
            ));
        }
        return Ok(query_name.to_pascal_case());
    }

    let var_decl_name = var_decl_name.clone();

    if let Some(var_decl_name) = var_decl_name {
        return Ok(var_decl_name.to_pascal_case());
    }

    Err(GetQueryNameError::EmptyQueryNameFromVarDecl)
}

pub fn generate_ts_interface(sql: &SQL, config: &Config) {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let sql_ast = Parser::parse_sql(&dialect, &sql.query).unwrap();
    let query_name = get_query_name(&sql);

    let mut result: HashMap<String, TsDataType> = HashMap::new();
    let mut params: HashMap<String, TsDataType> = HashMap::new();

    for sql in &sql_ast {
        match sql {
            Statement::Query(query) => {
                let body = &query.body;
                match body {
                    SetExpr::Select(select) => {
                        let projection = select.clone().projection;
                        let table_with_joins = select.clone().from;
                        for select_item in projection {
                            match select_item {
                                UnnamedExpr(unnamed_expr) => {
                                    println!("query name? {:?}", query_name);
                                    println!("unmapped expr {:?}", unnamed_expr);
                                }
                                ExprWithAlias { expr, alias } => todo!(),
                                QualifiedWildcard(_) => todo!(),
                                Wildcard => todo!(),
                            }
                        }
                    }
                    _ => println!("hmm"),
                }
            }
            _ => println!("not sure"),
        }
    }
}
