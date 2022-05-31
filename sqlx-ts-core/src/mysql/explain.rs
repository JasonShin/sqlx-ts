use mysql::prelude::*;
use mysql::*;
use sqlx_ts_common::cli::Cli;
use sqlx_ts_common::config::Config;
use sqlx_ts_common::SQL;
use std::borrow::Borrow;
use swc_common::errors::Handler;

pub fn explain(sqls: &Vec<SQL>, handler: &Handler, cli_args: &Cli) -> bool {
    let config = Config::new(cli_args.to_owned());
    let mut failed = false;

    for sql in sqls {
        let span = sql.span.to_owned();
        let explain_query = format!("EXPLAIN {}", sql.query);
        let connection = &config.get_correct_connection(&sql.query);

        if let Some(connection) = connection {
            let db_pass = &connection.db_pass;
            let db_name = &connection.db_name;
            let opts = OptsBuilder::new()
                .ip_or_hostname(Some(&connection.db_host))
                .tcp_port(&connection.db_port as u16)
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
        } else {
            handler.span_bug_no_panic(span, "Failed to find a matching DB connection for MySQL DB");
            failed = true;
        }
    }

    failed
}
