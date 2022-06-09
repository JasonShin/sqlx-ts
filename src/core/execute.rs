use sqlx_ts_common::config::{Config, DbConnectionConfig};
use sqlx_ts_common::SQL;
use sqlx_ts_common::{cli::Cli, types::DatabaseType};
use swc_common::errors::Handler;

use crate::mysql::explain as mysql_explain;
use crate::postgres::explain as postgres_explain;

pub fn execute(queries: &Vec<SQL>, handler: &Handler, cli_args: &Cli) -> bool {
    // TODO: later we will add mysql_explain, sqlite_explain depending on the database type
    let mut failed = false;

    for sql in queries {
        let config = Config::new(cli_args.to_owned());
        let connection = &config.get_correct_connection(&sql.query);

        failed = match connection.db_type {
            DatabaseType::Postgres => postgres_explain::explain(&sql, &connection, &handler),
            DatabaseType::Mysql => mysql_explain::explain(&sql, &connection, &handler),
        }
    }

    failed
}
