---
sidebar_position: 3
---

# UPDATE statements

## MySQL

Query params within an update statement can be converted into TypeScript types as well

```typescript
const someQuery = sql`
UPDATE items
JOIN tables ON tables.id = items.table_id
SET items.food_type = ?
WHERE tables.id = ?
`
```

would generate following

```typescript
export type SomeQueryParams = [string, number]

export type SomeQueryResult = number

export interface ISomeQueryQuery {
    params: SomeQueryParams
    result: SomeQueryResult
}
```

## PostgreSQL

As query params of PostgresSQL uses the numbered parameters, it's meaningless to generate a nested array respresnetation of them.

If you have

```typescript
const someQuery = sql`
UPDATE items
JOIN tables ON tables.id = items.table_id
SET items.food_type = $2
WHERE tables.id = $1
`
```

Above query will generate the following typings

```typescript
export type SomePostgresInputQueryParams = [string, number, number, number, number, number, number];

export interface ISomePostgresInputQueryResult {
    
};

export interface ISomePostgresInputQueryQuery {
    params: SomePostgresInputQueryParams;
    result: ISomePostgresInputQueryResult;
};
```
