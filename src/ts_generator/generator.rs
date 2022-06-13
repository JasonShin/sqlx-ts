use sqlparser::{dialect::GenericDialect, parser::Parser};
use sqlparser::ast::{ Statement, SetExpr };

use crate::common::{SQL, config::Config};

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
                        for select_item in projection {
                            println!("checking each select item {:#?}", select_item);
                        }
                    },
                    _ => println!("hmm")
                }
            },
            _ => println!("not sure")
        }
    }
}
