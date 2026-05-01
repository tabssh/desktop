# TabSSH Desktop - Claude Project Tracker

**Last Updated:** 2026-05-01
**Version:** 0.1.0 (early development — not yet released)
**Status:** 🚧 **Phase 2 in progress** — see [Current Implementation Status](#-current-implementation-status). The "100% complete" framing in `STATUS.md` / `PROGRESS_REPORT.md` / `TODO.AI.md` (Dec 2025) was aspirational; those docs are slated for deletion. This file is the source of truth.
**Build Status:** ❌ **Does not compile.** Verified 2026-05-01 via `docker run --rm -v $(pwd):/workspace -w /workspace tabssh-builder cargo check`: **50 errors + 17 warnings**. Concentration:
- `src/sftp/client.rs` — 42 errors, almost entirely russh-sftp 2.0 API drift (`From<&Cow<'_, str>>` not implemented, `setstat` no longer on `SftpSession`, method-arity mismatches on read/write methods)
- `src/ssh/forwarding.rs` — 5 errors
- `src/sftp/browser.rs`, `src/terminal/emulator.rs` — 3 each (Copy-trait + unused-variable warnings the latter)
- Misc 1–2 each in ssh/connection, ssh/config_parser, ssh/active_session, sftp/transfer, sftp/operations, ui/components, ui/keyboard, ui/screens/{settings,sftp_browser_ui}
- Down from the 61-error count COMPILATION_STATUS.md captured 2025-12-20 — the russh handler-lifetime issues are gone; russh-sftp shape-mismatch is now the dominant blocker.

**🎯 Goal:** Cross-platform desktop SSH/SFTP client — Windows, Linux, macOS, BSD — feature parity with the Android app where applicable, plus desktop-native conveniences.

**📱 Android Reference:** `../android/` — see `../android/CLAUDE.md` (synced 2026-04-28). Current Android state:
- ~215 Kotlin files, ~65,534 LOC, Room database **v26** (25 forward migrations from v1)
- 23 Activities, 7 Fragments, 1 ForegroundService
- Waves 1.X through 9.2 + Issue #163–#175 shipped:
  - Wave roll-up: env vars, agent forwarding, remote file editor in SFTP, chmod editor, SCP fallback, OpenSSH user certs, Telnet, GUI theme editor, Workspaces, command palette / quick switcher / history palette, broadcast input, split view, color tags, PIN lock, background tunnels, 24-bit color, foldable + sw720dp layouts, cluster command live streaming, cloud host import (DigitalOcean / Hetzner / Linode / Vultr), 3-way merge with conflict UI, SSH config export, bulk import, VM serial console via hypervisor API, Mosh native binaries cross-compiled per ABI, ANR watchdog + crash reporter
  - Post-audit (2026-04-28+): multi-tab same-host independent shells (#163), Active Sessions strip (#165), edge-swipe tab switching (#168 — mobile-only), tmux/screen/zellij auto-launch + post-connect script (#170), always-on keepalive (#166 — removed per-profile toggle), centralised error dialogs with Copy (#167), recordable macros DB v25→v26 (#173), hardware-kbd modifier-aware nav keys + AltGr (#171), cold-start ANR fixes (#158), cold-start commit-id marker (#164), repo cleanup (#160), on-screen kbd ergonomics (#161, #162), QR pairing (mobile shipped 2026-04-28 — desktop side WIP, see §QR pairing)
- DB schema additions vs prior sync: v24 `connections.remote_command` (Issue #37); v25 `connections.ip_mode` (auto/ipv4/ipv6, Issue #6); v26 `macros` table (Issue #173 raw byte sequences, distinct from snippets).
- See `../android/FEATURES_AUDIT.md` for the full have/want/drop matrix vs JuiceSSH/Termius — the desktop should prioritise Tier-1 items there.

**Android App Status (synced 2026-04-28):**
- ✅ Core SSH (password, public key, keyboard-interactive, OpenSSH user certs)
- ✅ Universal SSH key support — OpenSSH / PEM / PKCS#8 / PuTTY v2/v3; RSA / ECDSA / Ed25519 / DSA
- ✅ In-app SSH key generation
- ✅ **SAF-based universal cloud sync** — works with any storage provider (Google Drive, Dropbox, OneDrive, Nextcloud, local). AES-256-GCM + PBKDF2 (100k iterations). 3-way merge with conflict UI. Per-entity sync toggles. Zero Google services dependency.
- ✅ **Hypervisor management** — Proxmox VE, XCP-ng, Xen Orchestra (REST + WebSocket live updates), VMware ESXi/vCenter
- ✅ **VM serial console via hypervisor** (no VM network required — works during OS install / for VMs without network)
- ✅ Mosh — native cross-compiled `mosh-client` binaries per ABI (Wave 9.2)
- ✅ X11 forwarding — `X11ForwardingManager` (Wave 7)
- ✅ Telnet (RFC 854) — Wave 2.3
- ✅ Workspaces, command palette, quick switcher, history palette, split view, broadcast input, cluster commands
- ✅ Color tags per host, GUI theme editor, 23 built-in terminal themes
- ✅ PIN code app lock + biometric, screenshot protection (FLAG_SECURE)
- ✅ Cloud host import — DigitalOcean / Hetzner / Linode / Vultr (opt-in, tokens in Keystore not DB)
- ✅ Bulk import / export, SSH config round-trip, encrypted ZIP backup
- ✅ ANR watchdog + crash reporter (debug builds auto-on, release opt-in)

**Desktop-Specific Advantages (where mobile constraints don't apply):**
- ✅ **Single static Rust binary** — drop into `$PATH` and run; no JVM, no Android SDK, no runtime install
- ✅ **Native `~/.ssh/` access** — direct read of the OS user's SSH config, keys, and known_hosts. Platform-specific paths:
  - Linux/BSD: `~/.ssh/`
  - macOS: `~/.ssh/`
  - Windows: `%USERPROFILE%\.ssh\`
- ✅ **Native crate ecosystem — no JNI / NDK pain.** Implement features in pure Rust where mobile shipped wrappers:
  - SSH: `russh` / `russh-keys` / `russh-sftp` (pure Rust SSH2)
  - Terminal: `alacritty_terminal` + `vte` (proven VT/xterm emulation)
  - SSH config: `ssh2-config` for `~/.ssh/config` parsing
  - SSH key parsing: `ssh-key` crate (universal — OpenSSH/PEM/PKCS#8/PuTTY)
  - SSH key types: `ed25519-dalek`, `rsa`, `p256`, `p384`, `p521` for generation
  - Mosh: native client implementation in Rust (avoid the cross-compile dance the Android app does for `libmosh-client.so`)
  - X11: `x11rb` (pure Rust X11 protocol)
  - Keychain: `keyring` crate (Linux Secret Service / macOS Keychain / Windows Credential Manager — single API, no platform code in the app layer)
  - Clipboard: `arboard` (cross-platform)
  - Crypto: `aes-gcm`, `pbkdf2`, `argon2`, `ring`
- ✅ **OS keychain integration** is first-class (mobile has Keystore; desktop has Keychain/Credential Manager/Secret Service)
- ✅ **Larger screen real estate** — split panes, multiple windows, real keyboard shortcuts
- ✅ **Native performance** (no JVM overhead, no GC)
- ✅ **Smaller binaries** (8–14 MB static vs 30 MB APK)
- ✅ **No Google Play Services anywhere** — already a non-issue on desktop
- ✅ **Cross-platform** — Linux, macOS, Windows, FreeBSD, OpenBSD, NetBSD; amd64 + arm64 (11 binary variants)

**Mobile-only features that don't carry over to desktop:**
- ❌ Foreground service notification (desktop has system tray instead)
- ❌ SAF-based cloud sync (desktop uses filesystem watchers + the user's existing sync app, e.g. Nextcloud client, rclone, syncthing)
- ❌ Volume keys → font size (use Ctrl+scroll on desktop)
- ❌ Pinch-zoom (use Ctrl+scroll)
- ❌ Swipe between tabs (use Ctrl+Tab / mouse)
- ❌ On-screen keyboard (desktop has a real keyboard)
- ❌ Foldable book-mode / sw720dp layouts (desktop windows are resizable anyway)
- ❌ Tasker integration (desktop equivalents would be Linux .desktop files / shell scripts)

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture & Technology Stack](#architecture--technology-stack)
3. [Current Implementation Status](#current-implementation-status)
4. [Feature Comparison with Android](#feature-comparison-with-android)
5. [Binary Naming & Distribution](#binary-naming--distribution)
6. [Project Structure](#project-structure)
7. [Core Dependencies](#core-dependencies)
8. [Build System](#build-system)
9. [Development Roadmap](#development-roadmap)
10. [Testing Strategy](#testing-strategy)
11. [Security Considerations](#security-considerations)
12. [Performance Targets](#performance-targets)
13. [Distribution & Packaging](#distribution--packaging)
14. [Android App Feature Reference](#android-app-feature-reference)
15. [GitHub Actions CI/CD](#github-actions-cicd)
16. [Contributing Guidelines](#contributing-guidelines)
17. [Resources & Links](#resources--links)

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

### 🐳 Docker-First Development

**CRITICAL: Rust is NOT installed locally. ALL operations use Docker.**

- ✅ All builds use Docker (via Makefile)
- ✅ All tests run in Docker containers
- ✅ CI/CD uses Docker exclusively
- ✅ Cross-compilation via Docker with musl targets
- ✅ No local Rust toolchain required

```bash
make build      # Builds with Docker → ./binaries
make test       # Tests in Docker
make release    # Release build → ./releases
make docker     # Multi-arch Docker image (amd64, arm64)
```

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
- **Tags:** 
  - `:latest` - Always latest build
  - `:0.1.0` - Version from Cargo.toml
  - `:16cba3f1` - Git commit ID (8 chars)
  - `:2512` - YYMM format (December 2025)

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
  - Tags: `:latest`, `:{version}`, `:{commit}`, `:{YYMM}`
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

## Android App Feature Sync (Latest: 2026-04-28)

### Current Android App Status

**Production-track on Android:** baseline v1.0.0 plus Waves 1.X – 9.2 shipped. See `../android/CLAUDE.md` ("What landed since 2026-02-11") and `../android/FEATURES_AUDIT.md` for the parity tracker.

**The mobile→desktop port list is grouped by what has a clean Rust crate path vs what needs design work.**

### 🟢 Easy ports — pure Rust crate available

#### 1. SSH core + key management — 🔴 Desktop TODO
- **Universal key parser:** `ssh-key` crate handles OpenSSH, PEM (PKCS#1), PKCS#8, PuTTY v2/v3 — no need to re-implement BouncyCastle's logic.
- **Key generation:** `ed25519-dalek`, `rsa`, `p256` / `p384` / `p521` cover the algorithms mobile supports.
- **OpenSSH user certificates** (`*-cert.pub`, mobile Wave 2.2): `ssh-key` supports OpenSSH cert format.
- **Storage:** OS keychain via `keyring` crate (single API across Linux/macOS/Windows).

#### 2. Terminal + protocol features — 🟡 Desktop Partial
- **Mosh:** mobile ships cross-compiled `libmosh-client.so` per Android ABI. Desktop can use a pure-Rust client implementation, sidestepping the cross-compile workflow entirely.
- **X11 forwarding:** mobile has `X11ForwardingManager` (Wave 7). Desktop already runs on X11 / Wayland natively — `x11rb` for protocol-level work.
- **Telnet (Wave 2.3):** trivial in Rust; many crates available.
- **Terminal renderer:** `alacritty_terminal` already lined up in the dependency list.

#### 3. SSH config — 🔴 Desktop TODO
- **Import:** `ssh2-config` parses `~/.ssh/config` directly. Desktop has the advantage of being able to read the user's existing config without copy-paste.
- **Round-trip export (Wave 6.1):** mobile has `SSHConfigExporter` — replicate the same generator logic in Rust.
- **Bulk import (Waves 1, 6.4/6.5):** CSV/JSON via `serde`; PuTTY .ppk lists via `ssh-key`.

#### 4. Cloud host import — 🔴 Desktop TODO
- **Mobile providers:** DigitalOcean, Hetzner, Linode, Vultr (Wave 5.1). Each has a documented JSON REST API.
- **Desktop:** `reqwest` for HTTP, `serde` for JSON, `keyring` for token storage. Same opt-in-only model — no auto-discovery.
- **Token isolation:** mobile stores tokens under `cloud_token_${id}` in Keystore (not in the DB). Replicate via `keyring`.

### 🟡 Adapted ports — different mechanism, same intent

#### 5. Cloud sync — 🔴 Desktop TODO
- **Mobile:** SAF-based (storage provider apps handle the sync). Three-way merge, conflict UI, AES-256-GCM + PBKDF2 (100k iter).
- **Desktop:** filesystem watchers + the user's existing sync app (Nextcloud client, syncthing, rclone, OneDrive client, Dropbox client). TabSSH writes encrypted blobs to a folder; the user picks where the folder lives.
- **Crates:** `notify` for filesystem watch, `aes-gcm` + `pbkdf2` for the same on-wire format mobile uses (so the encrypted blobs are interchangeable across phone and desktop).

#### 6. UX adaptations
| Mobile | Desktop equivalent |
|--------|---------------------|
| Swipe between tabs | Ctrl+Tab / mouse click on tab |
| Volume keys → font size | Ctrl+Scroll |
| Long-press URL → Open/Copy dialog | Ctrl+Click → open in browser |
| Foreground-service notification | System tray icon |
| On-screen 1-5 row keyboard | Real keyboard (always available) |
| Foldable book-mode / sw720dp | Resizable windows + split panes |
| Pinch-zoom | Ctrl+Scroll |

#### 7. Hypervisor management — 🔴 Desktop TODO
- **Mobile (Wave 7):** Proxmox VE, XCP-ng, Xen Orchestra (REST + WebSocket live updates), VMware ESXi/vCenter, with VM serial console via hypervisor API for VMs without network.
- **Desktop:** all the same providers can be reached over HTTP/WebSocket from Rust (`reqwest`, `tokio-tungstenite`). Larger screen makes the VM list / hypervisor dashboard much nicer than on mobile.

#### 8. UI features that benefit from desktop — 🔴 Desktop TODO
- **Workspaces (Wave 2.5):** named tab groups. Desktop has multi-window, so this maps naturally — each workspace can be its own window.
- **Command palette (Ctrl+K), Quick switcher (Ctrl+J), History palette (Ctrl+R):** keyboard-centric features designed for desktop ergonomics; mobile already has them.
- **Split view (Wave 2.8):** mobile has it; desktop is the natural home for multi-pane.
- **Broadcast input / cluster commands:** great fit on desktop where you can have many panes visible simultaneously.
- **GUI theme editor (Wave 2.4):** mobile has it; reuse the JSON theme format mobile reads.
- **Color tags per host (Wave 3.1):** trivial DB column + UI accent.

### 🔴 Mobile-specific — does not map

- Android Widget (home screen)
- Custom multi-touch tmux/screen gestures (mobile Wave 4.f)
- Tasker integration
- Voice typing affordance
- ANR watchdog (Android-specific concept; desktop has its own crash reporting via `panic::set_hook`)
- Android keyboard customisation (1-5 rows of soft keys)
- SAF document URIs (desktop reads / writes files directly)

### 🟢 Desktop-only wins — no mobile equivalent

- **`~/.ssh/` direct access** — read the user's existing config and keys without import. Major UX win over mobile.
- **System tray + auto-launch** — connect on login if user wants
- **Multiple windows** — group hosts however the user prefers
- **Real CLI mode** — `tabssh user@host` invocation from a shell
- **Native package distribution** — apt, brew, winget, AUR, pkgsrc, ports
- **OS-native font rendering** at any size (no Termux dependency)
- **Real X11 forwarding** that works with desktop X servers, not headless

---

## Comparison with Android Version (synced 2026-04-28)

Status legend: ✅ shipped on this platform · 🟡 partial · 🔴 TODO · 🚫 doesn't apply

| Capability | Android | Desktop | Notes |
|------------|---------|---------|-------|
| **Language** | Kotlin (JVM) | Rust 2021 | — |
| **UI Framework** | Material Design 3 / Jetpack Compose | egui (pure Rust) | — |
| **SSH Library** | JSch (mwiede 2.27.7) | russh (pure Rust) | — |
| **Terminal** | Termux emulator | alacritty_terminal | — |
| **Database** | Room (SQLite) v26 | rusqlite (SQLite) | Schema lives in `src/storage/`. Mobile schema is the spec; desktop should track it via numbered migrations. Latest mobile additions: v24 `connections.remote_command`, v25 `connections.ip_mode`, v26 `macros` table |
| **Core SSH** | ✅ password / pubkey / keyboard-int | ✅ password + pubkey | Keyboard-interactive 🔴 |
| **Universal key parser** | ✅ OpenSSH/PEM/PKCS#8/PuTTY | 🟡 partial | Use `ssh-key` crate |
| **In-app key generation** | ✅ all algorithms | 🔴 TODO | `ed25519-dalek` + `rsa` + `p256/384/521` |
| **OpenSSH user certificates** (Wave 2.2) | ✅ | 🔴 TODO | `ssh-key` supports this |
| **Mosh** | ✅ Wave 9.2 (cross-compiled .so per ABI) | 🔴 TODO | Pure Rust client preferred over wrapping mosh-client |
| **X11 forwarding** | ✅ Wave 7 (`X11ForwardingManager`) | 🔴 TODO | `x11rb` crate |
| **Telnet** | ✅ Wave 2.3 | 🔴 TODO | — |
| **SFTP browser** | ✅ dual-pane | 🔴 TODO | `russh-sftp` crate |
| **Remote file editor in SFTP** | ✅ Wave 1.7 | 🔴 TODO | — |
| **chmod editor** | ✅ Wave 1.8 | 🔴 TODO | — |
| **SCP fallback** | ✅ Wave 1.9 | 🔴 TODO | — |
| **Port forwarding (-L/-R/-D)** | ✅ + bind-all | 🔴 TODO | `russh` channel API |
| **Background tunnels** | ✅ Wave 3.3 | 🔴 TODO | — |
| **ProxyJump cascading** | ✅ | 🔴 TODO | — |
| **SSH config import** | ✅ | 🔴 TODO | `ssh2-config` crate; desktop reads `~/.ssh/config` directly |
| **SSH config export round-trip** | ✅ Wave 6.1 | 🔴 TODO | — |
| **Bulk import (CSV/JSON/PuTTY)** | ✅ Waves 1, 6.4/6.5 | 🔴 TODO | — |
| **Workspaces (named tab groups)** | ✅ Wave 2.5 | 🔴 TODO | New SQLite table |
| **Command palette (Ctrl+K)** | ✅ Wave 2.6 | 🔴 TODO | — |
| **Quick switcher (Ctrl+J)** | ✅ Wave 2.6 | 🔴 TODO | — |
| **History palette (Ctrl+R)** | ✅ Wave 2.10 | 🔴 TODO | — |
| **Split view** | ✅ Wave 2.8 | 🔴 TODO | egui has good multi-pane support |
| **Broadcast input** | ✅ Wave 2.7 | 🔴 TODO | — |
| **Cluster commands + live streaming** | ✅ Wave 4.e | 🔴 TODO | — |
| **GUI theme editor** | ✅ Wave 2.4 | 🔴 TODO | — |
| **23 built-in terminal themes** | ✅ | 🔴 TODO (10+ planned) | Reuse mobile's JSON theme files |
| **Per-host color tags** | ✅ Wave 3.1 | 🔴 TODO | Single column on the connection table |
| **PIN code app lock** | ✅ Wave 3.2 | 🔴 TODO | — |
| **Biometric app lock** | ✅ | 🟡 partial | Touch ID via `keyring`/`security-framework` on macOS, Windows Hello on Windows |
| **Screenshot protection (FLAG_SECURE)** | ✅ | 🚫 | Desktop OSes don't expose an equivalent universally |
| **Hypervisor management** | ✅ Proxmox / XCP-ng / Xen Orchestra / VMware (Wave 7) | 🔴 TODO | All HTTP/WS — easy port |
| **VM serial console via hypervisor** | ✅ Wave 7 | 🔴 TODO | — |
| **Cloud host import** | ✅ DO/Hetzner/Linode/Vultr (Wave 5.1) | 🔴 TODO | `reqwest` + `keyring` |
| **Cloud sync** | ✅ SAF + AES-256-GCM/PBKDF2 + 3-way merge | 🔴 TODO | Filesystem watch + user's sync app |
| **Snippets library + variable prompts** | ✅ | 🔴 TODO | — |
| **Identity abstraction** | ✅ | 🔴 TODO | — |
| **Connection groups/folders** | ✅ | 🔴 TODO | — |
| **Encrypted ZIP backup/restore** | ✅ | 🔴 TODO | — |
| **ANR watchdog + crash reporter** | ✅ debug auto-on, release opt-in | 🟡 partial | `panic::set_hook` for crashes; ANR concept doesn't apply, but "UI thread frozen" detection does |
| **Find/search in scrollback** | ✅ Wave 1 | 🔴 TODO | — |
| **24-bit true color** | ✅ Wave 4.a | 🔴 TODO | `alacritty_terminal` supports it |
| **Multi-tab same-host independent shells** | ✅ Issue #163 | 🔴 TODO | Per-tab `ChannelShell`/`ChannelExec` on a single SSH session; sibling tabs survive when one shell exits. `russh::Channel` per tab |
| **Active Sessions strip** | ✅ Issue #165 | 🔴 TODO | Top-of-window list of running tabs with terminal title (OSC 0/2) + connection-state dot; tap to focus |
| **Tmux/Screen/Zellij auto-launch + post-connect script** | ✅ Issue #170 | 🔴 TODO | `profile.multiplexerMode` (`AUTO_ATTACH`/`CREATE_NEW`) + `profile.postConnectScript` — both defined but unwired in old desktop schema |
| **Always-on keepalive** | ✅ Issue #166 | 🔴 TODO | 60s server-alive interval, count-max 3, no per-profile toggle. Apply to russh session config |
| **Centralised error dialogs with Copy** | ✅ Issue #167 | 🔴 TODO | All `showError`/"Failed" routed through `DialogUtils.showErrorDialog`; clipboard via `arboard` |
| **Recordable macros** | ✅ Issue #173 (DB v26) | 🔴 TODO | Capture raw byte sequences (escape codes, paste payloads, modifier-composed Ctrl/Alt); replay verbatim. Distinct from snippets |
| **Hardware-kbd modifier-aware nav keys + AltGr** | ✅ Issue #171 | 🔴 TODO | xterm-style `\e[1;<mod><letter>` for Shift/Ctrl/Alt + arrows / HOME / END / PG family. AltGr distinguished from real Alt |
| **Cold-start commit-id marker** | ✅ Issue #164 | 🔴 TODO | Log `## binary built from: <commit> ##` once per commit-id change. Resolves at build time via `build.rs`, falls back to `release.txt` |
| **QR pairing (desktop → mobile)** | ✅ mobile shipped 2026-04-28 | 🔴 TODO | **Desktop is the sender** — see §QR pairing below for the full Rust TODO checklist (qrcodegen + ciborium + argon2 + aes-gcm) |
| **Tasker integration** | ✅ | 🚫 | Desktop equivalent is shell scripting / `.desktop` actions |
| **Foldable + tablet-sw720dp layouts** | ✅ Waves 4.b/4.c | 🚫 | Desktop windows are resizable anyway |
| **Custom keyboard 1-5 rows** | ✅ | 🚫 | Real keyboard always available |
| **Volume keys → font size** | ✅ | 🚫 | Use Ctrl+Scroll instead |
| **Pinch-zoom font size** | ✅ | 🚫 | Use Ctrl+Scroll instead |
| **Swipe between tabs** | ✅ | 🚫 | Use Ctrl+Tab |
| **URL detection long-press** | ✅ | 🔴 TODO (Ctrl+Click instead) | — |
| **Direct ~/.ssh/ access** | 🚫 (Android sandbox) | 🆕 desktop-only | Major UX win |
| **System tray** | 🚫 (Android equivalent: foreground-service notification) | 🔴 TODO | `tray-icon` crate |
| **CLI mode (`tabssh user@host` from shell)** | 🚫 | 🆕 desktop-only | — |
| **Native package distribution** | (F-Droid/Play planned) | 🔴 TODO | apt, brew, winget, AUR, pkgsrc, pkg, ports |
| **Binary Size** | 30 MB APK / ~7.4 MB after R8 | ~10 MB (static, stripped) | ✅ |
| **Platforms** | Android only | Linux / macOS / Windows / FreeBSD / OpenBSD / NetBSD (amd64+arm64, 11 variants) | ✅ |
| **Runtime dependencies** | Java + Android SDK | None (statically linked) | ✅ |
| **Memory safety** | GC + JNI unsafe surface | Rust compile-time guarantees | ✅ |
| **Performance** | JVM overhead | Native, no GC | ✅ |

For the full mobile feature catalog see `../android/FEATURES_AUDIT.md` — the Tier-1/Tier-2 lists there are also the desktop's natural priority order.

---

## QR pairing (desktop → mobile) {#qr-pairing-desktop--mobile}

**Status:** Mobile side shipped 2026-04-28. Desktop is the **sender** — this checklist is the canonical TODO. Wire format is fixed (mobile won't change).

**Goal:** Add an existing TabSSH connection from desktop to a phone without retyping. Desktop renders an encrypted QR + 6-digit code; phone scans, enters the code, imports.

**Wire format (mobile is the spec, desktop must conform):**

`QrPayload`: CBOR `{ version: u8 = 1, salt: [u8;16], nonce: [u8;12], ciphertext: bytes }` → base64-encoded → rendered as QR in byte mode (ZXing's `ScanContract` returns `String`, so base64 is required for clean round-trip).

`ciphertext` is `AES-256-GCM(key, nonce, CBOR(PairingPayload))` where `key = Argon2id(password=6-digit-code, salt=salt, m=64MiB, t=3, p=1)`.

`PairingPayload`:
```
{
  version: u8 = 1,
  expires_at: u64,                  // unix seconds, ~60s after generation
  device_label: Option<String>,     // "Alice's Linux desktop"
  connections: [ConnectionProfile], // see §18.4 of ../android/AI.md
  groups: [Group],                  // optional, only those referenced
  identities: [Identity],           // optional, only those referenced
}
```

**No password, no private key.** Only public-key fingerprint + comment if the user wants to ride a key. Phone generates its own keypair locally if it wants matching auth.

**Capacity ceiling:** QR Code byte mode at ECC-L = 2,953 bytes. Cap v1 at 10 connections per QR (~2,800 bytes with RSA-4096 public keys).

**Threat model:** assume the QR is photographed by anyone with line-of-sight. The 6-digit code is the second factor; Argon2id m=64MiB t=3 makes brute-forcing 1M codes cost ~12 days/core, far longer than the 60s TTL.

**Rust desktop TODO:**
- [ ] Add deps: `qrcodegen`, `ciborium`, `argon2`, `aes-gcm`, `rand`, `base64` (note: BouncyCastle's Argon2id parameters m=64 MiB / t=3 / p=1 must match `argon2::Params::new(64*1024, 3, 1, ...)` exactly — bytes-vs-MiB semantics matter)
- [ ] `src/pairing/payload.rs` — `PairingPayload`, `ConnectionProfile` (subset, no secrets), serde encode
- [ ] `src/pairing/encrypt.rs` — generate code/salt/nonce; Argon2id; AES-GCM encrypt; serialise QrPayload
- [ ] `src/pairing/qr.rs` — render `QrPayload` (base64) → QR bitmap via `qrcodegen`
- [ ] `src/ui/pairing_dialog.rs` — egui state machine + QR display + countdown (`[Idle] → [Selecting] → [Generating] → [Active] → [Expired]`)
- [ ] Wire menu entry: File → Pair Phone…
- [ ] Tests: round-trip encrypt/decrypt with known test vectors, **commit them — mobile reuses the same vectors**
- [ ] After desktop ships: run mobile-side ImportFromQrActivity against our QRs to verify wire compatibility

**Why ZXing on mobile (not ML Kit):** ML Kit Barcode pulls in `com.google.android.gms:play-services-base` even bundled. TabSSH targets de-Googled ROMs → no Google Play Services dep allowed. Doesn't affect us on desktop, but constrains the Android side.

Reference: `../android/AI.md` §18.

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

### Git workflow & commit rules

These rules are imported from the android sibling project's `AI.md` §17 (rule 12) and runbook so both repos commit identically.

1. **Save the commit message to `{project_root}/.git/COMMIT_MESS` first.** Project convention: the maintainer can then `git commit -F .git/COMMIT_MESS` directly. Overwrite the file each time. Do not save to `/tmp/` paths. Do not paste the message inline only — the file is the source of truth.
2. **Use `gitcommit all`** to stage + commit + push. With `.git/COMMIT_MESS` present, that one command does the right thing. Do not use `-m` flags — the auto-message generator will override `-m` content. The proper flow is:
   ```bash
   # 1. write the message
   cat > .git/COMMIT_MESS <<'EOF'
   📝 short subject line with leading + trailing emoji 📝

   Optional body explaining the why.
   EOF
   # 2. commit + push
   gitcommit all
   ```
3. **Never add `Co-Authored-By` (or any attribution footer / "Generated with" line).** The maintainer authors every commit personally — there is no separate co-author. End the commit body at the last description line; no trailer.
4. **Use heredoc** for multi-line messages (avoids shell-escaping issues).
5. **Match the existing emoji style** when writing messages — recent commits use leading + trailing emoji like `📝 …text… 📝` or `🗃️ …text… 🗃️`. Pick what fits the change (📝 docs / 🗃️ refactor / 🔧 config / 🆕 feature / 🐛 fix).
6. **Don't bypass hooks** with `--no-verify` or signing flags. If a hook fails, fix the underlying issue.
7. **Don't amend pushed commits.** Always create a new commit. (See gitcommit's `fixup` subcommand for the safe equivalent on the most recent local commit.)

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
- ✅ **Terminal I/O Integration - COMPLETE!**
  - ✅ SSH channel → Terminal buffer (live data)
  - ✅ Terminal UI → SSH channel (keyboard input)
  - ✅ PTY resize handling
  - ✅ Keyboard event to escape sequences
  - ✅ Control keys (Ctrl+A-Z)
  - ✅ Function keys (F1-F12)
  - ✅ Arrow keys, Home, End, Page Up/Down
  - ✅ Session event polling
- ✅ **Terminal Rendering - COMPLETE!**
  - ✅ egui canvas rendering
  - ✅ Auto-sizing to available space
  - ✅ Status bar with connection info
  - ✅ Real-time updates (60 FPS)

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
- ✅ Authentication types (password, public key) - **FULLY WORKING!**
- ✅ Active session tracking
- ✅ Async runtime integration (Tokio)
- ✅ **SSH Connection Implementation - COMPLETE!**
  - ✅ Password authentication working
  - ✅ SSH key authentication working
  - ✅ Shell channel management
  - ✅ PTY allocation and resizing
  - ✅ Data send/receive
  - ✅ Connection lifecycle management
- ✅ **Active Session Management - COMPLETE!**
  - ✅ Background async session threads
  - ✅ Event-driven architecture (SessionEvent)
  - ✅ Command system (SessionCommand)
  - ✅ Channel I/O (read from SSH, write to SSH)
  - ✅ Graceful disconnection

### 🚧 **In Progress** (Phase 2) - Actually ~90% Done!

- ✅ SSH connection implementation (COMPLETE!)
- ✅ Terminal I/O (COMPLETE!)
- ✅ Terminal renderer (COMPLETE!)
- 🚧 Host key verification (basic implementation done)
- 🚧 Session persistence (database ready, integration pending)

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

### 📈 **Progress: ~50% Complete**

| Component | Progress | Status |
|-----------|----------|--------|
| Project Structure | 100% | ✅ Complete |
| UI Framework | 85% | ✅ Core complete, polish needed |
| Terminal Emulation | 90% | ✅ Full VT100/xterm + I/O working |
| SSH Core | 85% | ✅ Connect, auth, I/O complete |
| Storage | 80% | ✅ Schema done, usage needed |
| SFTP | 5% | ❌ Stub only |
| Platform Integration | 0% | ❌ Not started |
| Testing | 0% | ❌ No tests |

**Code compiles successfully!** ✅

The application has a **fully functional SSH client** with:
- Working password & SSH key authentication
- Live terminal I/O (read and write)
- Full keyboard input handling
- Terminal rendering with colors
- Connection management UI
- Multi-tab support
- Session state management
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

---

## Android App Feature Reference (synced 2026-04-28)

**Location:** `../android/` — see `../android/CLAUDE.md` and `../android/FEATURES_AUDIT.md`.

### Implementation Priority for Desktop

Aligned with `../android/FEATURES_AUDIT.md` — the Android-side audit categorises every feature as 🔥 Tier 1 (finish-the-half-done + quick wins), 🚀 Tier 2 (meaningful new capabilities), 🎯 Tier 3 (polish), 🧊 Tier 4 (speculative). The same priority ordering makes sense for desktop. The **comparison table above** is the authoritative status; this section just orders the work.

#### 🔥 Phase 3 — Tier 1 essentials (ship to reach SSH-client viability)
- Complete SSH (keyboard-interactive auth, agent forwarding) — `russh` already supports
- SFTP browser with the dual-pane layout mobile has — `russh-sftp`
- In-SFTP file editor (mobile Wave 1.7) — open file → edit → save back
- chmod editor in SFTP (Wave 1.8)
- SCP fallback (Wave 1.9)
- Port forwarding: -L / -R / -D (SOCKS) — `russh` channel API
- Background tunnels (Wave 3.3)
- ProxyJump cascading
- SSH config import — **read `~/.ssh/config` directly** via `ssh2-config` (desktop-native advantage)
- SSH config round-trip export (Wave 6.1)
- Bulk import: CSV / JSON / PuTTY (Waves 1, 6.4/6.5)
- Universal SSH key parser via `ssh-key` crate (all formats from mobile)
- In-app key generation (RSA, ECDSA, Ed25519)
- OpenSSH user certificate auth (Wave 2.2)
- Per-connection env vars (Wave 1.2)
- Find/search in scrollback (Wave 1)
- Reconnect button on disconnected tab

#### 🚀 Phase 4 — Tier 2 capabilities (where desktop shines)
- Workspaces (named tab groups, Wave 2.5) — natural fit for multi-window
- Command palette Ctrl+K, Quick switcher Ctrl+J, History palette Ctrl+R (Waves 2.6, 2.10)
- Split view (Wave 2.8) — multi-pane terminals, big desktop win
- Broadcast input (Wave 2.7) + Cluster commands with live streaming (Wave 4.e)
- GUI theme editor (Wave 2.4) — reuse mobile's JSON theme format
- 23 built-in themes (already enumerated in mobile's `BuiltInThemes.kt`)
- Per-host color tags (Wave 3.1)
- Snippets library with prompt-style variables `{?password}` etc.
- Identity abstraction (reusable credentials)
- Connection groups/folders
- Hypervisor management — Proxmox / XCP-ng / Xen Orchestra (REST + WebSocket) / VMware (Wave 7)
- VM serial console via hypervisor API (no VM network needed)
- Cloud host import — DigitalOcean / Hetzner / Linode / Vultr (Wave 5.1)
- 24-bit true-color rendering (Wave 4.a)
- Telnet protocol (Wave 2.3)

#### 🎯 Phase 5 — Tier 3 polish + platform integration
- Theme JSON import/export
- Encrypted ZIP backup/restore
- Cloud sync via filesystem watch + user's sync app (encrypted blobs interchangeable with mobile)
- Platform keychain integrations (`keyring` crate covers Linux/macOS/Windows in one API)
- System tray (`tray-icon`) + auto-launch on login
- CLI mode: `tabssh user@host` invocation from shell
- Native installers: `.deb`, `.rpm`, AUR `PKGBUILD`, AppImage, `.dmg`, Homebrew formula, MSI, WinGet manifest, Scoop, FreeBSD `pkg`/ports, OpenBSD packages, NetBSD `pkgsrc`
- PIN code app lock (Wave 3.2 mobile equivalent)
- Crash reporter via `panic::set_hook`
- Settings persistence (mostly stub'd today)
- Connection history view (Wave 3.5)
- What's-new / changelog screen on update (Wave 3.6)

#### 🧊 Phase 6 — Tier 4 / situational
- Foldable layout — N/A on desktop
- Voice typing — N/A on desktop
- Multi-language support (mirror mobile's en/es/fr/de strings)
- Accessibility audits (screen reader compatibility)
- Performance monitor with charts (mobile uses MPAndroidChart; Rust equivalent: `egui_plot`)

### What desktop should NOT bother porting

- Foreground service notification (use system tray)
- SAF document URIs (read files directly)
- On-screen keyboard customisation
- Volume key bindings, pinch-zoom (use Ctrl+Scroll)
- Swipe gestures (use Ctrl+Tab + mouse)
- ANR watchdog (use panic-handler crash reporting; UI-thread freeze detection is optional)
- Tasker integration (use shell scripts / .desktop actions)
- Android widget

---

## GitHub Actions CI/CD

**Reference:** `../android/.github/workflows/` (Complete CI/CD setup)

### Required Workflows

#### 1. CI Workflow (`.github/workflows/ci.yml`)

```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Build Docker image
        run: make docker
      
      - name: Compile check
        run: |
          docker run --rm -v $(pwd):/workspace tabssh-builder:latest \
            cargo check --all-targets
      
      - name: Run tests
        run: |
          docker run --rm -v $(pwd):/workspace tabssh-builder:latest \
            cargo test
      
      - name: Clippy
        run: |
          docker run --rm -v $(pwd):/workspace tabssh-builder:latest \
            cargo clippy -- -D warnings
      
      - name: Format check
        run: |
          docker run --rm -v $(pwd):/workspace tabssh-builder:latest \
            cargo fmt --check
      
      - name: Security audit
        run: |
          docker run --rm -v $(pwd):/workspace tabssh-builder:latest \
            cargo audit
```

#### 2. Release Workflow (`.github/workflows/release.yml`)

Based on Android release workflow, adapted for Rust:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Get version
        id: version
        run: |
          TAG_NAME=${GITHUB_REF#refs/tags/}
          VERSION=${TAG_NAME#v}
          COMMIT=$(git rev-parse --short=8 HEAD)
          YYMM=$(date "+%y%m")
          
          echo "TAG_NAME=$TAG_NAME" >> $GITHUB_OUTPUT
          echo "VERSION=$VERSION" >> $GITHUB_OUTPUT
          echo "COMMIT=$COMMIT" >> $GITHUB_OUTPUT
          echo "YYMM=$YYMM" >> $GITHUB_OUTPUT
      
      - name: Build Docker image
        run: make docker
      
      - name: Build all platforms
        run: |
          # Build Linux amd64
          docker run --rm -v $(pwd):/workspace tabssh-builder:latest \
            cargo build --release --target x86_64-unknown-linux-musl
          cp target/x86_64-unknown-linux-musl/release/tabssh \
            tabssh-linux-amd64-${{ steps.version.outputs.VERSION }}
          
          # Build Linux arm64 (requires cross)
          docker run --rm -v $(pwd):/workspace tabssh-builder:latest \
            cross build --release --target aarch64-unknown-linux-musl
          cp target/aarch64-unknown-linux-musl/release/tabssh \
            tabssh-linux-arm64-${{ steps.version.outputs.VERSION }}
      
      - name: Generate checksums
        run: |
          sha256sum tabssh-* > checksums-${{ steps.version.outputs.VERSION }}.txt
      
      - name: Create source archive
        run: |
          tar --exclude-vcs --exclude='./target' --exclude='./binaries' \
              --exclude='./releases' \
              -czf tabssh-${{ steps.version.outputs.VERSION }}-source.tar.gz .
      
      - name: Generate release notes
        run: |
          echo "# TabSSH Desktop ${{ steps.version.outputs.VERSION }}" > RELEASE.md
          echo "" >> RELEASE.md
          echo "🦀 Rust-based cross-platform SSH client" >> RELEASE.md
          echo "" >> RELEASE.md
          echo "## Downloads" >> RELEASE.md
          echo "- tabssh-linux-amd64 - Linux x86_64 (static musl)" >> RELEASE.md
          echo "- tabssh-linux-arm64 - Linux ARM64 (static musl)" >> RELEASE.md
          echo "- tabssh-${VERSION}-source.tar.gz - Source code" >> RELEASE.md
          echo "" >> RELEASE.md
          echo "## Checksums" >> RELEASE.md
          echo "\`\`\`" >> RELEASE.md
          cat checksums-${{ steps.version.outputs.VERSION }}.txt >> RELEASE.md
          echo "\`\`\`" >> RELEASE.md
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          name: "TabSSH Desktop ${{ steps.version.outputs.VERSION }}"
          files: |
            tabssh-linux-amd64-${{ steps.version.outputs.VERSION }}
            tabssh-linux-arm64-${{ steps.version.outputs.VERSION }}
            tabssh-${{ steps.version.outputs.VERSION }}-source.tar.gz
            checksums-${{ steps.version.outputs.VERSION }}.txt
          body_path: RELEASE.md
          draft: false
          prerelease: false
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

#### 3. Development Builds (`.github/workflows/development.yml`)

```yaml
name: Development Builds

on:
  push:
    branches: [ develop ]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Build Docker image
        run: make docker
      
      - name: Build debug
        run: make build
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: tabssh-dev-${{ github.sha }}
          path: binaries/*
```

### Docker Image Tags Strategy

Following Android app pattern, use 4 tags:
- `:latest` - Always current build
- `:{version}` - Semantic version (e.g., `:0.1.0`)
- `:{commit}` - Git commit (e.g., `:16cba3f1`)
- `:{YYMM}` - Year-month snapshot (e.g., `:2512`)

