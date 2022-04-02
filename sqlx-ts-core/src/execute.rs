use postgres::{Error, Row};
use sqlx_ts_common::SQL;
use swc_common::errors::Handler;

use crate::postgres::explain as postgres_explain;

pub fn execute(queries: &Vec<SQL>, handler: &Handler) -> bool {
    // TODO: later we will add mysql_explain, sqlite_explain depending on the database type
    postgres_explain::explain(&queries, &handler)
}
