use crate::common::types::{DatabaseType, JsExtension, LogLevel};
use clap::Parser;

impl ToString for JsExtension {
    fn to_string(&self) -> String {
        match self {
            JsExtension::Ts => ".ts".to_string(),
            JsExtension::Js => ".js".to_string(),
        }
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Cli {
    /// Path to the Typescript or Javascript project
    #[clap(parse(from_os_str))]
    pub path: std::path::PathBuf,

    /// Javascript Extension
    #[clap(
    value_enum,
    long,
    default_value_t=JsExtension::Ts
    )]
    pub ext: JsExtension,

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

    /// Folder paths to ignore
    #[clap(long, multiple_values = true)]
    pub ignore: Vec<String>,

    /// Path to the file based configuration
    #[clap(long, parse(from_os_str))]
    pub config: Option<std::path::PathBuf>,

    /// generate types of raw SQLs using default configuration
    #[clap(long, short)]
    pub generate_types: bool,

    /// generates types in a target directory path or a file
    #[clap(long, parse(from_os_str))]
    pub generate_path: Option<std::path::PathBuf>,

    #[clap(long, short)]
    pub message_format: Option<String>,

    /// log level to be used for the CLI info > warn > error
    #[clap(value_enum, long)]
    pub log_level: Option<LogLevel>,
}
