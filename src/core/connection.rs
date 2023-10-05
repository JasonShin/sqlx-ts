use crate::common::lazy::CONFIG;
use crate::common::SQL;
use crate::core::mysql::prepare as mysql_explain;
use crate::core::postgres::prepare as postgres_explain;
use crate::ts_generator::types::ts_query::TsQuery;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use color_eyre::Result;
use mysql::Conn as MySQLConn;
use postgres::Client as PostgresConn;
use sqlx::{Pool, MySql, Postgres};
use swc_common::errors::Handler;

/// Enum to hold a specific database connection instance
pub enum DBConn {
    MySQLPooledConn(Mutex<Pool<MySql>>),
    PostgresConn(Mutex<Pool<Postgres>>),
}

impl DBConn {
    pub fn prepare(
        &self,
        sql: &SQL,
        should_generate_types: &bool,
        handler: &Handler,
    ) -> Result<(bool, Option<TsQuery>)> {
        let (explain_failed, ts_query) = match &self {
            DBConn::MySQLPooledConn(_conn) => mysql_explain::prepare(&self, sql, should_generate_types, handler)?,
            DBConn::PostgresConn(_conn) => postgres_explain::prepare(&self, sql, should_generate_types, handler)?,
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
