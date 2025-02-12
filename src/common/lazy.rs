use crate::common::cli::Cli;
use crate::common::config::Config;
use crate::common::types::DatabaseType;
use crate::core::connection::{DBConn, DBConnections};
use crate::core::mysql::pool::MySqlConnectionManager;
use crate::core::postgres::pool::PostgresConnectionManager;
use crate::ts_generator::information_schema::DBSchema;
use clap::Parser;
use std::sync::LazyLock;
use std::{collections::HashMap, sync::Arc};
use tokio::{runtime::Handle, sync::Mutex, task};

// The file contains all implicitly dependent variables or state that files need for the logic
// We have a lot of states that we need to drill down into each methods
pub static CLI_ARGS: LazyLock<Cli> = LazyLock::new(|| {
  println!("Initializing CLI ARGS");
  let cli = Cli::parse();
  println!("parsed CLI {:?}", cli);
  cli
});
pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::new);

// This is a holder for shared DBSChema used to fetch information for information_schema table
// By having a singleton, we can think about caching the result if we are fetching a query too many times
pub static DB_SCHEMA: LazyLock<Arc<Mutex<DBSchema>>> = LazyLock::new(|| Arc::new(Mutex::new(DBSchema::new())));
// TODO: move this to errors.rs
pub static ERR_DB_CONNECTION_ISSUE: LazyLock<String> = LazyLock::new(|| {
  "Unable to connect to the database, please check the connection configuration again https://jasonshin.github.io/sqlx-ts/api/1.connecting-to-db.html".to_string()
});

// This variable holds database connections for each connection name that is defined in the config
// We are using lazy_static to initialize the connections once and use them throughout the application
pub static DB_CONN_CACHE: LazyLock<HashMap<String, Arc<Mutex<DBConn>>>> = LazyLock::new(|| {
  LazyLock::force(&CONFIG);
  let mut cache = HashMap::new();
  for connection in CONFIG.connections.keys() {
    let connection_config = CONFIG
      .connections
      .get(connection)
      .unwrap_or_else(|| panic!("Invalid connection name - {connection}"));
    let db_type = connection_config.db_type.to_owned();
    let conn = match db_type {
      DatabaseType::Mysql => task::block_in_place(|| {
        Handle::current().block_on(async {
          let mysql_cred = CONFIG.get_mysql_cred_str(connection_config);
          let mysql_cred = mysql_cred.as_str();
          let manager = MySqlConnectionManager::new(mysql_cred.to_string(), connection.to_string());
          let pool = bb8::Pool::builder()
            .max_size(connection_config.pool_size)
            .connection_timeout(std::time::Duration::from_secs(connection_config.connection_timeout))
            .build(manager)
            .await
            .expect(&ERR_DB_CONNECTION_ISSUE);

          DBConn::MySQLPooledConn(Mutex::new(pool))
        })
      }),
      DatabaseType::Postgres => task::block_in_place(|| {
        Handle::current().block_on(async {
          let postgres_cred = CONFIG.get_postgres_cred(connection_config);
          let manager = PostgresConnectionManager::new(postgres_cred);
          let pool = bb8::Pool::builder()
            .max_size(connection_config.pool_size)
            .connection_timeout(std::time::Duration::from_secs(connection_config.connection_timeout))
            .build(manager)
            .await
            .expect(&ERR_DB_CONNECTION_ISSUE);

          let db_conn = DBConn::PostgresConn(Mutex::new(pool));

          let conn = match &db_conn {
            DBConn::PostgresConn(conn) => conn,
            _ => panic!("Invalid connection type"),
          };

          if connection_config.pg_search_path.is_some() {
            let search_path_query = format!(
              "SET search_path TO {}",
              &connection_config.pg_search_path.clone().unwrap().as_str()
            );
            {
              let conn = conn.lock().await;
              let conn = conn.get().await.expect(&ERR_DB_CONNECTION_ISSUE);

              let err_search_path_query = format!(
                "Failed to execute the search_path query {:?}",
                search_path_query.as_str()
              );
              let err_search_path_query = err_search_path_query.as_str();
              conn
                .execute(&search_path_query, &[])
                .await
                .expect(err_search_path_query);
            }
          }
          db_conn
        })
      }),
    };
    cache.insert(connection.to_owned(), Arc::new(Mutex::new(conn)));
  }
  cache
});

// This variable holds a singleton of DBConnections that is used to get a DBConn from the cache
// DBConn is used to access the raw connection to the database or run `prepare` statement against each connection
pub static DB_CONNECTIONS: LazyLock<Arc<Mutex<DBConnections<'static>>>> = LazyLock::new(|| {
  let db_connections = DBConnections::new(&DB_CONN_CACHE);
  Arc::new(Mutex::new(db_connections))
});
