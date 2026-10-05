use crate::common::lazy::CONFIG;
use crate::common::types::DatabaseType;
use crate::common::SQL;
use crate::core::mysql::prepare as mysql_explain;
use crate::core::postgres::prepare as postgres_explain;
use crate::core::sqlite::prepare as sqlite_explain;
use crate::ts_generator::types::ts_query::TsQuery;
use bb8::Pool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::mysql::pool::MySqlConnectionManager;
use super::postgres::pool::PostgresConnectionManager;
use super::sqlite::pool::SqliteConnectionManager;
use crate::common::errors::DB_CONN_FROM_LOCAL_CACHE_ERROR;
use color_eyre::Result;
use swc_common::errors::Handler;

/// Enum to hold a specific database connection instance
#[allow(clippy::enum_variant_names)]
pub enum DBConn {
  MySQLPooledConn(Mutex<Pool<MySqlConnectionManager>>, String),
  PostgresConn(Mutex<Pool<PostgresConnectionManager>>, String),
  SqliteConn(Mutex<Pool<SqliteConnectionManager>>, String),
}

impl DBConn {
  pub async fn prepare(
    &self,
    sql: &SQL,
    should_generate_types: &bool,
    handler: &Handler,
  ) -> Result<(bool, Option<TsQuery>)> {
    let (explain_failed, ts_query) = match &self {
      DBConn::MySQLPooledConn(..) => mysql_explain::prepare(self, sql, should_generate_types, handler).await?,
      DBConn::PostgresConn(..) => postgres_explain::prepare(self, sql, should_generate_types, handler).await?,
      DBConn::SqliteConn(..) => sqlite_explain::prepare(self, sql, should_generate_types, handler).await?,
    };

    Ok((explain_failed, ts_query))
  }

  /// Get the database type for this connection
  pub fn get_db_type(&self) -> DatabaseType {
    match self {
      DBConn::MySQLPooledConn(..) => DatabaseType::Mysql,
      DBConn::PostgresConn(..) => DatabaseType::Postgres,
      DBConn::SqliteConn(..) => DatabaseType::Sqlite,
    }
  }

  pub fn get_connection_name(&self) -> &str {
    match self {
      DBConn::MySQLPooledConn(_, name) | DBConn::PostgresConn(_, name) | DBConn::SqliteConn(_, name) => name,
    }
  }
}

pub struct DBConnections<'a> {
  pub cache: &'a HashMap<String, Arc<Mutex<DBConn>>>,
}

impl<'a> DBConnections<'a> {
  pub fn new(cache: &'a HashMap<String, Arc<Mutex<DBConn>>>) -> Self {
    Self { cache }
  }

  pub fn get_connection(&mut self, raw_sql: &str) -> Arc<Mutex<DBConn>> {
    let db_conn_name = &CONFIG.get_correct_db_connection(raw_sql);

    let conn = self.cache.get(db_conn_name).expect(DB_CONN_FROM_LOCAL_CACHE_ERROR);
    conn.to_owned()
  }
}
