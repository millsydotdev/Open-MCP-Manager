const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const packageJsonPath = path.join(__dirname, '../package.json');
const cargoTomlPath = path.join(__dirname, '../Cargo.toml');

// Read package.json
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const version = packageJson.version;

// Read Cargo.toml
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');

// Update version in Cargo.toml
// Matches: version = "0.0.0"
const versionRegex = /^version\s*=\s*"(.*)"/m;
const match = cargoToml.match(versionRegex);

if (match) {
    if (match[1] !== version) {
        console.log(`Updating Cargo.toml version from ${match[1]} to ${version}`);
        cargoToml = cargoToml.replace(versionRegex, `version = "${version}"`);
        fs.writeFileSync(cargoTomlPath, cargoToml);

        // Add to git stage if we are in a git repo process (likely called by npm version)
        try {
            execSync(`git add "${cargoTomlPath}"`);
            console.log('Added Cargo.toml to git stage');
        } catch (e) {
            console.warn('Failed to add Cargo.toml to git stage (may not be in a git repo or git not found)');
        }
    } else {
        console.log('Cargo.toml version already matches package.json');
    }
} else {
    console.error('Could not find version field in Cargo.toml');
    process.exit(1);
}
