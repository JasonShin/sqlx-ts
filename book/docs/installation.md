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

From v0.50.0, the `sqlx-ts` binary comes as an optional dependency for your platform (e.g. `sqlx-ts-darwin-arm64`), so
Bun and Deno install it the same way as npm

```bash
# Bun
$ bun add --dev sqlx-ts
$ bunx sqlx-ts --version
# Deno
$ deno add --dev npm:sqlx-ts
$ deno run -A npm:sqlx-ts --version
```

The binary is not installed with `--omit=optional` or `--no-optional`. In that case, or on a platform without a prebuilt
binary, install the binary to a directory on your PATH with install.sh and `sqlx-ts` will use it

```bash
# --os: darwin | linux | win32, --cpu: arm64 | x64 | x32
$ curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os darwin --cpu arm64 --to ~/.local/bin
```

v0.49.0 and older download the binary in a `postinstall` step, which Bun and Deno do not run. For these versions, install
the binary with install.sh as above, or with Bun add sqlx-ts to `trustedDependencies` in package.json

```json
{
  "trustedDependencies": ["sqlx-ts"]
}
```

🚀 you now have `sqlx-ts` installed 🚀