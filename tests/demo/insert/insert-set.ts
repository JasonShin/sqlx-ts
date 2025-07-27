import { sql } from 'sqlx-ts'

const insertWithUnionSet = sql`
    INSERT INTO characters (id, name, gold)
    SELECT id, name, gold FROM characters
    WHERE id = $1
    UNION
    SELECT id, name, gold FROM characters
    WHERE id = $2;
`

const insertWithUnionSetAndAlias = sql`
INSERT INTO characters (id, name, gold)
SELECT id as character_id, name as character_name, gold FROM characters
WHERE id = $1
UNION
SELECT id as character_id, name as character_name, gold FROM characters
WHERE id = $2;
`

const insertWithUnionSetAndQuoted = sql`
INSERT INTO characters ("id", "name", "gold")
SELECT "id" as "character_id", "name" as "character_name", "gold" FROM characters
WHERE id = $1
UNION
SELECT "id" as "character_id", "name" as "character_name", "gold" FROM characters
WHERE id = $2;
`

const insertWithUnionSetAndParams = sql`
INSERT INTO characters (id, name, gold)
SELECT id, name, gold FROM characters
WHERE id = $1
UNION
SELECT id, name, gold FROM characters
WHERE id = $2;
`

const insertWithUnionSetAndParamsAndAlias = sql`
INSERT INTO characters (id, name, gold)
SELECT id as character_id, name as character_name, gold FROM characters
WHERE id = $1
UNION
SELECT id as character_id, name as character_name, gold FROM characters
WHERE id = $2;
`

const insertWithUnionSetAndParamsAndQuoted = sql`
INSERT INTO characters ("id", "name", "gold")
SELECT "id" as "character_id", "name" as "character_name", "gold" FROM characters
WHERE id = $1
UNION
SELECT "id" as "character_id", "name" as "character_name", "gold" FROM characters
WHERE id = $2;
`
