// This script is executed as part of sqlx-ts CI pipeline to test local ../scripts/install.sh changes
// DO NOT USE THIS IN PRODUCTION
let execSync = require('child_process').execSync
let tag = require('./package.json').version

const os = process.platform
const cpu = process.arch

execSync(`sh ../scripts/install.sh -s -- --os ${os} --cpu ${cpu} --tag ${tag} -f`, { stdio: 'inherit' })
console.info('sqlx-ts installation successful')
