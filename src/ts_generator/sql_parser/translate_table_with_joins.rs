use sqlparser::ast::{TableWithJoins, TableFactor};

/// Translates a select item's target table by looking for TableWithJoins
/// If the select item uses table alias, it should find the table name using the alias
/// If the select item does not have any alias or table name, it should pick the default table name
pub fn translate_table_with_joins(
    table_with_joins: &Vec<TableWithJoins>,
    potential_table_name: Option<String>
) -> Option<String> {
    let default_table_name = table_with_joins
    .get(0)
    .map(|x| {
        match &x.relation {
            TableFactor::Table { name, alias, args, with_hints } => {
                alias.clone()
                    .and_then(|x| {
                        Some(x.name.value)
                    })
                    .or_else(|| {
                        // Relation must have the first item which is the table name itself
                        Some(name.to_owned().0[0].to_string())
                    })
            },
            _ => None,
        }
    })
    .flatten();

    default_table_name
}

#[cfg(test)]
mod tests {
    use sqlparser::{ast::{SetExpr, Statement}, dialect::GenericDialect, parser::Parser};

    use super::translate_table_with_joins;

    #[test]
    fn test_test() {
        let sql = "
            SELECT id
            FROM items;
        ";

        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let sql_ast = Parser::parse_sql(&dialect, &sql).unwrap();
        let stmt = sql_ast[0].clone();
        match stmt {
            Statement::Query(query) => {
                let body = query.body;
                match body {
                    SetExpr::Select(select) => {
                        let select_item = select.clone().projection[0].clone();
                        let table_with_joins = select.clone().from;

                        println!("checking select item {:#?}", select_item);
                        // translate_table_with_joins(&table_with_joins, potential_table_name);
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
}
