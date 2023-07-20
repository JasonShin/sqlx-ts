use crate::common::lazy::{CLI_ARGS, CONFIG};
use crate::common::types::DatabaseType;
use crate::common::SQL;
use crate::core::mysql::prepare as mysql_explain;
use crate::core::postgres::prepare as postgres_explain;
use crate::ts_generator::generator::{write_colocated_ts_file, write_single_ts_file};

use color_eyre::eyre::Result;
use std::collections::HashMap;

use std::path::PathBuf;
use swc_common::errors::Handler;

pub fn execute(queries: &HashMap<PathBuf, Vec<SQL>>, handler: &Handler) -> Result<bool> {
    // TODO: later we will add mysql_explain, sqlite_explain depending on the database type
    let mut failed = false;
    let should_generate_types = &CONFIG
        .generate_types_config
        .to_owned()
        .filter(|x| x.enabled)
        .is_some();

    for (file_path, sqls) in queries {
        let mut sqls_to_write: Vec<String> = vec![];
        for sql in sqls {
            let connection = &CONFIG.get_correct_db_connection(&sql.query);

            let (explain_failed, ts_query) = match connection.db_type {
                DatabaseType::Postgres => postgres_explain::prepare(sql, should_generate_types, handler)?,
                DatabaseType::Mysql => mysql_explain::prepare(sql, should_generate_types, handler)?,
            };

            // If any prepare statement fails, we should set the failed flag as true
            failed = explain_failed;

            if *should_generate_types {
                let ts_query = ts_query.expect("Failed to generate types from query").to_string();
                sqls_to_write.push(ts_query);
            }
        }

        if *should_generate_types {
            let sqls_to_write = sqls_to_write.join("\n");
            if CLI_ARGS.generate_path.is_none() {
                println!(
                    "checking before writing the colocated file {:?} - {:?}",
                    file_path, sqls_to_write
                );
                // generates types colocated to source code
                write_colocated_ts_file(file_path, sqls_to_write)?;
            } else {
                // generates types in a single directory/file
                write_single_ts_file(sqls_to_write)?;
            }
        }
    }

    Ok(failed)
}
