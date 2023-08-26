import { sql } from 'sqlx-ts'


switch (true) {
    case true:
      const case1 = sql`SELECT id FROM items`;
      break;
    default:
      const case2 = sql`SELECT id FROM items`;
}
