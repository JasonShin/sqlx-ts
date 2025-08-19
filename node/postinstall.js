let execSync = require('child_process').execSync
let tag = require('./package.json').version

const os = process.platform
const cpu = process.arch

execSync(`curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | bash -s -- --os ${os} --cpu ${cpu} --tag ${tag} -f`, { stdio: 'inherit' })
console.info('sqlx-ts installation successful')
