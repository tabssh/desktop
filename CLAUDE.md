# TabSSH Desktop - Rust Cross-Platform SSH Client

**Last Updated:** 2025-10-18
**Version:** 0.1.0 (Planning Phase)
**Status:** 📋 Specification & Design

---

## Project Overview

**TabSSH Desktop** is a modern, cross-platform SSH/SFTP client written in Rust, designed for Windows, Linux, macOS, and BSD systems. Built with native performance, static linking, and true cross-platform binary distribution.

### Design Goals
- 🦀 **Pure Rust** - Memory-safe, fast, concurrent
- 📦 **Static Binaries** - No runtime dependencies (statically linked, no -musl suffix)
- 🎨 **Native UI** - Platform-native look and feel
- 🔐 **Security First** - Rust's memory safety + SSH best practices
- ⚡ **High Performance** - Async I/O, efficient terminal emulation
- 🌍 **True Cross-Platform** - Windows, Linux, macOS, BSD (amd64 + arm64)

---

## Binary Naming Convention

### Format: `tabssh-{os}-{arch}`

**Supported Platforms:**

| OS | Architecture | Binary Name | Notes |
|----|--------------|-------------|-------|
| Linux | x86_64 | `tabssh-linux-amd64` | Statically linked (musl) |
| Linux | aarch64 | `tabssh-linux-arm64` | Statically linked (musl) |
| macOS | x86_64 | `tabssh-macos-amd64` | Intel Macs |
| macOS | aarch64 | `tabssh-macos-arm64` | Apple Silicon (M1/M2/M3/M4) |
| Windows | x86_64 | `tabssh-windows-amd64.exe` | Static MSVC |
| Windows | aarch64 | `tabssh-windows-arm64.exe` | ARM Windows |
| FreeBSD | x86_64 | `tabssh-freebsd-amd64` | Statically linked |
| FreeBSD | aarch64 | `tabssh-freebsd-arm64` | Statically linked |
| OpenBSD | x86_64 | `tabssh-openbsd-amd64` | Statically linked |
| OpenBSD | aarch64 | `tabssh-openbsd-arm64` | Statically linked |
| NetBSD | x86_64 | `tabssh-netbsd-amd64` | Statically linked |

**Total:** 11 binary variants (expandable)

### Build Artifacts Structure
```
releases/
├── v0.1.0/
│   ├── tabssh-linux-amd64          # 8-12 MB
│   ├── tabssh-linux-arm64          # 8-12 MB
│   ├── tabssh-macos-amd64          # 10-14 MB
│   ├── tabssh-macos-arm64          # 10-14 MB
│   ├── tabssh-windows-amd64.exe    # 10-14 MB
│   ├── tabssh-windows-arm64.exe    # 10-14 MB
│   ├── tabssh-freebsd-amd64        # 8-12 MB
│   ├── tabssh-freebsd-arm64        # 8-12 MB
│   ├── tabssh-openbsd-amd64        # 8-12 MB
│   ├── tabssh-openbsd-arm64        # 8-12 MB
│   ├── tabssh-netbsd-amd64         # 8-12 MB
│   ├── checksums.txt               # SHA256 checksums
│   └── tabssh-desktop-0.1.0-source.tar.gz
```

---

## Architecture

### Technology Stack

#### Core
- **Language:** Rust 1.75+ (2021 edition)
- **SSH Library:** `russh` or `thrussh` (pure Rust SSH2 implementation)
- **Async Runtime:** `tokio` (multi-threaded async runtime)
- **Terminal Emulation:** `alacritty_terminal` or custom VT implementation
- **UI Framework:** See UI section below

#### UI Framework Options (Choose One)

**Option 1: egui (Recommended)**
- Pure Rust immediate-mode GUI
- Cross-platform (runs on all targets)
- Lightweight, fast, no native dependencies
- Good for terminal UI
- ~500KB overhead

**Option 2: iced**
- Elm-inspired, reactive UI
- Cross-platform, GPU-accelerated
- Beautiful, modern widgets
- Larger binary (~2MB overhead)

**Option 3: Tauri + Web Tech**
- Web frontend (HTML/CSS/JS or Svelte/React)
- Rust backend
- Larger binaries (~20-30MB)
- Not recommended for terminal app

**Option 4: druid**
- Native Rust UI
- Data-driven architecture
- Less active development

**RECOMMENDATION:** **egui** - Perfect for terminal-focused app, minimal overhead, pure Rust

#### Storage & Persistence
- **Database:** `rusqlite` (embedded SQLite)
- **Serialization:** `serde` + `bincode` or `serde_json`
- **Config Files:** `toml` or `ron` format
- **Keychain Integration:**
  - Linux: `secret-service` or `keyring-rs`
  - macOS: `security-framework` (Keychain API)
  - Windows: `windows` crate (DPAPI/Credential Manager)
  - BSD: File-based encryption with OS permissions

#### Crypto & Security
- **SSH:** `russh` (pure Rust SSH2)
- **Crypto:** `ring` or `rustls` (TLS), `argon2` (password hashing)
- **Key Management:** `rsa`, `ed25519-dalek`
- **Random:** `rand` with OS entropy

#### Terminal Emulation
- **Core:** `alacritty_terminal` (proven VT100/xterm emulation)
- **Rendering:** egui canvas or custom OpenGL
- **Fonts:** `fontdue` or `rusttype` (pure Rust font rendering)
- **Colors:** 256-color + true color support

---

## Features (Based on Android Version)

### Core SSH Features
- ✅ Multiple SSH connections (tab-based interface)
- ✅ SSH2 protocol support
- ✅ Multiple authentication methods:
  - Password
  - Public key (RSA, ED25519, ECDSA)
  - Keyboard-interactive
  - SSH Agent forwarding
- ✅ Host key verification (SHA256 fingerprints)
- ✅ Host key persistence and MITM detection
- ✅ Session persistence and reconnection
- ✅ Keep-alive and auto-reconnect

### Terminal Emulation
- ✅ Full VT100/VT220/xterm emulation
- ✅ 256-color and 24-bit true color
- ✅ UTF-8 support
- ✅ Configurable scrollback buffer (default: 10,000 lines)
- ✅ Text selection and clipboard integration
- ✅ Mouse support (SGR mouse mode)
- ✅ Alternate screen buffer
- ✅ Title escape sequences

### SFTP File Transfer
- ✅ Integrated SFTP browser
- ✅ Drag-and-drop file upload/download
- ✅ Resume interrupted transfers
- ✅ Multi-file batch transfers
- ✅ Progress tracking
- ✅ Permission management
- ✅ Symlink handling

### Advanced SSH Features
- ✅ Local port forwarding
- ✅ Remote port forwarding
- ✅ Dynamic (SOCKS) proxy
- ✅ X11 forwarding
- ✅ Agent forwarding
- ✅ Jump host / ProxyJump support
- ✅ SSH config file import (`~/.ssh/config`)
- ✅ Mosh protocol support (optional)

### UI/UX
- ✅ Browser-style tabs
- ✅ Keyboard shortcuts (Ctrl+T new tab, Ctrl+W close, etc.)
- ✅ Searchable connection list
- ✅ Quick connect bar
- ✅ Connection history
- ✅ Favorite/bookmark connections
- ✅ Connection groups/folders
- ✅ Split panes (future: multiple terminals in one window)

### Themes & Customization
- ✅ 10+ built-in color schemes:
  - Dracula
  - Solarized (Light & Dark)
  - Nord
  - Monokai
  - One Dark
  - Gruvbox
  - Tomorrow Night
  - High Contrast
  - Custom themes (JSON/TOML config)
- ✅ Font customization (size, family, ligatures)
- ✅ Opacity/transparency (platform-dependent)
- ✅ Cursor style (block, beam, underline)

### Security Features
- ✅ Secure credential storage (OS keychain/keyring)
- ✅ Master password protection (optional)
- ✅ Auto-lock on idle
- ✅ No plaintext password storage
- ✅ Encrypted session history
- ✅ Security audit log
- ✅ Certificate pinning

### Data Management
- ✅ Import/export connections (encrypted)
- ✅ Backup to file
- ✅ Sync across devices (file-based, manual)
- ✅ Bulk operations (import multiple hosts)
- ✅ Migration from other clients (PuTTY, etc.)

### Platform-Specific Features
- **macOS:**
  - Touch Bar support (future)
  - Keychain integration
  - System appearance detection (dark mode)
- **Windows:**
  - Windows Terminal integration
  - Credential Manager integration
  - WSL integration (future)
- **Linux:**
  - Freedesktop.org standards compliance
  - D-Bus integration
  - Wayland + X11 support
- **BSD:**
  - Native package formats (pkg, ports)

---

## Project Structure

```
tabssh/desktop/
├── src/                        # ALL SOURCE CODE
│   ├── main.rs                 # Entry point
│   ├── app.rs                  # Main application state
│   ├── ui/                     # UI layer (egui)
│   │   ├── mod.rs
│   │   ├── main_window.rs      # Main window with tabs
│   │   ├── terminal_view.rs    # Terminal rendering widget
│   │   ├── connection_manager.rs
│   │   ├── settings_dialog.rs
│   │   ├── sftp_browser.rs
│   │   └── theme.rs
│   ├── ssh/                    # SSH core
│   │   ├── mod.rs
│   │   ├── connection.rs       # SSH connection manager
│   │   ├── session.rs          # SSH session wrapper
│   │   ├── auth.rs             # Authentication handlers
│   │   ├── channel.rs          # Channel management
│   │   ├── forwarding.rs       # Port forwarding
│   │   └── agent.rs            # SSH agent integration
│   ├── sftp/                   # SFTP implementation
│   │   ├── mod.rs
│   │   ├── client.rs           # SFTP client
│   │   ├── transfer.rs         # File transfer manager
│   │   └── browser.rs          # File browser logic
│   ├── terminal/               # Terminal emulation
│   │   ├── mod.rs
│   │   ├── emulator.rs         # VT emulator (alacritty_terminal)
│   │   ├── renderer.rs         # Terminal renderer (egui canvas)
│   │   ├── buffer.rs           # Scrollback buffer
│   │   ├── grid.rs             # Character grid
│   │   └── ansi.rs             # ANSI escape parser
│   ├── storage/                # Data persistence
│   │   ├── mod.rs
│   │   ├── database.rs         # SQLite database
│   │   ├── config.rs           # Configuration management
│   │   ├── connections.rs      # Connection profiles
│   │   ├── keys.rs             # SSH key storage
│   │   └── history.rs          # Session history
│   ├── crypto/                 # Cryptography
│   │   ├── mod.rs
│   │   ├── keychain.rs         # OS keychain integration
│   │   ├── keys.rs             # SSH key management
│   │   └── encryption.rs       # Data encryption
│   ├── platform/               # Platform-specific code
│   │   ├── mod.rs
│   │   ├── macos.rs
│   │   ├── windows.rs
│   │   ├── linux.rs
│   │   └── bsd.rs
│   ├── config/                 # Configuration
│   │   ├── mod.rs
│   │   ├── settings.rs
│   │   ├── themes.rs
│   │   └── ssh_config.rs       # ~/.ssh/config parser
│   └── utils/
│       ├── mod.rs
│       ├── logging.rs
│       └── errors.rs
├── tests/                      # ALL TEST FILES
│   ├── integration/
│   │   ├── ssh_connection_test.rs
│   │   ├── sftp_test.rs
│   │   ├── terminal_test.rs
│   │   └── config_test.rs
│   ├── unit/
│   │   ├── ansi_parser_test.rs
│   │   ├── crypto_test.rs
│   │   └── storage_test.rs
│   └── common/
│       └── mod.rs              # Test utilities
├── scripts/                    # ALL PRODUCTION SCRIPTS
│   ├── docker/
│   │   └── Dockerfile          # Alpine-based Rust build image
│   ├── build/
│   │   ├── build-all.sh        # Build all targets
│   │   ├── build-linux.sh      # Linux builds (musl static)
│   │   ├── build-macos.sh      # macOS builds
│   │   ├── build-windows.sh    # Windows builds
│   │   └── build-bsd.sh        # BSD builds
│   └── release/
│       ├── package.sh          # Create release archives
│       ├── checksums.sh        # Generate SHA256 checksums
│       └── github-release.sh   # Publish to GitHub
├── Cargo.toml                  # Rust dependencies
├── Cargo.lock
├── Makefile                    # Build automation
├── .cargo/
│   └── config.toml             # Cross-compilation config
├── binaries/                   # Debug builds (gitignored)
├── releases/                   # Release builds (gitignored)
├── assets/                     # Embedded resources
│   ├── icons/
│   ├── themes/
│   └── fonts/
├── README.md
├── LICENSE.md
├── CLAUDE.md                   # This file
└── .github/
    └── workflows/
        └── release.yml         # CI/CD for multi-platform builds
```

---

## Cargo.toml (Core Dependencies)

```toml
[package]
name = "tabssh"
version = "0.1.0"
edition = "2021"
authors = ["TabSSH Contributors"]
license = "MIT"
description = "Cross-platform SSH/SFTP client with browser-style tabs"
repository = "https://github.com/tabssh/desktop"

[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# SSH
russh = "0.40"
russh-keys = "0.40"
russh-sftp = "2.0"

# UI (egui recommended)
eframe = { version = "0.25", default-features = false, features = ["default_fonts", "glow"] }
egui = "0.25"
egui_extras = { version = "0.25", features = ["image"] }

# Terminal emulation
alacritty_terminal = "0.22"
vte = "0.13"

# Storage
rusqlite = { version = "0.30", features = ["bundled"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Crypto
ring = "0.17"
ed25519-dalek = "2.1"
rsa = "0.9"
argon2 = "0.5"

# Keychain (platform-specific)
keyring = "2.1"

# Utilities
anyhow = "1.0"
thiserror = "1.0"
log = "0.4"
env_logger = "0.11"
dirs = "5.0"
chrono = "0.4"

[target.'cfg(target_os = "macos")'.dependencies]
security-framework = "2.9"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.52", features = ["Win32_Security_Credentials"] }

[target.'cfg(target_os = "linux")'.dependencies]
secret-service = "3.0"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"

[profile.release-small]
inherits = "release"
opt-level = "z"
strip = true
```

---

## Docker Build Environment

### Dockerfile (Alpine + Rust)

**Location:** `scripts/docker/Dockerfile`

```dockerfile
FROM alpine:latest

# Install Rust toolchain and build dependencies
RUN apk add --no-cache \
    rust \
    cargo \
    musl-dev \
    gcc \
    g++ \
    make \
    cmake \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    git \
    bash

# Set up Rust environment
ENV RUSTFLAGS="-C target-feature=-crt-static"
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:$PATH

# Install rustup (for cross-compilation targets)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

# Add musl targets for static linking
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl

# Install cross for cross-compilation
RUN cargo install cross --git https://github.com/cross-rs/cross

# Working directory
WORKDIR /workspace

# Default command
CMD ["cargo", "build", "--release"]
```

### Docker Image
- **Name:** `tabssh-rust-alpine`
- **Base:** `alpine:latest`
- **Rust:** Latest stable from rustup
- **Size:** ~800MB (with toolchains)
- **Purpose:** Static musl builds for Linux
- **Build:** `docker build -t tabssh-rust-alpine -f scripts/docker/Dockerfile .`

### Building with Docker

```bash
# Build Docker image
docker build -t tabssh-rust-alpine -f scripts/docker/Dockerfile .

# Build Linux x86_64 (static musl)
docker run --rm \
    -v $(pwd):/workspace \
    -w /workspace \
    tabssh-rust-alpine \
    cargo build --release --target x86_64-unknown-linux-musl

# Build Linux ARM64 (static musl)
docker run --rm \
    -v $(pwd):/workspace \
    -w /workspace \
    tabssh-rust-alpine \
    cargo build --release --target aarch64-unknown-linux-musl

# Output: target/{target}/release/tabssh
```

### Static Linking (No -musl suffix in binary name)

Binaries are statically linked with musl but named without the `-musl` suffix:
- `target/x86_64-unknown-linux-musl/release/tabssh` → `binaries/tabssh-linux-amd64`
- `target/aarch64-unknown-linux-musl/release/tabssh` → `binaries/tabssh-linux-arm64`

---

## Build Configuration

### Cross-Compilation Targets

**Linux Targets (musl for static linking):**
```bash
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
```

**macOS Targets:**
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

**Windows Targets:**
```bash
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
```

**BSD Targets:**
```bash
rustup target add x86_64-unknown-freebsd
rustup target add aarch64-unknown-freebsd
rustup target add x86_64-unknown-openbsd
rustup target add x86_64-unknown-netbsd
```

### Makefile Targets

```makefile
.PHONY: all build build-all release clean help

# Build for current platform
make build

# Build for all platforms (uses Docker + cross)
make build-all

# Build platform-specific
make build-linux-amd64      # Linux x86_64 (musl static)
make build-linux-arm64      # Linux ARM64 (musl static)
make build-macos-amd64      # macOS Intel
make build-macos-arm64      # macOS Apple Silicon
make build-windows-amd64    # Windows x86_64
make build-windows-arm64    # Windows ARM64
make build-freebsd-amd64    # FreeBSD x86_64
make build-openbsd-amd64    # OpenBSD x86_64

# Build Docker image
make docker-build

# Run tests
make test

# Release (builds all + publishes to GitHub)
make release VERSION=0.1.0

# Clean
make clean                  # Clean Rust build artifacts
make clean-all              # Clean everything including Docker

# Help
make help                   # Show all targets
```

### Makefile Structure

**Location:** `./Makefile`

Key targets:
- `build` - Build for current platform (debug)
- `build-all` - Build all 11 platform variants via Docker/cross
- `release` - Build all, create checksums, publish to GitHub
- `docker-build` - Build the Alpine Rust Docker image
- `test` - Run all tests
- `clean` - Remove build artifacts

Binary outputs:
- Debug: `./binaries/tabssh-{os}-{arch}`
- Release: `./releases/tabssh-{os}-{arch}`

---

## Development Roadmap

### Phase 1: Foundation (Weeks 1-4)
- [ ] Set up project structure
- [ ] Implement basic SSH connection (russh)
- [ ] Create egui window with tab support
- [ ] Basic terminal emulation (alacritty_terminal integration)
- [ ] SQLite database schema
- [ ] Configuration management

### Phase 2: Core Features (Weeks 5-8)
- [ ] Full terminal emulation (VT100/xterm)
- [ ] Terminal rendering in egui
- [ ] Connection manager UI
- [ ] SSH authentication (password, key)
- [ ] Host key verification
- [ ] Session persistence

### Phase 3: Advanced SSH (Weeks 9-12)
- [ ] SFTP browser implementation
- [ ] File transfer with progress
- [ ] Port forwarding (local, remote, dynamic)
- [ ] SSH agent integration
- [ ] SSH config file parser
- [ ] Jump host support

### Phase 4: UI Polish (Weeks 13-16)
- [ ] Theme system
- [ ] Settings dialog
- [ ] Keyboard shortcuts
- [ ] Context menus
- [ ] Drag-and-drop
- [ ] Search functionality

### Phase 5: Platform Integration (Weeks 17-20)
- [ ] macOS Keychain integration
- [ ] Windows Credential Manager
- [ ] Linux Secret Service
- [ ] System tray integration
- [ ] Auto-update mechanism
- [ ] Platform-specific installers

### Phase 6: Testing & Release (Weeks 21-24)
- [ ] Cross-platform testing
- [ ] Performance optimization
- [ ] Security audit
- [ ] Documentation
- [ ] CI/CD pipeline
- [ ] v0.1.0 release

**Total Estimated Time: 24 weeks (6 months)**

---

## Build & Release Workflow

### Local Development
```bash
# Debug build
cargo build

# Run locally
cargo run

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Cross-Compilation (Example: Linux → macOS ARM)
```bash
# Install cross
cargo install cross

# Build for macOS ARM
cross build --release --target aarch64-apple-darwin

# Output: target/aarch64-apple-darwin/release/tabssh
# Rename: mv target/aarch64-apple-darwin/release/tabssh releases/tabssh-macos-arm64
```

### Automated Release Build
```bash
# Build all targets
./scripts/build/build-all.sh

# Creates:
# releases/
# ├── tabssh-linux-amd64
# ├── tabssh-linux-arm64
# ├── tabssh-macos-amd64
# ├── tabssh-macos-arm64
# ├── tabssh-windows-amd64.exe
# ├── tabssh-windows-arm64.exe
# ├── tabssh-freebsd-amd64
# └── checksums.txt

# Package and create GitHub release
./scripts/release/package.sh 0.1.0
gh release create v0.1.0 --title "TabSSH Desktop v0.1.0" releases/*
```

---

## Binary Size Targets

| Platform | Uncompressed | Compressed (UPX) | Notes |
|----------|--------------|------------------|-------|
| Linux (musl) | 8-12 MB | 3-5 MB | Static, no deps |
| macOS | 10-14 MB | 4-6 MB | Universal binary possible |
| Windows | 10-14 MB | 4-6 MB | Static MSVC |
| FreeBSD | 8-12 MB | 3-5 MB | Static |

**Optimization strategies:**
- Strip symbols (`strip = true`)
- LTO (`lto = true`)
- Optimize for size (`opt-level = "z"`)
- Remove dead code
- Feature flags to exclude unused components

---

## Testing Strategy

### Unit Tests
- SSH connection logic
- Terminal emulation
- ANSI parsing
- Cryptographic operations
- Database operations

### Integration Tests
- Full SSH flow (connect, auth, exec, disconnect)
- SFTP operations
- Port forwarding
- Configuration management

### Platform Tests
- Automated tests on:
  - Ubuntu 22.04 (amd64)
  - macOS 13+ (arm64)
  - Windows 11 (amd64)
  - FreeBSD 14

### Performance Benchmarks
- Terminal rendering FPS
- Large file SFTP transfers
- Multiple concurrent connections
- Memory usage profiling

---

## Security Considerations

### Threat Model
- ✅ Protection against MITM attacks (host key verification)
- ✅ Secure credential storage (OS keychain)
- ✅ Encrypted session data
- ✅ No plaintext secrets in memory dumps
- ✅ Memory safety (Rust guarantees)
- ✅ Input validation (prevent command injection)

### Security Audits
- [ ] Initial security audit before v1.0
- [ ] Dependency vulnerability scanning (cargo-audit)
- [ ] Fuzzing critical parsers (cargo-fuzz)
- [ ] Regular dependency updates

---

## Performance Targets

### Terminal Rendering
- **Target:** 60 FPS sustained
- **Max latency:** <16ms per frame
- **Scrollback:** 10,000 lines with negligible impact

### SSH Throughput
- **SFTP:** 50+ MB/s on gigabit connection
- **Terminal:** <10ms input-to-screen latency
- **Connections:** 50+ concurrent sessions

### Memory Usage
- **Base:** <50 MB
- **Per connection:** <5 MB
- **10 active sessions:** <100 MB

### Startup Time
- **Cold start:** <500ms
- **Warm start:** <200ms

---

## Comparison with Android Version

| Feature | Android (Kotlin) | Desktop (Rust) |
|---------|------------------|----------------|
| Language | Kotlin | Rust |
| UI Framework | Material Design / Jetpack Compose | egui (pure Rust) |
| SSH Library | JSch (Java) | russh (pure Rust) |
| Terminal | Custom VT emulation | alacritty_terminal |
| Database | Room (SQLite) | rusqlite (SQLite) |
| Binary Size | 23MB (debug) / 7.4MB (release) | ~10MB (static) |
| Platforms | Android only | Win/Linux/Mac/BSD |
| Dependencies | Runtime (Java, Android SDK) | None (static binary) |
| Memory Safety | GC + some unsafe JNI | Rust compile-time guarantees |
| Performance | JVM overhead | Native, no GC |

---

## Distribution & Packaging

### Linux
- **AppImage** - Single-file executable (all distros)
- **Flatpak** - Sandboxed distribution
- **Snap** - Ubuntu/derivatives
- **.deb** - Debian/Ubuntu packages
- **.rpm** - Fedora/RHEL packages
- **AUR** - Arch User Repository (PKGBUILD)

### macOS
- **.dmg** - Drag-and-drop installer
- **Homebrew** - `brew install tabssh`
- **MacPorts** - `port install tabssh`
- **App Store** (future)

### Windows
- **.msi** - Windows Installer
- **WinGet** - `winget install tabssh`
- **Chocolatey** - `choco install tabssh`
- **Scoop** - `scoop install tabssh`

### BSD
- **FreeBSD pkg** - `pkg install tabssh`
- **FreeBSD ports** - `/usr/ports/net/tabssh`
- **OpenBSD packages**
- **NetBSD pkgsrc**

---

## Contributing Guidelines

### Code Style
- Follow Rust standard style (`rustfmt`)
- Use `clippy` for linting
- Document public APIs
- Write tests for new features

### Pull Request Process
1. Fork repository
2. Create feature branch
3. Write code + tests
4. Run `cargo fmt` and `cargo clippy`
5. Submit PR with description
6. Pass CI checks
7. Code review
8. Merge

---

## License

MIT License - Same as Android version

---

## Resources

### Rust SSH Libraries
- **russh:** https://crates.io/crates/russh
- **ssh2:** https://crates.io/crates/ssh2 (libssh2 bindings)
- **thrussh:** https://crates.io/crates/thrussh

### Terminal Emulation
- **alacritty_terminal:** https://crates.io/crates/alacritty_terminal
- **vte:** https://crates.io/crates/vte

### UI Frameworks
- **egui:** https://github.com/emilk/egui
- **iced:** https://github.com/iced-rs/iced
- **druid:** https://github.com/linebender/druid

### Cross-Compilation
- **cross:** https://github.com/cross-rs/cross
- **cargo-zigbuild:** https://github.com/rust-cross/cargo-zigbuild

### Packaging
- **cargo-bundle:** https://crates.io/crates/cargo-bundle
- **cargo-deb:** https://crates.io/crates/cargo-deb
- **cargo-wix:** https://crates.io/crates/cargo-wix

---

## Directory Organization Policy

### ✅ **Strict Structure Rules**

1. **ALL source code** → `src/`
   - Application code
   - Library modules
   - Platform-specific implementations
   - No source files outside `src/`

2. **ALL test files** → `tests/`
   - Integration tests
   - Unit tests
   - Test utilities and helpers
   - No test code in `src/`

3. **ALL production scripts** → `scripts/`
   - Build scripts
   - Release automation
   - Docker configurations
   - CI/CD helpers
   - No scripts in project root

4. **Build outputs** → Separate directories
   - Debug binaries → `binaries/` (gitignored)
   - Release binaries → `releases/` (gitignored)
   - Cargo artifacts → `target/` (gitignored)

5. **Configuration files** → Project root only
   - `Cargo.toml`, `Makefile`, `.gitignore`
   - `.cargo/config.toml` for cross-compilation
   - `README.md`, `LICENSE.md`, `CLAUDE.md`

### ❌ **Never**
- No source code in project root
- No test files mixed with source
- No build scripts outside `scripts/`
- No random utility files scattered around

---

## Quick Start

### 1. Initialize Project
```bash
cd /root/Projects/github/tabssh/desktop

# Initialize Cargo project
cargo init --name tabssh

# Create directory structure
mkdir -p src/{ui,ssh,sftp,terminal,storage,crypto,platform,config,utils}
mkdir -p tests/{integration,unit,common}
mkdir -p scripts/{docker,build,release}
mkdir -p assets/{icons,themes,fonts}
mkdir -p binaries releases
```

### 2. Set up Docker
```bash
# Create Dockerfile
cat > scripts/docker/Dockerfile << 'EOF'
FROM alpine:latest
RUN apk add --no-cache rust cargo musl-dev gcc g++ make openssl-dev git bash
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
WORKDIR /workspace
CMD ["cargo", "build", "--release"]
EOF

# Build Docker image
docker build -t tabssh-rust-alpine -f scripts/docker/Dockerfile .
```

### 3. Add Dependencies
Edit `Cargo.toml` with core dependencies (see Cargo.toml section above)

### 4. Create Minimal Test
```bash
# Create simple SSH connection test
cat > src/main.rs << 'EOF'
fn main() {
    println!("TabSSH Desktop - Rust SSH Client");
    println!("Version: 0.1.0");
}
EOF

cargo run
```

### 5. Build First Binary
```bash
# Local build
cargo build --release

# Docker build (Linux static)
docker run --rm -v $(pwd):/workspace tabssh-rust-alpine \
    cargo build --release --target x86_64-unknown-linux-musl

# Rename to our convention
cp target/x86_64-unknown-linux-musl/release/tabssh binaries/tabssh-linux-amd64
```

---

## Summary

### Project Specifications

| Aspect | Details |
|--------|---------|
| **Language** | Rust 1.75+ (2021 edition) |
| **UI** | egui (pure Rust, lightweight) |
| **SSH** | russh (pure Rust SSH2) |
| **Platforms** | Linux, macOS, Windows, FreeBSD, OpenBSD, NetBSD |
| **Architectures** | amd64 (x86_64), arm64 (aarch64) |
| **Binary Count** | 11 variants |
| **Binary Size** | 8-12 MB (static, stripped) |
| **Build Tool** | Cargo + Make + Docker (Alpine) |
| **Testing** | All tests in `tests/` |
| **Scripts** | All scripts in `scripts/` |

### Key Features (Parity with Android)
- ✅ Browser-style tabs for multiple SSH sessions
- ✅ Full VT100/xterm terminal emulation
- ✅ Integrated SFTP browser
- ✅ Port forwarding (local, remote, dynamic)
- ✅ 10+ color themes
- ✅ Secure credential storage (OS keychain)
- ✅ SSH config import
- ✅ Session persistence
- ✅ Cross-platform native UI

### Build Outputs
```
releases/v0.1.0/
├── tabssh-linux-amd64           # Static musl (no -musl suffix!)
├── tabssh-linux-arm64           # Static musl
├── tabssh-macos-amd64           # Intel Mac
├── tabssh-macos-arm64           # Apple Silicon
├── tabssh-windows-amd64.exe     # Windows x64
├── tabssh-windows-arm64.exe     # Windows ARM
├── tabssh-freebsd-amd64         # FreeBSD
├── tabssh-freebsd-arm64         # FreeBSD ARM
├── tabssh-openbsd-amd64         # OpenBSD
├── tabssh-openbsd-arm64         # OpenBSD
├── tabssh-netbsd-amd64          # NetBSD
├── checksums.txt                # SHA256
└── tabssh-desktop-0.1.0-source.tar.gz
```

---

**This specification provides a complete blueprint for TabSSH Desktop.**

**Structure: Clean. Build: Docker Alpine. Binaries: Static, no -musl suffix.**

**Ready to start development!** 🦀🚀
