use std::cell::RefCell;

use crate::common::config::Config;
use crate::common::SQL;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::{DBConn, TsQuery};
use mysql::prelude::*;
use mysql::*;
use swc_common::errors::Handler;

pub fn prepare(sql: &SQL, config: &Config, should_generate_types: &bool, handler: &Handler) -> (bool, Option<TsQuery>) {
    let connection_config = &config.get_correct_connection(&sql.query);
    let mut failed = false;

    let span = sql.span.to_owned();
    let explain_query = format!("PREPARE stmt FROM \"{}\"", sql.query);

    let db_pass = &connection_config.db_pass;
    let db_name = &connection_config.db_name;
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(&connection_config.db_host))
        .tcp_port(connection_config.db_port)
        .user(Some(&connection_config.db_user))
        .pass(db_pass.clone())
        .db_name(db_name.clone());
    let mut conn = Conn::new(opts).unwrap();

    let result: Result<Vec<Row>> = conn.query(explain_query);

    if let Err(err) = result {
        handler.span_bug_no_panic(span, err.to_string().as_str());
        failed = true;
    }

    let mut ts_query = None;

    if should_generate_types == &true {
        ts_query = Some(
            generate_ts_interface(
                sql,
                connection_config,
                &DBConn::MySQLPooledConn(&mut RefCell::new(&mut conn)),
                &config.generate_types_config,
            )
            .unwrap(),
        );
    }

    (failed, ts_query)
}
