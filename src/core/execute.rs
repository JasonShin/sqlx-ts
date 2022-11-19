use crate::common::cli::Cli;
use crate::common::config::Config;
use crate::common::types::DatabaseType;
use crate::common::SQL;
use crate::core::mysql::prepare as mysql_explain;
use crate::core::postgres::prepare as postgres_explain;
use crate::ts_generator::generator::get_query_ts_file_path;
use std::collections::HashMap;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use swc_common::errors::Handler;

pub fn execute(queries: &HashMap<PathBuf, Vec<SQL>>, handler: &Handler, cli_args: &Cli) -> bool {
    // TODO: later we will add mysql_explain, sqlite_explain depending on the database type
    let mut failed = false;
    let config = Config::new(cli_args.to_owned());
    let transformation_config = &config.transformation_config;

    for (file_path, sqls) in queries {
        let mut sqls_to_write: Vec<String> = vec![];
        for sql in sqls {
            let connection = &config.get_correct_connection(&sql.query);

            println!("checking connection {:?}", connection);
            let (explain_failed, ts_query) = match connection.db_type {
                DatabaseType::Postgres => postgres_explain::prepare(&sql, &config, &handler),
                DatabaseType::Mysql => mysql_explain::prepare(&sql, &config, &handler),
            };

            failed = explain_failed;

            if transformation_config.is_some() {
                sqls_to_write.push(ts_query.to_string());
            }
        }

        if transformation_config.is_some() {
            // Finally writes query typing files
            let query_ts_file_path = get_query_ts_file_path(&file_path).unwrap();
            if query_ts_file_path.exists() {
                remove_file(&query_ts_file_path).unwrap();
            }
            let mut file_to_write = File::create(query_ts_file_path).unwrap();
            let mut sqls_to_write = sqls_to_write.join("\n");

            file_to_write.write_all(sqls_to_write.as_ref()).unwrap();
        }
    }

    failed
}
