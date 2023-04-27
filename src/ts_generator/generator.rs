use std::fs::OpenOptions;
use std::io::Write;
use std::{
    fs::{remove_file, File},
    path::{Path, PathBuf},
};

use super::types::db_conn::DBConn;

use crate::common::lazy::CLI_ARGS;
use crate::common::SQL;
use crate::ts_generator::annotations::extract_result_annotations;
use crate::ts_generator::sql_parser::translate_stmt::translate_stmt;
use crate::ts_generator::types::ts_query::TsQuery;

use color_eyre::eyre::Result;
use convert_case::{Case, Casing};
use regex::Regex;
use sqlparser::{dialect::GenericDialect, parser::Parser};

use super::errors::TsGeneratorError;

pub fn get_query_name(sql: &SQL) -> Result<String> {
    let re = Regex::new(r"@name:(.+)").unwrap();
    let var_decl_name = &sql.var_decl_name;
    let captures = re.captures(sql.query.as_str());

    if let Some(captures) = captures {
        let query_name = captures
            .get(0)
            .unwrap()
            .as_str()
            .split(':')
            .last()
            .unwrap()
            .to_string();

        if query_name.is_empty() {
            return Err(TsGeneratorError::EmptyQueryNameFromAnnotation(sql.query.to_string()).into());
        }
        return Ok(query_name.to_case(Case::Pascal));
    }

    let var_decl_name = var_decl_name.clone();

    if let Some(var_decl_name) = var_decl_name {
        return Ok(var_decl_name.to_case(Case::Pascal));
    }

    Err(TsGeneratorError::EmptyQueryNameFromVarDecl(sql.query.to_string()).into())
}

/// Write colocated Type definition file next to the TS source code
pub fn write_colocated_ts_file(file_path: &PathBuf, sqls_to_write: String) -> Result<()> {
    let path = file_path.parent().unwrap();
    let file = file_path.file_name().unwrap();
    let file_name = file.to_str().unwrap().split('.').next().unwrap();

    let query_ts_file_path = path.join(Path::new(format!("{file_name}.queries.ts").as_str()));

    if query_ts_file_path.exists() {
        remove_file(&query_ts_file_path)?;
    }

    let mut file_to_write = File::create(query_ts_file_path)?;

    file_to_write.write_all(sqls_to_write.as_ref())?;
    Ok(())
}

pub fn write_single_ts_file(sqls_to_write: String) -> Result<()> {
    let mut output = CLI_ARGS.generate_path.to_owned().unwrap();
    if output.is_dir() {
        output = output.join("types.queries.ts");
    }

    let mut file_to_write = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open(&output)?;

    file_to_write.write_all(sqls_to_write.as_ref())?;
    Ok(())
}

pub fn generate_ts_interface<'a>(sql: &SQL, db_conn: &DBConn) -> Result<TsQuery> {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let sql_ast = Parser::parse_sql(&dialect, &sql.query).unwrap();
    let mut ts_query = TsQuery::new(get_query_name(sql)?);

    let annotated_result_types = extract_result_annotations(&sql.query);
    ts_query.set_annotated_results(annotated_result_types);

    for sql_statement in &sql_ast {
        translate_stmt(&mut ts_query, sql_statement, db_conn)?;
    }

    Ok(ts_query)
}
