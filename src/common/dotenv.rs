use crate::common::types::DatabaseType;
use dotenv;

#[derive(Clone, Debug)]
pub struct Dotenv {
  pub db_type: Option<DatabaseType>,
  pub db_user: Option<String>,
  pub db_host: Option<String>,
  pub db_port: Option<u16>,
  pub db_pass: Option<String>,
  pub db_name: Option<String>,
  pub db_url: Option<String>,
  pub pg_search_path: Option<String>,
}

impl Default for Dotenv {
  fn default() -> Self {
    Self::new(None)
  }
}

impl Dotenv {
  fn get_var(key: &str) -> Option<String> {
    dotenv::var(key).ok()
  }

  pub fn new(path_to_dotenv: Option<String>) -> Dotenv {
    if let Some(value) = path_to_dotenv {
      dotenv::from_path(value).ok();
    }

    Dotenv {
      db_type: match Self::get_var("DB_TYPE") {
        None => None,
        Some(val) => {
          if val == "mysql" {
            Some(DatabaseType::Mysql)
          } else {
            Some(DatabaseType::Postgres)
          }
        }
      },
      db_user: Self::get_var("DB_USER"),
      db_host: Self::get_var("DB_HOST"),
      db_port: Self::get_var("DB_PORT").map(|val| val.parse::<u16>().expect("DB_PORT is not a valid integer")),
      db_pass: Self::get_var("DB_PASS"),
      db_name: Self::get_var("DB_NAME"),
      db_url: Self::get_var("DB_URL"),
      pg_search_path: Self::get_var("PG_SEARCH_PATH"),
    }
  }
}
