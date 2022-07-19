use crate::common::cli::Cli;
use crate::common::config::Config;
use crate::common::types::DatabaseType;
use crate::common::SQL;
use crate::core::mysql::explain as mysql_explain;
use crate::core::postgres::explain as postgres_explain;
use std::collections::HashMap;
use std::path::PathBuf;
use swc_common::errors::Handler;

pub fn execute(queries: &HashMap<PathBuf, Vec<SQL>>, handler: &Handler, cli_args: &Cli) -> bool {
    // TODO: later we will add mysql_explain, sqlite_explain depending on the database type
    let mut failed = false;

    for (_, sqls) in queries {
        for sql in sqls {
            let config = Config::new(cli_args.to_owned());
            let connection = &config.get_correct_connection(&sql.query);

            failed = match connection.db_type {
                DatabaseType::Postgres => postgres_explain::explain(&sql, &config, &handler),
                DatabaseType::Mysql => mysql_explain::explain(&sql, &config, &handler),
            }
        }
    }

    failed
}
