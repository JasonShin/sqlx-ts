use crate::common::cli::Cli;
use crate::common::config::Config;
use crate::common::types::DatabaseType;
use crate::core::connection::{DBConn, DBConnections};
use crate::core::mysql::pool::MySqlConnectionManager;
use crate::core::postgres::pool::PostgresConnectionManager;
use crate::ts_generator::information_schema::DBSchema;
use clap::Parser;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};
use tokio::{runtime::Handle, sync::Mutex, task};

// The file contains all implicitly dependent variables or state that files need for the logic
// We have a lot of states that we need to drill down into each methods
lazy_static! {
    pub static ref CLI_ARGS: Cli = Cli::parse();
    pub static ref CONFIG: Config =  Config::new();

    // This is a holder for shared DBSChema used to fetch information for information_schema table
    // By having a singleton, we can think about caching the result if we are fetching a query too many times
    pub static ref DB_SCHEMA: Arc<Mutex<DBSchema>> = Arc::new(Mutex::new(DBSchema::new()));

    // This variable holds database connections for each connection name that is defined in the config
    // We are using lazy_static to initialize the connections once and use them throughout the application
    static ref DB_CONN_CACHE: HashMap<String, Arc<Mutex<DBConn>>> = {
        let mut cache = HashMap::new();
        for connection in CONFIG.connections.keys() {
            let connection_config = CONFIG.connections.get(connection).unwrap();
            let db_type = connection_config.db_type.to_owned();
            let conn = match db_type {
                DatabaseType::Mysql => {
                    task::block_in_place(|| Handle::current().block_on(async {
                        let mysql_cred = CONFIG.get_mysql_cred_str(connection_config);
                        let mysql_cred = mysql_cred.as_str();
                        let manager = MySqlConnectionManager::new(mysql_cred.to_string());
                        let pool = bb8::Pool::builder().max_size(10).build(manager).await.unwrap();
                        DBConn::MySQLPooledConn(Mutex::new(pool))
                    }))
                }
                DatabaseType::Postgres => {
                    task::block_in_place(|| Handle::current().block_on(async {
                        let postgres_cred = CONFIG.get_postgres_cred(connection_config);
                        let manager = PostgresConnectionManager::new(postgres_cred);
                        let pool = bb8::Pool::builder().max_size(10).build(manager).await.unwrap();
                        let db_conn = DBConn::PostgresConn(Mutex::new(pool));

                        let conn = match &db_conn {
                            DBConn::PostgresConn(conn) => conn,
                            _ => panic!("Invalid connection type"),
                        };

                        if connection_config.pg_search_path.is_some() {
                            let search_path_query = format!("SET search_path TO {}", &connection_config.pg_search_path.clone().unwrap().as_str());
                            {
                                let conn = conn.lock().await;
                                let conn = conn
                                    .get()
                                    .await
                                    .expect("Unable to connect to the database, please check the connection configuration again https://jasonshin.github.io/sqlx-ts/api/1.connecting-to-db.html");
                                conn.execute(&search_path_query, &[]).await
                                    .expect(format!("Failed to execute the search_path query {:?}", search_path_query));
                            }
                        }
                        db_conn
                    }))

                }
            };
            cache.insert(connection.to_owned(), Arc::new(Mutex::new(conn)));
        };
        cache
    };

    // This variable holds a singleton of DBConnections that is used to get a DBConn from the cache
    // DBConn is used to access the raw connection to the database or run `prepare` statement against each connection
    pub static ref DB_CONNECTIONS: Arc<Mutex<DBConnections<'static>>> = {
        let db_connections = DBConnections::new(&DB_CONN_CACHE);
        Arc::new(Mutex::new(db_connections))
    };
}
