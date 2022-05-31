use crate::cli::Cli;
use crate::dotenv::Dotenv;
use crate::types::DatabaseType;
use regex::Regex;
use serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbConnectionConfig {
    #[serde(rename = "DB_TYPE")]
    pub db_type: DatabaseType,
    #[serde(rename = "DB_HOST")]
    pub db_host: String,
    #[serde(rename = "DB_PORT")]
    pub db_port: i32,
    #[serde(rename = "DB_USER")]
    pub db_user: String,
    #[serde(rename = "DB_PASS")]
    pub db_pass: Option<String>,
    #[serde(rename = "DB_NAME")]
    pub db_name: Option<String>,
}

/// Config is used to determine connection configurations for primary target Database
/// It uses 2 sources of config and they are used in following priorities
/// 1. any configuration via CLI options
/// 2. any dotenv configured options
#[derive(Clone, Debug)]
pub struct Config {
    cli_args: Cli,
    dotenv: Dotenv,
    pub connections: HashMap<String, DbConnectionConfig>,
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
        let dotenv = Dotenv::new();

        Config {
            dotenv,
            cli_args: cli_args.to_owned(),
            connections: Self::build_connection_configs(&cli_args, &dotenv),
        }
    }

    /// Build the initial connection config to be used as a HashMap
    fn build_connection_configs(
        cli_args: &Cli,
        dotenv: &Dotenv,
    ) -> HashMap<String, DbConnectionConfig> {
        let default_config_path = PathBuf::from_str(".sqlxrc.json").unwrap();
        let file_config_path = &cli_args.config.clone().unwrap_or(default_config_path);
        let file_based_config = fs::read_to_string(&file_config_path);

        let mut connections: HashMap<String, DbConnectionConfig> = HashMap::new();

        if let Ok(file_based_config) = file_based_config {
            connections = serde_json::from_str(&file_based_config).unwrap();
        }

        connections["default"] =
            Self::get_default_connection_config(&cli_args, &dotenv, &connections.get("default"));

        connections
    }

    /// Figures out the default connection, default connection must exist for sqlx-ts to work
    /// It will retrieve a default connection in the following order
    /// 1. CLI arg
    /// 2. Environment variables
    /// 3. .sqlxrc.json configuration file
    fn get_default_connection_config(
        cli_args: &Cli,
        dotenv: &Dotenv,
        default_config: &Option<&DbConnectionConfig>,
    ) -> DbConnectionConfig {
        let db_type = &cli_args
            .db_type
            .or_else(|| default_config.map(|x| x.db_type))
            .expect(
                r"
             Failed to fetch DB type.
             Please provide it at least through a CLI arg or an environment variable or through
             file based configuration
             ",
            );

        let db_host = &cli_args
            .db_host
            .or_else(|| dotenv.db_host)
            .or_else(|| default_config.map(|x| x.db_host))
            .expect(
                r"
             Failed to fetch DB host.
             Please provide it at least through a CLI arg or an environment variable or through
             file based configuration
             ",
            );

        let db_port = &cli_args
            .db_port
            .or_else(|| dotenv.db_port)
            .or_else(|| default_config.map(|x| x.db_port))
            .expect(
                r"
             Failed to fetch DB port.
             Please provide it at least through a CLI arg or an environment variable or through
             file based configuration
             ",
            );

        let db_user = &cli_args
            .db_user
            .or_else(|| dotenv.db_host)
            .or_else(|| default_config.map(|x| x.db_user))
            .expect(
                r"
             Failed to fetch DB user.
             Please provide it at least through a CLI arg or an environment variable or through
             file based configuration
             ",
            );

        let db_pass = &cli_args
            .db_pass
            .or_else(|| dotenv.db_pass)
            .or_else(|| default_config.map(|x| x.db_pass.unwrap()));

        let db_name = &cli_args
            .db_name
            .or_else(|| dotenv.db_name)
            .or_else(|| default_config.map(|x| x.db_name.unwrap()));

        DbConnectionConfig {
            db_type: db_type.to_owned(),
            db_host: db_host.to_owned(),
            db_port: db_port.to_owned(),
            db_user: db_user.to_owned(),
            db_pass: db_pass.to_owned(),
            db_name: db_name.to_owned(),
        }
    }

    pub fn get_correct_connection(&self, raw_sql: &str) -> bool {
        if let Some(connections) = &self.connections {
            // NOTE: once we've fully migrated to using the HashMap approach, we can delete this if statement
            let mut connection_names = connections.keys();
            let re = Regex::new(r"(/*|//) db: (?P<conn>[\w]+)( */){0,}").unwrap();
            let found_matches = re.captures(raw_sql);

            // Self::get_default_connection_config(self);
            if let Some(found_match) = &found_matches {
                let detected_conn_name = &found_match[2];
                let conn = connections.get(detected_conn_name)
                    .expect("Failed to find a matching connection type - connection name: {detected_conn_name}");
            }
            self.get_default_connection_config(&connections.get("default"));
            false
        } else {
            /*DbConnectionConfig {
                db_host: &self.db_host,
            }*/
            false
        }
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
