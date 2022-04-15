use mysql::*;
use mysql::prelude::*;
use sqlx_ts_common::cli::Cli;
use sqlx_ts_common::config::Config;
use sqlx_ts_common::SQL;
use swc_common::errors::Handler;

pub fn explain(sqls: &Vec<SQL>, handler: &Handler, cli_args: &Cli) -> bool {
    let config = Config::new(cli_args.to_owned());
    // let mut conn = Client::connect(&config.get_postgres_cred(), NoTls).unwrap();

    false
}
