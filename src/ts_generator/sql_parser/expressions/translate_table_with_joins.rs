use sqlparser::ast::{Assignment, Expr, Join, SelectItem, TableFactor, TableWithJoins};

pub fn get_default_table(table_with_joins: &Vec<TableWithJoins>) -> String {
    table_with_joins
        .get(0)
        .and_then(|x| match &x.relation {
            TableFactor::Table {
                name,
                alias: _,
                args: _,
                with_hints: _,
                version: _,
                partitions: _,
            } => Some(name.to_string()),
            _ => None,
        })
        .expect("The query does not have a default table, impossible to generate types")
}

pub fn find_table_name_from_identifier(
    table_with_joins: &Vec<TableWithJoins>,
    identifiers: &Vec<String>, // can be the actual identifier or an alias
) -> Option<String> {
    let left = identifiers
        .get(0)
        .expect("The first identifier must exist in order to find the table name")
        .to_owned();
    let right = identifiers.get(1);
    let default_table_name = get_default_table(table_with_joins);

    // if the identifier of a compound identifier is exactly same as the default table name, we just return it
    if left == default_table_name || right.is_none() {
        // If right is none, it means we cannot further assume and try to find the table name
        // we should simply return the default table name
        return Some(default_table_name);
    }

    // Check the default table with joins to see if any left identifier (table name) matches any alias of tables or table name itself
    for relation in table_with_joins.iter().map(|tj| tj.relation.clone()) {
        match &relation {
            TableFactor::Table {
                name,
                alias,
                args: _,
                with_hints: _,
                version: _,
                partitions: _,
            } => {
                if Some(left.to_string()) == alias.to_owned().map(|a| a.to_string()) || left == name.to_string() {
                    // If the identifier matches the alias, then return the table name
                    return Some(name.to_string());
                }
            }
            _ => unimplemented!(),
        }
    }

    let joins = &table_with_joins
        .iter()
        .flat_map(|tj| tj.joins.clone())
        .collect::<Vec<Join>>();
    for join in &joins.clone() {
        match &join.relation {
            TableFactor::Table {
                name,
                alias,
                args: _,
                with_hints: _,
                version: _,
                partitions: _,
            } => {
                let alias = alias.to_owned().map(|x| x.to_string());
                let name = name.to_string();

                if Some(left.to_owned()) == alias || left == name {
                    return Some(name);
                }
            }
            _ => unimplemented!(),
        }
    }
    None
}

/// The function takes in an expression such as
///
/// Example 1:
/// given `SELECT id FROM items`
/// expression is `id`
///
pub fn translate_table_from_expr(table_with_joins: &Option<Vec<TableWithJoins>>, expr: &Expr) -> Option<String> {
    if table_with_joins.is_none() {
        return None;
    }

    let table_with_joins = table_with_joins.as_ref().unwrap();
    match expr {
        Expr::Identifier(_) => Some(get_default_table(table_with_joins)),
        Expr::CompoundIdentifier(compound_identifier) => {
            // Assumes that [0] of the compound identifiers is the alias that points to the table
            let identifiers = &compound_identifier.iter().map(|x| x.to_string()).collect();
            find_table_name_from_identifier(table_with_joins, identifiers)
        }
        _ => None,
    }
}

pub fn translate_table_from_assignments(
    table_with_joins: &Vec<TableWithJoins>,
    assignment: &Assignment,
) -> Option<String> {
    let identifier = assignment.id.get(0);
    match identifier {
        Some(identifier) => find_table_name_from_identifier(table_with_joins, &vec![identifier.value.to_string()]),
        None => Some(get_default_table(table_with_joins)),
    }
}

/// Translates a select item's target table by looking for TableWithJoins
/// If the select item uses table alias, it should find the table name using the alias
/// If the select item does not have any alias or table name, it should pick the default table name
pub fn translate_table_with_joins(
    table_with_joins: &Option<Vec<TableWithJoins>>,
    select_item: &SelectItem,
) -> Option<String> {
    if table_with_joins.is_none() {
        return None;
    }

    let table_with_joins = table_with_joins.as_ref().unwrap();
    let default_table_name = get_default_table(table_with_joins);

    match select_item {
        SelectItem::UnnamedExpr(expr) => {
            match expr {
                Expr::CompoundIdentifier(compound_identifier) => {
                    // Assumes that [0] of the compound identifiers is the alias that points to the table
                    let identifiers = &compound_identifier.iter().map(|x| x.to_string()).collect();
                    find_table_name_from_identifier(table_with_joins, identifiers)
                }
                _ => Some(default_table_name),
            }
        }
        SelectItem::Wildcard(_) => Some(default_table_name),
        SelectItem::ExprWithAlias { expr, alias: _ } => match &expr {
            Expr::Identifier(_) => {
                // if the select item is not a compount identifier with an expression, just return the default table name
                Some(default_table_name)
            }
            Expr::CompoundIdentifier(compound_identifier) => {
                let identifiers = &compound_identifier.iter().map(|x| x.to_string()).collect();
                find_table_name_from_identifier(table_with_joins, identifiers)
            }
            _ => Some(default_table_name),
        },
        // This condition would never reach because translate_table_with_joins is only used when processing non wildcard select items
        SelectItem::QualifiedWildcard(_, _) => {
            unimplemented!("QualifiedWildcard is not supported yet when translating table with joins")
        }
    }
}
