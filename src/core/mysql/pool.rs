use async_trait::async_trait;
use mysql_async::{prelude::*, Conn, Error, Opts};
use tokio::{runtime::Handle, task};

#[derive(Clone, Debug)]
pub struct MySqlConnectionManager {
    conn_url: String,
}

impl MySqlConnectionManager {
    pub fn new(conn_url: String) -> Self {
        Self { conn_url }
    }
}

#[async_trait]
impl bb8::ManageConnection for MySqlConnectionManager {
    type Connection = Conn;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn_opts = Opts::from_url(self.conn_url.as_str())?;
        Conn::new(conn_opts).await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.query("SELECT version()").await.map(|_: Vec<String>| ())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        task::block_in_place(|| Handle::current().block_on(async { conn.ping().await.is_err() }))
    }
}
