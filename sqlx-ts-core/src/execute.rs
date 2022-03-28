use crate::postgres::explain as postgres_explain;

pub fn execute(queries: &Vec<&str>) {
    postgres_explain::explain(&queries)
}
