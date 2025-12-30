#[cfg(test)]
mod mysql_delete_query_parameters {
  use assert_cmd::prelude::*;
  use pretty_assertions::assert_eq;
  use std::env;
  use std::fs;
  use std::io::Write;
  use std::process::Command;
  use tempfile::tempdir;

  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_binary_ops, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const someDeleteQuery = sql`
DELETE FROM inventory
WHERE id = ?
AND quantity > 1
OR character_id = ?;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeDeleteQueryParams = [number, number | null];

export interface ISomeDeleteQueryResult {
    
}

export interface ISomeDeleteQueryQuery {
    params: SomeDeleteQueryParams;
    result: ISomeDeleteQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_subquery, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const someDeleteQuery = sql`
DELETE FROM inventory
WHERE id = ?
AND quantity > 1
OR character_id = (SELECT id FROM characters WHERE id = ?);
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeDeleteQueryParams = [number, number];

export interface ISomeDeleteQueryResult {

}

export interface ISomeDeleteQueryQuery {
    params: SomeDeleteQueryParams;
    result: ISomeDeleteQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_handle_delete_with_join_using_aliases, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const deleteInventory = sql`
DELETE inv FROM inventory inv
INNER JOIN characters c ON inv.character_id = c.id
WHERE c.id = ?
  AND inv.id = ?;
`
"#,

//// Generated TS interfaces ////
r#"
export type DeleteInventoryParams = [number, number];

export interface IDeleteInventoryResult {

}

export interface IDeleteInventoryQuery {
    params: DeleteInventoryParams;
    result: IDeleteInventoryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_handle_delete_with_multiple_joins, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const deleteItems = sql`
DELETE i FROM items i
INNER JOIN inventory inv ON i.inventory_id = inv.id
INNER JOIN characters c ON inv.character_id = c.id
WHERE c.id = ?
  AND i.rarity = ?;
`
"#,

//// Generated TS interfaces ////
r#"
export type DeleteItemsParams = [number, string | null];

export interface IDeleteItemsResult {

}

export interface IDeleteItemsQuery {
    params: DeleteItemsParams;
    result: IDeleteItemsResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_handle_delete_with_left_join, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const deleteByCharacter = sql`
DELETE inv FROM inventory inv
LEFT JOIN characters c ON inv.character_id = c.id
WHERE c.id = ?
  AND inv.quantity = 0;
`
"#,

//// Generated TS interfaces ////
r#"
export type DeleteByCharacterParams = [number];

export interface IDeleteByCharacterResult {

}

export interface IDeleteByCharacterQuery {
    params: DeleteByCharacterParams;
    result: IDeleteByCharacterResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_handle_delete_with_comparison_and_join, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const deleteByQuantity = sql`
DELETE inv FROM inventory inv
INNER JOIN items i ON i.inventory_id = inv.id
WHERE i.rarity = ?
  AND inv.quantity < ?;
`
"#,

//// Generated TS interfaces ////
r#"
export type DeleteByQuantityParams = [string | null, number | null];

export interface IDeleteByQuantityResult {

}

export interface IDeleteByQuantityQuery {
    params: DeleteByQuantityParams;
    result: IDeleteByQuantityResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_handle_delete_with_or_conditions_and_join, TestConfig::new("mysql", true, None, None),

//// TS query ////
r#"
const deleteConditional = sql`
DELETE inv FROM inventory inv
INNER JOIN characters c ON inv.character_id = c.id
WHERE (c.id = ? OR c.name = ?)
  AND inv.quantity = 0;
`
"#,

//// Generated TS interfaces ////
r#"
export type DeleteConditionalParams = [number, string];

export interface IDeleteConditionalResult {

}

export interface IDeleteConditionalQuery {
    params: DeleteConditionalParams;
    result: IDeleteConditionalResult;
}
"#);
}
