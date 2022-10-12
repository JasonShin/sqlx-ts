use sqlparser::ast::{SelectItem, TableFactor, TableWithJoins};

/// Translates a select item's target table by looking for TableWithJoins
/// If the select item uses table alias, it should find the table name using the alias
/// If the select item does not have any alias or table name, it should pick the default table name
pub fn translate_table_with_joins(
    table_with_joins: &Vec<TableWithJoins>,
    select_item: &SelectItem,
) -> Option<String> {
    let default_table_name = table_with_joins
        .get(0)
        .map(|x| match &x.relation {
            TableFactor::Table {
                name,
                alias,
                args,
                with_hints,
            } => Some(name.to_owned().0[0].to_string()),
            _ => None,
        })
        .flatten()
        .unwrap();

    match select_item {
        SelectItem::UnnamedExpr(expr) => {
            match expr {
                sqlparser::ast::Expr::CompoundIdentifier(compound_identifier) => {
                    // Assumes that [0] of the compound identifiers is the alias that points to the table
                    let identifier = compound_identifier[0].to_string();

                    // if the identifier of a compound identifier is exactly same as the default table name, we just return it
                    if identifier == default_table_name {
                        return Some(default_table_name);
                    }

                    for relation in table_with_joins.into_iter().map(|tj| tj.relation.clone()) {
                        match &relation {
                            TableFactor::Table {
                                name,
                                alias,
                                args,
                                with_hints,
                            } => {
                                if Some(identifier.clone()) == alias.clone().map(|a| a.to_string())
                                {
                                    return Some(name.to_string());
                                }
                            }
                            TableFactor::Derived {
                                lateral,
                                subquery,
                                alias,
                            } => todo!(),
                            TableFactor::TableFunction { expr, alias } => todo!(),
                            TableFactor::UNNEST {
                                alias,
                                array_expr,
                                with_offset,
                            } => todo!(),
                            TableFactor::NestedJoin(_) => todo!(),
                        }
                    }

                    panic!("Cannot reach this point!")
                }
                _ => Some(default_table_name),
            }
        }
        SelectItem::Wildcard => Some(default_table_name),
        SelectItem::ExprWithAlias { expr, alias } => todo!(),
        SelectItem::QualifiedWildcard(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use sqlparser::{
        ast::{SetExpr, Statement},
        dialect::GenericDialect,
        parser::Parser,
    };

    use super::translate_table_with_joins;

    #[test]
    fn should_select_default_for_unnamed_expr() {
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

                        let result = translate_table_with_joins(&table_with_joins, &select_item);

                        assert_eq!(Some("items".to_string()), result)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    #[test]
    fn should_select_default_for_unnamed_expr_with_table_alias() {
        let sql = "
            SELECT x.id
            FROM items as x;
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

                        let result = translate_table_with_joins(&table_with_joins, &select_item);

                        assert_eq!(Some("items".to_string()), result)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
