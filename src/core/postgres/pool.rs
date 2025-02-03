pub use bb8;

use async_trait::async_trait;
use tokio;
use tokio_postgres::{Client, Error, NoTls};

pub struct PostgresConnectionManager {
  conn_url: String,
}

impl PostgresConnectionManager {
  pub fn new(conn_url: String) -> Self {
    Self { conn_url }
  }
}

#[async_trait]
impl bb8::ManageConnection for PostgresConnectionManager {
  type Connection = Client;
  type Error = Error;

  async fn connect(&self) -> Result<Client, Error> {
    let conn_url = self.conn_url.clone();

    let (client, connection) = tokio_postgres::connect(&self.conn_url, NoTls).await.map_err(|err| {
      match err.as_db_error() {
        Some(db_err) => {
          let message = format!("Postgres database connection error - code: {:?}, message: {:?}", db_err.code(), db_err.message());
          panic!("{message}")
        }
        None => panic!("Postgres database connection error: {err}"),
      }
    })?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
      if let Err(err) = connection.await {
        panic!("Postgres database connection error: {err}")
      }
    });
    Ok(client)
  }

  async fn is_valid(&self, client: &mut Client) -> Result<(), Error> {
    client.simple_query("SELECT 1;").await.map(|_| ())
  }

  fn has_broken(&self, client: &mut Client) -> bool {
    client.is_closed()
  }
}
