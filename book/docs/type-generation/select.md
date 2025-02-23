---
sidebar_position: 1
---

# SELECT statements

## Binary operations

If you have the follow MySQL query in typescript

```typescript
// example
const someQuery = sql`
SELECT *
FROM items
WHERE points > ?;
```

would generate

```typescript
export type SomeQueryParams = []

export interface ISomeQueryResult {
    id: string
    points: number
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
```

## IN list query

If you have the following MySQL query

```typescript
// example
const someQuery = sql`
SELECT *
FROM items
WHERE id IN (?);
```

would generate following typescript types

```typescript
export type SomeQueryParams = [Array<number>]

export interface ISomeQueryResult {
    id: string
    points: number
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
```

## Subqueries

Query params within subqueries are interpreted as well. If you have the following MySQL query

```typescript
const someQuery = sql`
SELECT id, points
FROM items
WHERE id IN (
    SELECT id FROM items
    WHERE
        points > ?
        AND id IN (SELECT id FROM items WHERE food_type = ?)
)
AND points < ?
`
```

would generate following type definitions

```typescript
export type SomeQueryParams = [number, string, number]

export interface ISomeQueryResult {
    id: string
    points: number
}

export interface ISomeQueryQuery {
    params: SomeQueryParams;
    result: ISomeQueryResult;
}
```

Note that `QueryParams` array respects the order of params present in the query above
