use crate::common::config::{Config, DbConnectionConfig};
use crate::common::lazy::CONFIG;
use crate::common::SQL;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::{DBConn, TsQuery};
use postgres::{Client, NoTls};
use std::cell::RefCell;

use swc_common::errors::Handler;

fn get_postgres_cred(conn: &DbConnectionConfig) -> String {
    format!(
        "postgresql://{user}:{pass}@{host}:{port}/{db_name}",
        user = &conn.db_user,
        pass = &conn.db_pass.as_ref().unwrap_or(&"".to_string()),
        host = &conn.db_host,
        port = &conn.db_port,
        // This is to follow the spec of Rust Postgres
        // `db_user` name gets used if `db_name` is not provided
        // https://docs.rs/postgres/latest/postgres/config/struct.Config.html#keys
        db_name = &conn.db_name.clone().unwrap_or((&conn.db_user).to_owned()),
    )
}

pub fn prepare<'a>(sql: &SQL, should_generate_types: &bool, handler: &Handler) -> (bool, Option<TsQuery>) {
    let connection = &CONFIG.get_correct_connection(&sql.query);

    let mut failed = false;

    let span = sql.span.to_owned();
    // todo: update it to use prepare stmt
    let prepare_query = format!("PREPARE sqlx_stmt AS {}", sql.query);

    let postgres_cred = &get_postgres_cred(connection);
    let mut conn = Client::connect(postgres_cred, NoTls).unwrap();
    let result = conn.query(prepare_query.as_str(), &[]);

    if let Err(e) = result {
        handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
        failed = true;
    } else {
        // We should only deallocate if the prepare statement was executed successfully
        conn.query("DEALLOCATE sqlx_stmt", &[]).unwrap();
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        ts_query =
            Some(generate_ts_interface(sql, connection, &DBConn::PostgresConn(&mut RefCell::new(&mut conn))).unwrap());
    }

    (failed, ts_query)
}
