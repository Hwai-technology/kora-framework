#!/usr/bin/env node

/**
 * Kora CLI npm Executable Wrapper
 * Dispatches commands to the native Kora binary.
 */

const { spawn, execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

function findKoraBinary() {
  // 1. Check local packaged binary
  const localBinary = path.join(__dirname, '..', 'bin', process.platform === 'win32' ? 'kora.exe' : 'kora');
  if (fs.existsSync(localBinary)) {
    return localBinary;
  }

  // 2. Check if kora is in system PATH
  try {
    const whichCmd = process.platform === 'win32' ? 'where kora' : 'which kora';
    const foundPath = execSync(whichCmd, { stdio: ['ignore', 'pipe', 'ignore'] }).toString().trim().split('\n')[0].trim();
    if (foundPath && fs.existsSync(foundPath)) {
      return foundPath;
    }
  } catch (e) {
    // Not found in PATH
  }

  // 3. Check ~/.cargo/bin/kora
  const homeDir = process.env.HOME || process.env.USERPROFILE || '';
  const cargoBinary = path.join(homeDir, '.cargo', 'bin', process.platform === 'win32' ? 'kora.exe' : 'kora');
  if (homeDir && fs.existsSync(cargoBinary)) {
    return cargoBinary;
  }

  return null;
}

const binaryPath = findKoraBinary();

if (!binaryPath) {
  console.error('\x1b[31merror:\x1b[0m Kora native binary was not found on your system.');
  console.error('\nTo install Kora natively, run:');
  console.error('  \x1b[36mcurl -fsSL https://raw.githubusercontent.com/Hwai-technology/kora-framework/main/install.sh | bash\x1b[0m');
  console.error('\nOr install via Cargo:');
  console.error('  \x1b[36mcargo install --git https://github.com/Hwai-technology/kora-framework kora-cli\x1b[0m');
  console.error('\nAlternatively, run Kora via Docker container:');
  console.error('  \x1b[36mdocker run -it --rm -p 7582:7582 ghcr.io/hwai-technology/kora-framework:latest\x1b[0m\n');
  process.exit(1);
}

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  env: process.env
});

child.on('exit', (code) => {
  process.exit(code || 0);
});

