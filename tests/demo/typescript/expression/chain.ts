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

function optionalParent(): null | { child: (string) => null } {
    return {
        child: function (sqlString: string) {
            return null
        }
    }
}

optionalParent()?.child(sql`
-- @name: optional parent child
SELECT id FROM items
`)