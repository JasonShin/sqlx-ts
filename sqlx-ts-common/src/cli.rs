use clap::{ArgEnum, Parser};

#[derive(ArgEnum, Debug, Clone)]
pub enum JsExtension {
    Ts,
    Js,
}

impl ToString for JsExtension {
    fn to_string(&self) -> String {
        match self {
            JsExtension::Ts => ".ts".to_string(),
            JsExtension::Js => ".js".to_string(),
        }
    }
}

#[derive(ArgEnum, Debug, Clone)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    SQLite,
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Cli {
    /// Path to the Typescript or Javascript project
    #[clap(parse(from_os_str))]
    pub path: std::path::PathBuf,

    /// Javascript Extension
    #[clap(
    arg_enum,
    long,
    default_value_t=JsExtension::Ts
    )]
    pub ext: JsExtension,

    /// Type of primary database to connect
    #[clap(
    arg_enum,
    long,
    default_value_t=DatabaseType::Postgres
    )]
    pub db_type: DatabaseType,

    /// Primary DB host
    #[clap(long)]
    pub db_host: Option<String>,

    /// Primary DB Port
    #[clap(long)]
    pub db_port: Option<i32>,

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
    #[clap(long, parse(from_os_str), multiple_values = true)]
    pub ignore: Vec<std::path::PathBuf>,
}
