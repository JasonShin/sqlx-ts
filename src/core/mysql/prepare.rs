use crate::common::lazy::THREAD_RUNTIME;
use crate::common::SQL;
use crate::core::connection::DBConn;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;
use sqlx::mysql;
use tokio::task::LocalSet;

use std::borrow::BorrowMut;
use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL.
/// Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to generate_types = true
pub async fn prepare(
    thread_local: &LocalSet,
    db_conn: &DBConn,
    sql: &SQL,
    should_generate_types: &bool,
    handler: &Handler,
) -> Result<(bool, Option<TsQuery>)> {
    let mut failed = false;

    let span = sql.span.to_owned();
    let explain_query = format!("PREPARE stmt FROM \"{}\"", sql.query);

    let conn = match &db_conn {
        DBConn::MySQLPooledConn(conn) => conn,
        _ => panic!("Invalid connection type"),
    };

    let result = thread_local.block_on(&THREAD_RUNTIME, sqlx::query(&explain_query).fetch_all(conn));

    if let Err(err) = result {
        handler.span_bug_no_panic(span, err.to_string().as_str());
        failed = true;
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        ts_query = Some(generate_ts_interface(sql, &thread_local, &db_conn).await?);
    }

    Ok((failed, ts_query))
}
