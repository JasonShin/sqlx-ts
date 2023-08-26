import { sql } from 'sqlx-ts'

function parent() {
    return {
        child: function (sqlString: string) {
            return null
        }
    }
}

parent().child(sql`
-- @name: parent child
SELECT id FROM items
`)