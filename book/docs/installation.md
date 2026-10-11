---
sidebar_position: 2
---

# Installation

With npm:

```bash
$ npm install --save-dev sqlx-ts
```

With yarn:

```bash
$ yarn add --dev sqlx-ts
```

Verify the installation of `sqlx-ts` binary by running

```bash
$ npx sqlx-ts --version
```

## Bun and Deno

Bun and Deno do not run the npm `postinstall` step that downloads the `sqlx-ts` binary. Install the npm module for the
`sql` tag, then install the binary to a directory on your PATH with install.sh

```bash
# Bun
$ bun add --dev sqlx-ts
# Deno
$ deno add --dev npm:sqlx-ts

# --os: darwin | linux | win32, --cpu: arm64 | x64 | x32
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os darwin --cpu arm64 --to ~/.local/bin
```

`bunx sqlx-ts` and `sqlx-ts` in package.json scripts then use the binary from your PATH. If the binary is missing, they
print the command to install it.

With Bun, you can instead let it run the `postinstall` step by adding sqlx-ts to `trustedDependencies` in package.json

```json
{
  "trustedDependencies": ["sqlx-ts"]
}
```

🚀 you now have `sqlx-ts` installed 🚀