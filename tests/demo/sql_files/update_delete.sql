-- @name: update query
UPDATE items SET name = $1 WHERE id = $2 RETURNING id, name;

-- @name: delete query
DELETE FROM items WHERE id = $1 RETURNING id;
