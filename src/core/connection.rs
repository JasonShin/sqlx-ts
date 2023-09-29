use crate::common::lazy::CONFIG;
use crate::common::types::DatabaseType;
use std::collections::{hash_map::Entry, HashMap};
use std::sync::{Arc, RwLock};
use std::{cell::RefCell, rc::Rc, sync::Mutex};

use mysql::Conn as MySQLConn;
use postgres::Client as PostgresConn;
use postgres::{Client as PGClient, NoTls as PGNoTls};

/// Enum to hold a specific database connection instance
pub enum DBConn {
    // TODO: Maybe we can also pass down db_name through DBConn
    MySQLPooledConn(Mutex<MySQLConn>),
    PostgresConn(Mutex<PostgresConn>),
}

pub struct DBConnections {
    conns: HashMap<String, Arc<DBConn>>,
}

// TODO: Add tests around failed connection
impl<'a> DBConnections {
    pub fn new() -> Self {
        Self { conns: HashMap::new() }
    }

    /// Returns a connection from the DBConnection map
    /// It also lazily creates a connection if it doesn't exist
    pub fn get_connection(&mut self, raw_sql: &str) -> Arc<DBConn> {
        let (db_conn_name, db_conn_config) = &CONFIG.get_correct_db_connection(raw_sql);

        if let Some(conn) = self.conns.get(db_conn_name) {
            return conn.clone();
        }

        let conn = match &db_conn_config.db_type {
            DatabaseType::Postgres => {
                let postgres_cred = &CONFIG.get_postgres_cred(db_conn_config);
                DBConn::PostgresConn(Mutex::new(PGClient::connect(postgres_cred, PGNoTls).unwrap()))
            }
            DatabaseType::Mysql => {
                let opts = CONFIG.get_mysql_cred(&db_conn_config);
                let mut conn = MySQLConn::new(opts).unwrap();
                DBConn::MySQLPooledConn(Mutex::new(conn))
            }
        };

        let _ = &self.conns.insert(db_conn_name.to_owned(), Arc::new(conn));
        let conn = self.conns.get(db_conn_name).unwrap();
        conn.clone()
    }
}
