#!/usr/bin/env node

/**
 * this script synchronizes the version from Cargo.toml to the npm package.json
 * run it after updating the version in Cargo.toml
 */

const fs = require("fs");
const path = require("path");

const cargoTomlPath = path.join(__dirname, "..", "Cargo.toml");
const packageJsonPath = path.join(
  __dirname,
  "..",
  "npm-package",
  "package.json"
);

const cargoToml = fs.readFileSync(cargoTomlPath, "utf8");

const versionMatch = cargoToml.match(/^version\s*=\s*"([^"]+)"/m);
if (!versionMatch) {
  console.error("could not find version in Cargo.toml");
  process.exit(1);
}

const cargoVersion = versionMatch[1];
console.log(`found version in Cargo.toml: ${cargoVersion}`);

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));

const oldVersion = packageJson.version;
packageJson.version = cargoVersion;

fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + "\n");

console.log(
  `updated npm package.json version: ${oldVersion} -> ${cargoVersion}`
);
