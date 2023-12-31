#[cfg(test)]
mod tests {
    use sqlparser::{
        ast::{SetExpr, Statement},
        dialect::GenericDialect,
        parser::Parser,
    };

    use crate::ts_generator::sql_parser::expressions::translate_table_with_joins::translate_table_with_joins;

    #[test]
    fn should_select_default_for_unnamed_expr() {
        let sql = "
            SELECT id
            FROM items;
        ";

        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let sql_ast = Parser::parse_sql(&dialect, sql).unwrap();
        let stmt = sql_ast[0].clone();
        if let Statement::Query(query) = stmt {
            let body = *query.body;
            match body {
                SetExpr::Select(select) => {
                    let select_item = select.projection[0].clone();
                    let table_with_joins = select.from;

                    let result = translate_table_with_joins(&Some(table_with_joins), &select_item);

                    assert_eq!(Some("items".to_string()), result)
                }
                _ => (),
            }
        }
    }

    #[test]
    fn should_select_default_for_unnamed_expr_with_table_alias() {
        let sql = "
            SELECT x.id
            FROM items as x;
        ";

        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let sql_ast = Parser::parse_sql(&dialect, sql).unwrap();
        let stmt = sql_ast[0].clone();
        match stmt {
            Statement::Query(query) => {
                let body = *query.body;
                match body {
                    SetExpr::Select(select) => {
                        let select_item = select.projection[0].clone();
                        let table_with_joins = select.from;

                        let result = translate_table_with_joins(&Some(table_with_joins), &select_item);

                        assert_eq!(Some("items".to_string()), result)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    #[test]
    fn should_select_join_table_for_unnamed_expr_with_table_alias() {
        let sql = "
            SELECT x.id, tables.id
            FROM items AS x
            JOIN tables ON x.table_id = tables.id;
        ";

        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let sql_ast = Parser::parse_sql(&dialect, sql).unwrap();
        let stmt = sql_ast[0].clone();
        match stmt {
            Statement::Query(query) => {
                let body = *query.body;
                match body {
                    SetExpr::Select(select) => {
                        // choosing `tables.id`
                        let select_item = select.projection[1].clone();
                        let table_with_joins = select.from;

                        let result = translate_table_with_joins(&Some(table_with_joins), &select_item);

                        assert_eq!(Some("tables".to_string()), result)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    #[test]
    fn should_select_join_table_for_expr_with_alias() {
        let sql = "
            SELECT items.id as idz
            FROM items
        ";

        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let sql_ast = Parser::parse_sql(&dialect, sql).unwrap();
        let stmt = sql_ast[0].clone();
        match stmt {
            Statement::Query(query) => {
                let body = *query.body;
                match body {
                    SetExpr::Select(select) => {
                        // choosing `items.id`
                        let select_item = select.projection[0].clone();
                        let table_with_joins = select.from;

                        let result = translate_table_with_joins(&Some(table_with_joins), &select_item);

                        assert_eq!(Some("items".to_string()), result)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
