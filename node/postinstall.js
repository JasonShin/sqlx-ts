let execSync = require('child_process').execSync

const os = process.platform
const cpu = process.arch

console.error('testing installation')
execSync(`curl -LSfs https://jasonshin.github.io/sqlx-ts/install.sh | bash -s -- --os ${os} --cpu ${cpu} -f`, { stdio: 'inherit' })
console.error('sqlx-ts installation successful')
