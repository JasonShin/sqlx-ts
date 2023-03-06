use crate::common::lazy::CONFIG;
use crate::common::SQL;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::TsQuery;
use mysql::prelude::*;
use mysql::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL.
/// Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to generate_types = true
pub fn prepare(sql: &SQL, should_generate_types: &bool, handler: &Handler) -> (bool, Option<TsQuery>) {
    let connection_config = CONFIG.get_correct_db_connection(&sql.query);
    let opts = CONFIG.get_mysql_cred(&connection_config);
    let mut conn = Conn::new(opts).unwrap();

    let mut failed = false;

    let span = sql.span.to_owned();
    let explain_query = format!("PREPARE stmt FROM \"{}\"", sql.query);

    let result: Result<Vec<Row>> = conn.query(explain_query);

    if let Err(err) = result {
        handler.span_bug_no_panic(span, err.to_string().as_str());
        failed = true;
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        ts_query = Some(generate_ts_interface(sql, &DBConn::MySQLPooledConn(&mut RefCell::new(&mut conn))).unwrap());
    }

    (failed, ts_query)
}
