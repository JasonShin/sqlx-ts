use crate::common::lazy::{CONFIG, DB_CONNECTIONS};
use crate::common::SQL;
use mysql::Conn as MySQLConn;
use crate::core::connection::DBConn;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;
use mysql::prelude::*;
use mysql::*;

use std::cell::RefCell;
use std::rc::Rc;
use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL.
/// Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to generate_types = true
pub fn prepare(conn: &RefCell<&mut MySQLConn>, sql: &SQL, should_generate_types: &bool, handler: &Handler) -> Result<(bool, Option<TsQuery>)> {

    let mut failed = false;

    let span = sql.span.to_owned();
    let explain_query = format!("PREPARE stmt FROM \"{}\"", sql.query);

    let result: Result<Vec<Row>, _> = conn.borrow_mut().query(explain_query);

    if let Err(err) = result {
        handler.span_bug_no_panic(span, err.to_string().as_str());
        failed = true;
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        ts_query = Some(generate_ts_interface(
            sql,
            &DBConn::MySQLPooledConn(conn),
        )?);
    }

    Ok((failed, ts_query))
}
