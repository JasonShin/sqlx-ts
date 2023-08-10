use crate::common::lazy::{CONFIG, DB_SCHEMA};
use crate::common::SQL;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;
use mysql::prelude::*;
use mysql::*;

use std::cell::RefCell;
use swc_common::errors::Handler;

/// Runs the prepare statement on the input SQL.
/// Validates the query is right by directly connecting to the configured database.
/// It also processes ts interfaces if the configuration is set to generate_types = true
pub fn prepare(sql: &SQL, should_generate_types: &bool, handler: &Handler) -> Result<(bool, Option<TsQuery>)> {
    let connection_config = CONFIG.get_correct_db_connection(&sql.query);
    let opts = CONFIG.get_mysql_cred(&connection_config);
    let mut conn = Conn::new(opts)?;

    let mut failed = false;

    let span = sql.span.to_owned();
    let explain_query = format!("PREPARE stmt FROM \"{}\"", sql.query);

    let result: Result<Vec<Row>, _> = conn.query(explain_query);

    if let Err(err) = result {
        handler.span_bug_no_panic(span, err.to_string().as_str());
        failed = true;
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        let mysql_conn = &mut RefCell::new(&mut conn);
        let mysql_conn = &DBConn::MySQLPooledConn(mysql_conn);
        DB_SCHEMA.fetch_enums(connection_config.db_name.unwrap().as_str(), &mysql_conn);

        ts_query = Some(generate_ts_interface(sql, &mysql_conn)?);
    }

    Ok((failed, ts_query))
}
