import { sql } from 'sqlx-ts'
import { QueryTypes, Sequelize } from 'sequelize'

// Array expression with sql, it should skip generating the type as we cannot figure out the name to use
const [] = sql`
-- @name: testQueryWithoutVariableDeclaration
SELECT * FROM items
`

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


async function demo() {
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

const arr = [sql`
-- @name: arrayQuery  
SELECT * FROM items
`]

const obj = {
  a: sql`
  -- @name: anotherTestObjectQuery  
  SELECT * FROM items
  `,
  b: {
    c: sql`
    -- @name: nestedTestObjectQuery
    SELECT * FROM items
    `
  }
}

const tpl = `${sql`
-- @name: tplQuery
SELECT * FROM items
`}`

const arrow = () => sql`
-- @name: arrowQuery
SELECT * FROM items
`

class TestClass {
  private sql1 = sql`
  -- @name: testClassPropertyQuery
    SELECT * FROM items
  `
  constructor(z: string) {
    const query = sql`
    -- @name: testClassConstructorQuery
    SELECT * FROM items
    `
  }

  someMethod() {
    const query = sql`
    -- @name: testClassMethodQuery
    SELECT * FROM items
    `
  }
}

class ChildClass extends TestClass {
  constructor() {
    super(sql`
      -- @name: testChildClassConstructorQuery
      SELECT * FROM items
    `)
  }

  hmm() {
    let z = this.someMethod
  }
}

interface TestInterface {
  sql1: string
  get sql2(): string
  set sql3(value: string)
}

module TestModule {
  const moduleSql = sql`SELECT * FROM items`
}

let name: any = 'test'
let companyName = <string>name
let partnerName = name as string
let someName = 'test' as const

(
  sql`
  -- @name: testParenthesisQuery
  SELECT * FROM items
  `
)

const something = false
const somethingElse = something ?? sql`
-- @name: testNullishCoalescingQuery
SELECT * FROM items
`


function *yieldMethod() {
  yield sql`
    -- @name: testYieldQuery
    SELECT * FROM items
  `
}

(async () => {
  await sql`
    -- @name: testAwaitQuery
    SELECT * FROM items
  `
  await sql`
    -- @name: testAwaitQuery2
    SELECT * FROM items
  `

  const awaitClientQuery = await client.query(sql`
      SELECT * FROM items;
  `)

  const [rows, i] = await connection.execute<Rows<IGetItems2Result>>(sql`
    -- @name: getItemsWithRows
    SELECT * FROM items
  `)

  await connection.execute(sql`
  -- @name: testInsert
  -- @db: db_mysql
  INSERT INTO items (food_type, points, time_takes_to_cook, table_id) VALUES (?, ?, 1, 1);
  `)

  connection.destroy()

})();

// AutoAccessor
class AutoAccessorTest {
  accessor autoAccessorProp: string = sql`
  SELECT * FROM items;
  `
}


const getResource = () => ({
  [Symbol.asyncDispose]: async () => {
    const testAsyncUsing = sql`
    SELECT * FROM items;
    `
  },
});

{
  await using resource = getResource();
}
