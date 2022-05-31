use postgres::{Client, NoTls};
use sqlx_ts_common::cli::Cli;
use sqlx_ts_common::config::{Config, DbConnectionConfig};
use sqlx_ts_common::SQL;
use swc_common::errors::Handler;

fn get_postgres_cred(conn: &DbConnectionConfig) -> String {
    format!(
        "host={} user={} password={} port={:?}",
        &conn.db_host,
        &conn.db_user,
        &conn.db_pass.as_ref().unwrap_or(&"".to_string()),
        &conn.db_port,
    )
}

pub fn explain<'a>(sqls: &Vec<SQL>, handler: &Handler, cli_args: &Cli) -> bool {
    let config = Config::new(cli_args.to_owned());

    let mut failed = false;

    for sql in sqls {
        let span = sql.span.to_owned();
        let explain_query = format!("EXPLAIN {}", sql.query);
        let connection = &config.get_correct_connection(&sql.query);

        if let Some(connection) = connection {
            let mut conn = Client::connect(get_postgres_cred(&connection).as_str(), NoTls).unwrap();
            let result = conn.query(explain_query.as_str(), &[]);
        }

        if let Err(e) = result {
            handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
            failed = true;
        }
    }

    failed
}
