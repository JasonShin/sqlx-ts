use clap::ArgEnum;
use serde::{Serialize, Deserialize};

#[derive(ArgEnum, Debug, Clone)]
pub enum JsExtension {
    Ts,
    Js,
}

#[derive(ArgEnum, Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    Postgres,
    Mysql,
}
