use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::common::config::DbConnectionConfig;
use crate::common::string_cases::ConvertCase;
use crate::common::SQL;
use crate::ts_generator::sql_parser::handle_sql_statement;
use crate::ts_generator::types::TsQuery;
use regex::Regex;
use sqlparser::{dialect::GenericDialect, parser::Parser};

use super::errors::TsGeneratorError;
use super::types::DBConn;

pub fn get_query_name(sql: &SQL) -> Result<String, TsGeneratorError> {
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

pub fn get_query_ts_file_path(file_path: &PathBuf) -> Result<PathBuf, TsGeneratorError> {
    let path = file_path.parent().unwrap();
    let file = file_path.file_name().unwrap();
    let file_name = file.to_str().unwrap().split(".").next().unwrap();

    let result = path.join(Path::new(format!("{file_name}.queries.ts").as_str()));
    Ok(result)
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

    let db_name = db_connection_config
        .db_name
        .clone()
        .expect("DB_NAME is required to generate Typescript type definitions");

    for sql_statement in &sql_ast {
        handle_sql_statement(&mut ts_query, &sql_statement, db_name.as_str(), &db_conn)?;
    }

    // generate path/file_name.queries.ts
    let query_ts_file_path = get_query_ts_file_path(&sql.file_path).unwrap();
    // write ts_query to the query_ts_file_path
    let mut file_to_write = File::create(query_ts_file_path).unwrap();
    file_to_write
        .write_all(ts_query.to_string().as_ref())
        .unwrap();
    Ok(())
}
