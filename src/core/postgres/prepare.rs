use crate::common::SQL;
use crate::core::connection::DBConn;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;

use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL. Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to `generate_types = true`
pub async fn prepare(
    db_conn: &DBConn,
    sql: &SQL,
    should_generate_types: &bool,
    handler: &Handler,
) -> Result<(bool, Option<TsQuery>)> {
    let mut failed = false;

    let mut conn = match &db_conn {
        DBConn::PostgresConn(conn) => conn,
        _ => panic!("Invalid connection type"),
    };

    let span = sql.span.to_owned();

    let prepare_query = format!("PREPARE sqlx_stmt AS {}", sql.query);

    let result = sqlx::query(&prepare_query).execute(conn).await;

    if let Err(e) = result {
        handler.span_bug_no_panic(span, e.to_string().as_str());
        failed = true;
    } else {
        println!("checking if the prepare statement was executed successfully {:?}", result);
        // We should only deallocate if the prepare statement was executed successfully
        sqlx::query("DEALLOCATE sqlx_stmt").execute(conn).await?;
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        ts_query = Some(generate_ts_interface(sql, &db_conn).await?);
    }

    Ok((failed, ts_query))
}
