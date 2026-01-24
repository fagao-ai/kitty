const fs = require('fs')

// Get version from package.json
const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'))
const version = pkg.version

// Update Cargo.toml
const cargoPath = 'src-tauri/Cargo.toml'
let cargo = fs.readFileSync(cargoPath, 'utf8')
cargo = cargo.replace(/^version = .*/m, `version = "${version}"`)
fs.writeFileSync(cargoPath, cargo)

console.log(`Updated Cargo.toml version to ${version}`)
