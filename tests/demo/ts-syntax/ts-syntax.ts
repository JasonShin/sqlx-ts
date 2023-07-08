
/*
import { sql } from 'sqlx-ts'
import { QueryTypes, Sequelize } from 'sequelize'

// Array expression with sql, it should skip generating the type as we cannot figure out the name to use
const [] = sql`SELECT * FROM items`

// Expression without variable declaration
sql`
-- @name: testQuery1
SELECT * FROM items
`

/////////////////
// expressions //
/////////////////

const query0 = sql`
SELECT id, points
FROM items;
`;

const queryCompount = sql`
SELECT items.id, tables.id
FROM items
JOIN tables ON items.table_id = tables.id;
`;

const query1 = sql`SELECT * FROM items;`;
// variable de
const query2 = sql`
   SELECT * FROM items;
`;

///////////////
// functions //
///////////////
function test() {
  const query3 = sql`
        SELECT * FROM items;
    `;

  return sql`
    -- @name: testQuery
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

  throw sql`
    -- @name: testQuery
    SELECT * FROM items
    `;
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

///////////////////////////////////
// Interface, type, enum, module //
///////////////////////////////////

interface TestInterface {
  name: string;
}

type TestType = number;

enum TestEnum {
  a,
  b,
  c,
}

module TestModule {
}

///// Sequelize /////
const sequelize = new Sequelize('postgres://127.0.0.1')
*/

import { QueryTypes, Sequelize } from 'sequelize'
import { sql } from 'sqlx-ts'

async function zz() {
  const sequelize = new Sequelize('postgres://')
  const result = await sequelize.query(sql`
    -- @name: testSequelizeQuery
    SELECT * FROM items
    WHERE id = $1;
  `!, {
    type: QueryTypes.SELECT,
    replacements: [],
  })
}


///// Empty /////
// const empty = sql``
