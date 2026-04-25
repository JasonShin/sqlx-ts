import { sql } from 'sqlx-ts'
import {
    IGetAllItemsResult,
    IGetItemsByFoodTypeResult, GetItemsByFoodTypeParams,
    InsertItemParams, IInsertItemResult,
    UpdateItemParams, IUpdateItemResult,
    DeleteItemParams, IDeleteItemResult,
    IGetEventsWithScoreResult, GetEventsWithScoreParams,
    IGetItemsWithTableResult,
} from './index.queries'

// SELECT all items
const getAllItems = sql`
    -- @name: getAllItems
    -- @db: db_sqlite
    SELECT * FROM items;
`

// SELECT with WHERE clause using ? placeholder
const getItemsByFoodType = sql`
    -- @name: getItemsByFoodType
    -- @db: db_sqlite
    SELECT * FROM items WHERE food_type = ?;
`

// INSERT with parameters
const insertItem = sql`
    -- @name: insertItem
    -- @db: db_sqlite
    INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
    VALUES (?, ?, ?, ?);
`

// UPDATE with parameters
const updateItem = sql`
    -- @name: updateItem
    -- @db: db_sqlite
    UPDATE items SET food_type = ? WHERE id = ?;
`

// DELETE with parameter
const deleteItem = sql`
    -- @name: deleteItem
    -- @db: db_sqlite
    DELETE FROM items WHERE food_type = ?;
`

// SELECT with JOIN
const getItemsWithTable = sql`
    -- @name: getItemsWithTable
    -- @db: db_sqlite
    SELECT items.id, items.food_type, tables.number as table_number
    FROM items
    JOIN tables ON items.table_id = tables.id;
`

// Query with multiple types (nullable, boolean, real, datetime, json)
const getEventsWithScore = sql`
    -- @name: getEventsWithScore
    -- @db: db_sqlite
    SELECT * FROM events WHERE score > ?;
`
