---
sidebar_position: 5
---

# Annotations for overrides

Annotation is a way to configure custom settings for the type generator. Here are the supported annotations

| Annotation      | Description                                                                                                                     |
|-----------------|---------------------------------------------------------------------------------------------------------------------------------|
| @name           | name to be used when generating types. It will override the default name (variable name) and it will be formatted in camelcase. |
| @db             | name of the database you'd like to use for the query. By default it will use `default`                                          |
| @result [name]  | override result type of a field in a query                                                                                      |
| @param  [index] | override parameter type of a field in a query                                                                                   |

```typescript
// example
const someQuery = sql`
-- @name: simpleQuery
-- @db: mysql
SELECT *
FROM items;
`
```

Above example query will generate following types

```typescript
export type SimpleQueryParams = []

export interface ISimpleQueryResult {
    id: string
    points: number
}

export interface ISimpleQueryQuery{
    params: SimpleQueryParams;
    result: ISimpleQueryResult;
}
```

Note that types are generated with `SimpleQuery` namespace instead of the variable name `SomeQuery`. This is the result of setting `@name` annotation
in the query. Also, it will use `mysql` database connection that you configured in `.sqlxrc.json` as a result of setting `@db`.

# Overrides

SQLX-TS cannot generate typing for complex SQL syntax such as JSON functions. In this scenario, sqlx-ts will return `any` and you can use annotations
to override the result to a type that you anticipate.

## Supported types

Type override supports these types `string`, `number`, `boolean`, `object`, `null`, `any`, and `never`

## Overriding results

```typescript
const someQuery = sql`
-- @result points: number | null
SELECT
    id,
    JSON_EXTRACT(items.c, "$.points") as points
FROM items
`
```

would generate

```typescript
export type SimpleQueryParams = []

export interface ISimpleQueryResult {
    id: string
    points: number
}

export interface ISimpleQueryQuery{
    params: SimpleQueryParams;
    result: ISimpleQueryResult;
}
```

It's important that you give it an alias so SQLX-TS can match it with your custom annotation.


> Rust provides pattern matching that enforces you to handle all patterns of an enum.
> Based on this, we can exhaustively handle all SQL syntax and narrow down the patterns that SQLX-TS cannot handle.
> If you ever use following SQL syntax, it is encouraged to override the type using @result annotation.

## Overriding params

Overriding generated type for a param works by adding an annotation `-- @param <index>: type`.
- `index` is the position of the query parameter within your SQL
- type is the type to override

### MySQL

If you have a MySQL query like following, you can override param types like in the example below

```typescript
const someQuery = sql`
-- @db: mysql
-- @param 1: number
-- @param 2: string
SELECT
    id, points
FROM items
WHERE points < ?
AND name = ?
`
```

it would generate the following type definitions

```typescript
export type SimpleQueryParams = [number, string]

export interface ISimpleQueryResult {
    id: string
    points: number
}

export interface ISimpleQueryQuery{
    params: SimpleQueryParams;
    result: ISimpleQueryResult;
}
```

### Postgres

If you have a Postgres query like following, you can override param types like in the example below

```typescript
const someQuery = sql`
-- @db: postgres
-- @param 1: number
-- @param 2: string
SELECT
    id, points
FROM items
WHERE points < $2
AND name = $1
`
```
it would generate the following type definitions

```typescript
export type SimpleQueryParams = [string, number]

export interface ISimpleQueryResult {
    id: string
    points: number
}

export interface ISimpleQueryQuery{
    params: SimpleQueryParams;
    result: ISimpleQueryResult;
}
```

It will respect the order of query parameters set by you `$1` and `$2`, and generate the params in the order that it detects
