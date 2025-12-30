import { sql } from 'sqlx-ts'

// simple select
const selectSql1 = sql`SELECT * FROM items`

// single join
const selectSql2 = sql`
SELECT *
FROM items
JOIN inventory ON items.inventory_id = inventory.id
`

// subquery
const selectSql3 = sql`
SELECT
    (SELECT quantity FROM inventory WHERE inventory.id = items.inventory_id) AS inventory_quantity
FROM items
`

// wildcard
const selectSql4 = sql`
SELECT items.*
FROM items;
`

// table wit joins
const selectSql5 = sql`
SELECT inventory.*
FROM items
JOIN inventory ON items.inventory_id = inventory.id
`

// Various operators
const selectSql6 = sql`
SELECT id
FROM inventory
WHERE quantity BETWEEN $1 AND $2;
`

// Where condition expressions
const selectSql9 = sql`
SELECT *
FROM items
WHERE $1;
`

const selectSql10 = sql`
SELECT *
FROM quests
WHERE completed IS TRUE;
`

// IS DISTINCT FROM operator as part of the WHERE statement
const selectSql11 = sql`
SELECT id, quantity, id IS DISTINCT FROM $1 AS hmm
FROM inventory
WHERE id IS DISTINCT FROM $2
`

const selectSql12 = sql`
SELECT id
FROM inventory
WHERE inventory.id = $1
`

const selectSql13 = sql`
SELECT *
FROM items
WHERE name LIKE $1
`

// SELECT with quoted table names
const selectSql14 = sql`
SELECT * FROM "items" WHERE id = $1
`

const selectSql15 = sql`
SELECT "items"."id", "inventory"."id" AS "id2", inventory.id
FROM "items"
JOIN "inventory" ON "items"."inventory_id" = "inventory"."id"
`

// SELECT IFNULL with ENUM type
const selectSql16 = sql`
-- @db: db_mysql
SELECT IFNULL(name, 'defaultName') AS name FROM factions;
`

// SELECT IFNULL with VARCHAR type
const selectSql17 = sql`
-- @db: db_mysql
SELECT IFNULL(rarity, 'common') AS item_rarity FROM items;
`

// SELECT IFNULL with INT type
const selectSql18 = sql`
-- @db: db_mysql
SELECT IFNULL(quantity, 0) AS qty FROM inventory;
`

// SELECT IFNULL with SMALLINT type
const selectSql19 = sql`
-- @db: db_mysql
SELECT IFNULL(level, 1) AS character_level FROM characters;
`

// SELECT IFNULL with DOUBLE type
const selectSql20 = sql`
-- @db: db_mysql
SELECT IFNULL(gold, 0.0) AS gold_amount FROM characters;
`

// SELECT IFNULL with TEXT type
const selectSql21 = sql`
-- @db: db_mysql
SELECT IFNULL(description, 'No description') AS description FROM factions;
`

// SELECT COALESCE with multiple arguments
const selectSql22 = sql`
-- @db: db_mysql
SELECT COALESCE(rarity, flavor_text, 'unknown') AS item_info FROM items;
`

// SELECT COALESCE with VARCHAR column
const selectSql23 = sql`
-- @db: db_mysql
SELECT COALESCE(name, 'Unknown Character') AS char_name FROM characters;
`

// SELECT COALESCE with INT column
const selectSql24 = sql`
-- @db: db_mysql
SELECT COALESCE(required_level, 1) AS quest_level FROM quests;
`

// SELECT NULLIF with VARCHAR
const selectSql25 = sql`
-- @db: db_mysql
SELECT NULLIF(name, 'default') AS guild_name FROM guilds;
`

// SELECT NULLIF with INT
const selectSql26 = sql`
-- @db: db_mysql
SELECT NULLIF(quantity, 0) AS inv_quantity FROM inventory;
`

// SELECT NVL with VARCHAR (Oracle-style, but should work)
const selectSql27 = sql`
-- @db: db_mysql
SELECT IFNULL(guild_rank, 'Member') AS guild_rank FROM guild_members;
`

// SELECT IFNULL with compound identifier
const selectSql28 = sql`
-- @db: db_mysql
SELECT IFNULL(items.name, 'Unknown Item') AS item_name
FROM items;
`

// SELECT IFNULL with JOIN
const selectSql29 = sql`
-- @db: db_mysql
SELECT
  IFNULL(items.name, 'No item') AS item_name,
  IFNULL(inventory.quantity, 0) AS qty
FROM items
JOIN inventory ON items.inventory_id = inventory.id;
`

// SELECT multiple type-polymorphic functions in one query
const selectSql30 = sql`
-- @db: db_mysql
SELECT
  IFNULL(name, 'Unknown') AS char_name,
  COALESCE(level, 1) AS lvl,
  NULLIF(gold, 0) AS non_zero_gold
FROM characters;
`
