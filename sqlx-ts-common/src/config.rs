use crate::cli::Cli;
use std::env::var;

/// Config is used to determine connection configurations for primary target Database
/// It uses 2 sources of config and they are used in following priorities
/// 1. any configuration via CLI options
/// 2. any dotenv configured options
pub struct Config {
    db_host: String,
    db_port: i32,
    db_user: String,
    db_pass: String,
}

impl Config {
    pub fn new(cli_args: Cli) -> Config {
        return Config {
            db_host: cli_args.db_host.unwrap_or(var("DB_HOST").unwrap()),
            db_port: cli_args
                .db_port
                .unwrap_or(var("DB_PORT").unwrap().parse::<i32>().unwrap()),
            db_user: cli_args.db_user.unwrap_or(var("DB_USER").unwrap()),
            db_pass: cli_args.db_pass.unwrap_or(var("DB_PASS").unwrap()),
        };
    }

    pub fn get_postgres_cred(&self) -> String {
        format!(
            "host={} user={} password={} port={:?}",
            self.db_host, self.db_user, self.db_pass, self.db_port
        )
    }
}
