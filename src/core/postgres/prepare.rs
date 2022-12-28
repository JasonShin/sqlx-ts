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

pub fn prepare<'a>(
    sql: &SQL,
    config: &Config,
    should_generate_types: &bool,
    handler: &Handler,
) -> (bool, Option<TsQuery>) {
    let connection = &config.get_correct_connection(&sql.query);

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
    }

    let deallocate_result = conn.query("DEALLOCATE sqlx_stmt", &[]);
    match deallocate_result {
        Ok(_) => {}
        Err(_) => {}
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        let generate_types_config = &config.generate_types_config;
        ts_query = Some(
            generate_ts_interface(
                sql,
                connection,
                &DBConn::PostgresConn(&mut RefCell::new(&mut conn)),
                generate_types_config,
            )
            .unwrap(),
        );
    }

    (failed, ts_query)
}
