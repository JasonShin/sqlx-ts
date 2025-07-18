use super::types::NamingConvention;
use crate::common::dotenv::Dotenv;
use crate::common::lazy::{CLI_ARGS, CONFIG};
use crate::common::logger::*;
use crate::common::types::{DatabaseType, LogLevel};
use colored::Colorize;
use regex::Regex;
use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SqlxConfig {
  pub log_level: Option<LogLevel>,
  #[serde(rename = "generateTypes")]
  pub generate_types: Option<GenerateTypesConfig>,
  pub connections: HashMap<String, DbConnectionConfig>,
}

pub const fn default_bool<const V: bool>() -> bool {
  V
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenerateTypesConfig {
  #[serde(default = "default_bool::<false>")]
  pub enabled: bool,
  #[deprecated]
  #[serde(rename = "convertToCamelCaseColumnName", default = "default_bool::<false>")]
  pub convert_to_camel_case_column_name: bool,
  #[serde(rename = "columnNamingConvention")]
  pub column_naming_convention: Option<NamingConvention>,
  pub generate_path: Option<PathBuf>,
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
  #[serde(rename = "PG_SEARCH_PATH")]
  pub pg_search_path: Option<String>,
  #[serde(rename = "POOL_SIZE", default = "default_pool_size")]
  pub pool_size: u32,
  #[serde(rename = "CONNECTION_TIMEOUT", default = "default_connection_timeout")]
  pub connection_timeout: u64,
}

fn default_pool_size() -> u32 {
  10
}

fn default_connection_timeout() -> u64 {
  5
}

/// Config is used to determine connection configurations for primary target Database
/// It uses 2 sources of config and they are used in following priorities
/// 1. any configuration via CLI options
/// 2. any dotenv configured options
#[derive(Clone, Debug)]
pub struct Config {
  pub generate_types_config: Option<GenerateTypesConfig>,
  pub connections: HashMap<String, DbConnectionConfig>,
  pub ignore_patterns: Vec<String>,
  pub log_level: LogLevel,
}

impl Default for Config {
  fn default() -> Self {
    Self::new()
  }
}

impl Config {
  pub fn new() -> Config {
    let dotenv = Dotenv::new(CLI_ARGS.env.clone());

    let default_config_path = PathBuf::from_str(".sqlxrc.json").unwrap();
    let default_ignore_config_path = PathBuf::from_str(".sqlxignore").unwrap();
    let file_config_path = &CLI_ARGS.config.clone().unwrap_or(default_config_path);
    let connections = Self::build_configs(&dotenv, file_config_path);
    let generate_types_config = Self::generate_types_config(file_config_path);

    let generate_types_config =
      generate_types_config.and_then(|config| if config.enabled { Some(config) } else { None });
    let ignore_patterns = Self::get_ignore_patterns(&default_ignore_config_path);
    let log_level = Self::get_log_level(file_config_path);
    Config {
      connections,
      generate_types_config,
      ignore_patterns,
      log_level,
    }
  }

  fn get_ignore_patterns(ignore_config_path: &PathBuf) -> Vec<String> {
    let mut base_ignore_patterns = vec!["*.queries.ts".to_string(), "*.queries.js".to_string()];
    base_ignore_patterns.extend(CLI_ARGS.ignore.clone());
    let file_based_ignore_config = fs::read_to_string(ignore_config_path);

    if file_based_ignore_config.is_err() {
      return base_ignore_patterns.to_vec();
    }

    let file_based_ignore_config = &file_based_ignore_config
        .expect("Failed to read the .sqlxignore file. Please check the contents of the ignore file and try again - https://jasonshin.github.io/sqlx-ts/api/2.ignore-patterns.html");
    let file_based_ignore_config = file_based_ignore_config.split('\n');
    let file_based_ignore_config: Vec<&str> = file_based_ignore_config.clone().collect();

    let custom_ignore_configs = &file_based_ignore_config
      .iter()
      .filter(|x| !x.is_empty())
      .map(|x| x.to_string())
      .collect::<Vec<String>>();

    base_ignore_patterns.extend(custom_ignore_configs.to_vec());
    base_ignore_patterns.clone()
  }

  /// Retrieves the configuration required for generating typescript interface
  /// If there is CLI_ARGS.generate_types set already, it would prioritise using CLI_ARGS
  fn generate_types_config(file_config_path: &PathBuf) -> Option<GenerateTypesConfig> {
    let file_based_config = fs::read_to_string(file_config_path);
    let file_based_config = &file_based_config.map(|f| serde_json::from_str::<SqlxConfig>(f.as_str()).unwrap());

    let cli_default = GenerateTypesConfig {
      enabled: CLI_ARGS.generate_types,
      convert_to_camel_case_column_name: false,
      column_naming_convention: None,
      generate_path: CLI_ARGS.generate_path.to_owned(),
    };

    if let Ok(file_based_config) = &file_based_config {
      if let Some(generate_types) = &file_based_config.generate_types {
        let generate_types = generate_types.clone();
        // If the file config is provided, we will return the file config's default values but CLI config as priority
        return Some(GenerateTypesConfig {
          enabled: CLI_ARGS.generate_types || generate_types.enabled,
          generate_path: generate_types.generate_path.or(CLI_ARGS.generate_path.to_owned()),
          column_naming_convention: generate_types.column_naming_convention,
          convert_to_camel_case_column_name: generate_types.convert_to_camel_case_column_name,
        });
      }
      // If the file config is not provided, we will return the CLI arg's default values
      Some(cli_default)
    } else {
      Some(cli_default)
    }
  }

  /// Build the initial connection config to be used as a HashMap
  fn build_configs(dotenv: &Dotenv, file_config_path: &PathBuf) -> HashMap<String, DbConnectionConfig> {
    let file_based_config = fs::read_to_string(file_config_path);
    let file_based_config = &file_based_config.map(|f| {
      let result = serde_json::from_str::<SqlxConfig>(f.as_str());

      if result.is_err() {
        panic!(
          "{}",
          format!(
            "Empty or invalid JSON provided for file based configuration - config file: {file_config_path:?}, error: {result:?}",
          )
        );
      }

      result.unwrap()
    });

    let connections = &mut file_based_config
      .as_ref()
      .map(|config| config.connections.clone())
      .unwrap_or_else(|_| {
        Self::warning(
          format!("Failed to read config file from the path: {file_config_path:?}").as_str(),
          Self::get_log_level(file_config_path),
        );
        Default::default()
      });

    connections.insert(
      "default".to_string(),
      Self::get_default_connection_config(dotenv, &connections.get("default")),
    );

    connections.to_owned()
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
      .unwrap_or_else(|| {
        Self::error("Unable to retrieve a database type, please check your configuration and try again");
        panic!("")
      });

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

    let pg_search_path = &CLI_ARGS
      .pg_search_path
      .clone()
      .or_else(|| dotenv.pg_search_path.clone())
      .or_else(|| default_config.map(|x| x.pg_search_path.clone()).flatten());

    let pool_size = default_config
      .map(|x| x.pool_size)
      .or_else(|| Some(default_pool_size()))
      .unwrap();

    let connection_timeout = default_config
      .map(|x| x.connection_timeout)
      .or_else(|| Some(default_connection_timeout()))
      .unwrap();

    DbConnectionConfig {
      db_type: db_type.to_owned(),
      db_host: db_host.to_owned(),
      db_port: db_port.to_owned(),
      db_user: db_user.to_owned(),
      db_pass: db_pass.to_owned(),
      db_name: db_name.to_owned(),
      pg_search_path: pg_search_path.to_owned(),
      pool_size,
      connection_timeout,
    }
  }

  /// By passing in a SQL query, for example
  /// e.g.
  ///     -- @db: postgres
  ///     SELECT * FROM some_table;
  ///
  /// The method figures out the connection name to connect in order to validate the SQL query
  ///
  /// If you pass down a query with a annotation to specify a DB
  /// e.g.
  ///     -- @db: postgres
  ///     SELECT * FROM some_table;
  ///
  /// It should return the connection for postgres.
  ///
  /// If you pass down a query without an annotation
  /// e.g.
  ///     SELECT * FROM some_table;
  ///
  /// It should return the connection name that is available based on your connection configurations
  pub fn get_correct_db_connection(&self, raw_sql: &str) -> String {
    let re = Regex::new(r"(/*|//|--) @db: (?P<conn>[\w]+)( */){0,}").unwrap();
    let found_matches = re.captures(raw_sql);

    if let Some(found_match) = &found_matches {
      let detected_conn_name = &found_match[2];

      return detected_conn_name.to_string();
    }

    "default".to_string()
  }

  /// This is to follow the spec of connection string for MySQL
  /// https://dev.mysql.com/doc/connector-j/8.1/en/connector-j-reference-jdbc-url-format.html
  pub fn get_mysql_cred_str(&self, conn: &DbConnectionConfig) -> String {
    format!(
      "mysql://{user}:{pass}@{host}:{port}/{db_name}",
      user = &conn.db_user,
      pass = &conn.db_pass.as_ref().unwrap_or(&"".to_string()),
      host = &conn.db_host,
      port = &conn.db_port,
      db_name = &conn.db_name.clone().unwrap_or(conn.db_user.to_owned()),
    )
    .to_string()
  }

  pub fn get_postgres_cred(&self, conn: &DbConnectionConfig) -> String {
    format!(
      "postgresql://{user}:{pass}@{host}:{port}/{db_name}",
      user = &conn.db_user,
      pass = &conn.db_pass.as_ref().unwrap_or(&"".to_string()),
      host = &conn.db_host,
      port = &conn.db_port,
      // This is to follow the spec of Rust Postgres
      // `db_user` name gets used if `db_name` is not provided
      // https://docs.rs/postgres/latest/postgres/config/struct.Config.html#keys
      db_name = &conn.db_name.clone().unwrap_or(conn.db_user.to_owned()),
    )
  }

  pub fn get_log_level(file_config_path: &PathBuf) -> LogLevel {
    let file_based_config = fs::read_to_string(file_config_path);
    let file_based_config = &file_based_config.map(|f| serde_json::from_str::<SqlxConfig>(f.as_str()).unwrap());
    let log_level_from_file = file_based_config.as_ref().ok().and_then(|config| config.log_level);

    CLI_ARGS.log_level.or(log_level_from_file).unwrap_or(LogLevel::Info)
  }

  /// Custom logger for Config
  /// lazy::logger cannot be used as it requires Config itself to be initialised
  #[allow(clippy::print_stdout)]
  fn warning(message: &str, log_level: LogLevel) {
    if log_level.gte(&LogLevel::Warning) {
      let level = "[WARN]".yellow();
      println!("{level} {message}");
    }
  }

  /// Custom logger for Config
  /// lazy::logger cannot be used as it requires Config itself to be initialised
  #[allow(clippy::print_stderr)]
  fn error(message: &str) {
    let level = "[ERROR]".red();
    eprintln!("{level} {message}")
  }
}
