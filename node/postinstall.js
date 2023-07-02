let exec = require('child_process').exec

console.log('checking post install ??')
exec('curl -LSfs https://github.com/JasonShin/sqlx-ts/releases/download/v0.1.0-alpha.16/sqlx-ts-v0.1.0-alpha.16-macos-arm.zip --output test.zip', (err) => {
    console.error(err)
})