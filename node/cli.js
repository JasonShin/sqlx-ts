#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Determine the binary name based on platform
const platform = process.platform;
const binaryName = platform === 'win32' ? 'sqlx-ts.exe' : 'sqlx-ts';
const binaryPath = path.join(__dirname, binaryName);

// Check if binary exists
if (!fs.existsSync(binaryPath)) {
  console.error(`ERROR: sqlx-ts binary not found at ${binaryPath}`);
  console.error('Please ensure the package was installed correctly (postinstall script should have downloaded it).');
  process.exit(1);
}

// Spawn the binary with all arguments passed through
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  windowsHide: true
});

// Handle exit
child.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
  } else {
    process.exit(code || 0);
  }
});

// Handle errors
child.on('error', (err) => {
  console.error(`ERROR: Failed to execute sqlx-ts binary: ${err.message}`);
  process.exit(1);
});
