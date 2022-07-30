import { sql as aliased } from "sqlx-ts";
/////////////////
// expressions //
/////////////////

const query1 = aliased`SELECT * FROM items;`;
// variable de
const query2 = aliased`
   SELECT * FROM items;
`;

///////////////
// functions //
///////////////

function test() {
  const name = "sqlx-ts";
  const query3 = aliased`
        SELECT * FROM items;
    `;

  return aliased`
        INSERT INTO
    items (food_type, time_takes_to_cook, table_id, points)
    VALUES ('sushi', 1, 1, 20);
    `;
}
