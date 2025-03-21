---
sidebar_position: 4
---

# DELETE statement

To read more about how SQLX-TS translates query parameters, [visit this page](/type-generation#capabilities)

## MySQL

Query params within a DELETE statement will be converted Typescript types

```typescript
const someQuery = sql`
DELETE FROM items WHERE id = ?
`
```

would generate following

```typescript
export type SomeQueryParams = [number]

export interface ISomeQueryQuery {
    params: SomeQueryParams
    results: null
}
```

## PostgreSQL

If you the following query in your Typescript code

```typescript
const someQuery = sql`
DELETE FROM items WHERE id = $1;
`
```

would result in

```typescript
export type SomeQueryParams = [number]

export interface ISomeQueryQuery {
    params: SomeQueryParams
    results: null
}
```
