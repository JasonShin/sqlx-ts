use crate::common::lazy::CONFIG;
use crate::common::SQL;
use crate::core::connection::DBConn;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;
use std::borrow::BorrowMut;

use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL. Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to `generate_types = true`
pub fn prepare(
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
    let searh_path = &CONFIG.get_pg_search_path(&sql.query);

    let span = sql.span.to_owned();

    // If postgres connection has search path, configure it now for this connection
    if searh_path.is_some() {
        let search_path_query = format!("SET search_path TO {}", &searh_path.clone().unwrap().as_str());
        let result = conn
            .lock()
            .unwrap()
            .borrow_mut()
            .query(search_path_query.as_str(), &[]);

        if let Err(e) = result {
            handler.span_bug_no_panic(span.clone(), e.as_db_error().unwrap().message());
            failed = true;
        }
    }

    let prepare_query = format!("PREPARE sqlx_stmt AS {}", sql.query);
    let result = conn.lock().unwrap().borrow_mut().query(prepare_query.as_str(), &[]);

    if let Err(e) = result {
        handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
        failed = true;
    } else {
        // We should only deallocate if the prepare statement was executed successfully
        let _ = &conn
            .lock()
            .unwrap()
            .borrow_mut()
            .query("DEALLOCATE sqlx_stmt", &[])
            .unwrap();
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        ts_query = Some(generate_ts_interface(sql, db_conn)?);
    }

    Ok((failed, ts_query))
}
