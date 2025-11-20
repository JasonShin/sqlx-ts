import { sql } from 'sqlx-ts'

// CAST function
const castFunction = sql`
-- @name: cast function
SELECT
  id,
  name,
  CAST(id AS TEXT) AS id_as_text,
  CAST(id AS FLOAT) AS id_as_float,
  CAST(id AS BIGINT) AS id_as_bigint
FROM items
`

// PostgreSQL casting syntax
const postgresqlCasting = sql`
-- @name: postgresql casting
SELECT
  id,
  name,
  id::TEXT AS id_as_text,
  id::FLOAT AS id_as_float,
  id::VARCHAR(10) AS id_as_varchar
FROM items
`

// Casting with operations
const castingWithOperations = sql`
-- @name: casting with operations
SELECT
  id,
  name,
  (id::FLOAT / 3)::NUMERIC(10, 2) AS id_div_3_precise,
  CAST(CONCAT(id, '') AS INTEGER) AS id_from_concat
FROM items
`

// NULL casting
const nullCasting = sql`
-- @name: null casting
SELECT
  id,
  name,
  CAST(NULL AS INTEGER) AS null_int,
  CAST(NULL AS TEXT) AS null_text
FROM items
`
