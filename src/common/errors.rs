// Common error messages
pub static DB_CONN_POOL_RETRIEVE_ERROR: &str =
  "Failed to retrieve a connection from the pool. Increase the pool size and try again";
pub static DB_SCHEME_READ_ERROR: &str = "Failed to read the database schema to retrieve details. Please raise the issue on https://github.com/JasonShin/sqlx-ts/issues";
pub static DB_CONN_FROM_LOCAL_CACHE_ERROR: &str = "Failed to retrieve a connection from local cache, check the database name annotated in your query and connections config in your configuration file";
