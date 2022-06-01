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

pub fn explain<'a>(sql: &SQL, connection: &DbConnectionConfig, handler: &Handler) -> bool {
    let mut failed = false;

    let span = sql.span.to_owned();
    let explain_query = format!("EXPLAIN {}", sql.query);

    let mut conn = Client::connect(get_postgres_cred(&connection).as_str(), NoTls).unwrap();
    let result = conn.query(explain_query.as_str(), &[]);

    if let Err(e) = result {
        handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
        failed = true;
    }

    failed
}
