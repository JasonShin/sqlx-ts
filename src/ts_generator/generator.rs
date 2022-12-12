use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::common::config::{DbConnectionConfig, GenerateTypesConfig};
use crate::common::SQL;
use crate::ts_generator::annotations::extract_result_annotations;
use crate::ts_generator::sql_parser::translate_stmt::translate_stmt;
use crate::ts_generator::types::TsQuery;
use convert_case::{Case, Casing};
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
            return Err(TsGeneratorError::EmptyQueryNameFromAnnotation(sql.query.to_string()));
        }
        return Ok(query_name.to_case(Case::Pascal));
    }

    let var_decl_name = var_decl_name.clone();

    if let Some(var_decl_name) = var_decl_name {
        return Ok(var_decl_name.to_case(Case::Pascal));
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
    generate_types_config: &Option<GenerateTypesConfig>,
) -> Result<TsQuery, TsGeneratorError> {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let sql_ast = Parser::parse_sql(&dialect, &sql.query).unwrap();
    let mut ts_query = TsQuery {
        name: get_query_name(&sql)?,
        params: vec![],
        result: HashMap::new(),
    };

    let annotated_result_types = extract_result_annotations(&sql.query);

    let db_name = db_connection_config
        .db_name
        .clone()
        .expect("DB_NAME is required to generate Typescript type definitions");

    for sql_statement in &sql_ast {
        translate_stmt(
            &mut ts_query,
            &sql_statement,
            db_name.as_str(),
            &annotated_result_types,
            &db_conn,
            &generate_types_config,
        )?;
    }

    Ok(ts_query)
}
