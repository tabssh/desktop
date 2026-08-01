# TabSSH Desktop

Cross-platform SSH/SFTP/VNC client for developers and sysadmins. Browser-style tabbed sessions, integrated SFTP, port forwarding, hypervisor management, background host monitoring, and VNC — shipped as a single fully static Rust binary with no runtime dependencies. Desktop sibling to [TabSSH Android](https://github.com/tabssh/android), with byte-compatible sync and QR pairing.

[![CI](https://github.com/tabssh/desktop/actions/workflows/ci.yml/badge.svg)](https://github.com/tabssh/desktop/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/tabssh/desktop?label=release)](https://github.com/tabssh/desktop/releases)
[![License](https://img.shields.io/github/license/tabssh/desktop)](LICENSE.md)

> **Status (2026-08-01):** Early development. The GUI is the only runtime mode currently wired; core SSH sessions and terminal emulation are functional, while SFTP, hypervisor management, host-key verification, and several UI screens are still in progress. TUI and CLI connection modes are planned but not yet available. No stable release yet — install from source via Docker.

---

## 📦 Install

Download the latest release from [GitHub Releases](https://github.com/tabssh/desktop/releases/latest).

### Linux

| Arch | Binary |
|------|--------|
| x86\_64 | `tabssh-linux-x86_64` |
| aarch64 | `tabssh-linux-aarch64` |

```bash
ARCH=$(uname -m)
curl -LSsf "https://github.com/tabssh/desktop/releases/latest/download/tabssh-linux-${ARCH}" \
  -o /usr/local/bin/tabssh && chmod +x /usr/local/bin/tabssh
```

### macOS

| Arch | Binary |
|------|--------|
| Intel (x86\_64) | `tabssh-macos-x86_64` |
| Apple Silicon (aarch64) | `tabssh-macos-aarch64` |

```bash
ARCH=$(uname -m)
curl -LSsf "https://github.com/tabssh/desktop/releases/latest/download/tabssh-macos-${ARCH}" \
  -o /usr/local/bin/tabssh && chmod +x /usr/local/bin/tabssh
xattr -d com.apple.quarantine /usr/local/bin/tabssh 2>/dev/null || true
```

### Windows

| Arch | Binary |
|------|--------|
| x86\_64 | `tabssh-windows-x86_64.exe` |
| aarch64 | `tabssh-windows-aarch64.exe` |

Download the `.exe` for your architecture and add it to `%PATH%`.

### FreeBSD

| Arch | Binary |
|------|--------|
| x86\_64 | `tabssh-freebsd-x86_64` |
| aarch64 | `tabssh-freebsd-aarch64` |

```bash
ARCH=$(uname -m)
fetch -o /usr/local/bin/tabssh \
  "https://github.com/tabssh/desktop/releases/latest/download/tabssh-freebsd-${ARCH}"
chmod +x /usr/local/bin/tabssh
```

### OpenBSD

| Arch | Binary |
|------|--------|
| x86\_64 | `tabssh-openbsd-x86_64` |
| aarch64 | `tabssh-openbsd-aarch64` |

```bash
ARCH=$(uname -m)
ftp -o /usr/local/bin/tabssh \
  "https://github.com/tabssh/desktop/releases/latest/download/tabssh-openbsd-${ARCH}"
chmod +x /usr/local/bin/tabssh
```

### NetBSD

| Arch | Binary |
|------|--------|
| x86\_64 | `tabssh-netbsd-x86_64` |

```bash
curl -LSsf "https://github.com/tabssh/desktop/releases/latest/download/tabssh-netbsd-x86_64" \
  -o /usr/local/bin/tabssh && chmod +x /usr/local/bin/tabssh
```

---

## 🐳 Docker

Run the GUI with X11 forwarding:

```bash
# Clone and start
git clone https://github.com/tabssh/desktop.git
cd desktop

# X11 (Linux)
xhost +local:docker
docker compose -f docker/docker-compose.yml up gui

# Wayland
WAYLAND_DISPLAY=$WAYLAND_DISPLAY docker compose -f docker/docker-compose.yml up gui
```

Build and run the production image locally:

```bash
docker build -f docker/Dockerfile -t tabssh:latest .
docker run --rm \
  -e DISPLAY=$DISPLAY \
  -v /tmp/.X11-unix:/tmp/.X11-unix:ro \
  tabssh:latest
```

Development mode (live source mount, debug binary via `cargo run`):

```bash
docker compose -f docker/docker-compose.dev.yml up dev
```

Run tests:

```bash
docker compose -f docker/docker-compose.test.yml up test
```

---

## 🖥️ CLI

TabSSH exposes the standard universal flags today; direct-connect CLI/TUI
modes are planned (see the status note above) and not yet wired.

Available now:

```bash
# Show help / version
tabssh --help
tabssh --version

# Launch the GUI explicitly (default when a local display is present)
tabssh --ui gui

# Enable debug logging / control color output
tabssh --debug --color no
```

Planned (not yet available — `--ui tui|cli` currently reports that the mode
is unimplemented):

```bash
# Connect by user@host
tabssh user@host

# Connect by saved profile name
tabssh --connect "Production DB"

# SSH options
tabssh -p 2222 -i ~/.ssh/id_ed25519 user@host
```

---

## 🛠️ Development

**All Rust/Cargo commands run inside Docker — never on the host.**

### Prerequisites

- Docker

### Build

```bash
# Debug build (x86_64 musl)
make build

# Release builds (x86_64 + aarch64 musl)
make release

# Lint (fmt + clippy)
make check

# Tests
make test

# Build runtime Docker image
make docker

# Run GUI with X11 forwarding
make run-gui
```

### Make targets

| Target | Description |
|--------|-------------|
| `build` | Debug build for `x86_64-unknown-linux-musl` → `binaries/tabssh-linux-x86_64` |
| `release` | Release builds for both musl targets |
| `check` | `cargo fmt --check` + `cargo clippy -- -D warnings` |
| `test` | `cargo test --workspace --all-features` |
| `docker` | Build the production runtime image |
| `run-gui` | Run the GUI inside Docker with X11 forwarding |
| `clean` | Remove `binaries/` and `target/` |

### 🐳 Docker build

All compilation uses `casjaysdev/rust:latest` — no local Rust installation required:

```bash
# Build multi-arch production image
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -f docker/Dockerfile \
  -t tabssh:latest \
  --push .
```

### Project layout

```
src/
├── main.rs          # CLI entry point, --ui gui flag
├── lib.rs           # public API surface
├── app.rs           # eframe App implementation
├── assets.rs        # compile-time asset embedding (themes)
├── ssh/             # SSH session management (russh)
├── sftp/            # SFTP browser and transfers
├── terminal/        # VT100/xterm emulator
├── ui/              # egui panels and widgets
├── storage/         # SQLite via rusqlite
├── crypto/          # AES-GCM, PBKDF2, Argon2id, keychain
├── config/          # app config, theme structs
└── platform/        # OS-specific integrations
```

---

## 📄 License

MIT — see [LICENSE.md](LICENSE.md)
