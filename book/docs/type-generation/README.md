# TypeScript Types Generation

SQLX-TS provides powerful TypeScript type generation based on the raw SQL queries in your code. This feature significantly enhances type 
safety, eliminating the need for traditional ORMs.

Without automatic type generation, developers must manually maintain type definitions for each raw SQL query—a fragile and error-prone 
process that adds unnecessary overhead, especially in large projects. SQLX-TS streamlines this by ensuring type correctness directly from 
your queries, reducing maintenance and improving reliability.

## Getting started

The easiest way to enable type generation in SQLX-TS is by using the `-g` or `--generate-types` flag when running the SQLX-TS CLI.
It can be configured through configuration file as well, [click here to learn more](/connect/config-file)

```bash
$ cargo run ./samples/type-generator-test --generate-types --config=<path to .sqlxrc.json>
```

By default, sqlx-ts will generate type

[Please read more about .sqlxrc.json here](./2.1.configs-file-based.md)

## What's possible

sqlx-ts supports following type generations and other SQL syntax will be ignored from type generation

- SELECT statements
- INSERT statements
- UPDATE statements
- DELETE statements

Also type generation supports parameterised query as per the requirements of PREPARE statements of the databases
that sqlx-ts currently supports.
- [MySQL PREPARE](https://dev.mysql.com/doc/refman/8.0/en/prepare.html)
- [Postgres PREPARE](https://www.postgresql.org/docs/current/sql-prepare.html)

| Database | Parameterised Query | Example                                               |
|----------|---------------------|-------------------------------------------------------|
| MySQL    | ?                   | SELECT * FROM items WHERE points > ? AND points < ?   |
| Postgres | $1 $2 $3            | SELECT * FROM items WHERE points > $1 AND points < $2 |

in your codebase, if you have the following SQL query

```typescript
const simpleQuery = sql`
SELECT *
FROM items
WHERE points > ?
AND points < ?
`
```

by running sqlx-ts type generation against the query, it would generate

```typescript
export type ISimpleQueryParams = [number, number]

export interface ISimpleQueryResult {
    id: string
    points: number
}

export interface ISimpleQueryQuery{
    params: ISimpleQueryParams;
    result: ISimpleQueryResult;
}
```

sqlx-ts will pick up name of the variable and use it when generating the type definitions. You can override
the type name by setting the `@name` annotation, you can read more in the section [below](#annotations).

## Q/A

##### Why doesn't SQLx support named parameterised queries? `e.g. SELECT * FROM items WHERE point = :point`

We believe that there is no official way in Typescript to ensure the value level type-safety yet and providing named parameter does not guarantee
the true type safetiness that we are trying to achieve. Furthermore, name parameter is not the syntax supported by native database drivers, but
they are an additional syntax supported by popular ORM libraries such as Sequelize.

The closest type safety that exists in TypeScript world is by emulating Opaque type e.g. https://github.com/sindresorhus/type-fest/blob/main/source/opaque.d.ts.
In the future, sqlx-ts will support Opaque type overrides natively to solve this matter.

