use crate::common::cli::Cli;
use crate::common::dotenv::Dotenv;
use crate::common::types::DatabaseType;
use regex::Regex;
use serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SqlxConfig {
    pub transforms: Option<TransformConfig>,
    pub connections: HashMap<String, DbConnectionConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransformConfig {
    pub enabled: bool,
    #[serde(rename = "convertToCamelCaseColumnName")]
    pub convert_to_camel_case_column_name: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbConnectionConfig {
    #[serde(rename = "DB_TYPE")]
    pub db_type: DatabaseType,
    #[serde(rename = "DB_HOST")]
    pub db_host: String,
    #[serde(rename = "DB_PORT")]
    pub db_port: u16,
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
    pub cli_args: Cli,
    pub dotenv: Dotenv,
    pub transformation_config: Option<TransformConfig>,
    pub connections: HashMap<String, DbConnectionConfig>,
}

impl Config {
    pub fn new(cli_args: Cli) -> Config {
        let cli_args = &cli_args;
        let dotenv = Dotenv::new();

        let (transformation_config, connections) = Self::build_configs(&cli_args, &dotenv);
        let transformation_config = transformation_config
            .clone()
            .and_then(|config| if config.enabled { Some(config.clone()) } else { None });

        Config {
            dotenv: dotenv.clone(),
            cli_args: cli_args.to_owned(),
            connections,
            transformation_config,
        }
    }

    /// Build the initial connection config to be used as a HashMap
    fn build_configs(
        cli_args: &Cli,
        dotenv: &Dotenv,
    ) -> (Option<TransformConfig>, HashMap<String, DbConnectionConfig>) {
        let default_config_path = PathBuf::from_str(".sqlxrc.json").unwrap();
        let file_config_path = &cli_args.config.clone().unwrap_or(default_config_path);
        let file_based_config = fs::read_to_string(&file_config_path).unwrap();

        let configs = serde_json::from_str::<SqlxConfig>(&file_based_config).unwrap();
        let mut connections = configs.connections;
        connections.insert(
            "default".to_string(),
            Self::get_default_connection_config(&cli_args, &dotenv, &connections.get("default")),
        );

        (configs.transforms, connections)
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
            .clone()
            .or_else(|| dotenv.db_type.clone())
            .or_else(|| default_config.map(|x| x.db_type.clone()))
            .expect(
                r"
             Failed to fetch DB type.
             Please provide it at least through a CLI arg or an environment variable or through
             file based configuration
             ",
            );

        let db_host = &cli_args
            .db_host
            .clone()
            .or_else(|| dotenv.db_host.clone())
            .or_else(|| default_config.map(|x| x.db_host.clone()))
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
            .clone()
            .or_else(|| dotenv.db_user.clone())
            .or_else(|| default_config.map(|x| x.db_user.clone()))
            .expect(
                r"
             Failed to fetch DB user.
             Please provide it at least through a CLI arg or an environment variable or through
             file based configuration
             ",
            );

        let db_pass = &cli_args
            .db_pass
            .clone()
            .or_else(|| dotenv.db_pass.clone())
            .or_else(|| default_config.map(|x| x.db_pass.clone()).flatten());

        let db_name = &cli_args
            .db_name
            .clone()
            .or_else(|| dotenv.db_name.clone())
            .or_else(|| default_config.map(|x| x.db_name.clone()).flatten());

        DbConnectionConfig {
            db_type: db_type.to_owned().to_owned(),
            db_host: db_host.to_owned(),
            db_port: db_port.to_owned(),
            db_user: db_user.to_owned(),
            db_pass: db_pass.to_owned(),
            db_name: db_name.to_owned(),
        }
    }

    pub fn get_correct_connection(&self, raw_sql: &str) -> DbConnectionConfig {
        let re = Regex::new(r"(/*|//) db: (?P<conn>[\w]+)( */){0,}").unwrap();
        let found_matches = re.captures(raw_sql);

        if let Some(found_match) = &found_matches {
            let detected_conn_name = &found_match[2];
            return self.connections.get(detected_conn_name)
                .expect(format!("Failed to find a matching connection type - connection name: {detected_conn_name}").as_str())
                .clone();
        }

        self.connections.get("default")
            .expect(r"Failed to find the default connection configuration - check your configuration
              CLI options: https://jasonshin.github.io/sqlx-ts/user-guide/2.1.cli-options.html
              File based config: https://jasonshin.github.io/sqlx-ts/reference-guide/2.configs-file-based.html
            ").clone()
    }
}
