# Installation

## Rust

Add datafake-rs to your `Cargo.toml`:

```toml
[dependencies]
datafake-rs = "0.2"
```

Or use cargo add:

```bash
cargo add datafake-rs
```

## WebAssembly (Browser/Node.js)

### From npm

```bash
npm install datafake-wasm
```

### From Source

Build the WASM package yourself:

```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Clone and build
git clone https://github.com/GoPlasmatic/datafake-rs.git
cd datafake-rs/datafake-wasm
wasm-pack build --target web
```

## Feature Flags

The library has minimal dependencies by default. No feature flags are required for basic usage.

## Minimum Rust Version

datafake-rs requires Rust 2024 edition (Rust 1.85+).
