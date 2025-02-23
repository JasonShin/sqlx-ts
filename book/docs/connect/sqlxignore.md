# Ignore files

you can use `.sqlxignore` file in order to ignore certain files from being processed by SQLX-TS.

The `.sqlxignore` file must be located at the same directory as you run `sqlx-ts` CLI. Typically the file should live at the same level as _.gitignore._

By default, `*.queries.ts` and `*.queries.js` files are ignored.

You can include more files in the ignore list by creating `.sqlxignore`

```
*.queries.ts
*.something.*
```

`*` ignores all matching patterns.
- The first one ignores all files with `.queries.ts` extension
- The second one ignores all files with the string  `.something.` in the filename
