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
run_test!(should_pick_query_params_from_binary_ops, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someDeleteQuery = sql`
DELETE FROM inventory
WHERE id = $2
AND quantity > 1
OR character_id = $1;
`
"#,

//// Generated TS interfaces ////
r#"
export type SomeDeleteQueryParams = [number | null, number];

export interface ISomeDeleteQueryResult {
    
}

export interface ISomeDeleteQueryQuery {
    params: SomeDeleteQueryParams;
    result: ISomeDeleteQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_pick_query_params_from_subquery, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const someDeleteQuery = sql`
DELETE FROM inventory
WHERE id = $2
AND quantity > 1
OR character_id = (SELECT id FROM characters WHERE id = $1);
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
run_test!(should_handle_delete_using_with_aliases, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const deleteInventory = sql`
DELETE FROM inventory inv
USING characters c
WHERE inv.character_id = c.id
  AND c.id = $1
  AND inv.id = $2;
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
run_test!(should_handle_delete_using_with_multiple_tables, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const deleteItems = sql`
DELETE FROM items i
USING inventory inv, characters c
WHERE i.inventory_id = inv.id
  AND inv.character_id = c.id
  AND c.id = $1
  AND i.rarity = $2;
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
run_test!(should_handle_delete_using_with_comparison, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const deleteOldInventory = sql`
DELETE FROM inventory inv
USING characters c
WHERE inv.character_id = c.id
  AND c.id = $1
  AND inv.quantity > $2;
`
"#,

//// Generated TS interfaces ////
r#"
export type DeleteOldInventoryParams = [number, number | null];

export interface IDeleteOldInventoryResult {

}

export interface IDeleteOldInventoryQuery {
    params: DeleteOldInventoryParams;
    result: IDeleteOldInventoryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_handle_delete_using_with_in_clause, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const deleteByRarity = sql`
DELETE FROM inventory inv
USING items i
WHERE i.inventory_id = inv.id
  AND i.rarity = $1;
`
"#,

//// Generated TS interfaces ////
r#"
export type DeleteByRarityParams = [string | null];

export interface IDeleteByRarityResult {

}

export interface IDeleteByRarityQuery {
    params: DeleteByRarityParams;
    result: IDeleteByRarityResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_handle_delete_using_with_or_conditions, TestConfig::new("postgres", true, None, None),

//// TS query ////
r#"
const deleteConditional = sql`
DELETE FROM inventory inv
USING characters c
WHERE inv.character_id = c.id
  AND (c.id = $1 OR c.name = $2)
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
