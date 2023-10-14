use crate::common::cli::Cli;
use crate::common::config::Config;
use crate::common::types::DatabaseType;
use crate::core::connection::{DBConn, DBConnections};
use crate::ts_generator::information_schema::DBSchema;
use clap::Parser;
use lazy_static::lazy_static;
use sqlx::{mysql, postgres};
use tokio;
use tokio::runtime::Runtime;
use tokio::task::LocalSet;
// use mysql::Conn as MySQLConn;
// use postgres::{Client as PGClient, NoTls as PGNoTls};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// The file contains all implicitly dependent variables or state that files need for the logic
// We have a lot of states that we need to drill down into each methods
lazy_static! {
    pub static ref CLI_ARGS: Cli = Cli::parse();
    pub static ref CONFIG: Config =  Config::new();

    pub static ref THREAD_RUNTIME: Runtime = Runtime::new().unwrap();

    // This is a holder for shared DBSChema used to fetch information for information_schema table
    // By having a singleton, we can think about caching the result if we are fetching a query too many times
    pub static ref DB_SCHEMA: Mutex<DBSchema> = Mutex::new(DBSchema::new());

    // This variable holds database connections for each connection name that is defined in the config
    // We are using lazy_static to initialize the connections once and use them throughout the application
    static ref DB_CONN_CACHE: HashMap<String, Arc<Mutex<DBConn>>> = {

        let mut cache = HashMap::new();
        let local = tokio::task::LocalSet::new();

        for connection in CONFIG.connections.keys() {
            let connection_config = CONFIG.connections.get(connection).unwrap();
            let db_type = connection_config.db_type.to_owned();
            let conn = match db_type {
                DatabaseType::Mysql => {
                    let url = &CONFIG.get_mysql_cred_str(&connection_config);

                    let pool = local.block_on(&THREAD_RUNTIME, mysql::MySqlPoolOptions::new().max_connections(10).connect(url.as_str())).unwrap();

                    DBConn::MySQLPooledConn(pool)
                }
                DatabaseType::Postgres => {
                    let url = &CONFIG.get_postgres_cred(&connection_config);
                    let pool = local.block_on(&THREAD_RUNTIME, postgres::PgPoolOptions::new().max_connections(10).connect(url.as_str())).unwrap();
                    DBConn::PostgresConn(pool)
                }
            };
            cache.insert(connection.to_owned(), Arc::new(Mutex::new(conn)));
        };
        cache
    };

    // This variable holds a singleton of DBConnections that is used to get a DBConn from the cache
    // DBConn is used to access the raw connection to the database or run `prepare` statement against each connection
    pub static ref DB_CONNECTIONS: Mutex<DBConnections<'static>> = {
        let db_connections = DBConnections::new(&DB_CONN_CACHE);
        Mutex::new(db_connections)
    };
}
