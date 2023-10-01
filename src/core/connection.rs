use crate::common::SQL;
use crate::common::lazy::CONFIG;
use crate::common::types::DatabaseType;
use crate::ts_generator::types::ts_query::TsQuery;
use crate::core::mysql::prepare as mysql_explain;
use crate::core::postgres::prepare as postgres_explain;
use std::collections::{hash_map::Entry, HashMap};
use std::sync::{Arc, RwLock};
use std::{cell::RefCell, rc::Rc, sync::Mutex};

use mysql::Conn as MySQLConn;
use postgres::Client as PostgresConn;
use postgres::{Client as PGClient, NoTls as PGNoTls};
use swc_common::errors::Handler;
use color_eyre::Result;

/// Enum to hold a specific database connection instance
pub enum DBConn<'a> {
    // TODO: Maybe we can also pass down db_name through DBConn
    MySQLPooledConn(&'a Mutex<MySQLConn>),
    PostgresConn(&'a Mutex<PostgresConn>),
}

impl<'a> DBConn<'a> {
    pub fn prepare(&self, sql: &SQL, should_generate_types: &bool, handler: &Handler) -> Result<(bool, Option<TsQuery>)> {
        let (explain_failed, ts_query) = match &self {
            DBConn::MySQLPooledConn(conn) => {
                mysql_explain::prepare(DBConn::MySQLPooledConn(conn), sql, should_generate_types, handler)?
            }
            DBConn::PostgresConn(conn) => {
                postgres_explain::prepare(DBConn::PostgresConn(conn), sql, should_generate_types, handler)?
            }
        };

        Ok((explain_failed, ts_query))
    }
}

pub struct DBConnections<'a> {
    pub conns: HashMap<String, Arc<Mutex<DBConn<'a>>>>,
}

// TODO: Add tests around failed connection
impl<'a> DBConnections<'a> {
    pub fn new() -> Self {
        Self { conns: HashMap::new() }
    }

    pub fn add_connection(&'static mut self, name: String, conn: Arc<Mutex<DBConn<'a>>>) {
        let _ = self.conns.insert(name, conn);
    }

    /// Returns a connection from the DBConnection map
    /// It also lazily creates a connection if it doesn't exist
    pub fn get_connection(&mut self, raw_sql: &str) -> Arc<Mutex<DBConn>> {
        let (db_conn_name, db_conn_config) = &CONFIG.get_correct_db_connection(raw_sql);

        if let Some(conn) = self.conns.get(db_conn_name) {
            return conn.clone();
        }

        let _ = match &db_conn_config.db_type {
            DatabaseType::Postgres => {
                let postgres_cred = &CONFIG.get_postgres_cred(db_conn_config);
                let conn = DBConn::PostgresConn(Mutex::new(PGClient::connect(postgres_cred, PGNoTls).unwrap()));
                let _ = &self.conns.insert(db_conn_name.to_owned(), Arc::new(Mutex::new(conn)));
            }
            DatabaseType::Mysql => {
                let opts = CONFIG.get_mysql_cred(&db_conn_config);
                let mut conn = MySQLConn::new(opts).unwrap();
                let conn = DBConn::MySQLPooledConn(Mutex::new(conn));
                let _ = &self.conns.insert(db_conn_name.to_owned(), Arc::new(Mutex::new(conn)));
            }
        };

        let conn = self.conns.get(db_conn_name).unwrap();
        conn.to_owned()
    }
}
