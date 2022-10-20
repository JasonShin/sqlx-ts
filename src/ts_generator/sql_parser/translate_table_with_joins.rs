use sqlparser::ast::{Join, SelectItem, TableFactor, TableWithJoins, Expr};

fn find_table_name_from_identifier(
    table_with_joins: &Vec<TableWithJoins>,
    identifier: String, // can be the actual identifier or an alias
    default_table_name: String,
) -> Option<String> {
    // if the identifier of a compound identifier is exactly same as the default table name, we just return it
    if identifier == default_table_name {
        return Some(default_table_name);
    }

    println!("default table name {default_table_name}   {identifier}");
    for relation in table_with_joins.into_iter().map(|tj| tj.relation.clone()) {
        match &relation {
            TableFactor::Table {
                name,
                alias,
                args,
                with_hints,
            } => {
                if Some(identifier.to_owned()) == alias.to_owned().map(|a| a.to_string())
                {
                    return Some(name.to_string());
                }
            }
            _ => unimplemented!(),
        }
    }

    let joins = &table_with_joins
        .into_iter()
        .map(|tj| tj.joins.clone())
        .flatten()
        .collect::<Vec<Join>>();
    for join in &joins.clone() {
        match &join.relation {
            TableFactor::Table {
                name,
                alias,
                args,
                with_hints,
            } => {
                let alias = alias.to_owned().map(|x| x.to_string());
                let name = name.to_string();

                if Some(identifier.to_owned()) == alias || identifier == name {
                    return Some(name);
                }
            }
            _ => unimplemented!(),
        }
    }
    return None;
}

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
            } => Some(name.to_string()),
            _ => None,
        })
        .flatten()
        .unwrap();

    match select_item {
        SelectItem::UnnamedExpr(expr) => {
            match expr {
                Expr::CompoundIdentifier(compound_identifier) => {
                    // Assumes that [0] of the compound identifiers is the alias that points to the table
                    let identifier = compound_identifier[0].to_string();

                    find_table_name_from_identifier(table_with_joins, identifier, default_table_name)
                }
                _ => Some(default_table_name),
            }
        }
        SelectItem::Wildcard => Some(default_table_name),
        SelectItem::ExprWithAlias { expr, alias } => {
            match &expr {
                Expr::Identifier(ident) => todo!(),
                Expr::CompoundIdentifier(compound_identifier) => {
                    let identifier = compound_identifier[0].to_string();

                    find_table_name_from_identifier(table_with_joins, identifier, default_table_name)
                },
                _ => Some(default_table_name),
            }

        },
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

    #[test]
    fn should_select_join_table_for_unnamed_expr_with_table_alias() {
        let sql = "
            SELECT x.id, tables.id
            FROM items AS x
            JOIN tables ON x.table_id = tables.id;
        ";

        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let sql_ast = Parser::parse_sql(&dialect, &sql).unwrap();
        let stmt = sql_ast[0].clone();
        match stmt {
            Statement::Query(query) => {
                let body = query.body;
                match body {
                    SetExpr::Select(select) => {
                        // choosing `tables.id`
                        let select_item = select.clone().projection[1].clone();
                        let table_with_joins = select.clone().from;

                        let result = translate_table_with_joins(&table_with_joins, &select_item);

                        assert_eq!(Some("tables".to_string()), result)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
