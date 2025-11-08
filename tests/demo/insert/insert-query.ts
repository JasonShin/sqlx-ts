import { sql } from 'sqlx-ts'

const insertWithWildcard = sql`
INSERT INTO characters
SELECT * FROM characters
WHERE id = $1;
`

const insertWithSelector = sql`
INSERT INTO characters
SELECT id, name FROM characters
WHERE id = $1;
`

const insertWithAlias = sql`
INSERT INTO characters
SELECT id as character_id, name as character_name FROM characters
WHERE id = $1;
`

const insertWithQuoted = sql`
INSERT INTO characters
SELECT "id" as "character_id", "name" as "character_name" FROM characters
WHERE id = $1;
`

const insertWithParams = sql`
INSERT INTO characters (id, name)
SELECT id, name FROM characters
WHERE id = $1;
`

const insertWithParamsAndAlias = sql`
INSERT INTO characters (id, name)
SELECT id as character_id, name as character_name FROM characters
WHERE id = $1;
`

const insertWithParamsAndQuoted = sql`
INSERT INTO characters ("id", "name")
SELECT "id" as "character_id", "name" as "character_name" FROM characters
WHERE id = $1;
`

// Uncomment below once CTE features are supported
// // INSERT with DELETE...RETURNING (PostgreSQL feature)
// // Moves records from one table to another based on a condition
// const insertFromDelete = sql`
// WITH deleted AS (DELETE FROM characters WHERE id = $1 RETURNING *)
// INSERT INTO characters SELECT * FROM deleted;
// `
//
// const insertFromDeleteWithColumns = sql`
// WITH deleted AS (DELETE FROM characters WHERE id = $1 RETURNING id, name)
// INSERT INTO characters (id, name) SELECT id, name FROM deleted;
// `
//
// // INSERT with UPDATE...RETURNING (PostgreSQL feature)
// // Can insert the results of an update operation
// const insertFromUpdate = sql`
// WITH updated AS (UPDATE characters SET name = $2 WHERE id = $1 RETURNING *)
// INSERT INTO characters SELECT * FROM updated;
// `
//
// const insertFromUpdateWithColumns = sql`
// WITH updated AS (UPDATE characters SET name = $2 WHERE id = $1 RETURNING id, name)
// INSERT INTO characters (id, name) SELECT id, name FROM updated;
// `

