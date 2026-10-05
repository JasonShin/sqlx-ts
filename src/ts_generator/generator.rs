use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, path::Path};

use super::annotations::extract_param_annotations;

use crate::common::lazy::CONFIG;
use crate::common::SQL;
use crate::core::connection::DBConn;
use crate::ts_generator::annotations::extract_result_annotations;
use crate::ts_generator::sql_parser::translate_stmt::translate_stmt;
use crate::ts_generator::types::ts_query::TsQuery;

use crate::common::types::DatabaseType;
use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use convert_case::{Case, Casing};
use regex::Regex;
use sqlparser::{
  dialect::{Dialect, MySqlDialect, PostgreSqlDialect, SQLiteDialect},
  parser::Parser,
};

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
      .next_back()
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

fn normalize_imports(imports: &[String]) -> Vec<String> {
  imports
    .iter()
    .map(|import| {
      let import = import.trim();
      if import.ends_with(';') {
        import.to_string()
      } else {
        format!("{import};")
      }
    })
    .collect()
}

fn with_imports(imports: &[String], sqls_to_write: &str) -> String {
  if imports.is_empty() {
    return sqls_to_write.to_string();
  }
  format!("{}\n\n{sqls_to_write}", imports.join("\n"))
}

/// Write colocated Type definition file next to the TS source code
pub fn write_colocated_ts_file(file_path: &Path, sqls_to_write: String, imports: &[String]) -> Result<()> {
  let path = file_path.parent().unwrap();
  let file = file_path.file_stem().unwrap();
  let file_name = file.to_str().unwrap();
  let query_ts_file_path = path.join(Path::new(format!("{file_name}.queries.ts").as_str()));

  if query_ts_file_path.exists() {
    fs::remove_file(&query_ts_file_path)?;
  }

  let mut file_to_write = fs::File::create(query_ts_file_path)?;

  let sqls_to_write = with_imports(&normalize_imports(imports), &sqls_to_write);
  file_to_write.write_all(sqls_to_write.as_ref())?;
  Ok(())
}

/// Write a single TS file to a target destination according to CLI_ARGS.generate_path
pub fn write_single_ts_file(sqls_to_write: String, imports: &[String]) -> Result<()> {
  let generate_path = CONFIG.generate_types_config.clone().and_then(|x| x.generate_path);
  let output = generate_path.ok_or(eyre!(
    "TS generation path (--generate-path=) is required if you want to generate the SQL at a single path"
  ))?;

  let parent_output_path: Option<&Path> = output.parent();
  if let Some(parent_output_path) = parent_output_path {
    fs::create_dir_all(parent_output_path)?;
  }

  let imports = normalize_imports(imports);
  if !imports.is_empty() {
    let existing = fs::read_to_string(&output).unwrap_or_default();
    let mut lines = existing.lines().peekable();
    let mut merged_imports: Vec<String> = vec![];
    while let Some(line) = lines.next_if(|line| line.starts_with("import ")) {
      merged_imports.push(line.to_string());
    }
    for import in imports {
      if !merged_imports.contains(&import) {
        merged_imports.push(import);
      }
    }
    let existing_types = lines.collect::<Vec<_>>().join("\n");
    let existing_types = existing_types.trim_start();
    let types = if existing_types.is_empty() {
      sqls_to_write
    } else {
      format!("{existing_types}\n{sqls_to_write}")
    };
    fs::write(&output, with_imports(&merged_imports, &types))?;
    return Ok(());
  }

  let mut file_to_write = OpenOptions::new()
    .create(true)
    .read(true)
    .append(true)
    .open(&output)
    .unwrap_or_else(|_| {
      panic!(
        "Failed to write to file {:?} - check if the --generate-path provided is an existing folder",
        output
      )
    });

  file_to_write.write_all(sqls_to_write.as_ref())?;
  Ok(())
}

/// clears the target single TS file if it exists
pub fn clear_single_ts_file_if_exists() -> Result<()> {
  let generate_types_config = CONFIG.generate_types_config.clone();
  if generate_types_config.is_none() {
    return Ok(());
  }

  let generate_path = generate_types_config.and_then(|x| x.generate_path);
  if generate_path.is_none() {
    return Ok(());
  }

  let mut target = generate_path.unwrap();
  if target.is_dir() {
    target = target.join("types.queries.ts");
  }

  if target.exists() {
    fs::remove_file(target)?
  }
  Ok(())
}

pub async fn generate_ts_interface(sql: &SQL, db_conn: &DBConn) -> Result<TsQuery> {
  // Use the appropriate SQL dialect based on the database type
  let dialect: Box<dyn Dialect> = match db_conn.get_db_type() {
    DatabaseType::Postgres => Box::new(PostgreSqlDialect {}),
    DatabaseType::Mysql => Box::new(MySqlDialect {}),
    DatabaseType::Sqlite => Box::new(SQLiteDialect {}),
  };

  let sql_ast = Parser::parse_sql(&*dialect, &sql.query)?;
  let mut ts_query = TsQuery::new(get_query_name(sql)?);

  let annotated_result_types = extract_result_annotations(sql.query.as_str());
  ts_query.set_annotated_results(annotated_result_types);

  let annotated_param_types = extract_param_annotations(sql.query.as_str());
  ts_query.set_annotated_params(annotated_param_types);

  for sql_statement in &sql_ast {
    // The loot level statements cannot have any alias
    translate_stmt(&mut ts_query, sql_statement, None, db_conn).await?;
  }

  Ok(ts_query)
}
