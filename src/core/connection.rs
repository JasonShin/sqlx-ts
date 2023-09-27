use crate::common::types::DatabaseType;
use crate::common::lazy::CONFIG;
use std::{cell::RefCell, sync::Mutex, rc::Rc};
use std::collections::{hash_map::Entry, HashMap};

use postgres::{Client as PGClient, NoTls as PGNoTls};
use mysql::Conn as MySQLConn;
use postgres::Client as PostgresConn;

/// Enum to hold a specific database connection instance
pub enum DBConn<'a> {
    // TODO: Maybe we can also pass down db_name through DBConn
    MySQLPooledConn(&'a mut MySQLConn),
    PostgresConn(&'a mut PostgresConn),
}

pub struct DBConnections<'a> {
    conns: HashMap<String, DBConn<'a>>,
}

// TODO: Add tests around failed connection
impl<'a> DBConnections<'a> {
    pub fn new() -> Self {
        Self {
            conns: HashMap::new(),
        }
    }

    /// Returns a connection from the DBConnection map
    /// It also lazily creates a connection if it doesn't exist
    pub fn get_connection(&mut self, raw_sql: &str) -> DBConn {
        let (db_conn_name, db_conn_config) = &CONFIG.get_correct_db_connection(raw_sql);

        if let Some(conn) = self.conns.get(db_conn_name) {
            return *conn;
        }

        let conn = match &db_conn_config.db_type {
            DatabaseType::Postgres => {
                let postgres_cred = &CONFIG.get_postgres_cred(db_conn_config);
                DBConn::PostgresConn(&mut PGClient::connect(postgres_cred, PGNoTls).unwrap())
            },
            DatabaseType::Mysql => {
                let opts = CONFIG.get_mysql_cred(&db_conn_config);
                let mut conn = MySQLConn::new(opts).unwrap();
                DBConn::MySQLPooledConn(&mut conn)
            },
        };

        let conn = match self.conns.entry(db_conn_name.clone()) {
            Entry::Occupied(mut entry) => std::mem::replace(entry.get_mut(), conn),
            Entry::Vacant(entry) => {
                entry.insert(conn);
                conn
            },
        };

        conn
    }
}
