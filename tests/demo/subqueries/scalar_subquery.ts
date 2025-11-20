import { sql } from 'sqlx-ts'

// Scalar subquery in SELECT
const scalarSubqueryInSelect = sql`
-- @name: scalar subquery in select
SELECT
  id,
  name,
  (SELECT COUNT(*) FROM items i2 WHERE i2.rarity = items.rarity) AS same_rarity_count
FROM items
`

// Scalar subquery with aggregate
const scalarSubqueryWithAggregate = sql`
-- @name: scalar subquery with aggregate
SELECT
  id,
  name,
  rarity,
  (SELECT AVG(id) FROM items) AS avg_id
FROM items
`

// Multiple scalar subqueries
const multipleScalarSubqueries = sql`
-- @name: multiple scalar subqueries
SELECT
  id,
  name,
  (SELECT MAX(id) FROM items) AS max_id,
  (SELECT MIN(id) FROM items) AS min_id,
  (SELECT COUNT(*) FROM items) AS total_count
FROM items
`

// Scalar subquery with parameter
const scalarSubqueryWithParam = sql`
-- @name: scalar subquery with param
SELECT
  id,
  name,
  (SELECT COUNT(*) FROM items i2 WHERE i2.rarity = $1) AS filtered_count
FROM items
WHERE id < $2
`
