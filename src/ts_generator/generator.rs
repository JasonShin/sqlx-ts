use crate::common::{config::Config, SQL};
use crate::ts_generator::{generator, types::TsDataType};
use sqlparser::ast::{
    SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr, Wildcard},
    SetExpr, Statement,
};
use sqlparser::{dialect::GenericDialect, parser::Parser};
use std::collections::HashMap;

/*
fn get_query_name(sql: &SQL) -> String {
    if let Some(var_decl_name) = sql.var_decl_name {
        var_decl_name
    }
}
*/

pub fn generate_ts_interface(sql: &SQL, config: &Config) {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let sql_ast = Parser::parse_sql(&dialect, &sql.query).unwrap();

    let mut result: HashMap<String, TsDataType> = HashMap::new();
    let mut params: HashMap<String, TsDataType> = HashMap::new();

    println!("checking sql {:?}", sql);
    for sql in &sql_ast {
        match sql {
            Statement::Query(query) => {
                let body = &query.body;
                match body {
                    SetExpr::Select(select) => {
                        let projection = select.clone().projection;
                        let table_with_joins = select.clone().from;
                        for select_item in projection {
                            match select_item {
                                UnnamedExpr(unnamed_expr) => {
                                    println!("unmapped expr {:?}", unnamed_expr);
                                }
                                ExprWithAlias { expr, alias } => todo!(),
                                QualifiedWildcard(_) => todo!(),
                                Wildcard => todo!(),
                            }
                        }
                    }
                    _ => println!("hmm"),
                }
            }
            _ => println!("not sure"),
        }
    }
}
