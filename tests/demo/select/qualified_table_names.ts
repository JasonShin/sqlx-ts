import { sql } from 'sqlx-ts'

// Test qualified table name (schema.table)
const qualifiedTableName = sql`
-- @name: qualified table name
SELECT
  races.id,
  races.name
FROM public.races
WHERE races.id = 1
`

// Test multiple qualified table names in a JOIN
const qualifiedTableNameJoin = sql`
-- @name: qualified table name join
SELECT
  races.id AS race_id,
  races.name AS race_name,
  factions.id AS faction_id,
  factions.name AS faction_name
FROM public.races
INNER JOIN public.factions ON races.faction_id = factions.id
WHERE races.id = 1
`

// Test mixed qualified and unqualified table names
const mixedQualifiedNames = sql`
-- @name: mixed qualified names
SELECT
  r.id AS race_id,
  r.name AS race_name,
  f.id AS faction_id,
  f.name AS faction_name
FROM public.races AS r
INNER JOIN factions AS f ON r.faction_id = f.id
WHERE r.id = 1
`
