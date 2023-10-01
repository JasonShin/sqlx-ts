use super::connection::DBConn;
use crate::common::lazy::{CLI_ARGS, CONFIG, DB_CONNECTIONS};
use crate::common::types::DatabaseType;
use crate::common::SQL;
use crate::core::mysql::prepare as mysql_explain;
use crate::core::postgres::prepare as postgres_explain;
use crate::ts_generator::generator::{write_colocated_ts_file, write_single_ts_file};

use color_eyre::eyre::Result;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::Arc;

use std::path::PathBuf;
use swc_common::errors::Handler;

pub fn execute(queries: &HashMap<PathBuf, Vec<SQL>>, handler: &Handler) -> Result<bool> {
    let mut failed = false;
    let should_generate_types = &CONFIG
        .generate_types_config
        .to_owned()
        .filter(|x| x.enabled)
        .is_some();

    for (file_path, sqls) in queries {
        let mut sqls_to_write: Vec<String> = vec![];
        for sql in sqls {
            let mut connection = DB_CONNECTIONS.lock().unwrap();
            let connection = &connection.get_connection(&sql.query);
            let connection = Arc::clone(&connection);
            let connection = &connection.lock().unwrap();
            
            let (explain_failed, ts_query) = &connection.prepare(&sql, &should_generate_types, &handler)?;
            // If any prepare statement fails, we should set the failed flag as true
            // failed = explain_failed;

            if *should_generate_types {
                let ts_query = &ts_query.clone().expect("Failed to generate types from query");
                let ts_query = &ts_query.to_string();
                sqls_to_write.push(ts_query.to_owned());
            }
        }

        if *should_generate_types {
            let is_sqls_empty = sqls_to_write.len() == 0;
            let sqls_to_write = sqls_to_write.join("\n");

            if is_sqls_empty {
                continue;
            }

            if CLI_ARGS.generate_path.is_none() {
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
