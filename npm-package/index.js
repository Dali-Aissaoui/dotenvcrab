#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

class DotEnvCrab {
  constructor() {
    this.binaryPath = this._getBinaryPath();
  }

  /**
   * Get the path to the dotenvcrab binary
   * @returns {string} Path to the binary
   * @private
   */
  _getBinaryPath() {
    const binaryName = process.platform === 'win32' ? 'dotenvcrab.exe' : 'dotenvcrab';
    return path.join(__dirname, 'bin', binaryName);
  }

  /**
   * Validate an .env file against a schema
   * @param {Object} options - Validation options
   * @param {string} [options.envFile='.env'] - Path to .env file
   * @param {string} [options.schemaFile='env.schema.json'] - Path to schema file
   * @param {boolean} [options.strict=false] - Fail on extra keys
   * @param {boolean} [options.json=false] - Output in JSON format
   * @returns {Promise<Object>} - Validation result
   */
  validate(options = {}) {
    const {
      envFile = '.env',
      schemaFile = 'env.schema.json',
      strict = false,
      json = false,
    } = options;

    return new Promise((resolve, reject) => {
      // Build arguments
      const args = [];
      
      if (envFile) {
        args.push('--env', envFile);
      }
      
      if (schemaFile) {
        args.push('--schema', schemaFile);
      }
      
      if (strict) {
        args.push('--strict');
      }
      
      if (json) {
        args.push('--json');
      }

      // Spawn the process
      const process = spawn(this.binaryPath, args);
      
      let stdout = '';
      let stderr = '';
      
      process.stdout.on('data', (data) => {
        stdout += data.toString();
      });
      
      process.stderr.on('data', (data) => {
        stderr += data.toString();
      });
      
      process.on('close', (code) => {
        const result = {
          success: code === 0,
          exitCode: code,
          output: stdout.trim(),
          error: stderr.trim()
        };
        
        // Parse JSON output if requested
        if (json && result.success && result.output) {
          try {
            result.data = JSON.parse(result.output);
          } catch (err) {
            result.jsonParseError = err.message;
          }
        }
        
        if (result.success) {
          resolve(result);
        } else {
          reject(result);
        }
      });
      
      process.on('error', (err) => {
        reject({
          success: false,
          error: `Failed to execute dotenvcrab: ${err.message}`,
          exitCode: -1
        });
      });
    });
  }

  /**
   * Check if dotenvcrab is installed and working
   * @returns {Promise<boolean>} - True if working
   */
  async checkInstallation() {
    try {
      const process = spawn(this.binaryPath, ['--version']);
      
      return new Promise((resolve) => {
        process.on('close', (code) => {
          resolve(code === 0);
        });
        
        process.on('error', () => {
          resolve(false);
        });
      });
    } catch (err) {
      return false;
    }
  }
}

// Export the class
module.exports = DotEnvCrab;

// If called directly from the command line
if (require.main === module) {
  // Parse command line arguments and pass them to the binary
  const args = process.argv.slice(2);
  const binaryName = process.platform === 'win32' ? 'dotenvcrab.exe' : 'dotenvcrab';
  const binaryPath = path.join(__dirname, 'bin', binaryName);
  
  const child = spawn(binaryPath, args, { stdio: 'inherit' });
  
  child.on('close', (code) => {
    process.exit(code);
  });
}
