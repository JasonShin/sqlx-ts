use crate::common::SQL;
use crate::core::connection::DBConn;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;

use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL.
/// Validates the query is right by directly connecting to the configured SQLite database.
/// It also processes ts interfaces if the configuration is set to generate_types = true
pub async fn prepare(
  db_conn: &DBConn,
  sql: &SQL,
  should_generate_types: &bool,
  handler: &Handler,
) -> Result<(bool, Option<TsQuery>)> {
  let mut failed = false;

  let conn = match &db_conn {
    DBConn::SqliteConn(conn) => conn,
    _ => panic!("Invalid connection type"),
  };

  {
    let span = sql.span.to_owned();
    let query = sql.query.clone();
    let conn = conn.lock().await;
    let pool_conn = conn.get().await.unwrap();
    let inner = pool_conn.conn.clone();

    let result = tokio::task::spawn_blocking(move || {
      let conn = inner.lock().unwrap();
      // Use EXPLAIN to validate the SQL without executing it
      let explain_query = format!("EXPLAIN {}", query);
      conn.execute_batch(&explain_query)
    })
    .await
    .unwrap();

    if let Err(e) = result {
      handler.span_bug_no_panic(span, &e.to_string());
      failed = true;
    }
  }

  let mut ts_query = None;

  if should_generate_types == &true {
    ts_query = Some(generate_ts_interface(sql, db_conn).await?);
  }

  Ok((failed, ts_query))
}
