use crate::common::config::GenerateTypesConfig;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::information_schema::DBSchema;
use crate::ts_generator::types::{DBConn, TsFieldType};
use sqlparser::ast::{Join, SetExpr, Statement, TableFactor, TableWithJoins};
use sqlparser::test_utils::join;
use std::borrow::BorrowMut;
use std::collections::HashMap;

// should return a result
pub fn get_all_table_names_from_expr(sql_statement: &Statement) -> Result<Vec<String>, TsGeneratorError> {
    let table_with_joins: TableWithJoins = match sql_statement {
        Statement::Query(query) => match &query.body {
            SetExpr::Select(select) => Ok(select
                .from
                .get(0)
                .ok_or(TsGeneratorError::WildcardStatementWithoutTargetTables)?
                .to_owned()),
            _ => Err(TsGeneratorError::WildcardStatementDeadendExpression),
        },
        _ => Err(TsGeneratorError::WildcardStatementDeadendExpression),
    }?;

    let primary_table_name = match table_with_joins.relation {
        TableFactor::Table { name, .. } => Ok(name.to_string()),
        _ => Err(TsGeneratorError::WildcardStatementUnsupportedTableExpr),
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
    &tables.append(&mut join_tables);

    Ok(tables.clone())
}

/// Translates a wildcard expression of a SQL statement
/// @example
/// SELECT * FROM items
///
/// and it appends result into the hashmap for type generation
pub fn translate_wildcard_expr(
    db_name: &str,
    sql_statement: &Statement,
    result: &mut HashMap<String, Vec<TsFieldType>>,
    db_conn: &DBConn,
    generate_types_config: &Option<GenerateTypesConfig>,
) -> Result<(), TsGeneratorError> {
    let db_schema = DBSchema::new();
    let table_with_joins = get_all_table_names_from_expr(sql_statement)?;
    let table_with_joins = table_with_joins.iter().map(|s| s.as_ref()).collect();
    let all_fields = db_schema.fetch_table(&db_name, &table_with_joins, &db_conn);
    if let Some(all_fields) = all_fields {
        for key in all_fields.keys() {
            let field = all_fields.get(key).unwrap();
            let mut field_types = vec![field.field_type];
            if field.is_nullable {
                field_types.push(TsFieldType::Null);
            }

            result.insert(key.to_owned(), field_types);
        }
    }
    Ok(())
}
