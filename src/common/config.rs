use crate::common::cli::Cli;
use crate::common::dotenv::Dotenv;
use crate::common::types::DatabaseType;
use crate::common::lazy::CLI_ARGS;
use regex::Regex;
use serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SqlxConfig {
    pub generate_types: Option<GenerateTypesConfig>,
    pub connections: HashMap<String, DbConnectionConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenerateTypesConfig {
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
    pub dotenv: Dotenv,
    pub generate_types_config: Option<GenerateTypesConfig>,
    pub connections: HashMap<String, DbConnectionConfig>,
}

impl Config {
    pub fn new() -> Config {
        let dotenv = Dotenv::new();

        let (generate_types_config, connections) = Self::build_configs(&dotenv);
        let generate_types_config =
            generate_types_config.and_then(|config| if config.enabled { Some(config) } else { None });

        Config {
            dotenv,
            connections,
            generate_types_config,
        }
    }

    /// Build the initial connection config to be used as a HashMap
    fn build_configs(dotenv: &Dotenv) -> (Option<GenerateTypesConfig>, HashMap<String, DbConnectionConfig>) {
        let default_config_path = PathBuf::from_str(".sqlxrc.json").unwrap();
        let file_config_path = &CLI_ARGS.config.clone().unwrap_or(default_config_path);
        let file_based_config = fs::read_to_string(&file_config_path);

        let file_based_config = &file_based_config.map(|f| serde_json::from_str::<SqlxConfig>(f.as_str()).unwrap());

        let connections = &mut file_based_config
            .as_ref()
            .map(|config| config.connections.clone())
            .unwrap_or_default();

        let generate_types = &file_based_config
            .as_ref()
            .map(|config| config.generate_types.clone())
            .unwrap_or(None);

        connections.insert(
            "default".to_string(),
            Self::get_default_connection_config(dotenv, &connections.get("default")),
        );

        (generate_types.to_owned(), connections.to_owned())
    }

    /// Figures out the default connection, default connection must exist for sqlx-ts to work
    /// It will retrieve a default connection in the following order
    /// 1. CLI arg
    /// 2. Environment variables
    /// 3. .sqlxrc.json configuration file
    fn get_default_connection_config(
        dotenv: &Dotenv,
        default_config: &Option<&DbConnectionConfig>,
    ) -> DbConnectionConfig {
        let db_type = &CLI_ARGS
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

        let db_host = &CLI_ARGS
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

        let db_port = &CLI_ARGS
            .db_port
            .or(dotenv.db_port)
            .or_else(|| default_config.map(|x| x.db_port))
            .expect(
                r"
             Failed to fetch DB port.
             Please provide it at least through a CLI arg or an environment variable or through
             file based configuration
             ",
            );

        let db_user = &CLI_ARGS
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

        let db_pass = &CLI_ARGS
            .db_pass
            .clone()
            .or_else(|| dotenv.db_pass.clone())
            .or_else(|| default_config.map(|x| x.db_pass.clone()).flatten());

        let db_name = &CLI_ARGS
            .db_name
            .clone()
            .or_else(|| dotenv.db_name.clone())
            .or_else(|| default_config.map(|x| x.db_name.clone()).flatten());

        DbConnectionConfig {
            db_type: db_type.to_owned(),
            db_host: db_host.to_owned(),
            db_port: db_port.to_owned(),
            db_user: db_user.to_owned(),
            db_pass: db_pass.to_owned(),
            db_name: db_name.to_owned(),
        }
    }

    pub fn get_correct_connection(&self, raw_sql: &str) -> DbConnectionConfig {
        let re = Regex::new(r"(/*|//|--) @db: (?P<conn>[\w]+)( */){0,}").unwrap();
        let found_matches = re.captures(raw_sql);

        if let Some(found_match) = &found_matches {
            let detected_conn_name = &found_match[2];
            return self
                .connections
                .get(detected_conn_name)
                .unwrap_or_else(|| {
                    panic!("Failed to find a matching connection type - connection name: {detected_conn_name}")
                })
                .clone();
        }

        self.connections
            .get("default")
            .expect(
                r"Failed to find the default connection configuration - check your configuration
              CLI options: https://jasonshin.github.io/sqlx-ts/user-guide/2.1.cli-options.html
              File based config: https://jasonshin.github.io/sqlx-ts/reference-guide/2.configs-file-based.html
            ",
            )
            .clone()
    }
}
