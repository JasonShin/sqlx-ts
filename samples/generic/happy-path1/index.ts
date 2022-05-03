import { sql } from 'sqlx-ts'
/////////////////
// expressions //
/////////////////

const query1 = sql`SELECT * FROM items;`
// variable de
const query2 = sql`
   SELECT * FROM items;
`

///////////////
// functions //
///////////////
function test() {
    const query3 = sql`
        SELECT * FROM items;
    `

    return sql`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 20);
    `
}

///////////////////
// If statements //
///////////////////
if (true) {
    const query3 = sql`SELECT * FROM items;`
}

function testIfStatement() {
    if (true) {
        const query3 = sql`SELECT * FROM items;`
    }
}

//////////////////////
// Switch Statement //
//////////////////////

switch (true) {
    case true:
        const query4 = sql`SELECT * FROM items`;
        break;
    default:
        const query5 = sql`SELECT * FROM items`;
}

///////////////
// For loops //
///////////////

for (let i = 0; i < 10; i++) {
    const query3 = sql`SELECT * FROM items`;
}

const list = [1, 2, 3]
for (let n in list) {
    const query3 = sql`SELECT * FROM items`;
}

for (let n of list) {
    const query3 = sql`SELECT * FROM items`;
}

///////////////
// Try/Catch //
///////////////

try {
    const query3 = sql`SELECT * FROM items`;
} catch {
    const query3 = sql`SELECT * FROM items`;

    throw sql`SELECT * FROM items`
}

////////////////////
// with statement //
////////////////////

function with_stmt(o: string, n: number) {
    // @ts-ignore
    with (o) {
        const query3 = sql`SELECT * FROM items`;
    }
}

/////////////////////
// While Statement //
/////////////////////

let i = 0
while (i < 5) {
    const query = sql`SELECT * FROM items`;
    i++;
}


i = 0;
do {
    const query = sql`SELECT * FROM items`;
    i++;
} while (i < 5)

///////////
// Class //
///////////

class Foo {
    private bar() {
        const query = sql`SELECT * FROM items`;
    }

    public baz() {
        const query = sql`SELECT * FROM items`;
    }

    protected qux() {
        const query = sql`SELECT * FROM items`;
    }
}
