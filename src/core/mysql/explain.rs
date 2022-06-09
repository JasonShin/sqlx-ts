use crate::common::config::DbConnectionConfig;
use crate::common::SQL;
use mysql::prelude::*;
use mysql::*;
use std::borrow::Borrow;
use swc_common::errors::Handler;

pub fn explain(sql: &SQL, connection: &DbConnectionConfig, handler: &Handler) -> bool {
    let mut failed = false;

    let span = sql.span.to_owned();
    let explain_query = format!("EXPLAIN {}", sql.query);

    let db_pass = &connection.db_pass;
    let db_name = &connection.db_name;
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(&connection.db_host))
        .tcp_port(connection.db_port.clone())
        .user(Some(&connection.db_user))
        .pass(db_pass.clone())
        .db_name(db_name.clone());

    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let result: Result<Vec<Row>> = conn.query(explain_query);

    if let Err(err) = result {
        handler.span_bug_no_panic(span, err.to_string().as_str());
        failed = true;
    }

    failed
}
