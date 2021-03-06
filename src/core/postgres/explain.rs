use crate::common::config::DbConnectionConfig;
use crate::common::SQL;
use postgres::{Client, NoTls};
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

    let postgres_cred = &get_postgres_cred(&connection).clone();
    let mut conn = Client::connect(postgres_cred, NoTls).unwrap();
    let result = conn.query(explain_query.as_str(), &[]);

    if let Err(e) = result {
        handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
        failed = true;
    }

    failed
}
