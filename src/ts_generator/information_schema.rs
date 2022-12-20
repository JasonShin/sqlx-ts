use mysql;
use mysql::prelude::Queryable;
use postgres;
use std::cell::RefCell;
use std::collections::HashMap;

use super::types::{DBConn, TsFieldType};

#[derive(Debug, Clone, Copy)]
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
    /// tables cache
    tables_cache: HashMap<String, Fields>,
}

impl DBSchema {
    pub fn new() -> DBSchema {
        DBSchema {
            tables_cache: HashMap::new(),
        }
    }

    pub fn fetch_table(&self, database_name: &str, table_name: &Vec<&str>, conn: &DBConn) -> Option<Fields> {
        match &conn {
            DBConn::MySQLPooledConn(conn) => Self::mysql_fetch_table(&self, &database_name, &table_name, conn),
            DBConn::PostgresConn(conn) => Self::postgres_fetch_table(&self, &database_name, &table_name, conn),
        }
    }

    fn postgres_fetch_table(
        &self,
        database_name: &str,
        table_names: &Vec<&str>,
        conn: &RefCell<&mut postgres::Client>,
    ) -> Option<Fields> {
        let table_names = table_names
            .into_iter()
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
        WHERE TABLE_SCHEMA = '{}'
        AND TABLE_NAME IN ({})
                ",
            database_name, table_names,
        );

        let mut fields: HashMap<String, Field> = HashMap::new();
        let result = conn.borrow_mut().query(&query, &[]);

        if let Ok(result) = result {
            for row in result {
                let field_name: String = row.get(0);
                let field_type: String = row.get(1);
                let is_nullable: String = row.get(2);
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

    fn mysql_fetch_table(
        &self,
        database_name: &str,
        table_names: &Vec<&str>,
        conn: &RefCell<&mut mysql::Conn>,
    ) -> Option<Fields> {
        let table_names = table_names
            .into_iter()
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
        WHERE TABLE_SCHEMA = '{}'
        AND TABLE_NAME IN ({})
                ",
            database_name, table_names
        );

        let mut fields: HashMap<String, Field> = HashMap::new();
        let result = conn.borrow_mut().query::<mysql::Row, String>(query);

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
