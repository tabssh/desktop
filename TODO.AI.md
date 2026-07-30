# TabSSH Desktop — Work List

**Last verified:** 2026-07-28
**Build status:** ✅ Compiles and passes `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace --all-features` (555 tests, 0 failed) inside `casjaysdev/rust:latest` with `-e RUSTFLAGS=""`. `cargo tarpaulin --engine llvm --workspace --all-features --fail-under 60` passes at 70.33% coverage. Phase 0 is cleared.
**Toolchain infra note (2026-07-24):** a bootstrap pass tried `cargo build --release --target x86_64-unknown-linux-musl` inside `casjaysdev/rust:latest` (the mandated image) and hit an environment-level blocker before code errors could even be re-verified — the image's default entrypoint exits 1 during its SMTP-configuration step, never reaching `cargo`; bypassing the entrypoint (`--entrypoint /bin/sh`) skips whatever setup normally installs the musl toolchain, so `x86_64-linux-musl-gcc` is missing and the build fails at the linker stage instead. This is a defect in the `casjaysdev/rust:latest` image/entrypoint (pulled 2026-06-25 build), not in TabSSH's code — Phase 0's ~50 code errors above are still believed current but could not be freshly confirmed this session.
**Toolchain infra note (2026-07-27, `cargo test`):** the host triple in `casjaysdev/rust:latest` is `x86_64-unknown-linux-musl`, and `.cargo/config.toml`'s `rustflags = ["-C", "target-feature=+crt-static"]` for that target breaks compiling `async-recursion` (pulled in transitively via `zbus`/`arboard`/`keyring` for clipboard/secret-service support) with "cannot produce proc-macro ... target does not support these crate types". Fix: pass `-e RUSTFLAGS=""` on the `docker run` invocation to override the target-level rustflags for the test run — confirmed working (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace --all-features` all exit 0 with this override, 352 tests passing). `make test` can be verified green in this Docker image with this override; do not rely on the default `RUSTFLAGS` from `.cargo/config.toml` for host-native `cargo test`/`cargo clippy` runs.
**Feature parity vs android:** ~50%. Mobile is at v0.0.9 / DB v29 / ~215 Kotlin files / ~65k LOC, with OCI hypervisor support shipped. Desktop is ~59 Rust files / ~10k LOC.

**Recent mobile changes desktop must absorb** (added to phases below):
- OCI (Oracle Cloud Infrastructure) Compute as a 4th hypervisor target — Phase 2.5
- Reattach prompt for already-live tab sessions — Phase 1.6
- Drag-to-select range copy in the terminal — Phase 1.7
- Group sort + persistence (default alphabetical) — Phase 2.2
- `Ctrl+Space` / `Ctrl+@` / `Ctrl+[` etc chord routing — Phase 1.7
- Re-apply window prefs on focus event — Phase 3.5

This file is the live work list and is the source of truth for ordering. The companion project tracker holds architectural decisions, the parity matrix, the QR-pairing wire format, the dependency stack, and the build runbook. The two are kept in sync.

The phases below are dependency-ordered: `Phase 0 → Phase 1` is hard (you cannot ship features that don't compile); `Phase 1 → Phase 2 → …` is soft (later phases gain a lot from earlier ones being available, but partial overlap is fine when crates and modules are independent).

---

## Known issues (non-blocking)

- `cargo deny check bans` reports a `multiple-versions` warning for `winnow`
  (0.5.40 vs 0.7.15 pulled in via different dependency chains). Not a hard
  CI failure (`bans.multiple-versions = "warn"` in `deny.toml`), but should
  be resolved by bumping whichever direct dependency pins the older
  `winnow` range once one is available.
- `src/storage/sessions.rs` (`SavedSession::save`/`load_all`/`delete`) reads
  and writes a `saved_sessions` SQLite table, but `Database::initialize()`
  in `src/storage/database.rs` never creates that table (it only creates
  `connections`, `ssh_keys`, `known_hosts`, `themes`, `settings`). Every
  `SavedSession` method currently fails with "no such table: saved_sessions"
  against any database, including a freshly opened one. Found 2026-07-27
  while adding unit test coverage for `sessions.rs` (tests document the
  current failing behavior rather than asserting a nonexistent happy path).
  Fix: add a `CREATE TABLE IF NOT EXISTS saved_sessions (...)` migration to
  `Database::initialize()` matching the columns `SavedSession::save` writes
  (`id`, `connection_id`, `host`, `user`, `port`, `scrollback`, `cursor_row`,
  `cursor_col`, `created_at`).

---

## Phase 0 — get the build green (blocker)

50 compile errors as of 2026-05-01. Distribution and remediation:

| File | Errors | Cause | Fix |
|------|-------:|-------|-----|
| `src/sftp/client.rs` | 42 | russh-sftp 2.0 API drift — `From<&Cow<'_, str>>` not implemented for `String`, `setstat` removed from `SftpSession`, method-arity mismatches on `read`/`write`/`stat`/`close` | russh-sftp 2.1.x: methods are `&self`; `set_metadata(path, FileAttributes)` replaces `setstat`; `read_dir(path)` returns `ReadDir` iterator directly (no separate `open_dir`); `open()`/`create()` return `File` impls of `AsyncRead+AsyncWrite+AsyncSeek` — stream via `tokio::io::AsyncReadExt`/`AsyncWriteExt`; `into_owned()` for `Cow<str>` |
| `src/ssh/forwarding.rs` | 5 | `russh::client::Handle<H>` doesn't impl `Clone`; `stream` & `channel` move-after-borrow | Take `Arc<Handle<H>>`; replace dual-spawn read/write loops with `channel.into_stream()` + `tokio::io::copy_bidirectional` |
| `src/sftp/browser.rs` | 3 | `FileType` doesn't impl `Copy` — moves out of shared refs in sort comparator | `#[derive(Copy)]` on the enum |
| `src/terminal/emulator.rs` | 3 | warnings: unused `cols`/`rows`/`ctx` (look like errors in count, are warnings) | Prefix with `_` or wire up |
| `src/ssh/connection.rs` | 2 | `check_server_key` trait shape mismatch (russh 0.40 takes `self`, returns `Result<(Self, bool), _>` — not `&mut self -> Result<bool, _>`); `connect_key` arg type mismatch (`&PathBuf` vs `&str`) | Match the new shape; `&path.to_string_lossy()` |
| `src/ssh/config_parser.rs` | 2 | (audit pending) | |
| `src/sftp/transfer.rs` | 2 | (audit pending) | Drop the duplicate `TransferDirection`/`TransferStatus` enums; use `crate::sftp::{TransferDirection, TransferState}` |
| `src/sftp/operations.rs` | 2 | stub TODOs leaving unused vars | Wire to `SftpClient` or delete |
| `src/ui/screens/settings_screen.rs` | 2 | unused imports + missing `ctx` use | Fix or `_`-prefix |
| `src/ssh/active_session.rs` | 1 | same `check_server_key` shape issue | Same fix as `connection.rs` |
| `src/ui/keyboard.rs` | 1 | unused `Modifiers` import | Remove |
| `src/ui/components.rs` | 1 | f32-from-f64 fallback | `1.0_f32` |
| `src/ui/screens/sftp_browser_ui.rs` | 1 | unused `ctx` arg | `_ctx` or wire |

**Acceptance:** `make build` (which calls `cargo build --release` inside the `tabssh-builder` docker image) exits 0.

- [ ] Revisit `audit.toml` ignores once upstream fixes land — `quick-xml` 0.39.4 (RUSTSEC-2026-0195, RUSTSEC-2026-0194) is pinned by `wayland-scanner` via `eframe`/`winit`/`smithay-client-toolkit` with no newer `eframe` available yet; `rsa` 0.10.0-rc.18 (RUSTSEC-2023-0071, Marvin Attack) has no fixed release, pulled in via `russh`/`ssh-key` (2026-07-27 cargo audit)

---

## Phase 1 — Tier-1 SSH-client viability

Goal: a power user can daily-drive desktop for everyday SSH/SFTP work.

### 1.1 SSH core completeness
- [ ] Keyboard-interactive auth — wire russh's prompt callback to a UI dialog
- [ ] SSH agent forwarding — russh's `agent` feature
- [ ] OpenSSH user certificate auth (`*-cert.pub`) via `ssh-key` crate
- [ ] Per-connection env vars — DB column `env_vars`, sent via `russh::Channel::env`
- [ ] Always-on keepalive (60s, count-max 3) — apply unconditionally to every russh session per Issue #166
- [ ] Centralised connection-error dialog with **Copy** button — clipboard via `arboard` (Issue #167 equivalent)
- [ ] Wire host-key verification (TOFU/MITM prompt) into the connect flow — `HostKeyInfo`, `HostKeyInfo::from_public_key`, and `verify_host_key` (`src/ssh/connection.rs`) are fully implemented but have zero call sites (2026-07-27 clippy dead-code audit)

### 1.2 Universal SSH key support
- [ ] Parse OpenSSH v1 (`-----BEGIN OPENSSH PRIVATE KEY-----`)
- [ ] Parse PEM (PKCS#1, PKCS#8) — encrypted variants too
- [ ] Parse PuTTY `.ppk` v2 + v3
- [ ] In-app key generation: RSA / ECDSA P-256/P-384/P-521 / Ed25519 / DSA (`ed25519-dalek`, `rsa`, `p256`, `p384`, `p521`)
- [ ] Encrypted-passphrase round-trip (bcrypt KDF for OpenSSH v1)
- [ ] Export public key (OpenSSH line format)
- [ ] SHA-256 fingerprint helper
- [ ] Wire key discovery/loading into the connect flow — `find_default_keys`, `read_key`, `is_key_encrypted` (`src/ssh/auth.rs`) are fully implemented but have zero call sites (2026-07-27 clippy dead-code audit)

### 1.3 SFTP browser
- [ ] Wire `SftpClient` (`src/sftp/client.rs`) into `SftpBrowser`/the active session — the transport layer (list/upload/download/mkdir/rmdir/rename/stat/chmod) is fully implemented but nothing calls it yet; `SftpBrowser` (`src/sftp/browser.rs`) is UI-state-only with no network I/O (2026-07-27 clippy dead-code audit)
- [ ] Get the dual-pane browser working with the fixed `russh-sftp` calls
- [ ] Remote file editor inline (open → edit in egui textarea → save back) — Wave 1.7 equivalent
- [ ] chmod editor dialog — Wave 1.8 equivalent
- [ ] SCP fallback for servers without the SFTP subsystem — Wave 1.9 equivalent
- [ ] Drag-and-drop upload from local
- [ ] Multi-file selection + recursive folder transfer (matches mobile)
- [ ] Resume interrupted transfers
- [ ] Permissions display + edit
- [ ] Symlink handling (display as 🔗, follow on click with confirmation)

### 1.4 Port forwarding
- [ ] -L (local) — verify against russh API after Phase 0 rewrite
- [ ] -R (remote)
- [ ] -D (dynamic / SOCKS5) — already partly written, finalize after Phase 0
- [ ] Bind-to-all-interfaces option (mobile has it 🆕)
- [ ] Saved rules per host
- [ ] Quick start/stop toggle
- [ ] Background tunnels — run forwards without an attached terminal session (Wave 3.3)
- [ ] ProxyJump / `connect-via` cascading (multi-hop) — `connect_through_jump_host` (`src/ssh/connection.rs`) is already implemented but not yet called from the connect flow (2026-07-27 clippy dead-code audit)

### 1.5 SSH config
- [ ] Read `~/.ssh/config` directly via `ssh2-config` — don't import a copy; the desktop should reflect the user's existing setup live
- [ ] Round-trip export — write back valid `Host …` blocks (Wave 6.1)
- [ ] Bulk import: CSV / JSON / PuTTY .ppk lists / Terraform host inventories (Waves 1, 6.4/6.5)
- [ ] Honour `RemoteCommand` (DB v24)
- [ ] Honour `IPMode` / address family (DB v25)

### 1.6 Multi-tab + state
- [ ] Multi-tab same-host independent shells (Issue #163) — per-tab `russh::Channel`, sibling tabs survive when one shell exits
- [ ] Tmux/Screen/Zellij auto-launch + `postConnectScript` (Issue #170) — `multiplexerMode: AUTO_ATTACH | CREATE_NEW`
- [ ] Reconnect button on disconnected tab — replace auto-close as the only option
- [ ] Active Sessions strip (Issue #165) — top of window, dynamic title via OSC 0/2, connection-state dot, tap to focus
- [ ] **Reattach prompt** when clicking a connection that already has a live tab — "Reattach to existing session, or open a new connection?" (mobile #71/#76, May 2026). Also surface pooled-but-no-tab sessions
- [ ] Resume tabs on app restart (DB `tab_sessions` table)

### 1.7 Terminal experience
- [ ] Find/search in scrollback (Wave 1)
- [ ] 24-bit true color (Wave 4.a) — `alacritty_terminal` supports; verify rendering
- [ ] UTF-8 / Unicode handling (`alacritty_terminal` covers)
- [ ] Configurable scrollback (default 10,000 lines)
- [ ] Text selection + clipboard integration (`arboard`)
- [ ] Mouse support (SGR mouse mode)
- [ ] Alternate screen buffer
- [ ] Title escape sequences (OSC 0/2/7)
- [ ] Hardware-keyboard modifier-aware nav keys + AltGr (Issue #171) — xterm-style `\e[1;<mod><letter>` for Shift/Ctrl/Alt + arrows / HOME / END / PG family. AltGr distinguished from real Alt.
- [ ] **Ctrl chord routing for non-letter keys** (mobile #74, May 2026): `Ctrl+Space` → NUL (` `), `Ctrl+@` → NUL, `Ctrl+[` → ESC (``), `Ctrl+\` → FS (``), `Ctrl+]` → GS (``), `Ctrl+^` → RS (``), `Ctrl+_` → US (``), `Ctrl+?` → DEL (``). Mirrors mobile's `getCtrlCode()` table. tmux `C-Space` prefix users need this
- [ ] URL detection on text — Ctrl+Click → open in browser
- [ ] **Drag-to-select range copy** (mobile #73, May 2026) — click-and-drag selects a rectangular range in the terminal grid; release copies to clipboard via `arboard`. Add a long-press / right-click menu with "Copy", "Copy line", "Select all"
- [ ] Cursor styles (block / beam / underline)
- [ ] Bell options (audio / visual / silent)

### 1.8 Build/version plumbing
- [ ] Cold-start commit-id marker (Issue #164) — `build.rs` resolves the commit via `git rev-parse --short=8`, log on startup once per change. Persist in `~/.config/tabssh/last-commit` so it doesn't spam every cold start. Fall back to `release.txt` then `"unknown"`.

---

## Phase 2 — Tier-2 desktop-shines features

Where desktop ergonomics genuinely beat mobile.

### 2.1 Workspaces & navigation
- [ ] Workspaces (named tab groups, DB v21 equivalent, Wave 2.5) — desktop can give each its own window
- [ ] Command palette (Ctrl+K) — Wave 2.6
- [ ] Quick switcher (Ctrl+J) — Wave 2.6
- [ ] History palette (Ctrl+R) — Wave 2.10
- [ ] Split view (multi-pane terminals in one window) — Wave 2.8
- [ ] Tab reordering (drag-drop)

### 2.2 Multi-host orchestration
- [ ] Broadcast input (one keystroke → many tabs) — Wave 2.7
- [ ] Cluster commands with **live result streaming** — Wave 4.e
- [ ] Identity abstraction (reusable credentials)
- [ ] Connection groups/folders (DB v3 equivalent — hierarchical)
- [ ] **Group sort + persistence** (mobile #75, May 2026) — when groups land, default to alphabetical (Name A-Z); offer Name A-Z / Name Z-A / Custom drag-order; persist user choice. Mobile defaults groups to alphabetical because `sort_order` defaulted to insertion order which surprised users
- [ ] Per-host color tags (DB v22 equivalent, Wave 3.1)
- [ ] Group-inherited settings

### 2.3 Themes & visual customization
- [ ] GUI theme editor (Wave 2.4) — reuse mobile's JSON theme format (`kotlinx.serialization` ↔ `serde` JSON)
- [ ] All 23 built-in themes (port `BuiltInThemes.kt` enum table — Dracula, Solarized Dark/Light, Nord, One Dark, Monokai, Gruvbox Dark/Light, Tomorrow Night, GitHub Light, Atom One Dark, Material Dark, Tokyo Night/Light, Catppuccin Mocha, Rose Pine, Everforest, Kanagawa, Night Owl, Cobalt2, plus System Default / Dark / Light)
- [ ] WCAG 2.1 AA/AAA contrast validation (port `ThemeValidator.kt`)
- [ ] VS Code theme JSON compatibility (port `ThemeParser.kt`)
- [ ] Per-host theme override

### 2.4 Snippets & automation
- [ ] Snippets library — DB-backed
- [ ] Variable placeholders: built-in `{host}`/`{user}`/`{date}` + prompt-style `{?password}` etc.
- [ ] Categories / tags
- [ ] One-tap run on session
- [ ] Run on multiple hosts (cluster snippets)
- [ ] Macro recorder + replay (Issue #173, DB v26) — capture raw byte sequences (escape codes, paste payloads, modifier-composed Ctrl/Alt); replay verbatim. Distinct from snippets (typed text + variables)

### 2.5 Hypervisor management
- [ ] Proxmox VE — REST `/api2/json/access/ticket` + termproxy WebSocket (`wss://host:8006/api2/json/.../vncwebsocket`); frame format `"0:LENGTH:MSG"` (data) `"1:COLS:ROWS:"` (resize)
- [ ] XCP-ng / XenServer — XML-RPC `session.login_with_password` + console WebSocket; detect Xen Orchestra fronting and fall back to XO client
- [ ] Xen Orchestra — REST + WebSocket; auto-detect `/rest/v6` → `/rest/v5` → `/rest/v0`; `wss://host:port/api/` carries real-time `vm.{started,stopped,…}` / `snapshot.{created,deleted}` / `backup.completed` events; live-updates indicator
- [ ] VMware ESXi/vCenter — REST `/api/session` + `/api/vcenter/vm`; detect vCenter via `/api/vcenter/datacenter` probe
- [ ] **Oracle Cloud Infrastructure (OCI Compute)** — Path A onboarding (import `~/.oci/config` + `.pem` via the file picker); RSA-SHA256 HTTP request signing per draft-cavage-http-signatures-08; reject `security_token_file=` configs (1-hour CLI-only session tokens); region picker seeded with the 34 commercial regions + free-text; live `validateCredentials()` self-test; list / start / stop / softstop / reset / softreset Compute instances; public/private IP via primary-VNIC walk. Mobile spec: `../android/AI.md` §11.5. No console (deferred on mobile too — needs OCI's bastion-over-SSH path)
- [ ] **VM serial console via hypervisor API** (no VM network needed — works during OS install / for VMs without network) — Wave 7
- [ ] Snapshot list / create / delete / revert
- [ ] Backup job list / run / runs / details
- [ ] HypervisorEditActivity-equivalent CRUD UI
- [ ] DB schema parity: hypervisors table needs `auth_type` discriminator + 5 OCI columns (`oci_tenancy_ocid`, `oci_user_ocid`, `oci_region`, `oci_fingerprint`, `oci_compartment_ocid`) when desktop reaches mobile DB v29
- [ ] OCI secrets in the OS keyring under `oci_private_key_${id}` / `oci_passphrase_${id}` — never in DB; cleared on row delete
- [ ] Crates: `reqwest` for HTTP, `tokio-tungstenite` for WS, `serde_json` for REST bodies; `rsa` + `pkcs8` + `md-5` for OCI signing/fingerprinting (already pulled by §1.2)

### 2.6 Cloud host import (opt-in only)
- [ ] DigitalOcean — list droplets via API
- [ ] Hetzner Cloud — list servers via API
- [ ] Linode — list linodes via API
- [ ] Vultr — list instances via API
- [ ] Token storage in `keyring` (per-provider key `cloud_token_${id}`) — never in DB
- [ ] Refresh on demand; no auto-discovery
- [ ] `cloud_accounts` table (DB v23 equivalent)

### 2.7 Protocol expansion
- [ ] Telnet (RFC 854) — Wave 2.3
- [ ] HTTP/SOCKS proxy (already a config field — finish UI exposure)
- [ ] Per-connection startup script (post-connect — already covered above in 1.6)
- [ ] Session log auto-record (`SessionRecorder` equivalent) + `TranscriptViewer`

### 2.8 QR pairing — desktop side
**Mobile shipped 2026-04-28. Desktop is the SENDER.** Wire format is fixed (mobile won't change). See the project tracker's "QR pairing" section for full spec.

- [ ] Add deps: `qrcodegen`, `ciborium`, `argon2`, `aes-gcm`, `rand`, `base64`
- [ ] `src/pairing/payload.rs` — `PairingPayload`, `ConnectionProfile` subset (no secrets, no private keys), serde encode
- [ ] `src/pairing/encrypt.rs` — generate 6-digit code, 16-byte salt, 12-byte nonce; Argon2id (m=64MiB, t=3, p=1) → 32-byte key; AES-256-GCM encrypt; serialise `QrPayload`
- [ ] `src/pairing/qr.rs` — render `QrPayload` (base64) → QR bitmap via `qrcodegen`, ECC level L, byte mode
- [ ] `src/ui/pairing_dialog.rs` — egui state machine (`Idle → Selecting → Generating → Active → Expired`) + QR display + 60s countdown
- [ ] Wire menu entry: File → Pair Phone…
- [ ] Round-trip test vectors — commit them, mobile reuses
- [ ] Verify against mobile-side `ImportFromQrActivity` after first build

### 2.9 Cloud sync (filesystem-watch model)
- [ ] On-disk format byte-identical to mobile's `TABSSH_SYNC_V2` (32-byte header `TABSSH_SYNC_V2` + padding, 32-byte PBKDF2 salt, 12-byte AES-GCM IV, GZIP'd JSON `SyncDataPackage`, 128-bit GCM tag appended)
- [ ] AES-256-GCM via `aes-gcm` crate, PBKDF2-HMAC-SHA256 100k iterations via `pbkdf2` crate
- [ ] User points TabSSH at a folder; their existing sync app (Nextcloud / syncthing / rclone / OneDrive / Dropbox) handles transport
- [ ] Three-way merge with conflict UI — port `MergeEngine` + `ConflictResolver` semantics
- [ ] Per-entity sync toggles (mobile has 🆕)
- [ ] WiFi-only equivalent N/A on desktop (always on the LAN); skip
- [ ] Sync coverage matrix matches mobile: connections / stored_keys / themes / host_keys 3-way; workspaces / snippets / identities / connection_groups / trusted_certificates / hypervisor_profiles last-write-wins; cloud_accounts / tab_sessions / audit_log NOT synced
- [ ] `notify` crate watches the folder for remote-side changes
- [ ] Round-trip with mobile: blob written by desktop must decrypt on phone and vice versa (test vectors in both repos)

---

## Phase 3 — Tier-3 polish + platform integration

### 3.1 Theming & data export
- [ ] Theme JSON import/export
- [ ] Encrypted ZIP backup/restore — compatible with android's `BackupManager` format (`metadata.json`, `connections.json`, `keys.json`, `preferences.json`, `themes.json`, `certificates.json`, `host_keys.json`)

### 3.2 Platform keychain integration
- [ ] `keyring` crate — single API across Linux Secret Service / macOS Keychain / Windows Credential Manager
- [ ] BSD fallback: filesystem-encrypted blob in `~/.local/share/tabssh/secrets/` with OS permission lockdown (`0600`)
- [ ] Master password protection mode (optional — `argon2` to derive a wrap key)
- [ ] Auto-lock on idle
- [ ] PIN code app lock (Wave 3.2 equivalent)
- [ ] Biometric where available: Touch ID via `security-framework` on macOS, Windows Hello via `windows` crate, Linux fingerprint via PAM (best-effort; not all distros)

### 3.3 OS integration
- [ ] System tray (`tray-icon` crate) — show running connections, "Connect to…" submenu
- [ ] Auto-launch on login — `.desktop` autostart on Linux, LaunchAgent plist on macOS, Startup folder shortcut on Windows
- [ ] CLI mode — `tabssh user@host` invocation from a shell falls through to the GUI tab
- [ ] Native file-picker integration (`rfd` crate) for SFTP upload/download paths

### 3.4 Distribution & packaging
Per-OS packaging targets (every line is one task):
- [ ] `.deb` (Debian/Ubuntu) — `cargo-deb`
- [ ] `.rpm` (Fedora/RHEL) — `cargo-generate-rpm`
- [ ] AppImage — Linux universal
- [ ] Flatpak — sandboxed Linux
- [ ] Snap — Ubuntu / derivatives
- [ ] AUR `PKGBUILD` — Arch User Repository
- [ ] `.dmg` (macOS) — `create-dmg`
- [ ] Homebrew formula — `brew install tabssh`
- [ ] MacPorts portfile — `port install tabssh`
- [ ] `.msi` (Windows) — `cargo-wix`
- [ ] WinGet manifest — `winget install tabssh`
- [ ] Chocolatey package — `choco install tabssh`
- [ ] Scoop bucket — `scoop install tabssh`
- [ ] FreeBSD `pkg` + ports tree
- [ ] OpenBSD packages
- [ ] NetBSD `pkgsrc`

### 3.5 Reliability & observability
- [ ] **Re-apply window prefs on focus event** (mobile #77, May 2026) — fullscreen toggle / cursor style / font size / line spacing must take effect when the window regains focus, not only on cold restart. egui's `ViewportEvent::Focused` is the hook
- [ ] Crash reporter via `panic::set_hook` writing to `~/.local/share/tabssh/crashes/{ISO-timestamp}.txt`
- [ ] Auto-update mechanism (opt-in) — check GitHub Releases, verify SHA-256, download to temp, atomic-replace binary
- [ ] Log files with rotation (`audit_log_max_size_mb` equivalent) — `tracing-appender`
- [ ] Audit log of activity (mobile has `AuditLogActivity`) — viewable in-app
- [ ] Connection history view (separate from "last connected" timestamp)
- [ ] What's-new / changelog screen on update — read from `release.txt`

---

## Phase 4 — situational / lower priority

- [ ] X11 forwarding (`x11rb` crate, real X11 servers — much more useful than mobile's stub which currently just frameworks the manager)
- [ ] Mosh — pure-Rust client. Avoid the mobile cross-compile dance for `libmosh-client.so` per-ABI; instead implement the SSP/UDP framing in Rust against the existing mosh server protocol.
- [ ] Multi-language support (mirror mobile's en / es / fr / de) — `fluent-rs` or `gettext`
- [ ] Accessibility — screen-reader compat (Linux Orca, macOS VoiceOver, Windows Narrator)
- [ ] Performance monitor with charts (`egui_plot`) — CPU / RAM / disk / net live polling
- [ ] FIDO2 / hardware-key SSH auth (YubiKey via USB-HID) — `ctap-hid-fido2` crate
- [ ] Tab reordering UI (drag-drop already in 2.1; this is the polish on it)
- [ ] Custom SSH ciphers/MACs UI override (russh defaults are modern; expose for legacy gear)

---

## Phase 5 — research / speculative

These need design work before they become tasks:

- ML-DSA / post-quantum auth — once OpenSSH 9.x post-quantum is widespread (russh tracks JSch on this — JSch 2.27 already supports ML-KEM kex)
- Multi-host performance dashboard ("rack view")
- Performance benchmarks for terminal renderer FPS on each platform
- Fuzzing critical parsers (`cargo-fuzz` against `vte` and ssh-config parsers)
- Plugin SDK — explicitly REJECTED on mobile ("we ship everything built-in"). Same on desktop unless user-driven need emerges.

---

## Out of scope (mobile leads, desktop shouldn't follow)

| Mobile feature | Why not on desktop |
|---|---|
| Foreground service notification | Use system tray |
| SAF (Storage Access Framework) document URIs | Desktop reads files directly |
| On-screen keyboard (1-5 customisable rows) | Real keyboard always available |
| Volume key bindings, pinch-zoom | Use Ctrl+Scroll |
| Swipe gestures between tabs | Use Ctrl+Tab + mouse |
| ANR watchdog (android-specific concept) | `panic::set_hook` covers crashes; UI-thread freeze detection is optional polish |
| Tasker integration | Use shell scripts / `.desktop` actions |
| Android home-screen widget | N/A |
| Foldable layout / sw720dp / book-mode | Desktop windows are resizable anyway |
| FLAG_SECURE screenshot protection | Desktop OSes don't expose an equivalent universally |
| Custom multi-touch gestures | Desktop uses mouse |
| Voice typing into terminal | Use OS-level dictation |
| Shake-to-send-Tab gesture | N/A |
| Bluetooth keyboard pairing UI | OS handles |
| Edge-swipe tab switching (Issue #168) | Use Ctrl+Tab |
| Cross-platform desktop app (sic) | That **is** this project |

---

## Acceptance for "in line with android"

Considered done when, for every row in the project tracker's "Comparison with Android Version" matrix:

- ✅ rows show ✅ on the desktop column too, OR
- 🔴 rows have a tracked TODO above with file paths and crate choices, OR
- 🚫 rows are explicitly listed in "Out of scope" above with reason

Plus interop guarantees:

- The QR pairing wire format must be **byte-compatible** with mobile (test vectors checked into both repos; mobile-decode of desktop-generated QR succeeds and vice versa).
- The cloud sync `TABSSH_SYNC_V2` format must be **byte-compatible** with mobile (an encrypted blob written by desktop must round-trip through mobile and vice versa).
- The encrypted ZIP backup/restore format must be readable on both platforms.
- The 23 built-in theme JSONs must be byte-identical between platforms (one source of truth in `assets/themes/`).

---

## CI/CD follow-ups (found during AI.md Docker/CI alignment pass)

- Release Docker-image publish parity: `.github/workflows/release.yml` now
  builds/pushes/attests a multi-arch `ghcr.io` image (`publish-image` job),
  but `.gitlab-ci.yml`, `.gitea/workflows/release.yml`, and
  `.forgejo/workflows/release.yml` do not have an equivalent image-publish
  step. AI.md's CI/CD rules require the same gates on every provider — decide
  whether to add matching image-publish jobs to the other four providers'
  release pipelines (registry target per provider) or document why GitHub-only
  publishing is acceptable.
- `.github/workflows/release.yml` "Validate release tag" step deletes and
  re-pushes the triggering tag (`git tag -d` + `git push origin
  :refs/tags/$tag` + re-create + re-push). AI.md does not document this
  pattern; confirm it is intentional and still needed, or simplify.
- `.gitea/workflows/ci.yml` and `.forgejo/workflows/ci.yml` derive the binary
  name from `${{ github.event.repository.name }}`, while
  `.github/workflows/ci.yml` derives `CRATE_NAME` via `grep` on `Cargo.toml`.
  These will diverge if the crate name and repo name ever differ. Align all
  three on the same derivation method.

## rust-lint findings (pre-existing, unrelated to Docker/CI alignment pass)

- Makefile lines 32, 55: output binary name uses `amd64` — must use `x86_64`.
- Makefile line 61: output binary name uses `arm64` — must use `aarch64`.
- Makefile lines 13-19: `DOCKER_RUN` definition is missing cache mounts —
  add `-v $(CARGO_CACHE):/root/.cargo -v $(RUSTUP_CACHE):/root/.rustup
  -v $(SCCACHE_CACHE):/root/.cache/sccache -v $(CARGO_TARGET):/work/target`
  and define `CARGO_CACHE ?= $(HOME)/.cargo`, `RUSTUP_CACHE ?= $(HOME)/.rustup`,
  `SCCACHE_CACHE ?= $(HOME)/.cache/sccache`,
  `CARGO_TARGET ?= $(HOME)/.cache/cargo-target/$(PROJECT)`.
- Makefile lines 31, 54, 60, 94, 100, 106, 124: `DOCKER_RUN` invoked without a
  preceding `@mkdir -p $(CARGO_CACHE) $(RUSTUP_CACHE) $(SCCACHE_CACHE)
  $(CARGO_TARGET)` guard — add as the first recipe line in each target.
- Cargo.toml line 59: `[profile.release-small]` sets `strip = "symbols"`,
  which differs from `[profile.release]`'s `strip = true` — align unless
  release-small is intentionally a different minimal profile.
