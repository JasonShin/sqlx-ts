use crate::common::cli::Cli;
use crate::common::config::Config;
use crate::common::types::DatabaseType;
use crate::common::SQL;
use crate::core::mysql::explain as mysql_explain;
use crate::core::postgres::explain as postgres_explain;
use swc_common::errors::Handler;

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
