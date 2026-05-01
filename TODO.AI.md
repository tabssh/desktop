# TabSSH Desktop — Work List

**Last verified:** 2026-05-01
**Build status:** ❌ Does not compile (50 errors via `cargo check` in `tabssh-builder` docker image)
**Feature parity vs android:** see the "Comparison with Android Version" matrix in the project tracker — most rows are 🔴 TODO

This file is the live work list. It supersedes the December 2025 `STATUS.md` / `PROGRESS_REPORT.md` / `COMPILATION_STATUS.md` snapshots, which falsely claimed 75–100% completion when the project did not compile.

---

## Phase 0 — get the build green (blocker)

50 errors as of 2026-05-01. Distribution:

| File | Errors | Cause |
|------|-------:|-------|
| `src/sftp/client.rs` | 42 | russh-sftp 2.0 API drift — `From<&Cow<'_, str>>` not implemented for `String`, `setstat` removed from `SftpSession`, method-arity mismatches on `read`/`write`/`stat` |
| `src/ssh/forwarding.rs` | 5 | (audit pending) |
| `src/sftp/browser.rs` | 3 | `FileType` doesn't impl `Copy` — moves out of shared refs |
| `src/terminal/emulator.rs` | 3 | (warnings: unused vars; check for real errors too) |
| `src/ssh/connection.rs` | 2 | (audit pending) |
| `src/ssh/config_parser.rs` | 2 | (audit pending) |
| `src/sftp/transfer.rs` | 2 | (audit pending) |
| `src/sftp/operations.rs` | 2 | (audit pending) |
| `src/ui/screens/settings_screen.rs` | 2 | (audit pending) |
| `src/ssh/active_session.rs` | 1 | (audit pending) |
| `src/ui/keyboard.rs` | 1 | (audit pending) |
| `src/ui/components.rs` | 1 | (audit pending) |
| `src/ui/screens/sftp_browser_ui.rs` | 1 | (audit pending) |

**Fix order:**
1. SFTP client (42 errors) — read russh-sftp 2.0 docs for `SftpSession`. Likely needs `.read_dir()`/`.open()`/`.create()` returning futures of `russh_sftp::client::fs::File`, with `Cow<str>` → `String` via `.into_owned()` not `From`.
2. `FileType` → derive `Copy` + `Clone` (it's an enum of unit variants, no reason it isn't already).
3. SSH forwarding/connection/config_parser — likely API drift in russh 0.40 channel/handler API.
4. Misc UI errors — typically egui 0.25 method renames.

Acceptance: `make build` (which calls `cargo build --release` inside the docker image) exits 0.

---

## Phase 1 — Tier-1 SSH-client viability

Target: parity on day-to-day SSH features so a power user could daily-drive desktop. Order is dependency-aware.

**SSH core:**
- [ ] Keyboard-interactive auth (russh has it; just wire the prompts → UI dialog)
- [ ] SSH agent forwarding (russh `agent` feature)
- [ ] Universal key parser via `ssh-key` crate — OpenSSH v1, PEM (PKCS#1/8), PuTTY v2/v3
- [ ] In-app key generation: RSA / ECDSA P-256/384/521 / Ed25519 / DSA
- [ ] OpenSSH user certificate auth (`*-cert.pub`) — `ssh-key` already supports
- [ ] Per-connection env vars (DB column `env_vars`, sent via `russh::Channel::env`)
- [ ] Always-on keepalive (60s, count-max 3) — apply unconditionally to every russh session per Issue #166

**SFTP browser:**
- [ ] Get the dual-pane browser working with the fixed `russh-sftp` calls
- [ ] Remote file editor inline (open → edit in egui textarea → save back)
- [ ] chmod editor dialog
- [ ] SCP fallback for servers without SFTP subsystem

**Port forwarding:**
- [ ] -L (local) — likely already partly done; verify against russh API
- [ ] -R (remote)
- [ ] -D (dynamic / SOCKS5)
- [ ] Background tunnels (run forwards without an attached terminal session)
- [ ] ProxyJump cascading

**SSH config:**
- [ ] Read `~/.ssh/config` directly via `ssh2-config` — don't import a copy, the desktop should reflect the user's existing setup
- [ ] Round-trip export (write back valid `Host …` blocks)
- [ ] Bulk import: CSV / JSON / PuTTY .ppk lists

**Multi-tab + state:**
- [ ] Multi-tab same-host independent shells (Issue #163) — per-tab `russh::Channel`, sibling tabs survive when one shell exits
- [ ] Tmux/Screen/Zellij auto-launch + `postConnectScript` (Issue #170)
- [ ] Reconnect button on disconnected tab
- [ ] Active Sessions strip (Issue #165) — top of window, dynamic title via OSC 0/2

**Terminal:**
- [ ] Find/search in scrollback (Wave 1)
- [ ] 24-bit true color (`alacritty_terminal` supports — verify rendering)
- [ ] Hardware-keyboard modifier-aware nav keys + AltGr (Issue #171) — xterm-style `\e[1;<mod><letter>`
- [ ] Centralised error dialogs with Copy button (Issue #167) — clipboard via `arboard`

**Build/version:**
- [ ] Cold-start commit-id marker (Issue #164) — `build.rs` resolves the commit, log on startup once per change

---

## Phase 2 — Tier-2 desktop-shines features

Where desktop ergonomics genuinely beat mobile.

- [ ] Workspaces (named tab groups, DB v21 equivalent) — desktop can give each its own window
- [ ] Command palette (Ctrl+K), Quick switcher (Ctrl+J), History palette (Ctrl+R)
- [ ] Split view (multi-pane terminals in one window)
- [ ] Broadcast input + cluster commands with live streaming
- [ ] GUI theme editor — reuse mobile's JSON theme format (kotlinx.serialization is JSON; serde works)
- [ ] All 23 built-in themes (port `BuiltInThemes.kt` enum table)
- [ ] Per-host color tags (DB v22 equivalent)
- [ ] Snippets library + prompt-style variables `{?password}`
- [ ] Identity abstraction (reusable credentials)
- [ ] Connection groups/folders (DB v3 equivalent — hierarchical)
- [ ] Hypervisor management — Proxmox VE / XCP-ng / Xen Orchestra (REST + WS) / VMware ESXi/vCenter; `reqwest` + `tokio-tungstenite`
- [ ] VM serial console via hypervisor API (no VM network needed)
- [ ] Cloud host import — DigitalOcean / Hetzner / Linode / Vultr (opt-in, tokens in `keyring` not DB)
- [ ] Telnet protocol (RFC 854)
- [ ] Recordable macros (Issue #173, DB v26) — capture raw byte sequences
- [ ] **QR pairing — desktop side** (mobile shipped 2026-04-28). See "QR pairing" section in the project tracker

---

## Phase 3 — Tier-3 polish + platform integration

- [ ] Theme JSON import/export
- [ ] Encrypted ZIP backup/restore (compatible with android's BackupManager format)
- [ ] **Cloud sync — filesystem-watch model** (not SAF):
  - On-disk format byte-identical to mobile's `TABSSH_SYNC_V2` (32-byte header + 32-byte salt + 12-byte IV + GZIP'd JSON, AES-GCM, PBKDF2-HMAC-SHA256 100k iter, 256-bit key)
  - User points TabSSH at a folder; their existing sync app (Nextcloud / syncthing / rclone / OneDrive / Dropbox) handles transport
  - Three-way merge with conflict UI (port `MergeEngine` + `ConflictResolver`)
  - `notify` crate watches the folder
- [ ] Platform keychain via `keyring` crate (single API across Linux Secret Service / macOS Keychain / Windows Credential Manager)
- [ ] System tray (`tray-icon` crate) + auto-launch on login
- [ ] CLI mode: `tabssh user@host` invocation from shell, falls through to existing GUI tab
- [ ] Native installers: `.deb`, `.rpm`, AUR `PKGBUILD`, AppImage, `.dmg`, Homebrew formula, `.msi`, WinGet manifest, Scoop bucket, FreeBSD `pkg`/ports, OpenBSD packages, NetBSD `pkgsrc`
- [ ] PIN code app lock (Wave 3.2 equivalent)
- [ ] Crash reporter via `panic::set_hook` writing to `~/.local/share/tabssh/crashes/`
- [ ] Settings persistence (most current settings UI is stub)
- [ ] Connection history view (separate from "last connected")
- [ ] What's-new / changelog screen on update (read from `release.txt`)

---

## Phase 4 — situational / lower priority

- [ ] X11 forwarding (`x11rb` crate, real X11 servers — much more useful than mobile's stub)
- [ ] Mosh — pure-Rust client. Avoid the mobile cross-compile dance for `libmosh-client.so`
- [ ] Multi-language support (mirror mobile's en/es/fr/de)
- [ ] Accessibility audits (screen-reader compat — Linux Orca, macOS VoiceOver, Windows Narrator)
- [ ] Performance monitor with charts (`egui_plot`)
- [ ] HTTP/SOCKS proxy configuration UI

---

## Out of scope (where android leads but desktop shouldn't follow)

- Foreground service notification → use system tray
- SAF document URIs → desktop reads files directly
- On-screen keyboard customisation → real keyboard always available
- Volume key bindings, pinch-zoom → use Ctrl+Scroll
- Swipe gestures → use Ctrl+Tab + mouse
- ANR watchdog (android-specific) → `panic::set_hook` covers crashes; UI-thread freeze detection is optional polish
- Tasker integration → use shell scripts / `.desktop` actions
- Android widget → N/A
- Foldable layout / sw720dp / book-mode → desktop windows are resizable anyway
- FLAG_SECURE screenshot protection → desktop OSes don't expose an equivalent universally
- Cross-platform desktop app (sic) → that IS this project

---

## Acceptance for "in line with android"

Considered done when, for every row in the project tracker's "Comparison with Android Version" matrix:
- ✅ rows show ✅ on the desktop column too, OR
- 🔴 rows have a tracked TODO above with file paths and crate choices, OR
- 🚫 rows are explicitly listed in "Out of scope" above with reason

The QR pairing wire format must be byte-compatible with mobile (test vectors checked into both repos).

The cloud sync `TABSSH_SYNC_V2` format must be byte-compatible with mobile (an encrypted blob written by desktop must round-trip through mobile and vice versa).
