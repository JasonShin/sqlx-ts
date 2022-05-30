use postgres::{Client, NoTls};
use sqlx_ts_common::cli::Cli;
use sqlx_ts_common::config::Config;
use sqlx_ts_common::SQL;
use swc_common::errors::Handler;

fn get_postgres_cred(config: &Config) -> String {
    format!(
        "host={} user={} password={} port={:?}",
        &config.db_host,
        &config.db_user,
        &config.db_pass.as_ref().unwrap_or(&"".to_string()),
        &config.db_port,
    )
}

pub fn explain<'a>(sqls: &Vec<SQL>, handler: &Handler, cli_args: &Cli) -> bool {
    let config = Config::new(cli_args.to_owned());

    println!("config {:?}", config);
    let mut failed = false;

    for sql in sqls {
        let span = sql.span.to_owned();
        let explain_query = format!("EXPLAIN {}", sql.query);
        println!("postgres sql {explain_query}");
        &config.get_correct_connection(&sql.query);
        let mut conn = Client::connect(get_postgres_cred(&config), NoTls).unwrap();
        let result = conn.query(explain_query.as_str(), &[]);

        if let Err(e) = result {
            handler.span_bug_no_panic(span, e.as_db_error().unwrap().message());
            failed = true;
        }
    }

    failed
}
