#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');

function getBinaryPath() {
  const platform = os.platform();
  const arch = os.arch();

  let binaryName;

  if (platform === 'linux') {
    if (arch === 'x64') {
      binaryName = 'elm-package-mcp-server-linux-x86_64';
    } else {
      console.error(`Error: Unsupported Linux architecture: ${arch}`);
      process.exit(1);
    }
  } else if (platform === 'darwin') {
    if (arch === 'arm64') {
      binaryName = 'elm-package-mcp-server-macos-aarch64';
    } else if (arch === 'x64') {
      binaryName = 'elm-package-mcp-server-macos-x86_64';
    } else {
      console.error(`Error: Unsupported macOS architecture: ${arch}`);
      process.exit(1);
    }
  } else {
    console.error(`Error: Unsupported platform: ${platform}`);
    process.exit(1);
  }

  return path.join(__dirname, '..', 'binaries', binaryName);
}

const binaryPath = getBinaryPath();
const args = process.argv.slice(2);

const child = spawn(binaryPath, args, {
  stdio: 'inherit'
});

child.on('error', (err) => {
  console.error('Error executing binary:', err);
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code || 0);
});
