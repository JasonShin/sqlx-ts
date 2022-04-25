import { sql } from 'node'

// Querying from an unknown table
const someQuery = sql`SELECT * FROM itemzz`;

// Inserting more values than expected
const insertQuery = sql`
    INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 1, 1);
`
