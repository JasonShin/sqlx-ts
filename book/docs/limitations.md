---
sidebar_position: 6
---

# Limitations

The page aims to list down limitations of SQLX-TS. So for the users who are interested in using the tool, you have a clear idea what are the current limitations. However, as we evolve the tools, some of the limitations would be fixed in the future releases.

### 1. parsing of SQL is done using sqlparser-rs and any bugs in this modules would be inherited

[sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs) is an essential module of SQLX-TS in order to process SQLs into Typescript type definitions. As a result, any bug in this module will be inherited to sqlx-ts and we will need an update in the module in order to fix the problem. So far, sqlparser-rs is well maintained and being updated in the recent days.
