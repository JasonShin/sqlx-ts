use postgres::{Error, Row};
use swc_common::errors::Handler;
use sqlx_ts_common::SQL;
use crate::postgres::explain as postgres_explain;

pub fn execute(queries: &Vec<SQL>, handler: &Handler) -> bool {
    // TODO: later we will add mysql_explain, sqlite_explain depending on the database type
    postgres_explain::explain(&queries, &handler)
}
