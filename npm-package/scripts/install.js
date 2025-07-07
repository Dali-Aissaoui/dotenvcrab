#!/usr/bin/env node

const https = require("https");
const fs = require("fs");
const path = require("path");

const packageJson = require(path.join(__dirname, "..", "package.json"));
const version = packageJson.version;
const binaryName =
  process.platform === "win32" ? "dotenvcrab.exe" : "dotenvcrab";
const binPath = path.join(__dirname, "..", "bin", binaryName);

function getPlatformName() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "win32") {
    return arch === "x64" ? "windows-amd64" : "windows-386";
  } else if (platform === "darwin") {
    return arch === "arm64" ? "macos-arm64" : "macos-amd64";
  } else if (platform === "linux") {
    if (arch === "arm64" || arch === "arm") {
      return "linux-arm64";
    } else {
      return "linux-amd64";
    }
  }

  throw new Error(`Unsupported platform: ${platform} ${arch}`);
}

async function downloadBinary() {
  const platform = getPlatformName();
  const url = `https://github.com/Dali-Aissaoui/dotenvcrab/releases/download/v${version}/dotenvcrab-${platform}`;

  console.log(`Downloading dotenvcrab v${version} for ${platform}...`);

  const localBinaryPath = path.join(
    __dirname,
    "..",
    "..",
    "target",
    "release",
    "dotenvcrab"
  );
  if (fs.existsSync(localBinaryPath)) {
    console.log(
      `Found local binary at ${localBinaryPath}, using it instead of downloading`
    );
    fs.copyFileSync(localBinaryPath, binPath);
    return;
  }

  // Create bin directory if it doesn't exist
  const binDir = path.dirname(binPath);
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  // Download the file
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(binPath);

    https
      .get(url, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          // Handle redirects
          https
            .get(response.headers.location, (redirectResponse) => {
              redirectResponse.pipe(file);

              file.on("finish", () => {
                file.close(() => {
                  console.log("Download completed");
                  resolve();
                });
              });
            })
            .on("error", (err) => {
              fs.unlink(binPath, () => {});
              reject(err);
            });
        } else if (response.statusCode === 200) {
          response.pipe(file);

          file.on("finish", () => {
            file.close(() => {
              console.log("Download completed");
              resolve();
            });
          });
        } else {
          reject(
            new Error(`Failed to download binary: ${response.statusCode}`)
          );
        }
      })
      .on("error", (err) => {
        fs.unlink(binPath, () => {});
        reject(err);
      });
  });
}

function makeExecutable() {
  if (process.platform !== "win32") {
    console.log("Making binary executable...");
    fs.chmodSync(binPath, 0o755);
  }
}

function checkBinary() {
  try {
    if (fs.existsSync(binPath)) {
      console.log("dotenvcrab binary already exists");
      makeExecutable();
      return true;
    }
  } catch (err) {
    console.error("Error checking for existing binary:", err);
  }
  return false;
}

// Try to use system binary if available
function trySystemBinary() {
  try {
    const systemPaths = [
      "/usr/local/bin/dotenvcrab",
      "/usr/bin/dotenvcrab",
      "/opt/homebrew/bin/dotenvcrab",
    ];

    for (const systemPath of systemPaths) {
      if (fs.existsSync(systemPath)) {
        console.log(`Using system binary at ${systemPath}`);
        fs.symlinkSync(systemPath, binPath);
        return true;
      }
    }
  } catch (err) {
    console.error("error checking for system binary:", err);
  }
  return false;
}

async function install() {
  try {
    if (checkBinary()) {
      return;
    }

    if (trySystemBinary()) {
      return;
    }

    await downloadBinary();
    makeExecutable();

    console.log("dotenvcrab installed successfully");
  } catch (err) {
    console.error("Failed to install dotenvcrab:", err);
    process.exit(1);
  }
}

install();
