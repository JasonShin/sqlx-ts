use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::quoted_strings::*;
use color_eyre::eyre::Result;
use sqlparser::ast::{Assignment, AssignmentTarget, Expr, Join, SelectItem, TableFactor, TableWithJoins};

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
        with_ordinality: _,
        json_path: _,
        sample: _,
        index_hints: _,
      } => Some(DisplayObjectName(name).to_string()),
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
        with_ordinality: _,
        json_path: _,
        sample: _,
        index_hints: _,
      } => {
        let alias = alias.clone().map(|alias| DisplayTableAlias(&alias).to_string());
        let name = DisplayObjectName(name).to_string();
        if Some(left.to_string()) == alias || left == name {
          // If the identifier matches the alias, then return the table name
          return Ok(name.to_owned());
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
        name,
        alias,
        args: _,
        with_hints: _,
        version: _,
        partitions: _,
        with_ordinality: _,
        json_path: _,
        sample: _,
        index_hints: _,
      } => {
        let alias = alias.clone().map(|alias| DisplayTableAlias(&alias).to_string());
        let name = DisplayObjectName(name).to_string();
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
  // if the identifier of a compound identifier is exactly same as the default table name, we just return it
  if left == default_table_name || right.is_none() {
    // If right is none, it means we cannot further assume and try to find the table name
    // we should simply return the default table name
    return Ok(default_table_name);
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
        .map(|x| DisplayIndent(x).to_string())
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
  // In sqlparser 0.59.0, Assignment.id was replaced with Assignment.target
  // which is an AssignmentTarget enum (ColumnName or Tuple)
  let object_name = match &assignment.target {
    AssignmentTarget::ColumnName(name) => Some(name),
    AssignmentTarget::Tuple(names) => names.first(),
  };

  match object_name {
    Some(name) => {
      // Extract the first identifier from the ObjectName
      let first_part = name.0.first();
      match first_part {
        Some(part) => {
          if let Some(ident) = part.as_ident() {
            find_table_name_from_identifier(table_with_joins, &vec![ident.value.to_string()])
          } else {
            // If it's a function-based name, use default table
            Ok(get_default_table(table_with_joins))
          }
        }
        None => Ok(get_default_table(table_with_joins)),
      }
    }
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
        find_table_name_from_identifier(table_with_joins, identifiers)
      }
      _ => Ok(default_table_name),
    },
    // This condition would never reach because translate_table_with_joins is only used when processing non wildcard select items
    SelectItem::QualifiedWildcard(left, _) => Ok(left.to_string()),
  }
}
