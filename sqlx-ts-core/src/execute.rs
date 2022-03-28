use postgres::{Client, NoTls};

pub fn execute(query: &str) {
    let mut conn = Client::connect(
        "host=localhost user=postgres password=postgres port=54321",
        NoTls,
    )
    .unwrap();

    conn.prepare(query).unwrap();
}
