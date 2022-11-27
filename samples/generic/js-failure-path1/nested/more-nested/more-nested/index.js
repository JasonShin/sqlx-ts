import { sql } from "sqlx-ts";

/////////////////
// expressions //
/////////////////

const query1 = sql`SELECT * FROM nested_unknown1;`;

///////////////
// functions //
///////////////

function test() {
  const name = "sqlx-ts";
  const query3 = sql`
        SELECT * FROM nested_unknown2;
    `;

  return sql`
        INSERT INTO
    nested_unknown3 (food_type, time_takes_to_cook, table_id, points)
    VALUES ('sushi', 1, 1, 20);
    `;
}
