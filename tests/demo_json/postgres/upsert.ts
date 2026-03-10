import { sql } from 'sqlx-ts'

// INSERT ON CONFLICT DO NOTHING
const upsertDoNothing = sql`
-- @name: upsert do nothing
INSERT INTO items (id, name, rarity)
VALUES ($1, $2, $3)
ON CONFLICT (id) DO NOTHING
`

// INSERT ON CONFLICT DO UPDATE
const upsertDoUpdate = sql`
-- @name: upsert do update
INSERT INTO items (id, name, rarity)
VALUES ($1, $2, $3)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name, rarity = EXCLUDED.rarity
`

// INSERT ON CONFLICT with WHERE clause
const upsertWithWhere = sql`
-- @name: upsert with where
INSERT INTO items (id, name, rarity)
VALUES ($1, $2, $3)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name
WHERE items.rarity != 'legendary'
`

// INSERT ON CONFLICT with RETURNING
const upsertWithReturning = sql`
-- @name: upsert with returning
INSERT INTO items (id, name, rarity)
VALUES ($1, $2, $3)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name, rarity = EXCLUDED.rarity
RETURNING *
`
