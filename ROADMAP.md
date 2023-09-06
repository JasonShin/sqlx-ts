
##### Beta Release (< v1.0.0)

- [x] swc to parse JavaScript/TypeScript
- [x] pick all sql`` tagged template literals via parser logic
- [x] compilation success if there are no errors detected in SQLs
- [x] compilation failure if any error is detected in SQLs
- [x] CLI support for sqlx-ts binary - supporting parameters for folder source, database credentials and etc
- [x] Support for MySQL and PostgresSQL
- [x] Publish an NPM module with `sql` tagged template literal. It shouldn't do anything special but return the raw SQL back again


##### Release > v1.0.0

- [ ] JOIN table field name generation error
    - If you provide a join query that potentially has conflict in generated name via sqlx-ts, it should raise errors to indicate that the type generation has failed and provide an assistance to the users on adding aliases to the field names that are conflicting
- [ ] Support for multiple database connections to difference DBs at once
- [ ] Run sqlx-tx-core multi-threaded and check multiple files at once
    - [ ] at this point we should try bench-marking performance difference

##### Features backlog

- [ ] SQLite support
- [ ] MSSQL support

##### Phase@next

- [ ] Support for lazy loaded sqlx-ts
```javascript
function lazyLoaded() {
    const { sql: lazySqlx } = require('sqlx-ts')
} 
```
In the next phase, we will add a support to run SQLX against lazy loaded modules
