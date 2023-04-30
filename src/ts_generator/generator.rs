use std::fs::OpenOptions;
use std::io::Write;
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::types::db_conn::DBConn;

use crate::common::lazy::{CLI_ARGS, CONFIG};
use crate::common::SQL;
use crate::ts_generator::annotations::extract_result_annotations;
use crate::ts_generator::sql_parser::translate_stmt::translate_stmt;
use crate::ts_generator::types::ts_query::TsQuery;

use color_eyre::eyre::eyre;
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
        fs::remove_file(&query_ts_file_path)?;
    }

    let mut file_to_write = fs::File::create(query_ts_file_path)?;

    file_to_write.write_all(sqls_to_write.as_ref())?;
    Ok(())
}

/// Write a single TS file to a target destination according to CLI_ARGS.generate_path
pub fn write_single_ts_file(sqls_to_write: String) -> Result<()> {
    let generate_path = CONFIG
        .generate_types_config
        .clone()
        .map(|x| x.generate_path)
        .flatten();
    let mut output = generate_path.ok_or(eyre!(
        "TS generation path (--generate-path=) is required if you want to generate the SQL at a single path"
    ))?;

    let parent_output_path: Option<&Path> = output.parent();
    if parent_output_path.is_some() {
        fs::create_dir_all(parent_output_path.unwrap())?;
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

/// clears the target single TS file if it exists
pub fn clear_single_ts_file_if_exists() -> Result<()> {
    let generate_types_config = CONFIG.generate_types_config.clone();
    if generate_types_config.is_none() {
        return Ok(());
    }

    let generate_path = generate_types_config.map(|x| x.generate_path).flatten();
    if generate_path.is_none() {
        return Ok(());
    }

    let mut target = generate_path.to_owned().unwrap();
    if target.is_dir() {
        target = target.join("types.queries.ts");
    }

    if target.exists() {
        fs::remove_file(target)?
    }
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
