use crate::common::SQL;
use crate::core::connection::DBConn;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;
use mysql_async::{prelude::*, Row};

use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL.
/// Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to generate_types = true
pub async fn prepare(
  db_conn: &DBConn,
  sql: &SQL,
  should_generate_types: &bool,
  handler: &Handler,
) -> Result<(bool, Option<TsQuery>)> {
  let mut failed = false;

  let conn = match &db_conn {
    DBConn::MySQLPooledConn(conn) => conn,
    _ => panic!("Invalid connection type"),
  };

  {
    let explain_query = format!("PREPARE stmt FROM \"{}\"", sql.query);
    let span = sql.span.to_owned();
    let conn = conn.lock().await;
    let mut conn = conn.get().await
      .expect("Failed to retrieve a connection from the pool. Consider increasing the connection pool size");
    let result = conn.query::<Row, String>(explain_query).await;

    if let Err(err) = result {
      handler.span_bug_no_panic(span, err.to_string().as_str());
      failed = true;
    }
  }

  let mut ts_query = None;

  if should_generate_types == &true {
    ts_query = Some(generate_ts_interface(sql, db_conn).await?);
  }

  Ok((failed, ts_query))
}
