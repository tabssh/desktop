# TabSSH Desktop - Rust Cross-Platform SSH Client

**Last Updated:** 2025-12-19
**Version:** 0.1.0 (Active Development)
**Status:** 🚧 Phase 1-2: Foundation & Core Features

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
├── docker/                     # Docker build environment
│   └── Dockerfile              # Debian-based Rust build image with GUI support
├── scripts/                    # Build & release automation
│   ├── build/
│   │   └── build-all.sh        # Build all targets
│   └── release/
│       └── release.sh          # Release automation
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

### Dockerfile (Debian + Rust + GUI)

**Location:** `docker/Dockerfile`

```dockerfile
FROM rustlang/rust:nightly-bookworm

# Install build + runtime dependencies
RUN apt-get update && apt-get install -y \
    build-essential pkg-config cmake git \
    libssl-dev \
    libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libxkbcommon-dev libfontconfig1-dev libgtk-3-dev \
    libx11-6 libxcursor1 libxrandr2 libxi6 \
    libgl1-mesa-glx libgl1-mesa-dri libegl1-mesa \
    libwayland-client0 libwayland-egl1 \
    fonts-dejavu-core \
    musl-tools musl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

ENV CC_x86_64_unknown_linux_musl=musl-gcc
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc

WORKDIR /workspace
CMD ["cargo", "build"]
```

### Docker Image
- **Name:** `tabssh-builder`
- **Base:** `rustlang/rust:nightly-bookworm`
- **Rust:** Nightly (for latest features)
- **Size:** ~2GB (with toolchains + GUI deps)
- **Purpose:** Build environment with GUI support for testing
- **Build:** `docker build -t tabssh-builder -f docker/Dockerfile .`

### Building with Docker

```bash
# Build Docker image
docker build -t tabssh-builder -f docker/Dockerfile .

# Build for host (native binary with GUI support)
docker run --rm \
    -v $(pwd):/workspace \
    -w /workspace \
    -e DISPLAY=$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    tabssh-builder \
    cargo build --release

# Build Linux x86_64 (static musl)
docker run --rm \
    -v $(pwd):/workspace \
    -w /workspace \
    tabssh-builder \
    cargo build --release --target x86_64-unknown-linux-musl

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
.PHONY: build release test docker clean help

# Build for current platform (outputs to ./binaries/)
make build

# Release build (outputs to ./releases/ with archive)
make release

# Build platform-specific (future)
make build-linux-amd64      # Linux x86_64 (musl static)
make build-linux-arm64      # Linux ARM64 (musl static)
make build-macos-amd64      # macOS Intel
make build-macos-arm64      # macOS Apple Silicon
make build-windows-amd64    # Windows x86_64

# Build Docker image
make docker

# Run tests
make test

# Clean
make clean                  # Clean Rust build artifacts

# Help
make help                   # Show all targets
```

### Makefile Structure

**Location:** `./Makefile`

Key targets:
- `build` - Build binaries with Docker → `./binaries/`
- `release` - Release build with Docker → `./releases/` (includes archive, checksums, release.txt)
- `docker` - Build Docker image with buildx (multi-arch: linux/amd64, linux/arm64)
- `test` - Run all tests in Docker
- `clean` - Remove build artifacts

Binary outputs:
- Development: `./binaries/tabssh-{os}-{arch}`
- Release: `./releases/tabssh-{os}-{arch}` + `tabssh-{version}-source.tar.gz`

---

## Development Roadmap

### Phase 1: Foundation (Weeks 1-4) ✅ **COMPLETE**
- [x] Set up project structure
- [x] Create egui window with tab support
- [x] SQLite database schema
- [x] Configuration management
- [x] Implement basic SSH connection framework (russh)
- [x] Basic terminal buffer structure

### Phase 2: Core Features (Weeks 5-8) 🚧 **IN PROGRESS**
- [x] Connection manager UI
- [x] Terminal rendering in egui
- [x] Terminal buffer with scrollback
- [x] ANSI escape parser (VTE)
- [ ] Complete SSH authentication (password, key)
- [ ] Active SSH I/O integration
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
# Debug build (without Docker)
cargo build

# Run locally
cargo run

# Release build (optimized, without Docker)
cargo build --release

# Run tests
cargo test
```

### With Docker (Recommended)
```bash
# Build with Docker → outputs to ./binaries/
make build

# Run the built binary
./binaries/tabssh

# Run tests in Docker
make test
```

### Release Build
```bash
# Build release artifacts → outputs to ./releases/
make release

# Creates:
# ./releases/
# ├── tabssh                          # Native binary
# ├── tabssh-linux-amd64              # Static musl binary
# ├── checksums.txt                   # SHA256 checksums
# ├── release.txt                     # Version info (version, commit, date)
# └── tabssh-{version}-source.tar.gz  # Source archive (excludes .git, target/, binaries/, releases/)
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

### 2. Set up Docker (Already Done!)
```bash
# Docker image already exists at docker/Dockerfile
docker build -t tabssh-builder -f docker/Dockerfile .
```

### 3. Build & Run (Already Implemented!)
```bash
# Build with Docker
make build

# Run locally (requires GUI)
./binaries/tabssh

# Or run directly with cargo
cargo run
```

### 4. Release
```bash
# Build release with archive
make release

# Output in ./releases/:
# - tabssh
# - tabssh-linux-amd64
# - checksums.txt
# - release.txt (version info)
# - tabssh-{version}-source.tar.gz (source archive, excludes VCS)
```

---

---

## 📊 Current Implementation Status

### ✅ **Implemented** (Phase 1 Complete)

**Core Infrastructure:**
- ✅ Project structure with modular architecture (7,750+ lines of Rust code)
- ✅ Docker build environment (Debian-based with GUI support)
- ✅ Makefile automation (build, release, test targets)
- ✅ Git repository with proper .gitignore
- ✅ Build versioning with git commit tracking

**UI Layer (egui):**
- ✅ Main application window with sidebar navigation
- ✅ Tab manager with browser-style tabs
- ✅ Connection manager screen with list/grid views
- ✅ Connection editor modal dialog
- ✅ Terminal view screen (UI structure)
- ✅ Settings screen (placeholder)
- ✅ Quick connect dialog
- ✅ Password/key authentication dialog
- ✅ Keyboard shortcuts (Ctrl+T, Ctrl+W, Ctrl+Tab, Ctrl+1-9)
- ✅ Custom color scheme and styling

**Terminal Emulation:**
- ✅ Terminal buffer with scrollback (10,000 lines)
- ✅ Cell-based character grid
- ✅ ANSI escape sequence parser (VTE-based)
- ✅ Cursor management and positioning
- ✅ Alternate screen buffer support
- ✅ Color support (256-color + true color)
- ✅ Text attributes (bold, italic, underline, etc.)

**Storage & Data:**
- ✅ SQLite database with schema
- ✅ Connection profiles table
- ✅ SSH keys table
- ✅ Known hosts table
- ✅ Themes table
- ✅ Settings table
- ✅ Database initialization

**SSH Framework:**
- ✅ Session manager structure
- ✅ Connection configuration
- ✅ Authentication types (password, public key)
- ✅ Active session tracking
- ✅ Async runtime integration (Tokio)

### 🚧 **In Progress** (Phase 2)

- 🚧 SSH connection implementation (connect, authenticate, disconnect)
- 🚧 Terminal I/O (read/write to SSH channel)
- 🚧 Terminal renderer (display SSH output in egui)
- 🚧 Host key verification
- 🚧 Session persistence

### ❌ **Not Implemented** (Phases 3-6)

**Phase 3 - Advanced SSH:**
- ❌ SFTP browser implementation
- ❌ File transfer with progress
- ❌ Port forwarding (local, remote, dynamic)
- ❌ SSH agent integration
- ❌ SSH config file parser
- ❌ Jump host support

**Phase 4 - UI Polish:**
- ❌ Theme system (10+ color schemes)
- ❌ Settings persistence
- ❌ Context menus
- ❌ Drag-and-drop
- ❌ Search functionality

**Phase 5 - Platform Integration:**
- ❌ macOS Keychain integration
- ❌ Windows Credential Manager
- ❌ Linux Secret Service
- ❌ System tray integration
- ❌ Auto-update mechanism
- ❌ Platform-specific installers

**Phase 6 - Testing & Release:**
- ❌ Test suite (0 test files currently)
- ❌ Cross-platform testing
- ❌ Performance optimization
- ❌ Security audit
- ❌ Documentation
- ❌ CI/CD pipeline

### 📈 **Progress: ~35% Complete**

| Component | Progress | Status |
|-----------|----------|--------|
| Project Structure | 100% | ✅ Complete |
| UI Framework | 70% | 🚧 Core done, polish needed |
| Terminal Emulation | 60% | 🚧 Buffer done, I/O needed |
| SSH Core | 30% | 🚧 Framework done, connect needed |
| Storage | 80% | ✅ Schema done, usage needed |
| SFTP | 5% | ❌ Stub only |
| Platform Integration | 0% | ❌ Not started |
| Testing | 0% | ❌ No tests |

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
