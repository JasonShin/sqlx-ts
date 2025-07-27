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

