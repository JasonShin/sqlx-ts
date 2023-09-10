import { sql } from 'sqlx-ts'

if (true) {
    const ifstmt = sql`SELECT id FROM items;`;
}

function testIfStatement() {
    if (true) {
        const nestedIfStmt = sql`SELECT id FROM items;`;
    }
}
