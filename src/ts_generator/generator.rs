use crate::common::string_cases::ConvertCase;
use crate::common::{config::Config, SQL};
use crate::ts_generator::types::{TsDataType, TsQuery};
use regex::Regex;
use sqlparser::ast::TableWithJoins;
use sqlparser::ast::{
    SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr, Wildcard},
    SetExpr, Statement,
};
use sqlparser::{dialect::GenericDialect, parser::Parser};
use std::collections::HashMap;

use super::errors::TsGeneratorError;
use super::information_schema::{MySQLSchema, DBSchema};
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

fn get_table_name_for_field(tables: Vec<&TableWithJoins>) {

}

pub fn generate_ts_interface(sql: &SQL, config: &Config, db_conn: &DBConn) -> Result<(), TsGeneratorError> {
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

    let db_schema = match db_conn {
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
                        println!("table with joins {:?}", table_with_joins);
                        // then fetch information schema to figure out each field's details
                        for select_item in projection {
                            match select_item {
                                UnnamedExpr(unnamed_expr) => {
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

    Ok(())
}
