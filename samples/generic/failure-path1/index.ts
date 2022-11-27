import { sql } from "sqlx-ts";

// Querying from an unknown table
const someQuery = sql`SELECT * FROM indexjs_unknown`;

// Inserting more values than expected
const insertQuery = sql`
    INSERT INTO items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('steak', 1, 1, 1, 1);
`;

///////////////////
// If statements //
///////////////////
if (true) {
  const query3 = sql`SELECT * FROM if_statement1;`;
}

function testIfStatement() {
  if (true) {
    const query3 = sql`SELECT * FROM if_statement2;`;
  }
}

//////////////////////
// Switch Statement //
//////////////////////

switch (true) {
  case true:
    const query4 = sql`SELECT * FROM switch_statements1`;
    break;
  default:
    const query5 = sql`SELECT * FROM switch_statements2`;
}

///////////////
// For loops //
///////////////

for (let i = 0; i < 10; i++) {
  const query3 = sql`SELECT * FROM for_loops1`;
}

const list = [1, 2, 3];
for (let n in list) {
  const query3 = sql`SELECT * FROM for_loops2`;
}

for (let n of list) {
  const query3 = sql`SELECT * FROM for_loops3`;
}

///////////////
// Try/Catch //
///////////////

try {
  const query3 = sql`SELECT * FROM try1`;
} catch {
  const query3 = sql`SELECT * FROM catch1`;

  throw sql`SELECT * FROM throw1`;
}

////////////////////
// with statement //
////////////////////

function with_stmt(o: string, n: number) {
  // @ts-ignore
with (o) {
        const query3 = sql`SELECT * FROM with1`;
    }
}

/////////////////////
// While Statement //
/////////////////////

let i = 0;
while (i < 5) {
  const query = sql`SELECT * FROM while1`;
  i++;
}

i = 0;
do {
  const query = sql`SELECT * FROM do_while1`;
  i++;
} while (i < 5);

///////////
// Class //
///////////

class Foo {
  private bar() {
    const query = sql`SELECT * FROM class_private1`;
  }

  public baz() {
    const query = sql`SELECT * FROM class_public1`;
  }

  protected qux() {
    const query = sql`SELECT * FROM class_protected1`;
  }
}
