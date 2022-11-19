use crate::common::config::{Config, DbConnectionConfig};
use crate::common::SQL;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::{DBConn, TsQuery};
use postgres::{Client, NoTls};
use std::cell::RefCell;
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

pub fn prepare<'a>(sql: &SQL, config: &Config, handler: &Handler) -> (bool, TsQuery) {
    let connection = &config.get_correct_connection(&sql.query);

    let mut failed = false;

    let span = sql.span.to_owned();
    // todo: update it to use prepare stmt
    let explain_query = format!("EXPLAIN {}", sql.query);

    let postgres_cred = &get_postgres_cred(&connection).clone();
    let mut conn = Client::connect(postgres_cred, NoTls).unwrap();
    let result = conn.query(explain_query.as_str(), &[]);

    if let Err(e) = result {
        handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
        failed = true;
    }

    let transformation_config = &config.transformation_config;
    let ts_query = generate_ts_interface(
        &sql,
        &connection,
        &DBConn::PostgresConn(&mut RefCell::new(&mut conn)),
        &transformation_config,
    )
    .unwrap();

    (failed, ts_query)
}
