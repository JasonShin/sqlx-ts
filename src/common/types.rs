use clap::{ArgEnum, ValueEnum};
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(ArgEnum, Debug, Clone)]
pub enum FileExtension {
  Ts,
  Js,
  Sql,
  Mts,
  Cts,
  Mjs,
  Cjs,
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
  Postgres,
  Mysql,
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NamingConvention {
  Upper,
  Lower,
  Title,
  Camel,
  Pascal,
  Snake,
  Kebab,
}

impl NamingConvention {
  pub fn convert(&self, value: &str) -> String {
    match &self {
      NamingConvention::Upper => value.to_case(Case::Upper),
      NamingConvention::Lower => value.to_case(Case::Lower),
      NamingConvention::Title => value.to_case(Case::Title),
      NamingConvention::Camel => value.to_case(Case::Camel),
      NamingConvention::Pascal => value.to_case(Case::Pascal),
      NamingConvention::Snake => value.to_case(Case::Snake),
      NamingConvention::Kebab => value.to_case(Case::Kebab),
    }
  }
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
  Debug = 4,
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
