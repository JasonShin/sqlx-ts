
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

##### Phase@next

- [ ] Support for lazy loaded sqlx-ts
```javascript
function lazyLoaded() {
    const { sql: lazySqlx } = require('sqlx-ts')
} 
```
In the next phase, we will add a support to run SQLX against lazy loaded modules
