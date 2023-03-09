use std::cell::RefCell;

use mysql::Conn as MySQLConn;
use postgres::Client as PostgresConn;

/// Enum to hold a specific database connection instance
pub enum DBConn<'a> {
    // TODO: Maybe we can also pass down db_name through DBConn
    MySQLPooledConn(&'a mut RefCell<&'a mut MySQLConn>),
    PostgresConn(&'a mut RefCell<&'a mut PostgresConn>),
}
