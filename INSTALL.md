# Installation Guide

## Quick Install (Recommended)

### From Source

```bash
# Clone the repository
git clone https://github.com/Abil-Shrestha/tracer.git
cd tracer

# Build and install
cargo install --path .
```

The `trace` binary will be installed to `~/.cargo/bin/` (make sure it's in your PATH).

### Using Cargo

```bash
cargo install trace-tracker
```

## Platform-Specific Instructions

### macOS

**Option 1: Homebrew (Coming Soon)**
```bash
brew tap Abil-Shrestha/tracer
brew install tracer
```

**Option 2: From Source**
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install tracer
cargo install --git https://github.com/Abil-Shrestha/tracer
```

### Linux

**Ubuntu/Debian:**
```bash
# Install build dependencies
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install tracer
cargo install --git https://github.com/Abil-Shrestha/tracer
```

**Arch Linux (Coming Soon):**
```bash
yay -S trace-git
```

**From Binary:**
```bash
# Download from releases
wget https://github.com/Abil-Shrestha/tracer/releases/latest/download/tracer-linux-amd64

# Make executable
chmod +x tracer-linux-amd64

# Move to PATH
sudo mv tracer-linux-amd64 /usr/local/bin/tracer
```

### Windows

**Option 1: From Source**
```powershell
# Install Rust from https://rustup.rs/

# Install tracer
cargo install --git https://github.com/Abil-Shrestha/tracer
```

**Option 2: From Binary**
```powershell
# Download from releases
# https://github.com/Abil-Shrestha/tracer/releases/latest

# Add to PATH or run from download location
.\tracer.exe --help
```

## Verification

After installation, verify it works:

```bash
tracer --version
tracer --help
```

## Updating

### Cargo Install
```bash
cargo install trace-tracker --force
```

### From Source
```bash
cd tracer
git pull
cargo install --path . --force
```

### Homebrew
```bash
brew upgrade tracer
```

## Uninstalling

```bash
cargo uninstall tracer
```

## Troubleshooting

### Command not found

Make sure `~/.cargo/bin` is in your PATH:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"
```

### Build fails on Windows

Make sure you have the Visual C++ build tools installed:
- Download from: https://visualstudio.microsoft.com/downloads/
- Install "Desktop development with C++"

### SQLite errors

Trace uses bundled SQLite, so no external dependencies needed. If you see SQLite errors, try:

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

## Development Installation

For contributing or testing:

```bash
# Clone repo
git clone https://github.com/Abil-Shrestha/tracer.git
cd tracer

# Build in debug mode
cargo build

# Run tests
cargo test

# Run from source
cargo run -- --help

# Build optimized version
cargo build --release

# Binary will be in target/release/tracer
```

## Next Steps

After installation:

1. Initialize in your project: `tracer init`
2. Create your first issue: `tracer create "My first task"`
3. See ready work: `tracer ready`
4. Read the [README](README.md) for full documentation

## Getting Help

- Check the [FAQ](README.md#faq) in the README
- [Open an issue](https://github.com/Abil-Shrestha/tracer/issues) on GitHub
- Read [CONTRIBUTING.md](CONTRIBUTING.md) for development help
