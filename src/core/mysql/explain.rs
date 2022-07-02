use crate::common::config::Config;
use crate::common::SQL;
use crate::ts_generator::generator::generate_ts_interface;
use crate::ts_generator::types::DBConn;
use mysql::prelude::*;
use mysql::*;
use swc_common::errors::Handler;
use swc_ecma_ast::op;

pub fn explain(sql: &SQL, config: &Config, handler: &Handler) -> bool {
    let connection = &config.get_correct_connection(&sql.query);
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
    let mut conn = Conn::new(opts).unwrap();
    generate_ts_interface(&sql, &connection, &DBConn::MySQLPooledConn(&mut conn));

    let result: Result<Vec<Row>> = conn.query(explain_query);

    if let Err(err) = result {
        handler.span_bug_no_panic(span, err.to_string().as_str());
        failed = true;
    }

    failed
}
