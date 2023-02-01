use crate::common::cli::Cli;
use crate::common::config::Config;
use crate::common::types::DatabaseType;
use crate::common::SQL;
use crate::common::lazy::{CONFIG, CLI_ARGS};
use crate::core::mysql::prepare as mysql_explain;
use crate::core::postgres::prepare as postgres_explain;
use crate::ts_generator::generator::get_query_ts_file_path;
use std::collections::HashMap;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use swc_common::errors::Handler;

pub fn execute(queries: &HashMap<PathBuf, Vec<SQL>>, handler: &Handler) -> bool {
    // TODO: later we will add mysql_explain, sqlite_explain depending on the database type
    let mut failed = false;
    let generate_types_config = &CONFIG.generate_types_config;
    let should_generate_types =
        CLI_ARGS.generate_types || generate_types_config.to_owned().filter(|x| x.enabled).is_some();

    for (file_path, sqls) in queries {
        let mut sqls_to_write: Vec<String> = vec![];
        for sql in sqls {
            let connection = &CONFIG.get_correct_connection(&sql.query);

            let (explain_failed, ts_query) = match connection.db_type {
                DatabaseType::Postgres => postgres_explain::prepare(sql, &CONFIG, &should_generate_types, handler),
                DatabaseType::Mysql => mysql_explain::prepare(sql, &CONFIG, &should_generate_types, handler),
            };

            failed = explain_failed;

            if should_generate_types {
                let ts_query = ts_query.expect("Failed to generate types from query").to_string();
                sqls_to_write.push(ts_query);
            }
        }

        if should_generate_types {
            // Finally writes query typing files
            let query_ts_file_path = get_query_ts_file_path(file_path).unwrap();
            if query_ts_file_path.exists() {
                remove_file(&query_ts_file_path).unwrap();
            }
            let mut file_to_write = File::create(query_ts_file_path).unwrap();
            let sqls_to_write = sqls_to_write.join("\n");

            file_to_write.write_all(sqls_to_write.as_ref()).unwrap();
        }
    }

    failed
}
