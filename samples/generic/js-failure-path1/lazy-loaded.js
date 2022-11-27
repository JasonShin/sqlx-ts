// TODO: Support lazy loading in the future phases
function test() {
  const { sql: sqlx } = require("sqlx-ts");

  const someQuery = sqlx`SELECT * FROM lazy_unknown1`;

  return someQuery;
}

// Below syntax is not yet supported
function test2() {
  const SQLX = require("sqlx-ts");
  const someQuery = SQLX.sql`SELECT * FROM lazy_unknown2`;

  return someQuery;
}
test();
test2();
