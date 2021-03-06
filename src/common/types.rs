use clap::ArgEnum;
use serde::{Deserialize, Serialize};

#[derive(ArgEnum, Debug, Clone)]
pub enum JsExtension {
    Ts,
    Js,
}

#[derive(ArgEnum, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    Postgres,
    Mysql,
}
