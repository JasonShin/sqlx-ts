import { sql } from 'sqlx-ts'

while (true) {
    const whileSql = sql`SELECT id FROM items WHERE $1;`
}

let i = 0

do {
    const query = sql`SELECT id FROM items`;
    i++;
} while (i < 5);
