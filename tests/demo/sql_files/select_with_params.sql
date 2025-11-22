-- @name: select with params
SELECT id, name, rarity FROM items WHERE rarity = $1;
