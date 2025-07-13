use crate::common::types::{DatabaseType, FileExtension, LogLevel};
use clap::Parser;
use std::fmt;

impl fmt::Display for FileExtension {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let extension = match self {
      FileExtension::Ts => ".ts".to_string(),
      FileExtension::Js => ".js".to_string(),
      FileExtension::Sql => ".sql".to_string(),
    };
    write!(f, "{}", extension)
  }
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Cli {
  /// Path to the Typescript or Javascript project
  #[clap(parse(from_os_str))]
  pub path: std::path::PathBuf,

  /// file extensions
  #[clap(value_enum, long, multiple_values = true)]
  pub ext: Vec<FileExtension>,

  /// Type of primary database to connect
  #[clap(value_enum, long)]
  pub db_type: Option<DatabaseType>,

  /// Primary DB host
  #[clap(long)]
  pub db_host: Option<String>,

  /// Primary DB Port
  #[clap(long)]
  pub db_port: Option<u16>,

  /// Primary DB user
  #[clap(long)]
  pub db_user: Option<String>,

  /// Primary DB pass
  #[clap(long)]
  pub db_pass: Option<String>,

  /// Primary DB database name
  #[clap(long)]
  pub db_name: Option<String>,

  /// PostgreSQL schema search path (default is "$user,public") https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH
  #[clap(long)]
  pub pg_search_path: Option<String>,

  /// Folder paths to ignore
  #[clap(long, multiple_values = true)]
  pub ignore: Vec<String>,

  /// Path to the file based configuration
  #[clap(long, parse(from_os_str))]
  pub config: Option<std::path::PathBuf>,

  /// generate types of raw SQLs using default configuration
  #[clap(long, short)]
  pub generate_types: bool,

  /// generates types in a target file path (example: src/app/queries.ts)
  #[clap(long, parse(from_os_str))]
  pub generate_path: Option<std::path::PathBuf>,

  /// log level to be used for the CLI debug > info > warning > error
  #[clap(value_enum, long)]
  pub log_level: Option<LogLevel>,

  /// Dotfile name (example: .env or .env.dev) [default: .env or environment]
  #[clap(long)]
  pub env: Option<String>,
}
