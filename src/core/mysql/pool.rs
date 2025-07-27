use async_trait::async_trait;
use mysql_async::{prelude::*, Conn, Error, IoError, Opts};
use tokio::{runtime::Handle, task};

#[derive(Clone, Debug)]
pub struct MySqlConnectionManager {
  conn_url: String,
  // The connection name that user has defined in .sqlxrc.json
  connection_name: String,
}

impl MySqlConnectionManager {
  pub fn new(conn_url: String, connection_name: String) -> Self {
    Self {
      conn_url,
      connection_name,
    }
  }
}

#[async_trait]
impl bb8::ManageConnection for MySqlConnectionManager {
  type Connection = Conn;
  type Error = Error;

  async fn connect(&self) -> Result<Self::Connection, Self::Error> {
    let connection_name = &self.connection_name;
    let conn_opts = Opts::from_url(self.conn_url.as_str())?;

    let conn = Conn::new(conn_opts).await.map_err(|err| {
      match err {
        Error::Driver(driver_error) => {
          panic!("Driver error occurred while connecting to MySQL database - connection: {connection_name}, error: {driver_error}");
        }
        Error::Io(io_err) => {
          match io_err {
            IoError::Io(io_err) => {
              if io_err.kind() == std::io::ErrorKind::ConnectionRefused {
                panic!("Connection Refused - check your connection config for MySQL database - connection: {connection_name}")
              } else {
                panic!("I/O error occurred while connection to MySQL database - connection: {connection_name}, error: {io_err}")
              }
            }
          }
        }
        Error::Other(other_err) => {
          panic!("An unexpected error occurred while connecting to MySQL database - connection: {connection_name}, error: {other_err}");
        }
        Error::Server(server_err) => {
          panic!("Server error occurred while connecting to MySQL database - connection: {connection_name}, error: {server_err}");
        }
        Error::Url(_) => {
          panic!("Invalid URL format for MySQL connection string - connection: {connection_name}");
        }
      }
    }).unwrap();

    Ok(conn)
  }

  async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
    let connection_name = &self.connection_name;
    conn
      .query("SELECT 1")
      .await
      .map(|_: Vec<String>| ())
      .map_err(|err| panic!("Failed to validate MySQL connection for connection: {connection_name}. Error: {err}"))
  }

  fn has_broken(&self, conn: &mut Self::Connection) -> bool {
    task::block_in_place(|| Handle::current().block_on(async { conn.ping().await.is_err() }))
  }
}
