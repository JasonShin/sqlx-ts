<h1 align="center">SQLx-ts</h1>
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
- **Database Agnostic** - support for [PostgreSQL](http://postgresql.org/), [MySQL](https://www.mysql.com/), and [MSSQL](https://www.microsoft.com/en-us/sql-server)

### Installation

The binary name for sqlx-ts is `sqlx-ts`.

[Archives of precompiled binaries of sqlx-ts are available for Windows, macOS and Linux](https://github.com/JasonShin/sqlx-ts/releases). Linux and Windows binaries are static executables. Users of platforms not explicitly mentioned below are advised to download one of these archives.

If you're a **macOS** user, then you can install sqlx-ts from via install.sh:

```bash
$ curl -LSfs https://github.com/JasonShin/sqlx-ts/install.sh | \
    sh -s -- --os darwin
```

If you're a **Windows** user, then you can install sqlx-ts from via install.sh:

```bash
$ curl -LSfs https://github.com/JasonShin/sqlx-ts/install.sh | \
    sh -s -- --os win32
```

If you're a **Linux** user, then you can install sqlx-ts from via install.sh:

```bash
$ curl -LSfs https://github.com/JasonShin/sqlx-ts/install.sh | \
    sh -s -- --os linux
```

To install a specific artifact, [go to the release page to find the exact name of the artifact](https://github.com/JasonShin/sqlx-ts/releases)

```bash
$ curl -LSfs https://github.com/JasonShin/sqlx-ts/install.sh | \
    sh -s -- --artifact sqlx-ts_v0.1.0_x86_64-apple-darwin.tar.gz
```

Upgrading to a new version can be done by grabbing the next version of the sqlx-ts artifact and use `--force` command from install.sh

```bash
$ curl -LSfs https://github.com/JasonShin/sqlx-ts/install.sh | \
    sh -s -- --artifact sqlx-ts_v0.2.0_x86_64-apple-darwin.tar.gz --force
```

For more advanced usage, please check `--help` command of install.sh

```bash
$ curl -LSfs https://github.com/JasonShin/sqlx-ts/install.sh | \
    sh -s --help
```

### ROADMAP

##### Phase 1

- [x] swc to parse JavaScript/TypeScript
- [x] pick all sql`` tagged template literals via parser logic
- [x] compilation success if there are no errors detected in SQLs
- [x] compilation failure if any error is detected in SQLs
- [x] CLI support for sqlx-ts binary - supporting parameters for folder source, database credentials and etc
- [x] Support for MySQL and PostgresSQL
- [x] Publish an NPM module with `sql` tagged template literal. It shouldn't do anything special but return the raw SQL back again

##### Phase 2

- [ ] SQLite support
- [ ] MSSQL support
- [ ] Support for multiple database connections to difference DBs at once
- [ ] Run sqlx-tx-core multi-threaded and check multiple files at once
  - [ ] at this point we should try bench-marking performance difference

<br />

### Motivation

Rust's philosophy of guaranteed compile-time safety of your code has always inspired me. Rust is still new and many seasoned developers would view Rust's increased benefit/cost ratio claims are as yet unproven. However, there are lessons from these new technologies that we can bring back to our everyday languages such as JavaScript and TypeScript. [SQLx](https://github.com/launchbadge/sqlx) is a great example of this, although the idea isn't directly co-related to Rust, but its philosophy well-aligns with Rust's overall ecosystem.
