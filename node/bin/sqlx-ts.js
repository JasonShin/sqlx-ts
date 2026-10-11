#!/usr/bin/env node
const { spawnSync } = require('child_process')
const fs = require('fs')
const path = require('path')

const binaryName = process.platform === 'win32' ? 'sqlx-ts.exe' : 'sqlx-ts'
function findOnPath() {
  for (const dir of (process.env.PATH || '').split(path.delimiter)) {
    const candidate = path.join(dir, binaryName)
    if (fs.existsSync(candidate) && !fs.realpathSync(candidate).endsWith('.js')) {
      return candidate
    }
  }
}

let binary = path.join(__dirname, '..', binaryName)
if (!fs.existsSync(binary)) {
  binary = findOnPath()
}

if (!binary) {
  const cpu = process.arch === 'arm64' || process.arch === 'x64' ? process.arch : 'x32'
  console.error(`sqlx-ts binary is not installed.
Bun and Deno do not run the npm postinstall step that downloads it. Install it separately to a directory on your PATH:

  curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | sh -s -- --os ${process.platform} --cpu ${cpu} --to ~/.local/bin

See https://jasonshin.github.io/sqlx-ts for other installation options.`)
  process.exit(1)
}

const result = spawnSync(binary, process.argv.slice(2), { stdio: 'inherit' })
if (result.error) {
  throw result.error
}
process.exit(result.status ?? 1)
