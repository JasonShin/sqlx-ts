#!/usr/bin/env node
const { spawnSync } = require('child_process')
const fs = require('fs')
const path = require('path')

const platformPackage = `sqlx-ts-${process.platform}-${process.arch}`
const binaryName = process.platform === 'win32' ? 'sqlx-ts.exe' : 'sqlx-ts'

function findInPlatformPackage() {
  try {
    return require.resolve(`${platformPackage}/${binaryName}`)
  } catch {
    return undefined
  }
}

function findOnPath() {
  for (const dir of (process.env.PATH || '').split(path.delimiter)) {
    const candidate = path.join(dir, binaryName)
    if (fs.existsSync(candidate) && !fs.realpathSync(candidate).endsWith('.js')) {
      return candidate
    }
  }
}

const binary = findInPlatformPackage() || findOnPath()

if (!binary) {
  const cpu = process.arch === 'arm64' || process.arch === 'x64' ? process.arch : 'x32'
  console.error(`sqlx-ts binary is not installed. It comes from the optional dependency ${platformPackage}.
If you installed with --omit=optional or --no-optional, or copied node_modules from another OS, reinstall without them.
Otherwise install the binary separately to a directory on your PATH:

  curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os ${process.platform} --cpu ${cpu} --to ~/.local/bin

See https://jasonshin.github.io/sqlx-ts for other installation options.`)
  process.exit(1)
}

const result = spawnSync(binary, process.argv.slice(2), { stdio: 'inherit' })
if (result.error) {
  throw result.error
}
process.exit(result.status ?? 1)
