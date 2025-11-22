-- @name: insert query
INSERT INTO items (name, rarity) VALUES ($1, $2) RETURNING id, name;
