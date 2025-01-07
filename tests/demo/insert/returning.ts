import { sql } from 'sqlx-ts'

const insertWildcard = sql`
INSERT INTO items
VALUES (1, 'sword', 'epic', 'hmm', null)
RETURNING *;
`

const insertSelector = sql`
INSERT INTO items
VALUES (1, 'sword', 'epic', 'hmm', null)
RETURNING id, rarity;
`

const insertAlias = sql`
INSERT INTO items
VALUES (1, 'sword', 'epic', 'hmm', 1)
RETURNING id as id1, rarity as Rarity1;
`

const insertQuoted = sql`
INSERT INTO items
VALUES (1, 'sword', 'epic', 'hmm', 1)
RETURNING "id" as id1, rarity as "Rarity1";
`


const insertParams = sql`
INSERT INTO items (id, name, rarity, flavor_text, inventory_id)
VALUES (1, 'sword', 'epic', 'hmm', 1)
RETURNING "id" as id1, rarity as "Rarity1";
`
