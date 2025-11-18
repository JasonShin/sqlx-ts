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
SELECT id FROM items
`).pipe((str) => str)

const methodChainMultiple = Effect.succeed(sql`
SELECT id FROM items
`)
  .pipe((str) => str)
  .pipe((x) => x)

const methodChainWithMap = Effect.succeed(sql`
-- @name: method chain with map
SELECT id FROM items
`).map((result) => result)

// Nested method chains
const nestedChain = Effect.succeed(
  Effect.succeed(sql`
-- @name: nested chain
SELECT id FROM items
`).pipe((x) => x)
).pipe((y) => y)

// Promise-like chaining
const promiseChain = Promise.resolve(sql`
-- @name: promise chain
SELECT id FROM items
`)
  .then((result) => result)
  .catch((err) => err)

// Array method chaining
const arrayChain = [sql`
-- @name: array chain
SELECT id FROM items
`]
  .map((x) => x)
  .filter((x) => x)

// Complex expression with method chain
const complexChain = (
  Effect.succeed(sql`
-- @name: complex chain
SELECT id FROM items
  `).pipe((data) => data)
)

// Method chain in ternary operator
const ternaryChain = true
  ? Effect.succeed(sql`
-- @name: ternary chain
SELECT id FROM items
    `).pipe((x) => x)
  : null
