# TabSSH Desktop

> **Modern, cross-platform SSH/SFTP client built with Rust and egui**

[![Rust](https://img.shields.io/badge/rust-1.84%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Status](https://img.shields.io/badge/status-alpha-yellow.svg)](https://github.com/tabssh/desktop)

**TabSSH Desktop** is a native, high-performance SSH client designed for developers and system administrators who need powerful terminal management with a modern, intuitive interface.

## ✨ Features

### Current (v0.1.0 - Alpha)

- 🦀 **Pure Rust** - Memory-safe, fast, concurrent
- 🎨 **Native UI** - Built with egui for responsive, GPU-accelerated rendering
- 📑 **Browser-Style Tabs** - Manage multiple SSH sessions in one window
- ⌨️ **Keyboard Shortcuts** - Ctrl+T, Ctrl+W, Ctrl+Tab, and more
- 💾 **SQLite Database** - Persistent connection profiles and settings
- 🎯 **Quick Connect** - Fast SSH connection dialog
- 🔐 **Multiple Auth Methods** - Password and SSH key authentication

### Coming Soon

- 🔒 **SFTP Browser** - Integrated file transfer with drag-and-drop
- 🚀 **Port Forwarding** - Local, remote, and dynamic (SOCKS) tunneling
- 🎨 **10+ Themes** - Dracula, Solarized, Nord, and more
- 🔑 **Keychain Integration** - Secure credential storage
- 📁 **SSH Config Import** - Load connections from ~/.ssh/config
- 🌍 **Cross-Platform** - Linux, macOS, Windows, BSD

## 🚀 Quick Start

### Prerequisites

- **Docker** (for building)
- **X11 or Wayland** (for running GUI on Linux)

### Build & Run

```bash
# Clone repository
git clone https://github.com/tabssh/desktop.git
cd desktop

# Build with Docker (recommended)
make build

# Run
./binaries/tabssh

# Or build and run with cargo (requires Rust toolchain)
cargo run --release
```

### Using Make Targets

```bash
make build           # Build binaries with Docker → ./binaries/
make release         # Release build with archive → ./releases/
make test            # Run tests in Docker
make docker          # Build Docker image (buildx: amd64, arm64)
make clean           # Clean build artifacts
make help            # Show available targets
```

## 📋 Development Status

**Current Phase:** Phase 1-2 (Foundation & Core Features)  
**Progress:** ~35% complete  
**Version:** 0.1.0 (Alpha)

| Component | Status |
|-----------|--------|
| UI Framework | ✅ 70% (core done) |
| Terminal Emulation | 🚧 60% (buffer done, I/O needed) |
| SSH Core | 🚧 30% (framework done, connect needed) |
| SFTP | ❌ 5% (stub only) |
| Testing | ❌ 0% (no tests) |

See [CLAUDE.md](CLAUDE.md) for detailed roadmap and specifications.

## 🏗️ Architecture

```
TabSSH Desktop
├── UI Layer (egui)          → Browser-style tabs, connection manager
├── Terminal Emulation       → VTE parser, scrollback buffer
├── SSH Core (russh)         → Async connections, authentication
├── Storage (SQLite)         → Connection profiles, settings
├── Platform Integration     → Keychain, credentials
└── SFTP Client              → File browser, transfers
```

**Tech Stack:**
- **Rust 1.84+** (2021 edition)
- **egui 0.28** - Pure Rust immediate-mode GUI
- **russh 0.45** - Pure Rust SSH2 implementation
- **tokio** - Async runtime
- **rusqlite** - Embedded SQLite database
- **vte** - Terminal emulator parser

## 🎯 Roadmap

### Phase 1: Foundation ✅ (Complete)
- [x] Project structure and Docker build
- [x] egui UI with tabs
- [x] Terminal buffer and ANSI parser
- [x] SQLite database schema
- [x] SSH framework structure

### Phase 2: Core Features 🚧 (In Progress)
- [x] Connection manager UI
- [x] Terminal view rendering
- [ ] **Active SSH connections** ← *Current focus*
- [ ] Terminal I/O integration
- [ ] Host key verification
- [ ] Session persistence

### Phase 3: Advanced SSH (Next)
- [ ] SFTP browser
- [ ] Port forwarding
- [ ] SSH agent integration
- [ ] SSH config parser

### Phase 4-6: Polish, Platform, Testing
- [ ] Theme system
- [ ] Keychain integration
- [ ] Cross-platform builds
- [ ] Test suite
- [ ] Performance optimization

## 🔧 Building from Source

### With Docker (Recommended)

```bash
# Build Docker image
docker build -t tabssh-builder -f docker/Dockerfile .

# Build binary
docker run --rm -v $(pwd):/workspace tabssh-builder cargo build --release

# Run with GUI support
docker run --rm \
  -v $(pwd):/workspace \
  -e DISPLAY=$DISPLAY \
  -v /tmp/.X11-unix:/tmp/.X11-unix \
  tabssh-builder \
  ./target/release/tabssh
```

### With Local Rust Toolchain

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies (Debian/Ubuntu)
sudo apt-get install -y \
  pkg-config libssl-dev \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libfontconfig1-dev

# Build
cargo build --release

# Run
./target/release/tabssh
```

## 📦 Binary Naming Convention

Format: `tabssh-{os}-{arch}`

| OS | Architecture | Binary Name |
|----|--------------|-------------|
| Linux | x86_64 | `tabssh-linux-amd64` |
| Linux | aarch64 | `tabssh-linux-arm64` |
| macOS | x86_64 | `tabssh-macos-amd64` |
| macOS | aarch64 | `tabssh-macos-arm64` |
| Windows | x86_64 | `tabssh-windows-amd64.exe` |

## 🤝 Contributing

Contributions welcome! Please see our [contributing guidelines](CLAUDE.md#contributing-guidelines).

**Development Process:**
1. Fork repository
2. Create feature branch
3. Write code + tests
4. Run `cargo fmt` and `cargo clippy`
5. Submit PR

**Code Style:**
- Follow Rust standard style (`rustfmt`)
- Use `clippy` for linting
- Document public APIs
- Write tests for new features

## 📄 License

MIT License - See [LICENSE.md](LICENSE.md) for details.

## 🔗 Links

- **Documentation:** [CLAUDE.md](CLAUDE.md) - Complete specification and roadmap
- **Repository:** https://github.com/tabssh/desktop
- **Issues:** https://github.com/tabssh/desktop/issues

## 🙏 Acknowledgments

Built with these amazing Rust projects:
- [egui](https://github.com/emilk/egui) - Immediate mode GUI
- [russh](https://github.com/warp-tech/russh) - Pure Rust SSH
- [tokio](https://tokio.rs) - Async runtime
- [alacritty](https://github.com/alacritty/alacritty) - Terminal emulation inspiration

---

**Status:** 🚧 Alpha - Under active development  
**Maintained by:** TabSSH Contributors  
**Last Updated:** 2025-12-19
