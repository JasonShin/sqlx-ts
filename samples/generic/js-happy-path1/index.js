import { sql } from "sqlx-ts";

// Querying from an unknown table
const someQuery = sql`SELECT * FROM items`;

// Inserting more values than expected
const insertQuery = sql`
    INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 1);
`;

///////////////
// functions //
///////////////
function test() {
  const query3 = sql`
        SELECT * FROM items;
    `;

  return sql`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 20);
    `;
}

///////////////////
// If statements //
///////////////////
if (true) {
  const query3 = sql`SELECT * FROM items;`;
}

function testIfStatement() {
  if (true) {
    const query3 = sql`SELECT * FROM items;`;
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

const list = [1, 2, 3];
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

  throw sql`SELECT * FROM items`;
}

/////////////////////
// While Statement //
/////////////////////

let i = 0;
while (i < 5) {
  const query = sql`SELECT * FROM items`;
  i++;
}

i = 0;
do {
  const query = sql`SELECT * FROM items`;
  i++;
} while (i < 5);
