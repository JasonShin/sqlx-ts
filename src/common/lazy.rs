use crate::common::cli::Cli;
use crate::common::config::Config;
use crate::common::types::DatabaseType;
use crate::core::connection::{DBConnections, DBConn};
use crate::ts_generator::information_schema::DBSchema;
use clap::Parser;
use lazy_static::lazy_static;
use mysql::Conn as MySQLConn;
use std::sync::{Mutex, Arc};

// The file contains all implicitly dependent variables or state that files need for the logic
// We have a lot of states that we need to drill down into each methods
lazy_static! {
    pub static ref SOME_INT: i32 = 5;

    pub static ref CLI_ARGS: Cli = Cli::parse();
    pub static ref CONFIG: Config =  Config::new();

    // This is a holder for shared DBSChema used to fetch information for information_schema table
    // By having a singleton, we can think about caching the result if we are fetching a query too many times
    pub static ref DB_SCHEMA: Mutex<DBSchema> = Mutex::new(DBSchema::new());

    pub static ref DB_CONNECTIONS: Mutex<DBConnections<'static>> = {
        let db_connections = DBConnections::new();
        for connection in CONFIG.connections.keys() {
            let connection_config = CONFIG.connections.get(connection).unwrap();
            let db_type = connection_config.db_type.to_owned();
            match db_type {
                DatabaseType::Mysql => {
                    let opts = CONFIG.get_mysql_cred(&connection_config);
                    let mut conn = MySQLConn::new(opts).unwrap();
                    let conn = DBConn::MySQLPooledConn(&Mutex::new(conn));
                    db_connections.add_connection(connection.to_owned(), Arc::new(Mutex::new(conn)));
                }
                DatabaseType::Postgres => {
                    let db_conn = DBConn::PostgresConn(postgres::Conn::new(
                        connection_config.to_owned(),
                    ));
                    db_connections.add_connection(connection.to_owned(), Arc::new(Mutex::new(db_conn)));
                }
            };
        }
        Mutex::new(db_connections)
    };
}
