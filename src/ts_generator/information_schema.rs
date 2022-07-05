use mysql::Row;
use mysql::*;
use mysql::{prelude::Queryable, Conn};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;

use super::types::TsFieldType;

#[derive(Debug, Clone, Copy)]
pub struct Field {
    field_type: TsFieldType,
    is_nullable: bool,
}

pub type Fields = HashMap<String, Field>;

#[derive(Clone, PartialEq)]
struct ColumnsQueryResultRow {
    column_name: String,
    data_type: String,
    is_nullable: bool,
}

pub trait DBSchema {
    fn new() -> Self;
    fn fetch_table(
        &self,
        database_name: &String,
        table_name: &String,
        conn: &RefCell<&mut Conn>,
    ) -> Option<Fields>;
    // fn fetch_field(&self, database_name: String, table_name: String, field_name: String) -> Field;
}

pub struct MySQLSchema {
    pub tables: HashMap<String, Fields>,
}

impl DBSchema for MySQLSchema {
    fn new() -> MySQLSchema {
        MySQLSchema {
            tables: HashMap::new(),
        }
    }

    fn fetch_table(
        &self,
        database_name: &String,
        table_name: &String,
        conn: &RefCell<&mut Conn>,
    ) -> Option<Fields> {
        let table = self.tables.get(table_name.as_str());

        match table {
            Some(fields) => Some(fields.clone()),
            None => {
                let query = format!(
                    r"
                SELECT
                    COLUMN_NAME as column_name,
                    DATA_TYPE as data_type,
                    IS_NULLABLE as is_nulalble
                FROM information_schema.COLUMNS
                WHERE TABLE_SCHEMA = '{}'
                AND TABLE_NAME = '{}'
                        ",
                    database_name, table_name
                );

                let mut fields: HashMap<String, Field> = HashMap::new();
                let result = conn.borrow_mut().query::<Row, String>(query);

                if let Ok(result) = result {
                    for row in result {
                        let field_name: String = row.clone().take(0).unwrap();
                        let field_type: String = row.clone().take(1).unwrap();
                        let is_nullable: String = row.clone().take(2).unwrap();
                        let field = Field {
                            field_type: TsFieldType::get_ts_field_type_from_mysql_field_type(
                                field_type.to_owned(),
                            ),
                            is_nullable: is_nullable == "YES",
                        };
                        fields.insert(field_name.to_owned(), field);
                    }

                    return Some(fields);
                }

                None
            }
        }
    }
}

pub struct PostgresSchema {
    pub tables: HashMap<String, Fields>,
}
