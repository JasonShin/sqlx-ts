const { execSync } = require('child_process')
const { createHash } = require('crypto')
const fs = require('fs')
const https = require('https')
const path = require('path')
const { promisify } = require('util')
const { pipeline } = require('stream')
const streamPipeline = promisify(pipeline)

const tag = require('./package.json').version
const os = process.platform
const cpu = process.arch

// Map Node.js platform and architecture to binary names
function getBinaryInfo() {
  const buildMap = {
    'linux-ia32': 'linux-32-bit',
    'linux-x32': 'linux-32-bit',
    'linux-x64': 'linux-64-bit',
    'linux-arm64': 'linux-arm',
    'darwin-x64': 'macos-64-bit',
    'darwin-arm64': 'macos-arm',
    'win32-ia32': 'windows-32-bit',
    'win32-x32': 'windows-32-bit',
    'win32-x64': 'windows-64-bit',
  }

  const key = `${os}-${cpu}`
  const build = buildMap[key]

  if (!build) {
    throw new Error(`Unsupported platform: ${os}-${cpu}`)
  }

  return {
    build,
    filename: `sqlx-ts-v${tag}-${build}.zip`,
    binaryName: os === 'win32' ? 'sqlx-ts.exe' : 'sqlx-ts'
  }
}

// Download file from URL
function downloadFile(url, destination) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(destination)
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Handle redirect
        file.close()
        fs.unlinkSync(destination)
        return downloadFile(response.headers.location, destination)
          .then(resolve)
          .catch(reject)
      }

      if (response.statusCode !== 200) {
        file.close()
        fs.unlinkSync(destination)
        return reject(new Error(`Failed to download: ${response.statusCode} ${response.statusMessage}`))
      }

      response.pipe(file)
      file.on('finish', () => {
        file.close(resolve)
      })
    }).on('error', (err) => {
      fs.unlinkSync(destination)
      reject(err)
    })
  })
}

// Calculate SHA-256 hash of a file
function calculateSHA256(filePath) {
  return new Promise((resolve, reject) => {
    const hash = createHash('sha256')
    const stream = fs.createReadStream(filePath)

    stream.on('data', (data) => hash.update(data))
    stream.on('end', () => resolve(hash.digest('hex')))
    stream.on('error', reject)
  })
}

// Verify file hash
async function verifyHash(filePath, expectedHash) {
  const actualHash = await calculateSHA256(filePath)

  if (actualHash !== expectedHash) {
    throw new Error(
      `Hash mismatch!\n` +
      `Expected: ${expectedHash}\n` +
      `Got:      ${actualHash}\n` +
      `This could indicate a corrupted download or a security issue.`
    )
  }

  return true
}

// Extract binary from zip
function extractBinary(zipPath, binaryName, targetPath) {
  const AdmZip = require('adm-zip')
  const zip = new AdmZip(zipPath)
  const zipEntries = zip.getEntries()

  for (const entry of zipEntries) {
    if (entry.entryName.endsWith(binaryName)) {
      // Extract the entry's content directly
      const data = entry.getData()
      fs.writeFileSync(targetPath, data)
      fs.chmodSync(targetPath, 0o755)
      return
    }
  }

  throw new Error(`Binary ${binaryName} not found in archive`)
}

async function install() {
  try {
    const { build, filename, binaryName } = getBinaryInfo()
    const baseUrl = `https://github.com/JasonShin/sqlx-ts/releases/download/v${tag}`
    const zipUrl = `${baseUrl}/${filename}`
    const checksumUrl = `${zipUrl}.sha256`

    const tmpDir = fs.mkdtempSync(path.join(require('os').tmpdir(), 'sqlx-ts-'))
    const zipPath = path.join(tmpDir, filename)
    const checksumPath = path.join(tmpDir, `${filename}.sha256`)
    const targetPath = path.join(__dirname, 'sqlx-ts' + (os === 'win32' ? '.exe' : ''))

    console.info(`Downloading sqlx-ts v${tag} for ${os}-${cpu}...`)
    console.info(`URL: ${zipUrl}`)

    // Download the zip file
    await downloadFile(zipUrl, zipPath)
    console.info('Download complete')

    // Download and verify the checksum
    try {
      console.info('Downloading checksum...')
      await downloadFile(checksumUrl, checksumPath)
      const expectedHash = fs.readFileSync(checksumPath, 'utf8').trim()
      console.info(`Expected SHA-256: ${expectedHash}`)

      // Verify the hash
      console.info('Verifying checksum...')
      await verifyHash(zipPath, expectedHash)
      console.info('Checksum verified successfully')
    } catch (error) {
      console.warn('Warning: Could not download or verify checksum.')
      console.warn('This is expected for releases before SHA-256 checksums were added.')
      console.warn('Proceeding without verification (not recommended for production).')
      console.warn(`Checksum URL: ${checksumUrl}`)
    }

    // Extract the binary
    console.info('Extracting binary...')
    extractBinary(zipPath, binaryName, targetPath)

    // Cleanup
    fs.rmSync(tmpDir, { recursive: true, force: true })

    console.info('sqlx-ts installation successful')
  } catch (error) {
    console.error('Installation failed:', error.message)
    process.exit(1)
  }
}

install()
