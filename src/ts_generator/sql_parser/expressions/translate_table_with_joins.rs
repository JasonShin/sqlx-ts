use crate::common::table_name::TrimQuotes;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::quoted_strings::*;
use color_eyre::eyre::Result;
use sqlparser::ast::{Assignment, Expr, Join, ObjectName, SelectItem, TableFactor, TableWithJoins};

pub fn get_default_table(table_with_joins: &Vec<TableWithJoins>) -> String {
  table_with_joins
    .first()
    .and_then(|x| match &x.relation {
      TableFactor::Table {
        name,
        alias: _,
        args: _,
        with_hints: _,
        version: _,
        partitions: _,
      } => Some(DisplayTableName(name).to_string()),
      _ => None,
    })
    .expect("The query does not have a default table, impossible to generate types")
}

pub fn find_table_name_from_identifier(
  table_with_joins: &Vec<TableWithJoins>,
  identifiers: &Vec<String>, // can be the actual identifier or an alias
) -> Result<String, TsGeneratorError> {
  let left = identifiers
    .first()
    .expect("The first identifier must exist in order to find the table name")
    .to_owned();
  let right = identifiers.get(1);
  let default_table_name = get_default_table(table_with_joins);

  // if the identifier of a compound identifier is exactly same as the default table name, we just return it
  if left == default_table_name || right.is_none() {
    // If right is none, it means we cannot further assume and try to find the table name
    // we should simply return the default table name
    return Ok(default_table_name);
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
        let alias_quote_style = alias.to_owned().map(|a| a.name.quote_style).flatten();
        let alias: Option<String> = alias
          .to_owned()
          .map(|a| a.name.to_string().trim_table_name(alias_quote_style));
        let name = DisplayTableName(name).to_string();
        if Some(left.to_string()) == alias || left == name {
          // If the identifier matches the alias, then return the table name
          return Ok(name);
        }
      }
      _ => {
        return Err(TsGeneratorError::TableFactorWhileProcessingTableWithJoins(
          relation.to_string(),
        ))
      }
    }
  }

  let joins = &table_with_joins
    .iter()
    .flat_map(|tj| tj.joins.clone())
    .collect::<Vec<Join>>();
  for join in &joins.clone() {
    match &join.relation {
      TableFactor::Table {
        name: objectName,
        alias,
        args: _,
        with_hints: _,
        version: _,
        partitions: _,
      } => {
        let alias_quote_style = alias.to_owned().map(|a| a.name.quote_style).flatten();
        let alias = alias
          .to_owned()
          .map(|x| x.to_string().trim_table_name(alias_quote_style));
        let name_quote_style = objectName.0[0].quote_style;
        let name = objectName.to_string().trim_table_name(name_quote_style);

        if Some(left.to_owned()) == alias || left == name {
          return Ok(name);
        }
      }
      _ => {
        return Err(TsGeneratorError::TableFactorWhileProcessingTableWithJoins(
          join.to_string(),
        ));
      }
    }
  }
  Err(TsGeneratorError::UnknownErrorWhileProcessingTableWithJoins(
    "".to_string(),
  ))
}

/// The function takes in an expression such as
///
/// Example 1:
/// given `SELECT id FROM items`
/// expression is `id`
///
pub fn translate_table_from_expr(
  table_with_joins: &Option<Vec<TableWithJoins>>,
  expr: &Expr,
) -> Result<String, TsGeneratorError> {
  if table_with_joins.is_none() {
    return Err(TsGeneratorError::UnknownErrorWhileProcessingTableWithJoins(
      expr.to_string(),
    ));
  }

  let table_with_joins = table_with_joins.as_ref().unwrap();
  match expr {
    Expr::Identifier(_) => Ok(get_default_table(table_with_joins)),
    Expr::CompoundIdentifier(compound_identifier) => {
      // Assumes that [0] of the compound identifiers is the alias that points to the table
      let identifiers = &compound_identifier
        .iter()
        .map(|x| {
          let quote_style = x.quote_style;
          x.to_string().trim_table_name(quote_style)
        })
        .collect();
      find_table_name_from_identifier(table_with_joins, identifiers)
    }
    _ => Err(TsGeneratorError::UnknownErrorWhileProcessingTableWithJoins(
      expr.to_string(),
    )),
  }
}

pub fn translate_table_from_assignments(
  table_with_joins: &Vec<TableWithJoins>,
  assignment: &Assignment,
) -> Result<String, TsGeneratorError> {
  let identifier = assignment.id.first();
  match identifier {
    Some(identifier) => find_table_name_from_identifier(table_with_joins, &vec![identifier.value.to_string()]),
    None => Ok(get_default_table(table_with_joins)),
  }
}

/// Translates a select item's target table by looking for TableWithJoins
/// If the select item uses table alias, it should find the table name using the alias
/// If the select item does not have any alias or table name, it should pick the default table name
pub fn translate_table_with_joins(
  table_with_joins: &Option<Vec<TableWithJoins>>,
  select_item: &SelectItem,
) -> Result<String, TsGeneratorError> {
  if table_with_joins.is_none() {
    return Err(TsGeneratorError::UnknownErrorWhileProcessingTableWithJoins(
      "".to_string(),
    ));
  }

  let table_with_joins = table_with_joins.as_ref().unwrap();
  let default_table_name = get_default_table(table_with_joins);

  println!("checking select item: {select_item} - default_table_name: {default_table_name}");
  match select_item {
    SelectItem::UnnamedExpr(expr) => {
      match expr {
        Expr::CompoundIdentifier(compound_identifier) => {
          // Assumes that [0] of the compound identifiers is the alias that points to the table
          let identifiers = &compound_identifier
            .iter()
            .map(|x| DisplayIndent(x).to_string())
            .collect();
          find_table_name_from_identifier(table_with_joins, identifiers)
        }
        _ => Ok(default_table_name),
      }
    }
    SelectItem::Wildcard(_) => Ok(default_table_name),
    SelectItem::ExprWithAlias { expr, alias: _ } => match &expr {
      Expr::Identifier(_) => {
        // if the select item is not a compound identifier with an expression, just return the default table name
        Ok(default_table_name)
      }
      Expr::CompoundIdentifier(compound_identifier) => {
        let identifiers = &compound_identifier
          .iter()
          .map(|x| DisplayIndent(x).to_string())
          .collect();
        println!("checking CompoundIdentifier {:?}", identifiers);
        find_table_name_from_identifier(table_with_joins, identifiers)
      }
      _ => Ok(default_table_name),
    },
    // This condition would never reach because translate_table_with_joins is only used when processing non wildcard select items
    SelectItem::QualifiedWildcard(_, _) => {
      unimplemented!("QualifiedWildcard is not supported yet when translating table with joins")
    }
  }
}
