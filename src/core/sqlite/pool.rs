use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tokio::task;

/// A connection manager for SQLite that wraps rusqlite's synchronous Connection
/// behind an Arc<Mutex<>> for thread-safe access with bb8 connection pooling.
#[derive(Clone, Debug)]
pub struct SqliteConnectionManager {
  db_path: String,
  connection_name: String,
}

/// Wrapper around rusqlite::Connection to make it Send + Sync for bb8
pub struct SqliteConnection {
  pub conn: Arc<Mutex<Connection>>,
}

// Safety: rusqlite::Connection is not Send by default, but we protect it with Mutex
// and only access it via spawn_blocking
unsafe impl Send for SqliteConnection {}
unsafe impl Sync for SqliteConnection {}

impl SqliteConnectionManager {
  pub fn new(db_path: String, connection_name: String) -> Self {
    Self {
      db_path,
      connection_name,
    }
  }
}

#[derive(Debug)]
pub struct SqlitePoolError(pub String);

impl std::fmt::Display for SqlitePoolError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SQLite pool error: {}", self.0)
  }
}

impl std::error::Error for SqlitePoolError {}

impl bb8::ManageConnection for SqliteConnectionManager {
  type Connection = SqliteConnection;
  type Error = SqlitePoolError;

  async fn connect(&self) -> Result<Self::Connection, Self::Error> {
    let db_path = self.db_path.clone();
    let connection_name = self.connection_name.clone();

    let conn = task::spawn_blocking(move || {
      Connection::open(&db_path).unwrap_or_else(|err| {
        panic!(
          "Failed to open SQLite database at '{}' for connection '{}': {}",
          db_path, connection_name, err
        )
      })
    })
    .await
    .map_err(|e| SqlitePoolError(format!("Failed to spawn blocking task: {e}")))?;

    Ok(SqliteConnection {
      conn: Arc::new(Mutex::new(conn)),
    })
  }

  async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
    let inner = conn.conn.clone();
    task::spawn_blocking(move || {
      let conn = inner.lock().unwrap();
      conn
        .execute_batch("SELECT 1")
        .map_err(|e| SqlitePoolError(format!("SQLite connection validation failed: {e}")))
    })
    .await
    .map_err(|e| SqlitePoolError(format!("Failed to spawn blocking task: {e}")))?
  }

  fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
    false
  }
}
