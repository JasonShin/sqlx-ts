import { sql } from 'node'

const someQuery = sql`SELECT * FROM items`;

function test() {
    function test2() {
        const insertQuery = sql`
            INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
            VALUES ('steak', 1, 1, 1, true);
        `
    }
}

