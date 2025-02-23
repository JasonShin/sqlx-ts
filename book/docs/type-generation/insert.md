---
sidebar_position: 2
---

# INSERT statements

To read more about how sqlx-ts translates query parameters, [visit this page](/type-generation#capabilities)

## MySQL

Query params within an insert statement can be converted into TypeScript types as well

```typescript
const someQuery = sql`
INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
    (?, ?, ?, ?, ?),
    (?, ?, ?, ?, ?);
`
```

would generate following

```typescript
export type SomeQueryParams = [
    [number, string, number, number, number],
    [number, string, number, number, number]
]

export interface ISomeQueryQuery {
    params: SomeQueryParams
    result: null
}
```

it also supports type generation for RETURNING statement

```typescript
const insertWildcard = sql`
INSERT INTO items
VALUES (1, 'sword', 'epic', 'test', null)
RETURNING *;
`
```

generates the following result type

```typescript
export interface IInsertWildcardResult {
    flavorText: string | null;
    id: number;
    inventoryId: number | null;
    name: string;
    rarity: string | null;
};
```

## PostgreSQL

As query params of PostgresSQL uses the numbered parameters, it's meaningless to generate a nested array respresnetation of them.

If you have

```typescript
const somePostgresInputQuery = sql`
INSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)
VALUES
($2, $1, 2, $3, 2),
($5, 'test', $4, $7, $6);
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
