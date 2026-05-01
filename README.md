# TabSSH Desktop

🦀 **Cross-platform SSH/SFTP client built in Rust** — Linux / macOS / Windows / FreeBSD / OpenBSD / NetBSD on amd64 + arm64.

[![License](https://img.shields.io/badge/license-MIT-blue)]()
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)]()
[![Status](https://img.shields.io/badge/status-early%20development-yellow)]()

**Desktop sibling to [TabSSH Android](../android/).** Goal: feature parity with the mobile app where applicable, plus desktop-native conveniences (real `~/.ssh/` access, system tray, native window management, CLI mode, native package distribution).

> **Honest status (2026-05-01):** Early development. Does not yet compile cleanly. ~50% feature parity vs the Android app. The "100% complete" claims in older revisions of this file and in `STATUS.md` / `PROGRESS_REPORT.md` / `COMPILATION_STATUS.md` (Dec 2025) were aspirational and have been removed. See [TODO.AI.md](TODO.AI.md) for the live work list and the project tracker file in this directory for the parity matrix.

---

## 🎯 Goal

Cross-platform desktop SSH/SFTP client with:

- 🦀 **Pure Rust** — memory-safe, fast, concurrent
- 📦 **Static binaries** — no runtime dependencies (musl on Linux, static MSVC on Windows)
- 🎨 **Native UI** — egui (lightweight, GPU-accelerated)
- 🔐 **Security first** — Rust's memory safety + OS keychain integration + host-key TOFU + AES-GCM at rest
- 🌍 **True cross-platform** — Linux / macOS / Windows / FreeBSD / OpenBSD / NetBSD on amd64 + arm64 (11 binary variants)
- 🔄 **Mobile interop** — encrypted sync blobs and QR pairing payloads byte-compatible with the Android app

---

## 📊 Current state

| Aspect | Mobile (android) | Desktop (this repo) |
|---|---|---|
| Build | ✅ Compiles, ~215 Kotlin files / ~65k LOC | ❌ 50 compile errors verified 2026-05-01 |
| Database schema | Room v26, 25 forward migrations | rusqlite v1, no migrations yet |
| Lines of Rust/Kotlin | n/a | ~10k Rust |
| Feature parity | n/a (it's the reference) | ~50% with several large features stubbed |
| Tests | active suite | 0 working test files |
| Distribution | F-Droid + GitHub Releases (planned) | None yet |

**Honest progress per component:**

| Component | Progress | Notes |
|-----------|----------|-------|
| Project structure | 100% | Modules / Cargo / Docker / Makefile in place |
| UI framework (egui) | ~70% | Tab manager + connection list + terminal view; many screens stubbed |
| Terminal emulation | ~80% | VT100/xterm via `vte`; renderer in egui canvas |
| SSH core | ~60% | russh-based; password + pubkey work; keyboard-interactive + agent forwarding pending |
| SFTP | ~20% | Code exists but doesn't compile against russh-sftp 2.1.x — top blocker |
| Port forwarding | ~50% | -L / -R / -D scaffolded; Handle::clone + stream/channel ownership issues |
| Storage | ~40% | SQLite schema exists; not used by most code paths |
| Crypto / keychain | ~10% | Stub; `keyring` integration pending |
| Platform integration | ~5% | macos.rs/linux.rs/windows.rs/bsd.rs all stubs |
| Themes | ~10% | One default theme; 23 mobile themes not yet ported |
| Tests | 0% | No working tests today |
| Distribution / packaging | 0% | No installers, no published binaries |

---

## 🐳 Build prerequisites

**Rust is NOT installed locally — all builds happen inside Docker.**

```bash
# One-time: build the Docker image with the Rust toolchain + GUI deps
docker build -t tabssh-builder -f docker/Dockerfile .

# Build for the host
make build       # → ./binaries/tabssh

# Release build (multi-target)
make release     # → ./releases/tabssh-{os}-{arch}

# Tests (when the build is green)
make test
```

The Docker image bundles `rustlang/rust:nightly-bookworm` + GUI dev libraries (libxcb, libxkbcommon, libgtk-3, libgl, etc.) + the `x86_64-unknown-linux-musl` target for static Linux binaries.

---

## ⌨️ Keyboard shortcuts (planned)

| Shortcut | Action |
|----------|--------|
| `Ctrl+T` | New tab |
| `Ctrl+W` | Close tab |
| `Ctrl+Tab` / `Ctrl+Shift+Tab` | Next / previous tab |
| `Ctrl+1`-`9` | Switch to tab N |
| `Ctrl+K` | Command palette (Phase 2.1) |
| `Ctrl+J` | Quick switcher (Phase 2.1) |
| `Ctrl+R` | History palette (Phase 2.1) |
| `Ctrl+F` | Find in scrollback (Phase 1.7) |
| `Ctrl+Click` | Open URL in browser (replaces mobile long-press) |
| `Ctrl+Scroll` | Font zoom (replaces mobile pinch / volume keys) |
| `Ctrl+N` | New connection |
| `Ctrl+,` | Settings |

---

## 🏗️ Architecture

| Layer | Crate / module |
|---|---|
| Language | Rust 2021 edition (1.75+) |
| UI | `egui` + `eframe` |
| SSH | `russh` 0.40 + `russh-keys` + `russh-sftp` 2.1 |
| Terminal emulator | `vte` (escape parsing) + custom buffer + egui-canvas renderer |
| Async runtime | `tokio` (full features) |
| Storage | `rusqlite` (bundled SQLite) + `serde` + `toml` |
| Crypto | `aes-gcm`, `argon2`, `pbkdf2`, `ring`, `ed25519-dalek`, `rsa` |
| Keychain | `keyring` (cross-platform), `security-framework` (macOS), `windows` crate (Windows) |
| Clipboard | `arboard` (planned) |
| QR pairing | `qrcodegen`, `ciborium` (planned, Phase 2.8) |
| Hypervisor APIs | `reqwest` + `tokio-tungstenite` (planned, Phase 2.5) |

---

## 🛣️ Roadmap

In dependency order — see [TODO.AI.md](TODO.AI.md) for the full task list and the project tracker for architectural detail.

- **Phase 0** — fix the 50 compile errors (russh-sftp 2.1 API alignment is the bulk)
- **Phase 1** — Tier-1 SSH-client viability (keyboard-interactive auth, agent forwarding, working SFTP browser, port forwarding, `~/.ssh/config` direct read, universal key parser, in-app key generation, OpenSSH user certificates, multi-tab same-host, find-in-scrollback, 24-bit color)
- **Phase 2** — Tier-2 desktop-shines features (workspaces, command palette, split view, broadcast input, GUI theme editor, 23 built-in themes, snippets w/ prompt-style variables, hypervisor management, cloud host import, Telnet, recordable macros, **QR pairing — desktop side**, cloud sync byte-compat with mobile)
- **Phase 3** — Tier-3 polish + platform integration (encrypted ZIP backup, OS keychain integration, system tray, CLI mode, native installers for every supported OS, PIN lock, crash reporter)
- **Phase 4** — situational (X11 forwarding, pure-Rust Mosh, multi-language, accessibility audits, performance monitor, FIDO2)
- **Phase 5** — research / speculative (post-quantum, multi-host dashboard, fuzzing)

Out-of-scope items (mobile-only mechanics that don't map to desktop) are listed at the bottom of [TODO.AI.md](TODO.AI.md).

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). High-leverage starting points right now:

- **Phase 0 build fixes** — every contributor starts compiling. The 42-error russh-sftp 2.1 alignment in `src/sftp/client.rs` is the blast-radius reduction job.
- **Universal SSH key parser** — wrap `ssh-key` crate; mobile has the spec.
- **23 built-in themes** — copy the JSON definitions from mobile's `BuiltInThemes.kt`.
- **QR pairing desktop sender** — wire format is fixed (mobile shipped 2026-04-28); spec is in the project tracker.

---

## 📝 License

MIT — see [LICENSE.md](LICENSE.md). Same license as the Android app.

---

## 🔗 Links

- **Repository:** https://github.com/tabssh/desktop
- **Issues:** https://github.com/tabssh/desktop/issues
- **Android sibling:** [../android/](../android/) — the reference implementation
- **Audit / parity matrix:** `../android/FEATURES_AUDIT.md`
- **Mobile architecture spec:** `../android/AI.md`

---

## 🙏 Acknowledgments

- [russh](https://github.com/warp-tech/russh) — pure-Rust SSH2
- [russh-sftp](https://crates.io/crates/russh-sftp) — pure-Rust SFTP
- [egui](https://github.com/emilk/egui) — immediate-mode GUI
- [tokio](https://tokio.rs/) — async runtime
- TabSSH Android — original inspiration

---

**Built with 🦀 Rust**
