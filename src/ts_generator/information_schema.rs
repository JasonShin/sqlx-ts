use crate::common::errors::{DB_CONN_POOL_RETRIEVE_ERROR, DB_SCHEME_READ_ERROR};
use crate::common::logger::*;
use crate::core::connection::DBConn;
use crate::core::mysql::pool::MySqlConnectionManager;
use crate::core::postgres::pool::PostgresConnectionManager;
use bb8::Pool;
use mysql_async::prelude::Queryable;
use std::collections::HashMap;
use tokio::sync::Mutex;

use super::types::ts_query::TsFieldType;

#[derive(Debug, Clone)]
pub struct Field {
  pub field_type: TsFieldType,
  pub is_nullable: bool,
}

pub type Fields = HashMap<String, Field>;

pub struct DBSchema {
  // Holds cache details for table / columns of the target database
  tables_cache: HashMap<String, Fields>,
}

impl Default for DBSchema {
  fn default() -> Self {
    Self::new()
  }
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
      DBConn::PostgresConn(conn) => Self::postgres_fetch_table(self, &"public".to_string(), table_name, conn).await,
    };

    if let Some(result) = &result {
      let _ = &self.tables_cache.insert(table_key, result.clone());
    }

    result
  }

  async fn postgres_fetch_table(
    &self,
    schema: &String,
    table_names: &Vec<&str>,
    conn: &Mutex<Pool<PostgresConnectionManager>>,
  ) -> Option<Fields> {
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
          IS_NULLABLE as is_nulalble,
          TABLE_NAME as table_name,
          (
            select string_agg(e.enumlabel, ',')
          from pg_type t
              join pg_enum e on t.oid = e.enumtypid
              join pg_catalog.pg_namespace n ON n.oid = t.typnamespace
          where n.nspname = '{}'
          and t.typname = udt_name
          group by n.nspname, t.typname
          ) as enum_values
      FROM information_schema.COLUMNS
      WHERE TABLE_SCHEMA = '{}'
      AND TABLE_NAME IN ({});
                ",
      schema, schema, table_names,
    );

    let mut fields: HashMap<String, Field> = HashMap::new();

    let conn = conn.lock().await;
    let conn = conn.get().await.expect(DB_CONN_POOL_RETRIEVE_ERROR);
    let result = conn.query(&query, &[]).await;

    if let Ok(result) = result {
      for row in result {
        let field_name: String = row.get(0);
        let field_type: String = row.get(1);
        let is_nullable: String = row.get(2);
        let table_name: String = row.get(3);
        let enum_values: Option<Vec<String>> = row
          .try_get(4)
          .ok()
          .map(|val: String| val.split(",").map(|x| x.to_string()).collect());

        let field = Field {
          field_type: TsFieldType::get_ts_field_type_from_postgres_field_type(
            field_type.to_owned(),
            field_name.to_owned(),
            table_name,
            enum_values,
          ),
          is_nullable: is_nullable == "YES",
        };
        if field.field_type == TsFieldType::Any {
          let message = format!(
            "The column {field_name} of type {field_type} will be translated any as it isn't supported by sqlx-ts"
          );
          info!(message);
        }
        fields.insert(field_name.to_owned(), field);
      }

      return Some(fields);
    }

    None
  }

  async fn mysql_fetch_table(
    &self,
    table_names: &Vec<&str>,
    conn: &Mutex<Pool<MySqlConnectionManager>>,
  ) -> Option<Fields> {
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
            IS_NULLABLE as is_nulalble,
            TABLE_NAME,
            (
              SELECT REPLACE(
                  TRIM(TRAILING ')' FROM
                  TRIM(LEADING '(' from
                  TRIM(LEADING 'enum' FROM COLUMN_TYPE)))
                , '\''
                , ''
              )
              FROM information_schema.COLUMNS subcols
              WHERE subcols.TABLE_SCHEMA = (SELECT DATABASE())
                AND subcols.TABLE_NAME = C.TABLE_NAME
                AND subcols.COLUMN_NAME = C.COLUMN_NAME
            ) AS enums
        FROM information_schema.COLUMNS C
        WHERE TABLE_SCHEMA = (SELECT DATABASE())
        AND TABLE_NAME IN ({})
                ",
      table_names
    );

    let mut fields: HashMap<String, Field> = HashMap::new();
    let conn = conn.lock().await;
    let mut conn = conn.get().await.expect(DB_CONN_POOL_RETRIEVE_ERROR);
    let result = conn.query::<mysql_async::Row, String>(query).await;

    if let Ok(result) = result {
      for row in result {
        let field_name: String = row.clone().take(0).expect(DB_SCHEME_READ_ERROR);
        let field_type: String = row.clone().take(1).expect(DB_SCHEME_READ_ERROR);
        let is_nullable: String = row.clone().take(2).expect(DB_SCHEME_READ_ERROR);
        let table_name: String = row.clone().take(3).expect(DB_SCHEME_READ_ERROR);

        let enum_values: Option<Vec<String>> = if field_type == "enum" {
          let enums: String = row.clone().take(4).expect(DB_SCHEME_READ_ERROR);
          let enum_values: Vec<String> = enums.split(",").map(|x| x.to_string()).collect();
          Some(enum_values)
        } else {
          None
        };
        let field = Field {
          field_type: TsFieldType::get_ts_field_type_from_mysql_field_type(
            field_type.to_owned(),
            table_name.to_owned(),
            field_name.to_owned(),
            enum_values.to_owned(),
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
