import { sql } from 'sqlx-ts'

// Simulating Effect-like API for demonstration
class Effect<T> {
  static succeed<T>(value: T): Effect<T> {
    return new Effect(value)
  }

  constructor(private value: T) {}

  pipe<U>(fn: (value: T) => U): Effect<U> {
    return new Effect(fn(this.value))
  }

  map<U>(fn: (value: T) => U): Effect<U> {
    return new Effect(fn(this.value))
  }
}

// Issue #224: Method chaining with .pipe() - these should be recognized
const methodChainSingle = Effect.succeed(sql`
-- @name: method chain single
SELECT name FROM user
`).pipe((str) => str)

const methodChainMultiple = Effect.succeed(sql`
-- @name: method chain multiple
SELECT id, email FROM accounts
`)
  .pipe((str) => str)
  .pipe((x) => x)

const methodChainWithMap = Effect.succeed(sql`
-- @name: method chain with map
SELECT * FROM products WHERE active = true
`).map((result) => result)

// Nested method chains
const nestedChain = Effect.succeed(
  Effect.succeed(sql`
-- @name: nested chain
SELECT id, status FROM orders
`).pipe((x) => x)
).pipe((y) => y)

// Promise-like chaining
const promiseChain = Promise.resolve(sql`
-- @name: promise chain
SELECT username, created_at FROM users
`)
  .then((result) => result)
  .catch((err) => err)

// Array method chaining
const arrayChain = [sql`
-- @name: array chain
SELECT * FROM items WHERE category = 'electronics'
`]
  .map((x) => x)
  .filter((x) => x)

// Complex expression with method chain
const complexChain = (
  Effect.succeed(sql`
-- @name: complex chain
SELECT price, discount FROM sales
  `).pipe((data) => data)
)

// Method chain in ternary operator
const ternaryChain = true
  ? Effect.succeed(sql`
-- @name: ternary chain
SELECT * FROM events WHERE date > NOW()
    `).pipe((x) => x)
  : null
