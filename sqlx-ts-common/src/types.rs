use clap::ArgEnum;
use serde::{Deserialize, Serialize};

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
