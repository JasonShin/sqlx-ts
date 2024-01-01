use crate::common::lazy::CONFIG;
use crate::common::SQL;
use crate::core::mysql::prepare as mysql_explain;
use crate::core::postgres::prepare as postgres_explain;
use crate::ts_generator::types::ts_query::TsQuery;
use std::collections::HashMap;
use std::sync::Arc;
use bb8::Pool;
use tokio::sync::Mutex;

use color_eyre::Result;
use swc_common::errors::Handler;

use super::postgres::pool::PostgresConnectionManager;
use super::mysql::pool::MySqlConnectionManager;

/// Enum to hold a specific database connection instance
pub enum DBConn {
    MySQLPooledConn(Mutex<Pool<MySqlConnectionManager>>),
    PostgresConn(Mutex<Pool<PostgresConnectionManager>>),
}

impl DBConn {
    pub async fn prepare(
        &self,
        sql: &SQL,
        should_generate_types: &bool,
        handler: &Handler,
    ) -> Result<(bool, Option<TsQuery>)> {
        let (explain_failed, ts_query) = match &self {
            DBConn::MySQLPooledConn(_conn) => mysql_explain::prepare(self, sql, should_generate_types, handler).await?,
            DBConn::PostgresConn(_conn) => postgres_explain::prepare(self, sql, should_generate_types, handler).await?,
        };

        Ok((explain_failed, ts_query))
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

        let conn = self
            .cache
            .get(db_conn_name)
            .expect("Failed to get the connection from cache");
        conn.to_owned()
    }
}
