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

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(config.db_host))
        .tcp_port(config.db_port as u16)
        .user(Some(config.db_user))
        .pass(config.db_pass)
        .db_name(config.db_name);
    println!("checking opts {:?}", opts);
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    for sql in sqls {
        let span = sql.span.to_owned();
        let explain_query = format!("EXPLAIN {}", sql.query);
        let result: Result<Vec<Row>> = conn.query(explain_query);

        if let Err(err) = result {
            handler.span_bug_no_panic(span, err.to_string().as_str());
            failed = true;
        }
    }

    failed
}
