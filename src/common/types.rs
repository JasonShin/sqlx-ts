use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Debug, Clone)]
pub enum JsExtension {
    Ts,
    Js,
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    Postgres,
    Mysql,
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info = 3,
    Warning = 2,
    Error = 1,
}

impl LogLevel {
    /// Check if the current log level is greater than or equal to the other log level
    pub fn gte(&self, other: &Self) -> bool {
        *self as u8 >= *other as u8
    }
}
