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
    let (client, connection) = tokio_postgres::connect(&self.conn_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move { connection.await.map(|_| ()) });
    Ok(client)
  }
  //
  // async fn connect(&self) -> Result<Client, Error> {
  //   match tokio_postgres::connect(&self.conn_url, NoTls).await {
  //     Ok((client, connection)) => {
  //       tokio::spawn(async move {
  //         if let Err(err) = connection.await {
  //           panic!("Unexpected error occurred in the Postgres database connection");
  //         }
  //       });
  //       Ok(client)
  //     }
  //     Err(err) => {
  //       match err {
  //         tokio_postgres::Error::Io(io_err) {
  //           panic!(
  //             "Connection refused - check if the database server is running and accessible"
  //           );
  //         }
  //       }
  //     }
  //   }
  // }
  //

  async fn is_valid(&self, client: &mut Client) -> Result<(), Error> {
    client.simple_query("SELECT 1;").await.map(|_| ())
  }

  fn has_broken(&self, client: &mut Client) -> bool {
    client.is_closed()
  }
}
