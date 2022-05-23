use sqlx_ts_common::cli::{Cli, DatabaseType};
use sqlx_ts_common::SQL;
use swc_common::errors::Handler;

use crate::mysql::explain as mysql_explain;
use crate::postgres::explain as postgres_explain;

pub fn execute(queries: &Vec<SQL>, handler: &Handler, cli_args: &Cli) -> bool {
    // TODO: later we will add mysql_explain, sqlite_explain depending on the database type
    match cli_args.db_type {
        DatabaseType::Postgres => postgres_explain::explain(&queries, &handler, &cli_args),
        DatabaseType::Mysql => mysql_explain::explain(&queries, &handler, &cli_args),
    }
}
