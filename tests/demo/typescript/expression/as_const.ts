import { sql } from 'sqlx-ts'

// Basic as const
const basicAsConst = {
  somequery: sql`SELECT id, name FROM items`
} as const

// Function returning as const
const functionReturningAsConst = {
  somequery: () => sql`SELECT id, name FROM items WHERE rarity = $1`
} as const

// Nested as const
const nestedAsConst = {
  queries: {
    item: sql`SELECT id, name FROM items WHERE id = $1`
  }
} as const

// as const with type assertion
const asConstWithTypeAssertion = {
  query: sql`SELECT id, name FROM items`
} as const satisfies Record<string, any>
