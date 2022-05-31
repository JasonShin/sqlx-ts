use crate::types::DatabaseType;
use std::env::var;

#[derive(Clone, Debug)]
pub struct Dotenv {
    pub db_type: Option<DatabaseType>,
    pub db_user: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<i32>,
    pub db_pass: Option<String>,
    pub db_name: Option<String>,
}

impl Dotenv {
    pub fn new() -> Dotenv {
        Dotenv {
            db_type: match var("DB_TYPE").ok() {
                None => None,
                Some(val) => {
                    if val == "mysql" {
                        Some(DatabaseType::Mysql)
                    } else {
                        Some(DatabaseType::Postgres)
                    }
                }
            },
            db_user: var("DB_USER").ok(),
            db_host: var("DB_HOST").ok(),
            db_port: var("DB_PORT")
                .ok()
                .map(|val| val.parse::<i32>().expect("DB_PORT is not a valid integer")),
            db_pass: var("DB_PASS").ok(),
            db_name: var("DB_NAME").ok(),
        }
    }
}