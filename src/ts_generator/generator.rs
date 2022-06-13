use sqlparser::ast::{SetExpr, Statement};
use sqlparser::{dialect::GenericDialect, parser::Parser};

use crate::common::{config::Config, SQL};

pub fn generate_ts_interface(sql: &SQL, config: &Config) {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let sql_ast = Parser::parse_sql(&dialect, &sql.query).unwrap();

    for sql in &sql_ast {
        match sql {
            Statement::Query(query) => {
                let body = &query.body;
                match body {
                    SetExpr::Select(select) => {
                        let projection = select.clone().projection;
                        let from = select.clone().from;

                        println!("checking from {:?}", from);
                        for select_item in projection {
                            println!("checking each select item {:#?}", select_item);
                        }
                    }
                    _ => println!("hmm"),
                }
            }
            _ => println!("not sure"),
        }
    }
}
