use postgres::Client;

pub fn explain<'a>(conn: &'a mut Client, sql: &'a str) {
    let explain_query = format!("EXPLAIN {}", sql);
    conn.query(explain_query.as_str(), &[]).unwrap();
}
