use postgres::{Client, NoTls};

pub fn explain<'a>(queries: &Vec<&str>) {
    let mut conn = Client::connect(
        "host=localhost user=postgres password=postgres port=54321",
        NoTls,
    ).unwrap();

    let explain_queries: Vec<Result<_, _>> = queries.iter()
        .map(|x| format!("EXPLAIN {}", x))
        .map(|x| conn.query(x.as_str(), &[]))
        .collect();

    println!("checking explain queries {:?}", explain_queries);
}

