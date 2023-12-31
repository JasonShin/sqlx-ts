use mysql_async::prelude::*;
use postgres;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::core::connection::DBConn;

use super::types::ts_query::TsFieldType;

#[derive(Debug, Clone)]
pub struct Field {
    pub field_type: TsFieldType,
    pub is_nullable: bool,
}

pub type Fields = HashMap<String, Field>;

#[derive(Clone, PartialEq)]
struct ColumnsQueryResultRow {
    column_name: String,
    data_type: String,
    is_nullable: bool,
}

pub struct DBSchema {
    tables_cache: HashMap<String, Fields>,
}

impl DBSchema {
    pub fn new() -> DBSchema {
        DBSchema {
            tables_cache: HashMap::new(),
        }
    }

    /// fetch table's column details from information_schema of each database type
    ///
    /// # MySQL Notes
    /// - TABLE_SCHEMA in MySQL is basically the `database_name`, so it requires passing in database name as an argument
    ///
    /// # PostgreSQL Notes
    /// - PostgresSQL would utilise SEARCH_PATH option to search for the table in the database https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH
    pub async fn fetch_table(&mut self, table_name: &Vec<&str>, conn: &DBConn) -> Option<Fields> {
        let table_key: String = table_name.join(",");
        let cached_table_result = self.tables_cache.get(table_key.as_str());

        if let Some(cached_table_result) = cached_table_result {
            return Some(cached_table_result.clone());
        }

        let result = match &conn {
            DBConn::MySQLPooledConn(conn) => Self::mysql_fetch_table(self, table_name, conn).await,
            DBConn::PostgresConn(conn) => Self::postgres_fetch_table(self, table_name, conn).await,
        };

        if let Some(result) = &result {
            let _ = &self.tables_cache.insert(table_key, result.clone());
        }

        result
    }

    async fn postgres_fetch_table(&self, table_names: &Vec<&str>, conn: &Mutex<postgres::Client>) -> Option<Fields> {
        let table_names = table_names
            .iter()
            .map(|x| format!("'{x}'"))
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(
            r"
        SELECT
            COLUMN_NAME as column_name,
            DATA_TYPE as data_type,
            IS_NULLABLE as is_nulalble
        FROM information_schema.COLUMNS
        WHERE TABLE_SCHEMA = 'public'
        AND TABLE_NAME IN ({})
                ",
            table_names,
        );

        let mut fields: HashMap<String, Field> = HashMap::new();
        let result = conn.lock().await.borrow_mut().query(&query, &[]);

        if let Ok(result) = result {
            for row in result {
                let field_name: String = row.get(0);
                let field_type: String = row.get(1);
                let is_nullable: String = row.get(2);
                let field = Field {
                    field_type: TsFieldType::get_ts_field_type_from_postgres_field_type(field_type.to_owned()),
                    is_nullable: is_nullable == "YES",
                };
                fields.insert(field_name.to_owned(), field);
            }

            return Some(fields);
        }

        None
    }

    async fn mysql_fetch_table(&self, table_names: &Vec<&str>, conn: &Mutex<mysql_async::Pool>) -> Option<Fields> {
        let table_names = table_names
            .iter()
            .map(|x| format!("'{x}'"))
            .collect::<Vec<_>>()
            .join(",");
        let query = format!(
            r"
        SELECT
            COLUMN_NAME as column_name,
            DATA_TYPE as data_type,
            IS_NULLABLE as is_nulalble
        FROM information_schema.COLUMNS
        WHERE TABLE_SCHEMA = (SELECT DATABASE())
        AND TABLE_NAME IN ({})
                ",
            table_names
        );

        let mut fields: HashMap<String, Field> = HashMap::new();
        // TODO: replace with proper error types
        let mut conn = conn.lock().await.get_conn().await.unwrap();
        let result = conn.query::<mysql_async::Row, String>(query).await;

        if let Ok(result) = result {
            for row in result {
                let field_name: String = row.clone().take(0).unwrap();
                let field_type: String = row.clone().take(1).unwrap();
                let is_nullable: String = row.clone().take(2).unwrap();
                let field = Field {
                    field_type: TsFieldType::get_ts_field_type_from_mysql_field_type(field_type.to_owned()),
                    is_nullable: is_nullable == "YES",
                };
                fields.insert(field_name.to_owned(), field);
            }

            return Some(fields);
        }

        None
    }
}
