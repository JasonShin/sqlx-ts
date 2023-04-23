use crate::common::lazy::CONFIG;
use crate::common::SQL;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::TsQuery;
use postgres::{Client, NoTls};
use std::cell::RefCell;

use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL. Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to `generate_types = true`
pub fn prepare<'a>(sql: &SQL, should_generate_types: &bool, handler: &Handler) -> (bool, Option<TsQuery>) {
    let connection = &CONFIG.get_correct_db_connection(&sql.query);
    let postgres_cred = &CONFIG.get_postgres_cred(connection);
    let mut conn = Client::connect(postgres_cred, NoTls).unwrap();

    let mut failed = false;

    let span = sql.span.to_owned();
    let prepare_query = format!("PREPARE sqlx_stmt AS {}", sql.query);
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
        ts_query = Some(generate_ts_interface(sql, &DBConn::PostgresConn(&mut RefCell::new(&mut conn))).unwrap());
    }

    (failed, ts_query)
}
