use postgres::{Client, Error, NoTls, Row};
use sqlx_ts_common::cli::Cli;
use sqlx_ts_common::config::Config;
use sqlx_ts_common::SQL;
use swc_common::errors::Handler;

pub fn explain<'a>(sqls: &Vec<SQL>, handler: &Handler, cli_args: &Cli) -> bool {
    let config = Config::new(cli_args.to_owned());
    let mut conn = Client::connect(&config.get_postgres_cred(), NoTls).unwrap();

    let mut failed = false;

    for sql in sqls {
        let span = sql.span.to_owned();
        let explain_query = format!("EXPLAIN {}", sql.query);
        println!("postgres sql {explain_query}");
        let result = conn.query(explain_query.as_str(), &[]);

        if let Err(e) = result {
            handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
            failed = true;
        }
    }

    failed
}
