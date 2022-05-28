use crate::cli::{Cli, DatabaseType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var;
use std::fs;
use std::path::{PathBuf};
use std::str::FromStr;
use serde;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbConnectionConfig {
    #[serde(rename="DB_TYPE")]
    pub db_type: String,
    #[serde(rename="DB_HOST")]
    pub db_host: String,
    #[serde(rename="DB_PORT")]
    pub db_port: i32,
    #[serde(rename="DB_USER")]
    pub db_user: String,
    #[serde(rename="DB_PASS")]
    pub db_pass: Option<String>,
    #[serde(rename="DB_NAME")]
    pub db_name: Option<String>,
}

/// Config is used to determine connection configurations for primary target Database
/// It uses 2 sources of config and they are used in following priorities
/// 1. any configuration via CLI options
/// 2. any dotenv configured options
#[derive(Clone, Debug)]
pub struct Config {
    pub db_host: String,
    pub db_port: i32,
    pub db_user: String,
    pub db_pass: Option<String>,
    pub db_name: Option<String>,
    pub connections: Option<HashMap<String, DbConnectionConfig>>,
}

fn required_var_msg(key: &str) -> String {
    format!(
        "{} is not provided neither by an environment variable or CLI argument",
        key
    )
}

impl Config {
    pub fn new(cli_args: Cli) -> Config {
        let cli_args = &cli_args;

        Config {
            db_host: match &cli_args.db_host {
                Some(db_host) => db_host.to_owned(),
                None => var("DB_HOST").expect(required_var_msg("DB_HOST").as_str()),
            },
            db_port: match cli_args.db_port {
                Some(db_port) => db_port,
                None => var("DB_PORT")
                    .map(|x| x.trim().to_owned())
                    .map(|x| {
                        x.to_string()
                            .parse::<i32>()
                            .expect("DB_PORT is not a valid integer")
                    })
                    .expect(required_var_msg("DB_PORT").as_str()),
            },
            db_user: match &cli_args.db_user {
                Some(db_user) => db_user.to_owned(),
                None => var("DB_USER").expect(required_var_msg("DB_USER").as_str()),
            },
            db_pass: match &cli_args.db_pass {
                Some(db_pass) => Some(db_pass.to_owned()),
                None => var("DB_PASS").ok(),
            },
            db_name: match &cli_args.db_name {
                Some(db_name) => Some(db_name.to_owned()),
                None => var("DB_NAME").ok(),
            },
            connections: Self::build_connection_configs(&cli_args),
        }
    }

    fn build_connection_configs(cli_args: &Cli) -> Option<HashMap<String, DbConnectionConfig>> {
        let default_config_path = PathBuf::from_str(".sqlxrc.json").unwrap();
        let file_config_path = &cli_args.config.clone().unwrap_or(default_config_path);
        let file_based_config = fs::read_to_string(&file_config_path);
        if let Ok(file_based_config) = file_based_config {
            serde_json::from_str(&file_based_config).unwrap()
        }
        None
    }

    pub fn get_postgres_cred(&self) -> String {
        format!(
            "host={} user={} password={} port={:?}",
            self.db_host,
            self.db_user,
            self.db_pass.as_ref().unwrap_or(&"".to_string()),
            self.db_port,
        )
    }
}
