use mysql::{ prelude::*, Conn, Error };

#[derive(Clone, Debug)]
pub struct MySqlConnectionManager {
    conn_url: String,
}

impl MySqlConnectionManager {
    pub fn new(conn_url: String) -> Self {
        Self { conn_url }
    }
}

impl r2d2::ManageConnection for MySqlConnectionManager {
    type Connection = Conn;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Conn::new(self.conn_url.as_str())
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.query("SELECT version()").map(|_: Vec<String>| ())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        self.is_valid(conn).is_err()
    }
}

