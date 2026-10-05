use crate::common::config::{CustomTypeMapping, DbConnectionConfig};
use crate::common::errors::{DB_CONN_POOL_RETRIEVE_ERROR, DB_SCHEME_READ_ERROR};
use crate::common::lazy::CONFIG;
use crate::common::logger::*;
use crate::core::connection::DBConn;
use crate::core::mysql::pool::MySqlConnectionManager;
use crate::core::postgres::pool::PostgresConnectionManager;
use crate::core::sqlite::pool::SqliteConnectionManager;
use crate::ts_generator::sql_parser::quoted_strings::{DisplayIndent, DisplayObjectName};
use bb8::Pool;
use mysql_async::prelude::Queryable;
use sqlparser::ast::{visit_relations, Statement};
use std::collections::HashMap;
use std::future::Future;
use std::ops::ControlFlow;
use tokio::sync::Mutex;

use super::types::ts_query::TsFieldType;

#[derive(Debug, Clone)]
pub struct Field {
  pub field_type: TsFieldType,
  pub is_nullable: bool,
}

pub type Fields = HashMap<String, Field>;

fn resolve_field_type(
  conn_config: Option<&DbConnectionConfig>,
  db_types: &[&str],
  default_type: impl FnOnce() -> TsFieldType,
) -> TsFieldType {
  match conn_config.and_then(|x| x.find_type_mapping(db_types)) {
    Some(CustomTypeMapping::Simple(type_name)) => TsFieldType::Custom {
      type_name: type_name.to_owned(),
      import: None,
    },
    Some(CustomTypeMapping::WithImport { type_name, import }) => TsFieldType::Custom {
      type_name: type_name.to_owned(),
      import: Some(import.to_owned()),
    },
    None => default_type(),
  }
}

tokio::task_local! {
  // Maps table names to the schema (or the database in MySQL) they were qualified with in the query being translated
  static TABLE_SCHEMAS: HashMap<String, String>;
}

/// Collects schemas of qualified table names, e.g. `staff.announcements` -> ("announcements", "staff")
/// For `database.schema.table` names, the schema is used
pub fn collect_table_schemas(statements: &Vec<Statement>) -> HashMap<String, String> {
  let mut table_schemas = HashMap::new();
  let _ = visit_relations(statements, |relation| {
    let parts = &relation.0;
    if parts.len() >= 2 {
      if let Some(schema) = parts[parts.len() - 2].as_ident() {
        table_schemas.insert(
          DisplayObjectName(relation).to_string(),
          DisplayIndent(schema).to_string(),
        );
      }
    }
    ControlFlow::<()>::Continue(())
  });
  table_schemas
}

/// Runs the translation of a query with the schemas of its qualified table names
pub async fn with_table_schemas<F: Future>(table_schemas: HashMap<String, String>, f: F) -> F::Output {
  TABLE_SCHEMAS.scope(table_schemas, f).await
}

fn get_table_schema(table_name: &str) -> Option<String> {
  TABLE_SCHEMAS
    .try_with(|table_schemas| table_schemas.get(table_name).cloned())
    .ok()
    .flatten()
}

fn quote_literal(value: &str) -> String {
  format!("'{}'", value.replace('\'', "''"))
}

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
    let connection_name = conn.get_connection_name();
    let conn_config = CONFIG.connections.get(connection_name);
    let tables: Vec<(Option<String>, String)> = table_name
      .iter()
      .map(|table_name| (get_table_schema(table_name), table_name.to_string()))
      .collect();
    let table_key: String = format!(
      "{connection_name}:{}",
      tables
        .iter()
        .map(|(schema, table_name)| match schema {
          Some(schema) => format!("{schema}.{table_name}"),
          None => table_name.to_owned(),
        })
        .collect::<Vec<_>>()
        .join(",")
    );
    let cached_table_result = self.tables_cache.get(table_key.as_str());

    if let Some(cached_table_result) = cached_table_result {
      return Some(cached_table_result.clone());
    }

    let result = match &conn {
      DBConn::MySQLPooledConn(conn, _) => Self::mysql_fetch_table(self, &tables, conn, conn_config).await,
      DBConn::PostgresConn(conn, _) => Self::postgres_fetch_table(self, &tables, conn, conn_config).await,
      DBConn::SqliteConn(conn, _) => Self::sqlite_fetch_table(self, &tables, conn, conn_config).await,
    };

    if let Some(result) = &result {
      let _ = &self.tables_cache.insert(table_key, result.clone());
    }

    result
  }

  async fn postgres_fetch_table(
    &self,
    tables: &[(Option<String>, String)],
    conn: &Mutex<Pool<PostgresConnectionManager>>,
    conn_config: Option<&DbConnectionConfig>,
  ) -> Option<Fields> {
    // Tables without a schema are looked up in the public schema
    let table_conditions = tables
      .iter()
      .map(|(schema, table_name)| {
        let schema = schema.as_deref().unwrap_or("public");
        format!(
          "(TABLE_SCHEMA = {} AND TABLE_NAME = {})",
          quote_literal(schema),
          quote_literal(table_name)
        )
      })
      .collect::<Vec<_>>()
      .join(" OR ");

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
          where n.nspname = udt_schema
          and t.typname = udt_name
          group by n.nspname, t.typname
          ) as enum_values,
          UDT_NAME as udt_name
      FROM information_schema.COLUMNS
      WHERE {table_conditions};
                "
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
        let udt_name: String = row.try_get(5).unwrap_or_default();

        let field = Field {
          field_type: resolve_field_type(conn_config, &[&field_type, &udt_name], || {
            TsFieldType::get_ts_field_type_from_postgres_field_type(
              field_type.to_owned(),
              field_name.to_owned(),
              table_name,
              enum_values,
            )
          }),
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
    tables: &[(Option<String>, String)],
    conn: &Mutex<Pool<MySqlConnectionManager>>,
    conn_config: Option<&DbConnectionConfig>,
  ) -> Option<Fields> {
    // Tables without a database are looked up in the database of the connection
    let table_conditions = tables
      .iter()
      .map(|(database, table_name)| {
        let database = database
          .as_deref()
          .map(quote_literal)
          .unwrap_or_else(|| "(SELECT DATABASE())".to_string());
        format!(
          "(TABLE_SCHEMA = {database} AND TABLE_NAME = {})",
          quote_literal(table_name)
        )
      })
      .collect::<Vec<_>>()
      .join(" OR ");
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
              WHERE subcols.TABLE_SCHEMA = C.TABLE_SCHEMA
                AND subcols.TABLE_NAME = C.TABLE_NAME
                AND subcols.COLUMN_NAME = C.COLUMN_NAME
            ) AS enums,
            COLUMN_TYPE as column_type
        FROM information_schema.COLUMNS C
        WHERE {table_conditions}
                "
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
        let column_type: String = row.clone().take(5).unwrap_or_default();

        let enum_values: Option<Vec<String>> = if field_type == "enum" {
          let enums: String = row.clone().take(4).expect(DB_SCHEME_READ_ERROR);
          let enum_values: Vec<String> = enums.split(",").map(|x| x.to_string()).collect();
          Some(enum_values)
        } else {
          None
        };
        let field = Field {
          field_type: resolve_field_type(conn_config, &[&column_type, &field_type], || {
            TsFieldType::get_ts_field_type_from_mysql_field_type(
              field_type.to_owned(),
              table_name.to_owned(),
              field_name.to_owned(),
              enum_values.to_owned(),
            )
          }),
          is_nullable: is_nullable == "YES",
        };
        fields.insert(field_name.to_owned(), field);
      }

      return Some(fields);
    }

    None
  }

  async fn sqlite_fetch_table(
    &self,
    tables: &[(Option<String>, String)],
    conn: &Mutex<Pool<SqliteConnectionManager>>,
    conn_config: Option<&'static DbConnectionConfig>,
  ) -> Option<Fields> {
    let mut fields: HashMap<String, Field> = HashMap::new();
    let conn = conn.lock().await;
    let pool_conn = conn.get().await.expect(DB_CONN_POOL_RETRIEVE_ERROR);
    let inner = pool_conn.conn.clone();

    let tables = tables.to_vec();

    let result = tokio::task::spawn_blocking(move || {
      let conn = inner.lock().unwrap();
      let mut all_fields: HashMap<String, Field> = HashMap::new();

      for (schema, table_name) in &tables {
        // Tables qualified with an attached database are looked up in that database
        let query = match schema {
          Some(schema) => format!(
            "PRAGMA \"{}\".table_info({})",
            schema.replace('"', "\"\""),
            quote_literal(table_name)
          ),
          None => format!("PRAGMA table_info({})", quote_literal(table_name)),
        };
        let mut stmt = match conn.prepare(&query) {
          Ok(stmt) => stmt,
          Err(_) => continue,
        };

        let rows = match stmt.query_map([], |row| {
          let name: String = row.get(1)?;
          let type_name: String = row.get(2)?;
          let notnull: bool = row.get(3)?;
          let pk: i32 = row.get(5)?;
          Ok((name, type_name, notnull, pk, table_name.clone()))
        }) {
          Ok(rows) => rows,
          Err(_) => continue,
        };

        for (field_name, field_type, notnull, pk, tbl_name) in rows.flatten() {
          let field = Field {
            field_type: resolve_field_type(conn_config, &[&field_type], || {
              TsFieldType::get_ts_field_type_from_sqlite_field_type(field_type.to_owned(), tbl_name, field_name.clone())
            }),
            is_nullable: !notnull && pk == 0,
          };
          all_fields.insert(field_name, field);
        }
      }

      all_fields
    })
    .await;

    if let Ok(result) = result {
      if result.is_empty() {
        return None;
      }
      fields.extend(result);
      return Some(fields);
    }

    None
  }
}
