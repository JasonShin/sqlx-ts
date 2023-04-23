use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::types::ts_query::TsQuery;
use crate::ts_generator::types::{db_conn::DBConn, ts_query::TsFieldType};
use eyre::Result;
use sqlparser::ast::{Join, Query, SetExpr, TableFactor, TableWithJoins};

pub fn get_all_table_names_from_expr(query: &Query) -> Result<Vec<String>> {
    let table_with_joins: TableWithJoins = match &query.body {
        SetExpr::Select(select) => Ok(select
            .from
            .get(0)
            .ok_or(TsGeneratorError::WildcardStatementWithoutTargetTables(
                query.to_string(),
            ))?
            .to_owned()),
        _ => Err(TsGeneratorError::WildcardStatementDeadendExpression(query.to_string())),
    }?;

    let primary_table_name = match table_with_joins.relation {
        TableFactor::Table { name, .. } => Ok(name.to_string()),
        _ => Err(TsGeneratorError::WildcardStatementUnsupportedTableExpr(
            query.to_string(),
        )),
    }?;

    let mut join_tables = table_with_joins
        .joins
        .into_iter()
        .filter_map(|join| match join {
            Join { relation, .. } => match relation {
                TableFactor::Table { name, .. } => Some(name.to_string()),
                _ => unimplemented!(),
            },
        })
        .collect::<Vec<String>>();

    let tables = &mut vec![primary_table_name];
    tables.append(&mut join_tables);

    Ok(tables.clone())
}

/// Translates a wildcard expression of a SQL statement
/// @example
/// SELECT * FROM items
///
/// and it appends result into the hashmap for type generation
pub fn translate_wildcard_expr(query: &Query, ts_query: &mut TsQuery, db_conn: &DBConn) -> Result<()> {
    let table_with_joins = get_all_table_names_from_expr(query)?;
    let table_with_joins = table_with_joins.iter().map(|s| s.as_ref()).collect();
    let all_fields = DB_SCHEMA.fetch_table(&table_with_joins, db_conn);
    if let Some(all_fields) = all_fields {
        for key in all_fields.keys() {
            let field = all_fields.get(key).unwrap();
            let mut field_types = vec![field.field_type.clone()];
            if field.is_nullable {
                field_types.push(TsFieldType::Null);
            }

            ts_query.result.insert(key.to_owned(), field_types);
        }
    }
    Ok(())
}
