<h1 align="center">SQLx-ts</h1>

<div align="center">
    <a href='https://coveralls.io/github/JasonShin/sqlx-ts?branch=main'><img src='https://coveralls.io/repos/github/JasonShin/sqlx-ts/badge.svg?branch=main' alt='Coverage Status' /></a>
</div>

<div align="center">
 <strong>
   🧰 The Typescript/Javascript SQL Toolkit
 </strong>
</div>

<br />

<div align="center">
Built to free Node developers from ORMs' unpredictably generated SQL queries
</div>

<br />

SQLx-ts is a CLI application featuring compile-time checked queries without a DSL and prevents broken SQL queries being run during runtime.

- **Compile time checked queries** - never ship a broken SQL query to production
- **TypeScript type generations** - generates type definitions based on the raw SQLs and you can use them with any MySQL or PostgreSQL driver
- **Database Agnostic** - support for [PostgreSQL](http://postgresql.org/) and [MySQL](https://www.mysql.com/)
- **TypeScript and JavaScript** - supports for both [TypeScript](https://www.typescriptlang.org/) and JavaScript

<br>
<div align="center">
    <strong>
    📔 <a href="https://jasonshin.github.io/sqlx-ts/">Official Documentation</a>
    </strong>
</div>
<br>

### Installation

##### Install sqlx-ts npm module (recommended)

If you are using npm
```bash
$ npm install sqlx-ts
```

If you are using yarn
```bash
$ yarn add sqlx-ts
```

You can also install sqlx-ts globally
```bash
$ npm install -g sqlx-ts
```

Installing sqlx-ts using npm also installed `sqlx-ts` binary of the same version as the npm module.
Verify the installation by running

```bash
$ npx sqlx-ts --version
```

And to use sqlx-ts in your code

In TypeScript based projects:

```typescript
import { sql } from 'sqlx-ts'

// ...
const query = sql`SELECT * FROM some_table;`
// ...
```


In Babel based projects:

```javascript
import { sql } from 'sqlx-ts'
const query = sql`SELECT * FROM some_table;`

// ... or

const { sql } = require('sqlx-ts')
const query = sql`SELECT * FROM some_table;`
```

##### Installing binary separately

You may choose to install sqlx-ts separately instead of using `npm i`

###### Using install.sh

The binary name for sqlx-ts is `sqlx-ts`.

[Archives of precompiled binaries of sqlx-ts are available for windows, macOS and Linux](https://github.com/JasonShin/sqlx-ts/releases). Linux and Windows binaries are static executables. Users of platforms not explicitly mentioned below are advised to download one of these archives.

If you're a **macOS** user, then you can install sqlx-ts from via install.sh:

```bash
# macos & ARM CPU
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os darwin --cpu arm64
# macos & X64 CPU
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os darwin --cpu x64
```

If you're a **Windows** user, then you can install sqlx-ts from via install.sh:

```bash
# windows & x32
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os win32 --cpu x32
# windows & x64
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os win32 --cpu x32
```

If you're a **Linux** user, then you can install sqlx-ts from via install.sh:

```bash
# linux & x32
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os linux --cpu x32
# linux & x64
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os linux --cpu x64
# linux & arm
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os linux --cpu arm64
```

To install a specific artifact, [go to the release page to find the exact name of the artifact](https://github.com/JasonShin/sqlx-ts/releases)

```bash
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --artifact sqlx-ts-v0.1.0-macos-arm.zip
```

Upgrading to a new version can be done by grabbing the next version of the sqlx-ts artifact and use `--force` command from install.sh

```bash
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | \
    sh -s -- --artifact ssqlx-ts-v0.1.0-macos-arm.zip --force
```

For more advanced usage, please check `--help` command of install.sh

```bash
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | \
    sh -s -- --help
```

### Motivation

I would like to bring the powerful compile-time safety ideas to Node.js. [sqlx](https://github.com/launchbadge/sqlx) is a great example of this, as it provides compile-time check of SQLs within your Rust code and Rust itself provides a great environment for tools like sqlx. sqlx-ts is greatly inspired by [sqlx](https://github.com/launchbadge/sqlx), but solves additional problems of generating TypeScript interfaces based on the SQL queries that are present in your code.
