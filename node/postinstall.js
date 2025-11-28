const { ZipReader, BlobReader, Uint8ArrayWriter } = require('@zip.js/zip.js');
const { createHash } = require('crypto')
const fs = require('fs')
const https = require('https')
const path = require('path')
const os = require('os')
const tag = require('./package.json').version

const platform = process.platform
const cpu = process.arch

const colors = {
  reset: "\x1b[0m",
  red: "\x1b[31m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  cyan: "\x1b[36m",
}

const info = (msg) => console.log(`${colors.cyan}INFO: ${msg} ${colors.reset}`)
const success = (msg) => console.log(`${colors.green}SUCCESS: ${msg} ${colors.reset}`)
const warn = (msg) => console.warn(`${colors.yellow}WARNING: ${msg} ${colors.reset}`)
const error = (msg) => console.error(`${colors.red}ERROR: ${msg} ${colors.reset}`)

function getBinaryInfo() {
  let build = ''
  if (platform === 'darwin') {
    if (cpu === 'arm64') {
      build = 'macos-arm'
    } else {
      build = 'macos-64-bit'
    }
  } else if (platform === 'win32') {
    if (cpu === 'x64') {
      build = 'windows-64-bit'
    } else {
      build = 'windows-32-bit'
    }
  } else if (platform === 'linux') {
    if (cpu === 'x64') {
      build = 'linux-64-bit'
    } else if (cpu === 'arm64') {
      build = 'linux-arm'
    } else {
      build = 'linux-32-bit'
    }
  } else {
    throw new Error(`Unsupported platform: ${platform}-${cpu}`)
  }

  return {
    build,
    filename: `sqlx-ts-v${tag}-${build}.zip`,
    binaryName: platform === 'win32' ? 'sqlx-ts.exe' : 'sqlx-ts'
  }
}

// Download file from URL
function downloadFile(url, destination, redirectCount = 0) {
  return new Promise((resolve, reject) => {
    if (redirectCount > 5) {
      return reject(new Error("Too many redirects while downloading file"))
    }

    const file = fs.createWriteStream(destination)
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Handle redirects
        // Close the current file and delete it
        // Then download from the new location
        file.close()
        fs.unlinkSync(destination)
        return downloadFile(response.headers.location, destination, redirectCount + 1)
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

async function extractBinary(zipPath, binaryName, targetPath) {
  const zipData = fs.readFileSync(zipPath);
  const reader = new ZipReader(new BlobReader(new Blob([zipData])));

  const entries = await reader.getEntries();

  for (const entry of entries) {
    if (entry.filename.endsWith(binaryName)) {
      const writer = new Uint8ArrayWriter();
      const data = await entry.getData(writer);
      fs.writeFileSync(targetPath, Buffer.from(data));
      fs.chmodSync(targetPath, 0o755);
      await reader.close();
      return;
    }
  }

  throw new Error(`Binary ${binaryName} not found in zip`);
}

async function install() {
  try {
    const { filename, binaryName } = getBinaryInfo()
    const baseUrl = `https://github.com/JasonShin/sqlx-ts/releases/download/v${tag}`
    const zipUrl = `${baseUrl}/${filename}`
    const checksumUrl = `${zipUrl}.sha256`

    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'sqlx-ts-'))
    const zipPath = path.join(tmpDir, filename)
    const checksumPath = path.join(tmpDir, `${filename}.sha256`)
    info('checking checksum url:', checksumUrl)
    const targetPath = path.join(__dirname, 'sqlx-ts' + (platform === 'win32' ? '.exe' : ''))

    info(`Downloading sqlx-ts v${tag} for ${platform}-${cpu}...`)
    info(`URL: ${zipUrl}`)

    // Download the zip file
    await downloadFile(zipUrl, zipPath)
    success('Download complete')

    // Download and verify the checksum
    try {
      info('Downloading checksum...')
      await downloadFile(checksumUrl, checksumPath)
      const expectedHash = fs.readFileSync(checksumPath, 'utf8').trim()
      info(`Expected SHA-256: ${expectedHash}`)

      // Verify the hash
      info('Verifying checksum...')
      await verifyHash(zipPath, expectedHash)
      success('Checksum verified successfully')
    } catch (err) {
      warn('Warning: Could not download or verify checksum.')
      warn('This is expected for releases before SHA-256 checksums were added.')
      warn('Proceeding without verification (not recommended for production).')
      warn(`Checksum URL: ${checksumUrl}`)
    }

    // Extract the binary
    info('Extracting binary...')
    await extractBinary(zipPath, binaryName, targetPath)

    // Cleanup
    fs.rmSync(tmpDir, { recursive: true, force: true })

    info('sqlx-ts installation successful')
    process.exit(0)
  } catch (err) {
    error('Installation failed:', err.message)
    process.exit(1)
  }
}

install()
