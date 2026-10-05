#[cfg(test)]
mod postgres_custom_type_mapping_tests {
  use std::env;
  use std::fs;
  use std::io::Write;
  use tempfile::tempdir;

  use pretty_assertions::assert_eq;
  use test_utils::test_utils::TSString;
  use test_utils::{run_test, sandbox::TestConfig};

  #[rustfmt::skip]
run_test!(should_map_types_across_joins_with_enum_and_params, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const joinQuery = sql`
SELECT c.id AS id, c.experience AS experience, c.login_time AS login_time, f.name AS faction
FROM characters c
INNER JOIN races r ON r.id = c.race_id
INNER JOIN factions f ON f.id = r.faction_id
WHERE c.created_at > $1 AND c.experience >= $2 AND f.name = $3
`
"#,

//// Generated TS interfaces ////
r#"
import type { DateTime } from 'luxon';
import type { Faction } from './enums';

export type JoinQueryParams = [DateTime | null, string | null, Faction];

export interface IJoinQueryResult {
	experience: string | null;
	faction: Faction;
	id: number;
	login_time: DateTime | null;
}

export interface IJoinQueryQuery {
	params: JoinQueryParams;
	result: IJoinQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_map_types_in_left_join, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const leftJoinQuery = sql`
SELECT c.id AS id, gm.joined_at AS joined_at, c.last_trade_time AS last_trade_time
FROM characters c
LEFT JOIN guild_members gm ON gm.character_id = c.id
`
"#,

//// Generated TS interfaces ////
r#"
import type { DateTime } from 'luxon';

export type LeftJoinQueryParams = [];

export interface ILeftJoinQueryResult {
	id: number;
	joined_at: DateTime | null;
	last_trade_time: string | null;
}

export interface ILeftJoinQueryQuery {
	params: LeftJoinQueryParams;
	result: ILeftJoinQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_map_types_in_cte, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const cteQuery = sql`
WITH active AS (
  SELECT id, experience, logout_time FROM characters WHERE logout_time > $1
)
SELECT a.id AS id, a.experience AS experience, a.logout_time AS logout_time FROM active a
`
"#,

//// Generated TS interfaces ////
r#"
import type { Dayjs } from 'dayjs';

export type CteQueryParams = [Dayjs | null];

export interface ICteQueryResult {
	experience: string;
	id: number;
	logout_time: Dayjs;
}

export interface ICteQueryQuery {
	params: CteQueryParams;
	result: ICteQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_map_param_types_in_subquery, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const subqueryQuery = sql`
SELECT id, last_trade_time FROM characters
WHERE id IN (SELECT character_id FROM guild_members WHERE joined_at > $1)
`
"#,

//// Generated TS interfaces ////
r#"
import type { DateTime } from 'luxon';

export type SubqueryQueryParams = [DateTime | null];

export interface ISubqueryQueryResult {
	id: number;
	last_trade_time: string | null;
}

export interface ISubqueryQueryQuery {
	params: SubqueryQueryParams;
	result: ISubqueryQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_map_insert_params_and_returning, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const insertQuery = sql`
INSERT INTO characters (name, experience, login_time, logout_time)
VALUES ($1, $2, $3, $4)
RETURNING id, experience, created_at
`
"#,

//// Generated TS interfaces ////
r#"
import type { DateTime } from 'luxon';
import type { Dayjs } from 'dayjs';

export type InsertQueryParams = [string, string | null, DateTime | null, Dayjs | null];

export interface IInsertQueryResult {
	created_at: DateTime | null;
	experience: string | null;
	id: number;
}

export interface IInsertQueryQuery {
	params: InsertQueryParams;
	result: IInsertQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_map_update_params_and_returning, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const updateQuery = sql`
UPDATE characters SET experience = $1, logout_time = $2 WHERE login_time < $3
RETURNING id, experience, logout_time
`
"#,

//// Generated TS interfaces ////
r#"
import type { Dayjs } from 'dayjs';
import type { DateTime } from 'luxon';

export type UpdateQueryParams = [string | null, Dayjs | null, DateTime | null];

export interface IUpdateQueryResult {
	experience: string | null;
	id: number;
	logout_time: Dayjs | null;
}

export interface IUpdateQueryQuery {
	params: UpdateQueryParams;
	result: IUpdateQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_map_delete_params_and_returning, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const deleteQuery = sql`
DELETE FROM characters WHERE created_at < $1 RETURNING id, created_at
`
"#,

//// Generated TS interfaces ////
r#"
import type { DateTime } from 'luxon';

export type DeleteQueryParams = [DateTime | null];

export interface IDeleteQueryResult {
	created_at: DateTime | null;
	id: number;
}

export interface IDeleteQueryQuery {
	params: DeleteQueryParams;
	result: IDeleteQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_collect_imports_from_nested_json_object, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const jsonQuery = sql`
SELECT id, jsonb_build_object('experience', experience, 'loginTime', login_time) AS meta
FROM characters
`
"#,

//// Generated TS interfaces ////
r#"
import type { DateTime } from 'luxon';

export type JsonQueryParams = [];

export interface IJsonQueryResult {
	id: number;
	meta: { experience: string | null; loginTime: DateTime | null };
}

export interface IJsonQueryQuery {
	params: JsonQueryParams;
	result: IJsonQueryResult;
}
"#);

  #[rustfmt::skip]
run_test!(should_map_jsonb_without_import, TestConfig::new("postgres", true, None, Some(".sqlxrc.type_mapping.json".to_string())),
//// TS query ////
r#"
const jsonbColumnQuery = sql`
SELECT id, rewards FROM quests WHERE rewards = $1
`
"#,

//// Generated TS interfaces ////
r#"
export type JsonbColumnQueryParams = [Record<string, unknown> | null];

export interface IJsonbColumnQueryResult {
	id: number;
	rewards: Record<string, unknown> | null;
}

export interface IJsonbColumnQueryQuery {
	params: JsonbColumnQueryParams;
	result: IJsonbColumnQueryResult;
}
"#);
}
