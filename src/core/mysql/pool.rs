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
    Self { conn_url, connection_name }
  }
}

#[async_trait]
impl bb8::ManageConnection for MySqlConnectionManager {
  type Connection = Conn;
  type Error = Error;

  async fn connect(&self) -> Result<Self::Connection, Self::Error> {
    let connection_name = &self.connection_name;
    let conn_opts = Opts::from_url(self.conn_url.as_str())?;

    // here we can check if the connection is valid or not
    let conn = Conn::new(conn_opts).await.map_err(|err| {
      match err {
        Error::Driver(_) => {}
        Error::Io(io_err) => {
          match io_err {
            IoError::Io(io_err) => {
              if io_err.kind() == std::io::ErrorKind::ConnectionRefused {
                panic!("Connection Refused - check your connection config for MySQL database - connection: {connection_name}")
              }
            }
            IoError::Tls(_) => {}
          }
        }
        Error::Other(_) => {}
        Error::Server(_) => {}
        Error::Url(_) => {}
      }
    }).unwrap();
    Ok(conn)
  }

  async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
    conn.query("SELECT 1").await.map(|_: Vec<String>| ())
  }

  fn has_broken(&self, conn: &mut Self::Connection) -> bool {
    task::block_in_place(|| Handle::current().block_on(async { conn.ping().await.is_err() }))
  }
}
