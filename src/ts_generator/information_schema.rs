use mysql;
use mysql::prelude::Queryable;
use postgres;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::common::lazy::CONFIG;

use super::types::db_conn::DBConn;
use super::types::ts_query::TsFieldType;

#[derive(Debug, Clone)]
pub struct Field {
    pub field_type: TsFieldType,
    pub is_nullable: bool,
    // if enum_name existing when processing FIELD, we should use enum_type instead of field_type
    pub enum_type: Option<String>,
}

pub type Fields = HashMap<String, Field>;

pub type Enums = HashMap<String, Vec<String>>;

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

    /// fetch table's column details from information_schema of each database type
    ///
    /// # MySQL Notes
    /// - TABLE_SCHEMA in MySQL is basically the `database_name`, so it requires passing in database name as an argument
    ///
    /// # PostgreSQL Notes
    /// - TABLE_SCHEMA is PostgreSQL is basically 'public' by default. `database_name` is the name of the database itself
    pub fn fetch_table(&self, table_name: &Vec<&str>, conn: &DBConn) -> Option<Fields> {
        match &conn {
            DBConn::MySQLPooledConn(conn) => Self::mysql_fetch_table(self, table_name, conn),
            DBConn::PostgresConn(conn) => Self::postgres_fetch_table(self, table_name, conn),
        }
    }

    fn postgres_fetch_table(&self, table_names: &Vec<&str>, conn: &RefCell<&mut postgres::Client>) -> Option<Fields> {
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
        let result = conn.borrow_mut().query(&query, &[]);

        if let Ok(result) = result {
            for row in result {
                let field_name: String = row.get(0);
                let field_type: String = row.get(1);
                let is_nullable: String = row.get(2);
                let field = Field {
                    field_type: TsFieldType::get_ts_field_type_from_postgres_field_type(field_type.to_owned()),
                    is_nullable: is_nullable == "YES",
                    enum_type: if field_type == "USER DEFINED" {
                        Some(field_name.clone())
                    } else {
                        None
                    },
                };
                fields.insert(field_name.to_owned(), field);
            }

            return Some(fields);
        }

        None
    }

    fn mysql_fetch_table(&self, table_names: &Vec<&str>, conn: &RefCell<&mut mysql::Conn>) -> Option<Fields> {
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
        let result = conn.borrow_mut().query::<mysql::Row, String>(query);

        if let Ok(result) = result {
            for row in result {
                let field_name: String = row.clone().take(0).unwrap();
                let field_type: String = row.clone().take(1).unwrap();
                let is_nullable: String = row.clone().take(2).unwrap();
                let field = Field {
                    field_type: TsFieldType::get_ts_field_type_from_mysql_field_type(field_type.to_owned()),
                    is_nullable: is_nullable == "YES",
                    enum_type: if field_type == "enum" {
                        Some(field_name.clone())
                    } else {
                        None
                    },
                };
                fields.insert(field_name.to_owned(), field);
            }

            return Some(fields);
        }

        None
    }

    pub fn fetch_enums(&self, db_name: &str, conn: &DBConn) -> Option<Enums> {
        match &conn {
            DBConn::MySQLPooledConn(conn) => Self::mysql_fetch_enums(self, db_name, conn),
            DBConn::PostgresConn(_) => None,
        }
    }

    fn mysql_fetch_enums(&self, db_name: &str, conn: &RefCell<&mut mysql::Conn>) -> Option<Enums> {
        let query = format!(
            r"
SELECT col.column_name,
       col.data_type,
       trim(LEADING 'enum' FROM col.column_type) AS enum_values
FROM information_schema.columns col
JOIN information_schema.tables tab ON tab.table_schema = col.table_schema
                                   AND tab.table_name = col.table_name
                                   AND tab.table_type = 'BASE TABLE'
WHERE col.data_type IN ('enum')
      AND col.table_schema NOT IN ('information_schema', 'sys',
                                   'performance_schema', 'mysql')
     AND col.table_schema = '{}'
ORDER BY col.table_schema,
         col.table_name,
         col.ordinal_position;
        ",
            db_name,
        );
        let result = conn.borrow_mut().query::<mysql::Row, String>(query);

        if let Ok(rows) = result {
            let mut enums_result: Enums = HashMap::new();

            for row in &rows {
                let data_type: String = row.clone().take(1).unwrap();
                if data_type != "enum" {
                    continue;
                }

                let column_name: String = row.clone().take(0).unwrap();
                let enum_values: String = row.clone().take(2).unwrap();
                let enum_values = enum_values
                    // remove curly braces
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|x| x.replace("'", ""))
                    .collect::<Vec<_>>();
                println!("enum name and values {:?} : {:?}", column_name, enum_values);
                enums_result.insert(column_name, enum_values);
            }

            return Some(enums_result.clone());
        }

        None
    }
}
