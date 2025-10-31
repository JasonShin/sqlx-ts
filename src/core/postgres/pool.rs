pub use bb8;

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

impl bb8::ManageConnection for PostgresConnectionManager {
  type Connection = Client;
  type Error = Error;

  fn connect(&self) -> impl std::future::Future<Output = Result<Client, Error>> + Send {
    let conn_url = self.conn_url.clone();

    async move {
      let (client, connection) =
        tokio_postgres::connect(&conn_url, NoTls)
          .await
          .map_err(|err| match err.as_db_error() {
            Some(db_err) => {
              let message = format!(
                "Postgres database connection error - code: {:?}, message: {:?}",
                db_err.code(),
                db_err.message()
              );
              panic!("{message}")
            }
            None => panic!("Postgres database connection error: {err}"),
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
  }

  async fn is_valid(&self, client: &mut Client) -> Result<(), Error> {
    client.simple_query("SELECT 1;").await.map(|_| ())
  }

  fn has_broken(&self, client: &mut Client) -> bool {
    client.is_closed()
  }
}
