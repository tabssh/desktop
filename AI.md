# TabSSH Desktop — Implementation Spec (THE HOW)

> Paths are repo-relative unless prefixed with `/`. Crate references use the `crate::module` form. "Stub" or "framework only" means the file/struct exists but is not wired into a working user-facing flow — treat as future work. Where this file mentions the android sibling, the path is `../android/<file>`. Mobile is the protocol-and-format reference; desktop must round-trip with mobile for sync blobs and QR pairing payloads.
>
> **Last verified against:** `Cargo.toml` 0.1.0 / russh 0.40.2 / russh-sftp 2.1.1 / egui+eframe 0.25 / vte 0.13 / rusqlite 0.30 (bundled) / Rust nightly base image (`rustlang/rust:nightly-bookworm`) / Android sibling source read 2026-06-12.
>
> See [IDEA.md](IDEA.md) for goals, platform targets, and constraints (THE WHAT).

---

## Table of Contents

1. Project identity
2. High-level architecture
3. Build, toolchain, and dependencies
4. Application layer
5. SSH connection layer
6. Terminal emulation layer
7. Cryptography and key management
8. Storage and database
9. Sync system (filesystem-watch)
10. Backup and restore
11. Hypervisor integration
12. Cloud host import
13. VNC hosts (direct VNC)
14. Performance monitoring
15. Background monitoring
16. Session recording
17. Audit log
18. UI, theming, accessibility, i18n
19. Notifications, system tray, CLI mode
20. Build and release infrastructure
21. Module map
22. Known stubs and limitations
23. Editing guidelines for AI agents
24. QR pairing — desktop sender

---

## 1. Project identity

| Field | Value |
|---|---|
| **Crate name** | `tabssh` |
| **Display name** | TabSSH Desktop |
| **Type** | Cross-platform SSH/SFTP client with browser-style tabs |
| **Language** | Rust (2021 edition) |
| **MSRV** | 1.75 |
| **License** | MIT (same as android) |
| **Repository** | https://github.com/tabssh/desktop |
| **Distribution** | GitHub Releases + native package managers per platform (planned) |
| **Supported targets** | Linux (musl-static, glibc 2.17+), macOS (10.15+), Windows (10+), FreeBSD, OpenBSD, NetBSD on amd64 + arm64 — 11 binary variants |

**Design pillars (mirroring android, adapted to desktop):**
1. **Tabbed terminal sessions** — egui-based tab manager, browser-style; `Ctrl+T`/`Ctrl+W`/`Ctrl+Tab` are the primary navigation.
2. **Native filesystem sync** — encrypted blobs written to a folder; the user's existing sync app (Nextcloud / syncthing / rclone / OneDrive / Dropbox) handles transport. Wire-format byte-compatible with mobile's SAF blobs.
3. **Real terminal emulation** — `vte` parses, `alacritty_terminal` (or our own buffer over `vte`) renders, egui canvas paints. No reinventing escape parsing.
4. **OS-keychain-backed crypto** — `keyring` crate covers Linux Secret Service / macOS Keychain / Windows Credential Manager in one API.
5. **Single static binary** — no JVM, no Android SDK, no runtime install. `cargo build --release --target x86_64-unknown-linux-musl` produces 8–14 MB self-contained executables.
6. **Mobile interop** — encrypted sync blobs and QR pairing payloads round-trip between desktop and the android sibling. Test vectors live in both repos.

---

## 2. High-level architecture

State propagation: `tokio::sync::watch` and `tokio::sync::mpsc` are the canonical reactive primitives. UI layers subscribe to watch channels; managers publish state changes. Long-running tasks use mpsc commands+events. **No** equivalent of LiveData/Rx — stick to `tokio::sync` and `Arc<Mutex<T>>` / `Arc<RwLock<T>>` for shared state, channels for messaging.

Threading model:
- Tokio multi-threaded runtime for async I/O (SSH read loops, SFTP transfers, sync, hypervisor REST/WS, cloud API calls).
- egui runs on the main OS thread; cross-thread updates use `egui::Context::request_repaint()` + `tokio::sync::watch`.
- Database writes serialised through a single rusqlite connection wrapped in `Arc<Mutex<Connection>>` — rusqlite isn't `Send` for the same connection. For concurrency, use a connection pool (`r2d2_sqlite`) or per-task connections.
- Long-lived connections live in their own task; `ActiveSession` exposes mpsc command/event endpoints.
- Background monitoring runs as a periodic tokio task; performance metric collection runs as a short-lived task launched per terminal session.

Layered structure:
- **UI layer** (`src/ui/`) — egui screens / dialogs / components / tab manager
- **Managers** (`src/app.rs`) — `SessionManager`, `ActiveSession`, `Database`, `ThemeManager`, `KeyringStore`, `SettingsManager`
- **SSH** (`src/ssh/`) — russh wrapper, host-key verification, port forwarding, config parser, port knocking, X11 proxy, Mosh
- **Terminal** (`src/terminal/`) — vte parser, character grid, scrollback, egui canvas renderer, session recorder
- **Storage** (`src/storage/`) — rusqlite + entities + migrations + settings
- **Cross-cutting** — `src/crypto/`, `src/sync/`, `src/backup/`, `src/hypervisor/`, `src/cloud/`, `src/vnc/`, `src/monitoring/`, `src/pairing/`, `src/platform/`, `src/notifications/`

---

## 3. Build, toolchain, and dependencies

### 3.1 Toolchain

| Component | Version | File |
|---|---|---|
| Rust edition | 2021 | `Cargo.toml` |
| MSRV | 1.75 | `Cargo.toml` |
| Cargo | matches toolchain | — |
| Docker base | `rustlang/rust:nightly-bookworm` | `docker/Dockerfile` |
| musl target | `x86_64-unknown-linux-musl` | `.cargo/config.toml` |
| Make | GNU Make ≥ 4 | `Makefile` |

The Docker image bundles `rustup` plus apt packages: `build-essential pkg-config cmake git libssl-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libfontconfig1-dev libgtk-3-dev libx11-6 libxcursor1 libxrandr2 libxi6 libgl1-mesa-glx libgl1-mesa-dri libegl1-mesa libwayland-client0 libwayland-egl1 fonts-dejavu-core musl-tools musl-dev`. See `docker/Dockerfile`.

### 3.2 Build profiles

| Profile | Optimisations | Strip | LTO | codegen-units | panic |
|---|---|---|---|---|---|
| `release` | level 3 | yes | full | 1 | abort |
| `release-small` | level z | yes | full | 1 | abort |
| `dev` (default `cargo build`) | 0 | no | no | 16 | unwind |

Binary size targets: 8–14 MB stripped on Linux musl, 10–14 MB on macOS / Windows. With UPX compression: 3–5 MB on Linux, 4–6 MB on macOS/Windows.

### 3.3 Key dependencies

| Category | Crate | Version | Notes |
|---|---|---|---|
| Async runtime | `tokio` | 1.35+ | full features |
| SSH | `russh` | 0.40.2 | pure Rust SSH2 |
| SSH keys | `russh-keys` | 0.40 | parse / load keys for russh |
| SFTP | `russh-sftp` | 2.1.1 | pure Rust SFTP over a russh `Channel` |
| UI | `eframe` + `egui` | 0.25 | immediate-mode GUI |
| UI extras | `egui_extras` | 0.25 | image widgets |
| Terminal escape parser | `vte` | 0.13 | spec-correct ANSI parser |
| Storage | `rusqlite` | 0.30 | bundled SQLite |
| Serde | `serde` | 1.0 | derive |
| Serde JSON | `serde_json` | 1.0 | for theme files, REST bodies, cloud APIs |
| Serde TOML | `toml` | 0.8 | settings files |
| HTTP client | `reqwest` | 0.11+ | cloud APIs (AWS, GCP, Azure, DO, Hetzner, OCI) — TLS via `rustls` |
| WebSocket | `tokio-tungstenite` | 0.20+ | hypervisor console WS, Xen Orchestra events |
| HMAC / SHA | `hmac` + `sha2` | 0.12 / 0.10 | AWS SigV4 signing |
| JWT | `jsonwebtoken` | 9+ | GCP service-account JWT→OAuth2 |
| Errors | `anyhow` + `thiserror` | 1.0 | top-level + lib boundaries |
| Logging | `log` + `env_logger` | 0.4 + 0.11 | structured logging via tracing planned |
| Dirs | `dirs` | 5.0 | XDG / Apple / Windows config locations |
| Time | `chrono` | 0.4 | timestamps |
| UUID | `uuid` | 1.6 | v4 + serde |
| Path expand | `shellexpand` | 3.1 | `~/.ssh/config` paths |
| Regex | `regex` | 1.10+ | Mosh CONNECT parser, port-knock JSON, session file naming |

Per-platform crates (loaded only on the matching target):
- `cfg(target_os = "macos")`: `security-framework` 2.9 — Keychain + biometrics
- `cfg(target_os = "windows")`: `windows` 0.52 with `Win32_Security_Credentials` — Credential Manager
- `cfg(target_os = "linux")`: `keyring` 2.1 — Secret Service over D-Bus

Planned additions (not yet in `Cargo.toml` — see `TODO.AI.md`):

| Phase | Crate | Use |
|---|---|---|
| 1.2 | `ssh-key` | Universal key parser (OpenSSH v1 binary format / PEM / PKCS#8 / PuTTY v2/v3) |
| 1.2 | `ed25519-dalek`, `rsa`, `p256`, `p384`, `p521` | In-app key generation |
| 1.5 | `ssh2-config` | Parse `~/.ssh/config` |
| 1.7 | `arboard` | Cross-platform clipboard |
| 2.5 | `md5` | OCI fingerprint = MD5(SubjectPublicKeyInfo DER) |
| 2.8 | `qrcodegen`, `ciborium`, `argon2`, `aes-gcm`, `rand`, `base64` | QR pairing — desktop sender |
| 2.9 | `aes-gcm`, `pbkdf2`, `sha2`, `flate2`, `notify` | Cloud-sync wire compat with mobile + filesystem watcher |
| 3.3 | `tray-icon`, `notify-rust` | System tray icon + native notifications |
| 3.4 | `cargo-deb`, `cargo-generate-rpm`, `cargo-wix` | Packaging |
| 4 | `x11rb`, `egui_plot`, `ctap-hid-fido2`, `fluent-rs` | X11 forwarding, charts, FIDO2, i18n |

### 3.4 Repository layout

```
desktop/
├── src/                          # ALL Rust sources
│   ├── main.rs                   # Entry point
│   ├── lib.rs                    # Crate-level re-exports
│   ├── app.rs                    # Top-level egui app state
│   ├── ssh/                      # russh wrappers, session manager, config parser, port forwarding, port knocking, X11 proxy, Mosh
│   ├── sftp/                     # SFTP client + browser + transfer manager + operations
│   ├── terminal/                 # vte parser + buffer + cell + renderer + emulator + session recorder
│   ├── storage/                  # rusqlite + entities + migrations + settings
│   ├── crypto/                   # keyring wrapper + key parsing + symmetric crypto
│   ├── platform/                 # per-OS adapters (linux.rs / macos.rs / windows.rs / bsd.rs)
│   ├── config/                   # settings + themes
│   ├── ui/                       # egui screens / dialogs / components
│   ├── pairing/                  # (planned) QR pairing payload + crypto + QR render
│   ├── sync/                     # (planned) filesystem-watch sync + 3-way merge
│   ├── hypervisor/               # (planned) Proxmox / XCP-ng / XO / VMware / OCI / Libvirt
│   ├── cloud/                    # (planned) AWS / GCP / Azure / DigitalOcean / Hetzner host import
│   ├── vnc/                      # (planned) direct VNC client (RFB 3.8)
│   ├── monitoring/               # (planned) performance metrics + background host availability
│   ├── backup/                   # (planned) encrypted backup
│   ├── audit/                    # (planned) audit log manager
│   ├── notifications/            # (planned) system tray + native notifications
│   └── utils/                    # logging + errors + helpers
├── tests/                        # Integration tests
├── benches/                      # Criterion benchmarks
├── docker/Dockerfile             # Build image
├── scripts/                      # Build + release helpers
├── assets/                       # Embedded resources (icons, themes JSON, fonts)
├── Cargo.toml, Cargo.lock        # Dependencies
├── Makefile                      # Build automation
├── .cargo/config.toml            # Cross-compilation config
├── binaries/                     # Debug builds (gitignored)
├── releases/                     # Release builds (gitignored)
├── target/                       # Cargo artifacts (gitignored)
├── README.md, LICENSE.md
├── AI.md                         # This file (architectural ground truth)
├── TODO.AI.md                    # Live work list, phased
└── .github/                      # CI workflows
```

Strict structure rules:

1. ALL Rust source code lives under `src/`.
2. ALL tests under `tests/` (integration) or inline `#[cfg(test)] mod tests` blocks (unit).
3. ALL build/release scripts under `scripts/`.
4. Build outputs go to `binaries/` (debug), `releases/` (release), `target/` (cargo) — all gitignored.
5. Configuration files (`Cargo.toml`, `Makefile`, `README.md`, etc.) live in the project root only.
6. **No source code in the project root. No tests mixed with source. No build scripts outside `scripts/`.**

---

## 4. Application layer

### 4.1 Entry point and global state

`src/main.rs` configures the env_logger, sets up the tokio runtime, and hands off to `eframe::run_native` with a `TabSshApp` instance. `TabSshApp` (in `src/app.rs`) holds all globally-accessible managers and is the egui `App` impl.

Lazily-initialised singletons exposed on `TabSshApp`:
- `database: Arc<Mutex<Connection>>` — rusqlite handle
- `settings: Arc<RwLock<Settings>>` — TOML-backed configuration
- `keyring_store: KeyringStore` — `keyring` crate wrapper
- `session_manager: SessionManager` — open SSH sessions
- `theme_manager: ThemeManager` — theme switching + caching
- `tab_manager: TabManager` — open terminal tabs (one `ActiveSession` each)
- `transfer_manager: TransferManager` — SFTP transfer queue
- `audit_log: AuditLogManager` — 23-event-type audit trail (§17)
- `monitoring: BackgroundMonitor` — host availability prober (§15)

Crash handling: install a `panic::set_hook` early in `main` that writes `~/.local/share/tabssh/crashes/{ISO-timestamp}.txt` (or platform equivalent via `dirs::data_local_dir`). Debug builds also pop a `CrashReport` window via egui.

### 4.2 Screens (egui)

There are no Activities/Fragments — everything is a stateful struct rendered by an `update` method. The current and planned screen list:

| Screen / dialog | Purpose | File |
|---|---|---|
| `MainWindow` | Top-level chrome: sidebar, tab strip, content | `src/ui/main_window.rs` |
| `TabManager` (UI) | Browser-style tabs above terminals | `src/ui/tab_manager.rs` |
| `ConnectionListScreen` | List/grid of saved connections | `src/ui/screens/` |
| `ConnectionEditDialog` | Create/edit `ConnectionProfile` | (planned) |
| `TerminalView` | Renders a `TerminalBuffer` to canvas | `src/terminal/renderer.rs` + `src/ui/screens/` |
| `SftpBrowser` (UI) | Dual-pane SFTP browser | `src/ui/screens/sftp_browser.rs` |
| `SftpFileEditor` | Open remote file → edit → save back | (planned, Phase 1.3) |
| `SftpChmodDialog` | Permission editor | (planned, Phase 1.3) |
| `PortForwardingScreen` | Local/remote/dynamic forwards | (planned, Phase 1.4) |
| `SettingsScreen` | Layered prefs (general / terminal / SSH / security / sync / audit / logging) | `src/ui/screens/settings_screen.rs` |
| `KeyManagementScreen` | List/import/paste/generate/delete `StoredKey` | (planned, Phase 1.2) |
| `IdentityManagementScreen` | CRUD `Identity` | (planned, Phase 2.2) |
| `WorkspacesScreen` | Named tab groups | (planned, Phase 2.1) |
| `CommandPaletteDialog` | Ctrl+K | (planned, Phase 2.1) |
| `QuickSwitcherDialog` | Ctrl+J | (planned, Phase 2.1) |
| `HistoryPaletteDialog` | Ctrl+R | (planned, Phase 2.1) |
| `ThemeEditorScreen` | GUI theme editor | (planned, Phase 2.3) |
| `SnippetManagerScreen` | Snippets w/ prompt-style variables | (planned, Phase 2.4) |
| `MacroManagerScreen` | Record / play raw byte sequences | (planned, Phase 2.4) |
| `ClusterCommandScreen` | Broadcast input + cluster snippets | (planned, Phase 2.2) |
| `HypervisorListScreen` | Proxmox / XCP-ng / XO / VMware / OCI / Libvirt managers | (planned, Phase 2.5) |
| `VmConsoleScreen` | Hypervisor serial console (no SSH) | (planned, Phase 2.5) |
| `CloudManagerScreen` | AWS / GCP / Azure / DigitalOcean / Hetzner account config + host import | (planned, Phase 2.6) |
| `VncHostListScreen` | Direct VNC hosts | (planned, Phase 2.6) |
| `VncViewerScreen` | RFB 3.8 framebuffer viewer | (planned, Phase 2.6) |
| `PerformanceOverlay` | Live CPU/mem/disk/net graphs overlaid on a terminal tab | (planned, Phase 2.7) |
| `MultiHostDashboard` | Side-by-side metrics for multiple connections | (planned, Phase 2.7) |
| `MonitoringConfigScreen` | Per-connection monitoring slot configuration | (planned, Phase 2.7) |
| `TranscriptViewerScreen` | Replay recorded sessions | (planned, Phase 2.7) |
| `AuditLogViewerScreen` | Browse and filter audit log entries | (planned, Phase 3.5) |
| `LogViewerScreen` | App / debug log viewer | (planned, Phase 3.5) |
| `BackupRestoreDialog` | Encrypted backup | (planned, Phase 3.1) |
| `SyncSettingsScreen` | Folder picker, password, frequency, per-entity toggles | (planned, Phase 2.9) |
| `PairingDialog` | QR pairing — desktop sender | (planned, Phase 2.8) |
| `CrashReport` | Debug crash dump UI | (planned, Phase 3.5) |

### 4.3 Canonical user flows

**Quick connect:** `ConnectionListScreen` → click profile → `TabManager.new_tab(profile)` → `SessionManager.connect(profile)` → terminal renders live output.

**New connection:** `ConnectionListScreen` → "New" button → `ConnectionEditDialog` → save to `connections` table → returns to list.

**File transfer:** any open terminal → menu → `SftpBrowser` (uses the existing SSH session via `SftpClient::connect(channel)`) → upload/download routed through `TransferManager`.

**Hypervisor console:** `HypervisorListScreen` → manager screen → VM row → `VmConsoleScreen` opens `tokio_tungstenite` WS to the hypervisor and pipes its frames into a `TerminalBuffer` — same renderer the SSH path uses.

**Cloud import:** `CloudManagerScreen` → select provider → authenticate → browse returned hosts → "Add selected" → inserts `ConnectionProfile` rows.

**Performance monitoring:** open terminal tab → menu → "Performance Overlay" → `PerformanceOverlay` opens; runs `MetricsCollector` SSH commands (§14) every 5s, accumulates 60 data points, renders charts.

---

## 5. SSH connection layer

Module: `src/ssh/`. Built on **russh 0.40.2** (`Cargo.lock` pinned).

### 5.1 `SshConnection` and `SessionManager`

`src/ssh/connection.rs` wraps `russh::client::Handle<H>` (where `H` is our `SshClientHandler`).

Lifecycle:
1. (Optional) port-knock sequence before TCP connect — see §5.8.
2. `russh::client::connect(config, addr, handler)` returns a `Handle<H>`.
3. **Jump host / ProxyJump:** open an upstream session, request a `direct-tcpip` channel through it, hand the resulting `ChannelStream` to a fresh `russh::client::connect_stream` for the real target.
4. **HTTP/SOCKS proxy:** apply per-connection proxy config before TCP connect (TODO; russh doesn't bake this in).
5. Session config — `russh::client::Config { inactivity_timeout, ... }`. **Always-on keepalive** at 60s, count-max 3 — apply unconditionally per android Issue #166. The per-profile keepalive flag is gone; the SSH layer does it.
6. **Default cipher / MAC / kex preferences** mirror android (use russh's `Preferred` builder):
   ```
   ciphers  = aes256-gcm@openssh.com, aes128-gcm@openssh.com,
              aes256-ctr, aes192-ctr, aes128-ctr
   macs     = hmac-sha2-256-etm@openssh.com, hmac-sha2-512-etm@openssh.com,
              hmac-sha2-256, hmac-sha2-512
   kex      = curve25519-sha256, curve25519-sha256@libssh.org,
              ecdh-sha2-nistp256, ecdh-sha2-nistp384, ecdh-sha2-nistp521,
              diffie-hellman-group16-sha512, diffie-hellman-group14-sha256
   PreferredAuthentications = publickey, keyboard-interactive, password
   ```
7. **Authentication priority:** public key (`authenticate_publickey`) → password (`authenticate_password`) → keyboard-interactive (`authenticate_keyboard_interactive_start` + prompt callback). Never retries on auth failure.
8. **Auto-reconnect:** up to 3 retries with 5s exponential backoff for transient network errors (`tokio::time::sleep` between attempts). Never retries auth failures.

### 5.2 `SshClientHandler` and host-key verification

`SshClientHandler` impls `russh::client::Handler`. The trait shape in russh 0.40.2 is:
```rust
async fn check_server_key(self, key: &PublicKey) -> Result<(Self, bool), Self::Error>
```
**It consumes `self` and returns ownership back** — this is unusual but required. Our impl validates against the `host_keys` table (TOFU), shows the new-host or changed-host dialog if needed, then returns `Ok((self, accept))`.

Trust levels persisted to `host_keys`: `UNKNOWN` / `ACCEPTED` / `VERIFIED`. SHA-256 fingerprint + emoji visual fingerprint for the dialog.

### 5.3 Per-tab channels (Issue #163 equivalent)

Each `Tab` owns its own `russh::Channel<russh::client::Msg>` against a shared `Handle<H>`. Two tabs to the same profile = two `request_shell` (or `request_exec` if `profile.remote_command` is set) on one Handle. PTY resize is per-channel: `channel.window_change(cols, rows, ...).await`. Tab-close calls `channel.close()` without disturbing siblings.

The `Handle<H>` itself **does not implement `Clone`**. Wrap it in `Arc<Handle<H>>` if you need to share it across tasks (port forwarding, multi-tab, cluster commands).

### 5.4 Host-key store

`src/storage/database.rs` exposes `host_keys` table operations. `HostKeyVerifier::check(host, port, key)` returns `Accepted` / `NewHost` / `Changed`. UI dialogs are responsible for consent.

### 5.5 Port forwarding

`src/ssh/forwarding.rs` — `ForwardingManager` plus `PortForward` structs.

- **Local (`-L`):** `TcpListener::bind` on local addr → for each accept, `ssh_handle.channel_open_direct_tcpip(host, port, ...)` → `tokio::io::copy_bidirectional(local_stream, channel.into_stream())`.
- **Remote (`-R`):** request global `tcpip-forward` from server → handle incoming `forwarded-tcpip` channels and connect to local target.
- **Dynamic (`-D` / SOCKS5):** `TcpListener::bind` → SOCKS5 handshake (no auth, CONNECT only) → resolve target → `direct-tcpip` over SSH.
- **Bind-to-all-interfaces** is a per-forward toggle (mobile has it; desktop matches).
- **Background tunnels** (Wave 3.3 equivalent): forwards run in their own tokio tasks; can outlive the originating terminal tab.

### 5.6 SSH config import

`src/ssh/config_parser.rs` (planned to migrate to `ssh2-config` crate per Phase 1.5). Parses the standard directive set: `Host`, `HostName`, `User`, `Port`, `IdentityFile`, `ProxyJump`, `ProxyCommand`, `Compression`, `ServerAliveInterval`, `ConnectTimeout`, `Ciphers`, `MACs`, `KexAlgorithms`, `RemoteCommand`, `AddressFamily` / `IPMode`.

**Desktop-specific advantage:** read `~/.ssh/config` *in place*. The user's existing OpenSSH setup becomes the connection list automatically — no import step. Path resolves via `dirs::home_dir().join(".ssh/config")` and `shellexpand` for `~`-prefixed paths in `IdentityFile`.

### 5.7 SFTP

`src/sftp/client.rs` opens `russh_sftp::client::SftpSession` over a russh `Channel<russh::client::Msg>`:

```rust
channel.request_subsystem(true, "sftp").await?;
let sftp = SftpSession::new(channel.into_stream()).await?;
```

`SftpSession` exposes `&self` async methods: `read_dir(path) -> ReadDir` (iterator over `DirEntry` with `file_name() -> String` and `metadata() -> Metadata`), `open(path) -> File` and `create(path) -> File` (both impl `tokio::io::AsyncRead + AsyncWrite + AsyncSeek`), `metadata(path)`, `set_metadata(path, FileAttributes)`, `create_dir`, `remove_file`, `remove_dir`, `rename`.

Buffer size: 32 KB matches mobile. Streaming downloads/uploads use `tokio::io::AsyncReadExt::read` + `AsyncWriteExt::write_all` against the `File` handle directly; the dual progress callback signature `(transferred, total)` is preserved.

`TransferManager` queues transfers; per-transfer state: `Pending` / `InProgress` / `Completed` / `Failed(err)` / `Cancelled`.

### 5.8 Port knocking

`src/ssh/portknock.rs` — `PortKnocker` sends a timed sequence of packets before the main TCP connect.

**Wire format:** the `port_knock_sequence` column on `ConnectionProfile` is a JSON array:
```json
[{"port": 7000, "protocol": "TCP"}, {"port": 7001, "protocol": "UDP"}, {"port": 7002, "protocol": "TCP"}]
```
Supports both TCP and UDP knocks. Default inter-knock delay: **100 ms** (`DEFAULT_KNOCK_DELAY_MS = 100`).

Implementation:
- TCP knock: open a TCP stream with a short timeout, immediately close it (the SYN itself is the signal).
- UDP knock: `UdpSocket::bind("0.0.0.0:0")` → `send_to(b"", (host, port))`.
- Delay between knocks: `tokio::time::sleep(Duration::from_millis(delay_ms))`.
- After the final knock, wait `delay_ms` before returning so the server's firewall rule is active.

### 5.9 X11 forwarding proxy

`src/ssh/x11_proxy.rs` — `X11Proxy` bridges the SSH `x11` channel to a local X display.

On connection: bind a `TcpListener` on an OS-assigned port (`bind("127.0.0.1:0")`), record the assigned port. Send `request_x11` to the server with the display number derived from that port (display = (port - 6000)).

Buffer size: **8192 bytes** per read/write iteration.

Accept loop: for each incoming `x11` channel from the server, `tokio::spawn` a task that calls `connect_x_display()` and then `tokio::io::copy_bidirectional(x_stream, channel.into_stream())`.

`connect_x_display()` tries, in order:
1. On Linux, `$DISPLAY` environment variable parsed for a Unix socket path (typical X11).
2. Fallback: TCP connect to `localhost:6000`.

The desktop does **not** support the Termux:X11 or XSDL paths (those are Android-specific). On Linux/macOS the standard X11 Unix socket or TCP display is used.

### 5.10 Mosh

`src/ssh/mosh.rs` — `MoshHandoff` bootstraps Mosh over the existing SSH connection.

Bootstrap sequence (matching android `MoshHandoff.kt`):
1. `request_exec` on a new SSH channel: `mosh-server new -l LANG=en_US.UTF-8`
2. Read stdout until EOF. Parse with regex: `MOSH CONNECT (\d+) (\S+)` — group 1 = UDP port, group 2 = base64 session key.
3. If parsing succeeds, close the SSH channel and hand off UDP endpoint + key to the Mosh client.

Desktop uses a **pure-Rust Mosh client** (no native binary, unlike android's `libmosh-client.so`). The Mosh protocol implementation lives in `src/ssh/mosh_client.rs`.

`ConnectionProfile.mosh_mode` field: `"auto"` (try Mosh, fall back to SSH) / `"always"` / `"never"`. Default: `"auto"`.

### 5.11 Bulk import

`src/ssh/bulk_import.rs` — `BulkImportParser` detects and parses multiple host-list formats:

| Format | Detection | Notes |
|---|---|---|
| `CSV` | `.csv` extension or first line has commas + recognisable header | host, port, user, name columns |
| `JSON` | file starts with `[` or `{` | array of host objects |
| `PUTTY_REG` | `[HKEY_CURRENT_USER\Software\SimonTatham\PuTTY\Sessions\` line | Windows registry export |
| `TERRAFORM` | contains `resource "aws_instance"` or other resource blocks | HCL-style; detects `aws_instance`, `digitalocean_droplet`, `google_compute_instance`, `hcloud_server`, `linode_instance`, `vsphere_virtual_machine` resource types |

---

## 6. Terminal emulation layer

### 6.1 Parser + buffer + renderer split

Three concerns split:

- **`src/terminal/parser.rs`** — wraps `vte::Parser` and dispatches escape sequences to a `vte::Perform` impl. The Perform impl mutates the buffer.
- **`src/terminal/buffer.rs`** — character grid (rows × cols of `Cell`), scrollback ring buffer (default 10,000 lines), cursor position, alternate screen, dirty-row tracking.
- **`src/terminal/cell.rs`** — `Cell { ch: char, fg: Color, bg: Color, attrs: Attrs }` where `Attrs` packs bold / italic / underline / reverse / strikethrough / blink / dim.
- **`src/terminal/emulator.rs`** — high-level `TerminalEmulator` ties the parser and buffer together.
- **`src/terminal/renderer.rs`** — egui canvas painter. Reads dirty rows, paints to `egui::Painter` using a monospace font.

### 6.2 Capabilities

- VT100 / VT220 / xterm 256-color
- 24-bit true color — supported by `vte` parser; ensure renderer takes the full RGB
- UTF-8 / Unicode handling
- Configurable scrollback (default 10,000 lines; mobile default is 1,000 — desktop bigger by default since RAM is cheap)
- Mouse support — SGR mouse mode (`CSI ? 1006 h`)
- Alternate screen buffer (`CSI ? 1049 h`)
- Title escape sequences — OSC 0 / 2 / 7
- Bell — visual or audio (config: `terminal.bell_style = visual | audio | silent`)

### 6.3 Input

`src/ui/keyboard.rs` translates egui key events into ANSI escape sequences sent via `Tab::write_input(bytes)` → `russh::Channel::data(bytes)`.

Specifics ported from mobile Issue #171:
- Hardware-keyboard modifier-aware nav keys: xterm-style `\e[1;<mod><letter>` for Shift / Ctrl / Alt + arrows / Home / End / PgUp / PgDn / F1–F12.
- AltGr distinguished from real Alt — egui exposes the difference; AltGr-composed Unicode passes through unchanged.
- Ctrl+Shift+C / Ctrl+Shift+V for copy/paste (so Ctrl+C still sends SIGINT).
- Ctrl+Click on a URL → open in default browser via `open` crate (planned).

### 6.4 Hypervisor / VNC console reuse

The same `TerminalBuffer` + renderer is used by `VmConsoleScreen`. A `tokio_tungstenite::WebSocketStream` is wrapped to expose `AsyncRead` (parsing the per-vendor frame format inline — see §11) and fed into the parser like any SSH stream. This gives identical terminal behaviour for SSH and serial-console paths.

---

## 7. Cryptography and key management

### 7.1 Supported SSH key types

Defined in `src/crypto/keys.rs` (planned to use `ssh-key` crate):

| Type | Default size | Allowed sizes | OpenSSH name |
|---|---|---|---|
| RSA | 3072 | 2048, 3072, 4096 | `ssh-rsa` |
| ECDSA | 256 | 256 (P-256), 384 (P-384), 521 (P-521) | `ecdsa-sha2-nistp{256,384,521}` |
| Ed25519 | 256 | 256 | `ssh-ed25519` |
| DSA | 2048 | 1024, 2048, 3072 | `ssh-dss` (legacy, rarely used) |

OpenSSH user certificates (`*-cert.pub`) are supported via `ssh-key`'s cert format.

### 7.2 `KeyParser` (planned, Phase 1.2)

`src/crypto/keys.rs` uses `ssh-key` to auto-detect and parse:

**OpenSSH v1 binary format (PEM-armored new-format):**
- Magic: `"openssh-key-v1\0"` (16 bytes) at file start after PEM decode
- KDF field: `"bcrypt"` with salt + rounds for passphrase-protected keys; `"none"` for unprotected
- Key type string (e.g. `"ssh-ed25519"`, `"ecdsa-sha2-nistp256"`, `"ssh-rsa"`) dispatches to the matching deserializer
- `ssh-key` handles all of this natively

**Other formats:**
- PEM (PKCS#1, PKCS#8) — RSA, DSA, EC, encrypted variants
- OpenSSH public-key lines (`ssh-rsa …`, `ecdsa-sha2-… …`, `ssh-ed25519 …`)
- PuTTY `.ppk` v2 + v3

`ssh-key` covers all of the above natively, so we never reach for hand-rolled crypto here.

### 7.3 `KeyGenerator` (planned, Phase 1.2)

In-app generation per the algorithm matrix:
- RSA via `rsa::RsaPrivateKey::new`
- ECDSA via `p256` / `p384` / `p521` (separate crates per curve)
- Ed25519 via `ed25519-dalek::SigningKey`
- Output: `ssh-key`'s `PrivateKey::to_openssh` (bcrypt-encrypted with passphrase if supplied) plus the public key in OpenSSH authorized-keys-line format

EC public-key output: uncompressed point encoding (`0x04 || x || y`) on P-256/P-384/P-521.

SHA-256 fingerprint helper + emoji visual fingerprint matching mobile's format.

### 7.4 `KeyringStore` — password storage levels (mirror of android `SecurePasswordManager`)

`src/crypto/keychain.rs` uses the `keyring` crate plus per-level wrapping. Four levels:

| Level | Persistence | Notes |
|---|---|---|
| `Never` (0) | none | Always prompt |
| `SessionOnly` (1) | in-memory | Cleared on app exit |
| `Encrypted` (2, default) | OS keychain via `keyring` | Linux Secret Service / macOS Keychain / Windows Credential Manager |
| `Biometric` (3) | OS keychain + biometric gate | macOS Touch ID via `security-framework`, Windows Hello via `windows` crate, Linux best-effort PAM |

Cipher when an extra layer is needed (e.g. master-password mode wrapping the keyring entry): `aes-gcm` AES-256-GCM with a 12-byte IV, 128-bit tag (GCM_IV_LENGTH=12, GCM_TAG_LENGTH=16).

**Key alias scheme** (must match android `SecurePasswordManager` exactly):
- `tabssh_password_{connectionId}` — stored SSH password for a connection
- `tabssh_bio_password_{connectionId}` — biometric-gated variant

Configurable TTL (default 24 hours — `security.password_ttl_hours`).

If the OS keychain is unavailable (BSD without secret-service, e.g.), the manager auto-degrades to `SessionOnly` and warns the user once.

### 7.5 Symmetric crypto (sync + backup)

- `aes-gcm` for AES-256-GCM at rest (sync blobs, optional backup encryption)
- `pbkdf2` + `sha2` for PBKDF2-HMAC-SHA256 (sync key derivation, 100,000 iterations to match mobile)
- `argon2` for Argon2id (QR pairing key derivation, m=64 MiB / t=3 / p=1 to match mobile's BouncyCastle: `ARGON2_MEMORY_KIB = 64 * 1024`, `ARGON2_ITERATIONS = 3`, `ARGON2_PARALLELISM = 1`)
- `rand` (with OS entropy) for salt / IV / pairing-code generation

**Don't add ad-hoc AES code** — funnel everything through `src/crypto/encryption.rs` so the parameters stay byte-compatible with mobile.

---

## 8. Storage and database

### 8.1 SQLite via rusqlite

`src/storage/database.rs` opens a single `rusqlite::Connection` against `~/.local/share/tabssh/tabssh.db` (or platform equivalent via `dirs::data_local_dir`).

Concurrency model: rusqlite isn't `Send`, so we wrap the connection in `Arc<Mutex<Connection>>`. Long writes (sync apply, bulk import) take the mutex; reads also take it but are short. For higher throughput, swap in `r2d2_sqlite` (deferred until profiling shows lock contention).

### 8.2 Schema — desktop tracks mobile

The desktop schema is a port of mobile's Room schema. The android sibling runs at **version 3** with `fallbackToDestructiveMigration()` and 19 entity classes. Desktop versions are numbered independently but each version corresponds to a feature alignment with mobile.

| Desktop version | Mobile alignment | Tables / columns added |
|---|---|---|
| v1 (current) | partial | `connections`, `stored_keys`, `host_keys`, `themes`, `settings` (basic shape) |
| v2 (planned) | full entity set | sync columns on every entity; `connection_groups`; `snippets`; jump-host fields; `identities`; `audit_log`; port-knock fields; `macros` |
| v3 | hypervisors + cloud | `hypervisor_profiles`, `hypervisor_accounts`, `cloud_accounts`, `monitor_slots` |
| v4 | VNC + monitoring | `vnc_hosts`, `vnc_identities`; `workspaces` |

### 8.3 Entities (mirroring mobile's 19)

| Entity | Table | Notable fields | Module |
|---|---|---|---|
| `ConnectionProfile` | `connections` | `id` (UUID), `name`, `host`, `port`, `username`, `auth_type`, `key_id`, `identity_id`, `group_id`, `theme`, jump-host fields, `port_knock_sequence` (JSON), `mosh_mode` (default `"auto"`), `terminal_type` (default `"xterm-256color"`), `multiplexer_mode`, `post_connect_script`, `font_size_override`, sync metadata, `env_vars`, `agent_forwarding`, `protocol`, `color_tag` (ARGB int), `remote_command`, `ip_mode`, `oci_instance_id`, `advanced_settings` (JSON bag) | `src/storage/connection.rs` |
| `StoredKey` | `stored_keys` | `key_id`, `name`, `key_type`, `fingerprint`, `requires_passphrase`, `key_size`, `certificate` (OpenSSH cert bytes), sync metadata | `src/storage/keys.rs` |
| `HostKeyEntry` | `host_keys` | `id` (`host:port`), `key_type`, `public_key` (b64), `fingerprint`, `trust_level` | `src/storage/host_keys.rs` |
| `ThemeDefinition` | `themes` | terminal palette + UI overrides as JSON, `usage_count`, sync metadata | `src/storage/themes.rs` |
| `Identity` | `identities` | `username`, `auth_type`, `key_id`, encrypted `password`, sync metadata | (planned) |
| `Snippet` | `snippets` | `command`, `category`, `tags`, `usage_count`, `is_favorite`, `{var}` placeholders, sync metadata | (planned) |
| `Macro` | `macros` | recordable raw byte sequence (`sequence_b64`), `usage_count`, sync metadata | (planned) |
| `Workspace` | `workspaces` | named tab groups, `connection_ids` (JSON array), sync metadata | (planned) |
| `ConnectionGroup` | `connection_groups` | hierarchical (`parent_id`), `icon`, `color`, `is_collapsed`, `sort_order`, sync metadata | (planned) |
| `HypervisorProfile` | `hypervisor_profiles` | `type` (Proxmox/XCP-ng/VMware/OCI/Libvirt), credentials reference, `realm`, `verify_ssl`, `api_type_override`, `linked_connection_id` | (planned) |
| `HypervisorAccount` | `hypervisor_accounts` | per-hypervisor auth credentials (token in keyring, not DB) | (planned) |
| `CloudAccount` | `cloud_accounts` | `provider`, `enabled`, `last_refresh_at`, `last_count` (token in keyring, **not** in DB) | (planned) |
| `MonitorSlot` | `monitor_slots` | `connection_id`, `check_interval_minutes`, `last_checked_at`, `is_available`, `notify_on_change` | (planned) |
| `VncHost` | `vnc_hosts` | `id`, `name`, `host`, `port` (default 5900), `identity_id`, `color_tag`, sync metadata | (planned) |
| `VncIdentity` | `vnc_identities` | `id`, `name`, `username`, encrypted `password` | (planned) |
| `AuditLogEntry` | `audit_log` | `event_type`, `connection_id`, `command`, `output`, `exit_code`, `timestamp` | (planned) |
| `TrustedCertificate` | `trusted_certificates` | hostname, fingerprint, PEM, issuer, validity, `trust_level` | (planned) |
| `SyncState` | `sync_state` | per-(`entity_type`, `entity_id`) tracking, `conflict_status` | (planned) |
| `TabSession` | `tab_sessions` | persisted terminal state for restore | `src/storage/sessions.rs` |

### 8.4 Migrations

`src/storage/migrations/` (planned). Numbered SQL files (`001_initial.sql`, `002_sync_columns.sql`, …). Apply pending migrations on `Database::open()`. Never destructive. The schema state is recorded in a `_migrations` table.

### 8.5 Preferences

`src/storage/settings.rs` holds a `Settings` struct; serialised to TOML in `~/.config/tabssh/settings.toml`. **Mirror mobile keys** so sync round-trips don't drop unknown fields.

Notable defaults (matching android `PreferenceManager`):

| Key | Default | Notes |
|---|---|---|
| `security.password_storage_level` | `encrypted` | one of `never` / `session_only` / `encrypted` / `biometric` |
| `security.password_ttl_hours` | 24 | TTL for encrypted passwords |
| `terminal.scrollback_lines` | 10000 | per-tab scrollback (desktop default; mobile is 1000) |
| `terminal.font_size` | 14.0 | base font size; Ctrl+Scroll adjusts 8–32 |
| `terminal.font_family` | system monospace | configurable |
| `terminal.cursor_style` | `block` | `block` / `beam` / `underline` |
| `terminal.bell_style` | `visual` | `visual` / `audio` / `silent` |
| `terminal.detect_urls` | true | Ctrl+Click to open |
| `gesture.multiplexer_type` | `tmux` | `tmux` / `screen` / `zellij` |
| `gesture.multiplexer_session_name` | `tabssh` | session name for auto-attach |
| `audit.enabled` | false | `audit_log_enabled` — off by default (matches android `PreferenceManager`) |
| `audit.log_max_size_mb` | 100 | rolling cleanup (`DEFAULT_MAX_SIZE_MB = 100`) |
| `audit.log_max_age_days` | 30 | rolling cleanup (`DEFAULT_MAX_AGE_DAYS = 30`) |
| `host_log.filename_pattern` | `{user}_{host}` | configurable host log file template |
| `sync.enabled` | false | master toggle |
| `sync.frequency` | `manual` | `manual` / `15m` / `1h` / `6h` / `24h` |
| `sync.on_change` | false | debounced auto-sync after edits |
| `sync.path` | (unset) | folder to watch + write blobs |
| `connection.default_port` | 22 | new-profile default |
| `connection.default_user` | `$USER` | new-profile default |
| `connection.timeout_seconds` | 30 | TCP connect timeout |
| `connection.compression` | true | request `Compression yes` |
| `connection.keepalive_seconds` | 60 | always-on per Issue #166 — no toggle |
| `ui.theme` | system | `system` / `dark` / `light` |
| `ui.terminal_theme` | `Default Dark` | one of the 23 built-in themes |

**Sync uses a separate file** at `~/.config/tabssh/sync.toml` for the watch-path, the local password (never persisted; only a flag indicating one is set), and `last_sync_time`.

### 8.6 File storage

`src/storage/files.rs` (planned) manages app-internal directories under `dirs::data_local_dir().join("tabssh/")`:
- `ssh_keys/` — mode 0600 enforced on Unix
- `temp/` — staged uploads/downloads, TTL cleanup
- `downloads/` — user-facing download targets
- `backups/` — encrypted backup exports
- `crashes/` — panic-hook output
- `logs/` — rolling log files
- `transcripts/` — session recording files (§16)

---

## 9. Sync system (filesystem-watch)

Module: `src/sync/` (planned, Phase 2.9). Mobile uses SAF (Storage Access Framework); desktop uses **filesystem watching plus the user's existing sync app**. The wire format is identical, so encrypted blobs are interchangeable.

### 9.1 Wire format

Identical to android's `TABSSH_SYNC_V2` (`WIRE_VERSION = 2`):

| Offset | Bytes | Content |
|---|---|---|
| 0 | 32 | Header (`TABSSH_SYNC_V2` ASCII + null padding) |
| 32 | 32 | PBKDF2 salt |
| 64 | 12 | AES-GCM IV |
| 76 | … | Ciphertext: `AES-256-GCM(key, IV, GZIP(JSON(SyncDataPackage)))` |

GCM auth tag (128 bits) is appended by the cipher.

### 9.2 Crypto

- KDF: `pbkdf2::pbkdf2_hmac::<sha2::Sha256>(password, salt, 100_000, &mut key)` → 256-bit key
- Cipher: `aes_gcm::Aes256Gcm` with 12-byte IV, 128-bit tag
- Compression: `flate2::write::GzEncoder` (level: default)
- Password strength validator: `Weak` / `Fair` / `Good` / `Strong` / `VeryStrong` (≥12 chars + ≥3 character classes for `Strong`)

### 9.3 `SyncEncryptor`

`src/sync/encryption.rs` exposes `encrypt(password, package)` and `decrypt(password, blob)` returning `SyncDataPackage`. **Don't add ad-hoc AES code anywhere else** — funnel through here so the parameters stay byte-compatible with mobile.

### 9.4 `SyncDataPackage`

Verified against android `SyncModels.kt` — **exactly 10 entity types** are present in the wire package:

```json
{
  "metadata": { "device_id": "...", "device_name": "...", "sync_version": 7,
                "app_version": "0.1.0", "item_counts": { "connections": 12 } },
  "connections": [],
  "keys": [],
  "identities": [],
  "snippets": [],
  "workspaces": [],
  "groups": [],
  "themes": [],
  "macros": [],
  "settings": {},
  "vncHosts": []
}
```

Field names match the android `SyncDataPackage` data class field names exactly (camelCase for JSON). Use `#[serde(rename = "...")]` on Rust fields that differ.

### 9.5 `SyncDataCollector` and `SyncDataApplier`

- `src/sync/collector.rs` — `collect_all()` and `collect_changed_since(timestamp)` (delta sync via `modified_at`).
- `src/sync/applier.rs` — `apply_all(SyncDataPackage)` upserts into rusqlite with `OnConflictStrategy = Replace`.

**Sync coverage matrix** (verified against android `SyncDataPackage` and `SyncDataCollector`):

| Entity | Synced? | JSON field | Strategy |
|---|---|---|---|
| `connections` | ✅ | `connections` | 3-way merge (full `MergeEngine`) |
| `stored_keys` | ✅ | `keys` | 3-way merge |
| `identities` | ✅ | `identities` | last-write-wins |
| `snippets` | ✅ | `snippets` | last-write-wins |
| `workspaces` | ✅ | `workspaces` | last-write-wins |
| `connection_groups` | ✅ | `groups` | last-write-wins |
| `themes` | ✅ | `themes` | 3-way merge |
| `macros` | ✅ | `macros` | last-write-wins |
| `preferences` | ✅ | `settings` | per-category last-write-wins |
| `vnc_hosts` | ✅ | `vncHosts` | last-write-wins |
| `host_keys` | ❌ | — | per-device trust decisions |
| `cloud_accounts` | ❌ | — | per-device hardware-bound token in keyring |
| `monitor_slots` | ❌ | — | per-device monitoring config |
| `vnc_identities` | ❌ | — | per-device credentials |
| `hypervisor_accounts` | ❌ | — | per-device credentials |
| `hypervisor_profiles` | ❌ | — | per-device infrastructure |
| `trusted_certificates` | ❌ | — | per-device trust decisions |
| `audit_log` | ❌ | — | per-device security trail |
| `tab_sessions` | ❌ | — | per-device runtime state |

### 9.6 Three-way merge algorithm (`MergeEngine`)

`src/sync/merge.rs` performs **base / local / remote** three-way merge for `ConnectionProfile`, `StoredKey`, and `ThemeDefinition`. For each entity id present in either side, produce a `MergeResult<T>` with `merged`, `conflicts`, `deleted`, `added`, `updated` lists.

Cases:

- **Present locally and remotely** → field-level merge against base. Per-field divergence becomes a `Conflict`.
- **Present locally only** → if base had it, that's a `deleted-modified` conflict (remote deleted what we still have); otherwise a clean local-only add.
- **Present remotely only** → mirror of the above.
- **Absent on both** → if base had it, record as deleted on both sides.

`ConflictResolver` applies user decisions. Each `ConflictResolution` carries a `ConflictResolutionOption` (`KeepLocal` / `KeepRemote` / `Merge` / etc.) dispatched per entity type. Returns `ApplyResolutionsResult { success_count, total_count, errors }`.

UI front end: `ConflictResolutionDialog` (planned).

### 9.7 Filesystem watch and scheduling

- `src/sync/watcher.rs` — `notify::Watcher` watches `sync.path`. On modify/create event for the canonical sync filename, schedule a `download` (with the user's password held in memory or pulled from the keyring).
- `src/sync/scheduler.rs` — interval-based `tokio::time::interval` task per `sync.frequency` (`15m`/`1h`/`6h`/`24h`). Respects `sync.enabled`.
- **Debounced sync-on-change:** when local data changes (DAO write), schedule a `OneTimeUpload` 30 seconds out; subsequent changes within the window cancel and reschedule. Burst edits → one upload.

### 9.8 Metadata

`src/sync/metadata.rs` owns the device identity, in `~/.config/tabssh/sync_metadata.toml`:
- `device_id` — UUID generated on first run
- `device_name` — derived from `hostname` + os name (overridable in settings)
- `sync_version` — monotonically incremented per upload
- `last_sync_time`, `last_successful_sync` — timestamps

### 9.9 UI

`SyncSettingsScreen` (planned) provides folder picker (`rfd::FileDialog::pick_folder`), password setup, manual upload/download/clear actions, frequency selector, per-entity toggles, on-change debounce toggle.

---

## 10. Backup and restore

`src/backup/` (planned, Phase 3.1) produces a structured export — independent of the sync system. Format byte-compatible with android `BackupManager` / `BackupExporter`.

### 10.1 Constants

```rust
const BACKUP_VERSION: u32 = 3;
const WIRE_VERSION: u32 = 2;
```

### 10.2 Filename convention

`tabssh_backup_yyyyMMdd_HHmmss.tabssh` — the `.tabssh` extension is mandatory for format identification. Example: `tabssh_backup_20260612_143022.tabssh`.

### 10.3 Archive contents

The backup is a ZIP archive (unencrypted or AES-256-GCM-wrapped — see §10.5). ZIP entries mirror android `BackupExporter` which writes one JSON file per entity type:

```
metadata.json
connections.json
keys.json
identities.json
snippets.json
macros.json
workspaces.json
groups.json
themes.json
vnc_hosts.json
vnc_identities.json
hypervisor_profiles.json
hypervisor_accounts.json
cloud_accounts.json
monitor_slots.json
audit_log.json
preferences.json
secrets.json
```

`metadata.json` carries `BACKUP_VERSION`, `created_at`, `app_version`, `os_name`, `item_counts`.

`secrets.json` contains only references (key aliases), never plaintext credentials.

### 10.4 API

```rust
create_backup(output_path, include_sensitive, encrypt, password) -> BackupResult
restore_backup(input_path, password) -> RestoreResult
validate_backup(path) -> ValidationResult
```

### 10.5 Encryption wrapper

When `encrypt = true`, the ZIP bytes are wrapped in `AES-256-GCM(key, IV, zip_bytes)` with the same KDF parameters as sync (PBKDF2-HMAC-SHA256, 100k iterations). The outer file still ends in `.tabssh`.

---

## 11. Hypervisor integration

Module: `src/hypervisor/` (planned, Phase 2.5). Each platform has its own client; a unified `HypervisorConsoleManager` routes serial-console requests to the right WebSocket implementation.

### 11.1 Proxmox VE

`src/hypervisor/proxmox/api.rs` — REST.

| Operation | Endpoint |
|---|---|
| Auth | `POST /api2/json/access/ticket` (form-encoded; returns `authTicket` + `CSRFPreventionToken`) |
| List nodes | `GET /nodes` |
| List VMs | `GET /cluster/resources?type=vm` |
| Power | `POST /nodes/{node}/qemu/{vmid}/status/{start,stop,shutdown,reboot,reset}` |
| Termproxy | `POST /nodes/{node}/qemu/{vmid}/termproxy` → ticket + WS URL `wss://host:8006/api2/json/.../vncwebsocket?port=…&vncticket=…` |
| VNC proxy | `POST /nodes/{node}/qemu/{vmid}/vncproxy` |

Realm format: `user@pam` / `user@pve`. SSL bypass is opt-in.

### 11.2 XCP-ng / XenServer

`src/hypervisor/xcpng/api.rs` — XML-RPC.

- Auth: `session.login_with_password(username, password)`. Detect HTML responses (Xen Orchestra fronting) and fall back to the XO client.
- Operations: `VM.get_all`, `VM.start`, `VM.{clean,hard}_shutdown`, `VM.{clean,hard}_reboot`, `VM.get_consoles`, `console.get_location`.
- Console: WebSocket at the location returned by `console.get_location`, fallback `wss://host/console?ref={consoleRef}&session_id={sessionId}`.

### 11.3 Xen Orchestra

`src/hypervisor/xen_orchestra/api.rs` — REST + WebSocket.

- Auto-detect API version: probe `/rest/v6` → `/rest/v5` → `/rest/v0`.
- Auth: `POST /rest/vX/users/me/authentication_tokens` with HTTP Basic; resulting token sent as `authenticationToken` cookie / `Authorization: Bearer …`.
- ~26 REST methods covering VMs (list/get/start/stop/restart/reset/suspend/resume), snapshots (list/create/delete/revert), backup jobs (list/run/runs/details), pools, hosts.
- **WebSocket** at `wss://host:port/api/` carries real-time events: `vm.{started,stopped,suspended,restarted,created,deleted}`, `snapshot.{created,deleted}`, `backup.completed`. Implement `EventListener` with callbacks `on_vm_state_changed`, `on_vm_created/deleted`, `on_snapshot_created/deleted`, `on_backup_completed`, `on_connection_state_changed`, `on_error`. UI shows green "Live Updates" indicator when connected.
- Console: `get_console_websocket_url(vm_id)` queries `/rest/v0/vms/{vm_id}/console`, falls back to `wss://host:port/api/console/{vm_id}`.

### 11.4 VMware

`src/hypervisor/vmware/api.rs` — REST.

- Auth: `POST /api/session` with HTTP Basic → session ID cookie.
- Operations: `GET /api/vcenter/vm`, `POST /api/vcenter/vm/{id}/power?action={start,stop,reset}`.
- Detect vCenter vs standalone ESXi by probing `/api/vcenter/datacenter`.
- Console support: not yet implemented.

### 11.5 OCI (Oracle Cloud Infrastructure)

`src/hypervisor/oci/api.rs` — REST with cavage HTTP signatures.

**Authentication (cavage signing, matches android `OciApiClient`):**
- HTTP method: request signing uses RSA-SHA256 over the `(request-target)`, `host`, `date` headers (plus `content-type`, `content-length`, `x-content-sha256` for POST/PUT).
- Fingerprint format: MD5 hash of the SubjectPublicKeyInfo DER bytes, formatted as colon-separated lowercase hex pairs (e.g. `ab:cd:ef:01:23:45:67:89:ab:cd:ef:01:23:45:67:89`). This matches OCI's console fingerprint display.
- Key storage in OS keyring: `oci_private_key_{accountId}` for the private key, `oci_passphrase_{accountId}` for the optional passphrase.

**Supported regions:** 34 OCI regions (all current OCI commercial regions; region list embedded in `src/hypervisor/oci/regions.rs`).

**Operations:**
- List instances: `GET https://iaas.{region}.oraclecloud.com/20160918/instances?compartmentId={compartmentId}`
- Start/stop/reset: `POST /20160918/instances/{instanceId}?action={START,STOP,SOFTRESET,RESET}`
- Get VNIC (for public IP): `GET /20160918/vnicAttachments?compartmentId={compartmentId}&instanceId={instanceId}`

**Console connection:** OCI serial console via SSH tunnel (separate SSH key pair generated per session).

### 11.6 Libvirt

`src/hypervisor/libvirt/client.rs` — SSH-tunnelled virsh commands.

- **Transport:** reuse the existing SSH connection to the hypervisor host. Execute `virsh` commands via `request_exec`.
- **VM list:** `virsh list --all` → parse tabular output.
- **Power ops:** `virsh start {domain}`, `virsh shutdown {domain}`, `virsh destroy {domain}`, `virsh reboot {domain}`.
- **Console:** `virsh console {domain}` over SSH exec channel → pipe into `TerminalBuffer`.
- **VNC display:** `virsh vncdisplay {domain}` → returns `host:displayNum`; connect via direct VNC (§13).

### 11.7 `HypervisorConsoleManager` and console WebSocket frame formats

`src/hypervisor/console.rs`:
- `connect_proxmox_console`, `connect_xcpng_console`, `connect_xen_orchestra_console`, `connect_vmware_console`, `connect_oci_console` — each returns a `ConsoleConnection` exposing `AsyncRead` + `AsyncWrite`.
- Frame format per protocol:
  - **`ProxmoxTerm`:** text frames `"0:LENGTH:MSG"` (data) and `"1:COLS:ROWS:"` (resize)
  - **`ProxmoxVnc`:** raw RFB bytes (requires VNC viewer — §13)
  - **`Xcpng`, `Xo`, `Vmware`:** pass-through
- `wire_to_terminal(connection, terminal: &TerminalEmulator)` connects the streams to the terminal.

### 11.8 UI

`HypervisorListScreen` shows registered hypervisors; `HypervisorEditDialog` has dynamic field visibility (Proxmox shows realm, XCP-ng/VMware show API-type dropdown, OCI shows compartment ID + region picker), default ports (Proxmox 8006, XCP-ng/VMware 443, OCI uses HTTPS), "Import from SSH connection" pre-fill, `test_connection()` validation.

---

## 12. Cloud host import

Module: `src/cloud/` (planned, Phase 2.6). Fetch host lists from cloud providers and upsert them as `ConnectionProfile` rows.

### 12.1 AWS EC2

`src/cloud/aws.rs` — AWS SigV4 signing.

**Token format** stored in OS keyring (one entry per `CloudAccount`): `"AKID:SECRET:REGION"` (colon-delimited).

**Default SSH user:** `"ec2-user"` (used when no user is configured).

**SigV4 canonical request pipeline:**
1. Build canonical request: `METHOD\nCanonicalURI\nCanonicalQueryString\nCanonicalHeaders\nSignedHeaders\nHexHash(body)`.
2. String to sign: `AWS4-HMAC-SHA256\n{datetime}\n{date}/{region}/ec2/aws4_request\n{hash_of_canonical_request}`.
3. Signing key: `HMAC-SHA256(HMAC-SHA256(HMAC-SHA256(HMAC-SHA256("AWS4" || secret_key, date), region), "ec2"), "aws4_request")`.
4. Signature: `HMAC-SHA256(signing_key, string_to_sign)` → lowercase hex.
5. Authorization header: `AWS4-HMAC-SHA256 Credential={AKID}/{date}/{region}/ec2/aws4_request, SignedHeaders={list}, Signature={sig}`.

All HMAC operations use `hmac::Hmac::<sha2::Sha256>`.

**Endpoint:** `https://ec2.{region}.amazonaws.com/?Action=DescribeInstances&Version=2016-11-15`.

Parse the XML response for `instanceId`, `publicIpAddress` / `privateIpAddress`, `keyName`, tags (`Name`).

### 12.2 GCP Compute Engine

`src/cloud/gcp.rs` — JWT→OAuth2 flow.

**Credential format:** service account JSON containing `client_email`, `private_key` (PKCS#8 PEM — `BEGIN PRIVATE KEY` header), `project_id`.

**Default SSH user:** `""` (empty — GCP injects the user via OS Login or project metadata).

**JWT construction:**
- Header: `{"alg":"RS256","typ":"JWT"}`
- Payload: `{"iss": client_email, "scope": "https://www.googleapis.com/auth/compute.readonly", "aud": "https://oauth2.googleapis.com/token", "exp": now+3600, "iat": now}`
- Sign with RS256 using the service account private key (`jsonwebtoken` crate).

**Token endpoint:** `POST https://oauth2.googleapis.com/token` with `grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={jwt}`.

**List instances:** `GET https://compute.googleapis.com/compute/v1/projects/{project}/aggregated/instances` with `Authorization: Bearer {token}`.

### 12.3 Azure

`src/cloud/azure.rs` — OAuth2 client_credentials flow.

**Token format** stored in keyring: `"TENANT:CLIENT_ID:CLIENT_SECRET:SUBSCRIPTION_ID"`.

**Default SSH user:** `"azureuser"`.

**Token acquisition:** `POST https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token` with `grant_type=client_credentials&client_id={id}&client_secret={secret}&scope=https://management.azure.com/.default`.

**List VMs:** `GET https://management.azure.com/subscriptions/{sub}/providers/Microsoft.Compute/virtualMachines?api-version=2023-09-01` with `Authorization: Bearer {token}`.

Parse `properties.networkProfile.networkInterfaces` → resolve public IPs via `GET …/publicIPAddresses/…`.

### 12.4 DigitalOcean

`src/cloud/digitalocean.rs` — Bearer token, cursor pagination.

**Token format:** raw Bearer token stored in keyring.

**Pagination:** follow `links.pages.next` URL in each response until absent.

**List droplets:** `GET https://api.digitalocean.com/v2/droplets?page=1&per_page=200`.

Parse `networks.v4[].ip_address` (prefer `type: "public"`), `name`, `id`.

### 12.5 Hetzner

`src/cloud/hetzner.rs` — Bearer token, numeric pagination.

**Token format:** raw Bearer token stored in keyring.

**Pagination:** follow `meta.pagination.next_page` (integer, null when on last page).

**List servers:** `GET https://api.hetzner.cloud/v1/servers?page=1&per_page=50`.

Parse `public_net.ipv4.ip`, `name`, `id`, `server_type.name`.

---

## 13. VNC hosts (direct VNC)

Module: `src/vnc/` (planned, Phase 2.6). Connect directly to VNC servers without SSH tunnelling.

### 13.1 Entities

- `VncHost` — host, port (default 5900), display name, optional `identity_id`, `color_tag`, sync metadata.
- `VncIdentity` — display name, optional username, encrypted password stored in OS keyring.

### 13.2 RFB 3.8 client

`src/vnc/client.rs` — RFB protocol constants (verified against android `RfbConstants.kt`):

```rust
const VERSION_MESSAGE: &str = "RFB 003.008\n";

const SECURITY_TYPE_NONE: u8 = 1;
const SECURITY_TYPE_VNC_AUTH: u8 = 2;

const ENCODING_RAW: i32 = 0;
const ENCODING_COPY_RECT: i32 = 1;
const ENCODING_RRE: i32 = 2;
const ENCODING_HEXTILE: i32 = 5;
const ENCODING_ZLIB: i32 = 6;
const ENCODING_TIGHT: i32 = 7;
const ENCODING_ZRLE: i32 = 16;
const ENCODING_CURSOR: i32 = -239;
const ENCODING_DESKTOP_SIZE: i32 = -223;
```

Handshake:
1. Server sends 12-byte version string; client replies with `VERSION_MESSAGE`.
2. Server sends security-type list; client selects `SECURITY_TYPE_NONE` (1) or `SECURITY_TYPE_VNC_AUTH` (2).
3. For VNC auth: server sends 16-byte challenge; client DES-encrypts it with the password (reversed bit-order key per RFB spec) and sends 16-byte response.
4. Server sends 4-byte security-result (0 = OK).
5. ClientInit: send `shared-flag` (1 = shared).
6. ServerInit: receive framebuffer width, height, pixel format, name.

### 13.3 Framebuffer rendering

`src/vnc/renderer.rs` renders framebuffer updates to an `egui::TextureHandle`. Supported encodings: Raw, CopyRect (deferred). Hextile/ZRLE/Tight are planned. `ENCODING_DESKTOP_SIZE` triggers a resize of the texture. `ENCODING_CURSOR` updates a local cursor sprite.

### 13.4 SSH-tunnelled VNC

`VncHost` can optionally set `tunnel_via_connection_id` to route the VNC TCP stream through a `direct-tcpip` channel on an active SSH session. This is how Libvirt VNC consoles are accessed (§11.6).

---

## 14. Performance monitoring

Module: `src/monitoring/metrics.rs` (planned, Phase 2.7). Mirror of android `MetricsCollector`.

### 14.1 Data model

`MetricsHistory`: ring buffer of **60 data points** sampled at **5-second intervals** (5 minutes of history).

```rust
struct MetricPoint {
    timestamp: i64,
    cpu_percent: f32,
    memory_used_mb: u64,
    memory_total_mb: u64,
    disk_used_gb: u64,
    disk_total_gb: u64,
    net_rx_bytes: u64,
    net_tx_bytes: u64,
    load_1: f32,
    load_5: f32,
    load_15: f32,
}
```

### 14.2 Collection commands (exact shell strings, must match android)

| Metric | Command |
|---|---|
| CPU ticks | `cat /proc/stat \| head -1` |
| CPU count | `nproc` |
| Memory | `cat /proc/meminfo \| head -20` |
| Disk | `df -BG / \| tail -1` |
| Network | `cat /proc/net/dev \| grep -E 'eth0\|ens\|enp'` |
| Load | `cat /proc/loadavg` |
| Platform | `uname -a` then `cat /etc/os-release` |

Commands are run over the existing SSH exec channel (no separate connection). CPU idle delta is computed between consecutive `/proc/stat` samples to produce a percentage.

### 14.3 Connect paths

`MetricsCollector` is started via one of two paths:

1. **From an open terminal tab:** share the active SSH `Handle<H>`, open a new exec channel for each metric poll without blocking the terminal.
2. **On-demand connection:** when the user opens `PerformanceOverlay` for a connection that is not currently connected, establish a fresh SSH session with the same `ConnectionProfile`, run metrics only (no PTY allocation), disconnect on close.

### 14.4 UI

- `PerformanceOverlay` — floating egui window overlaid on the terminal tab. Shows sparkline charts for CPU, memory, disk, network using `egui_plot`.
- `MultiHostDashboard` — grid of metric cards for multiple `ConnectionProfile`s simultaneously. Each card has the same sparklines plus latency (ping).
- `MonitoringConfigScreen` — enables/disables a `MonitorSlot` for a given connection, sets the check interval.

---

## 15. Background monitoring

Module: `src/monitoring/availability.rs` (planned, Phase 2.7). Mirror of android `HostAvailabilityWorker`.

### 15.1 Constants

```rust
const TCP_TIMEOUT_MS: u64 = 5000;
const MAX_PARALLEL_PROBES: usize = 30;
const CHECK_INTERVAL_MINUTES: u64 = 15;
```

### 15.2 Architecture

`BackgroundMonitor` is a tokio task that wakes every `CHECK_INTERVAL_MINUTES` and probes all enabled `MonitorSlot`s.

For each due slot (`MonitorSlot::is_due(last_checked_at, check_interval_minutes)`):
1. Attempt TCP connect to `host:port` with `TCP_TIMEOUT_MS` timeout.
2. Record result: `is_available = true` if connect succeeds, `false` on timeout or refused.
3. Update `last_checked_at` and `is_available` in the DB.
4. If `notify_on_change` and availability changed since last check → emit a native notification (§19).

Probes run concurrently up to `MAX_PARALLEL_PROBES` via `tokio::task::JoinSet` or a semaphore.

### 15.3 Two-tier model

- **`MonitorSlot`** — lightweight TCP reachability check running in the background every 15 minutes. No SSH auth required.
- **`MetricsCollector`** (§14) — full SSH-authenticated performance data collection, launched on demand from the UI.

These are separate concerns. A host can have a `MonitorSlot` without ever opening a `MetricsCollector`, and vice versa.

---

## 16. Session recording

Module: `src/terminal/recorder.rs` (planned, Phase 2.7). Mirror of android `SessionRecorder`.

### 16.1 File naming

Output file: `session_{sanitized}_{yyyyMMdd_HHmmss}.log`

`sanitized` = connection name with all characters not matching `[a-zA-Z0-9_-]` replaced by `_`.

Example: a connection named "prod-web-01" at 2026-06-12 14:30:22 → `session_prod-web-01_20260612_143022.log`.

Files are written to `~/.local/share/tabssh/transcripts/` (or platform equivalent).

### 16.2 File format

```
TABSSH_RECORDING v1
connection: {connection_name}
host: {host}:{port}
user: {username}
started: {ISO-8601 timestamp}
---
{raw terminal byte stream with timing markers}
```

Timing markers use the `scriptreplay` format: one timing line (`{seconds_delta} {byte_count}\n`) before each data chunk, allowing frame-accurate replay.

### 16.3 `SessionRecorder` struct

```rust
struct SessionRecorder {
    file: tokio::fs::File,
    started_at: std::time::Instant,
    last_write: std::time::Instant,
}
```

Methods:
- `start(connection: &ConnectionProfile) -> Result<SessionRecorder>` — creates file, writes header.
- `write_bytes(&mut self, data: &[u8]) -> Result<()>` — writes timing line + data chunk.
- `stop(&mut self) -> Result<()>` — flushes, closes file.

The recorder hooks into `ActiveSession` between the SSH read loop and `TerminalEmulator::process`. Only active when `profile.record_session = true`.

### 16.4 `TranscriptViewerScreen`

Reads the timing log and replays bytes at the original speed (or at a configurable multiplier). Uses the same `TerminalEmulator` + `TerminalBuffer` + `TerminalRenderer` as live sessions — identical appearance.

---

## 17. Audit log

Module: `src/audit/` (planned, Phase 3.5). Mirror of android `AuditLogManager`.

### 17.1 Event types

**23 event types** (matches android):

```
CONNECTION_OPENED, CONNECTION_CLOSED, CONNECTION_FAILED,
COMMAND_EXECUTED, COMMAND_OUTPUT, COMMAND_FAILED,
FILE_UPLOADED, FILE_DOWNLOADED, FILE_DELETED, FILE_RENAMED,
PORT_FORWARD_STARTED, PORT_FORWARD_STOPPED,
KEY_ADDED, KEY_REMOVED, KEY_USED,
SETTINGS_CHANGED, BACKUP_CREATED, BACKUP_RESTORED,
SYNC_STARTED, SYNC_COMPLETED, SYNC_FAILED,
APP_STARTED, APP_STOPPED
```

### 17.2 `AuditLogManager`

`src/audit/manager.rs`:
- **Enabled flag:** `audit_log_enabled` preference key, default **OFF** (matches android default). Loading the manager is cheap even when disabled — all `log_event` calls are no-ops when disabled.
- **Max size:** `DEFAULT_MAX_SIZE_MB = 100`. When the `audit_log` table exceeds this limit, delete oldest rows.
- **Max age:** `DEFAULT_MAX_AGE_DAYS = 30`. Rows older than this are deleted on each cleanup sweep.
- **Cleanup:** run on startup and every 24 hours.

### 17.3 Syslog forwarding (optional)

When `audit.syslog_host` is configured, each event is also forwarded via UDP to the syslog server using **RFC 5424** format.

Syslog UDP send: `UdpSocket::bind("0.0.0.0:0")` → `send_to(formatted_message, syslog_host:port)`.

RFC 5424 message format: `<PRI>VERSION TIMESTAMP HOSTNAME APP-NAME PROCID MSGID STRUCTURED-DATA MSG`.

### 17.4 DB schema

`audit_log` table:
```sql
CREATE TABLE audit_log (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    connection_id TEXT,
    host TEXT,
    username TEXT,
    command TEXT,
    output TEXT,
    exit_code INTEGER,
    file_path TEXT,
    details TEXT,
    timestamp INTEGER NOT NULL
);
```

Index on `timestamp` for range queries and cleanup sweeps.

---

## 18. UI, theming, accessibility, i18n

### 18.1 Settings hierarchy

`SettingsScreen` is a tabbed pane:

| Tab | Scope |
|---|---|
| General | Theme (light/dark/system), language, behaviour, notifications |
| Terminal | Terminal theme, font, cursor, scrollback, bell, URL detection |
| SSH | Default user/port, timeouts, compression, Mosh |
| Security | Lock, biometric, host-key strict mode, port-knock default, password storage level |
| Sync | Folder, password, frequency, per-entity toggles, on-change debounce |
| Audit | Command auditing, log retention, syslog forwarding |
| Logging | Debug / host / error / audit logging |

Mobile's `Tasker` settings tab does not apply on desktop.

### 18.2 Themes

`src/config/themes.rs` mirrors mobile's structure:

- `Theme { id, name, author, is_dark, is_built_in, terminal_fg, terminal_bg, cursor, selection, highlight, ansi_palette: [Color; 16], ui_overrides, status_bar_tint, navigation_bar_tint }`
- `BuiltInThemes` — **23** built-ins (port mobile's `BuiltInThemes.kt` JSON definitions verbatim into `assets/themes/*.json`):
  System Default, Dark, Light, Dracula, Solarized Dark, Solarized Light, Nord, One Dark, Monokai, Gruvbox Dark, Gruvbox Light, Tomorrow Night, GitHub Light, Atom One Dark, Material Dark, Tokyo Night, Tokyo Light, Catppuccin Mocha, Rose Pine, Everforest, Kanagawa, Night Owl, Cobalt2.
- `ThemeManager` — caching, `tokio::sync::watch` for the current theme, switching, listeners.
- `ThemeParser` — serde-JSON, VS Code theme format compatibility, hex conversions.
- `ThemeValidator` — WCAG 2.1 AA/AAA contrast ratio checks (4.5:1 minimum), colour-blindness validation, auto-fix recommendations.

### 18.3 Accessibility

- High-contrast palette toggle for users with `accessibility.high_contrast = true`.
- Keyboard navigation: Tab / arrow / Enter / Esc throughout the UI; Ctrl+T / Ctrl+W / Ctrl+Tab / Ctrl+1–9 / Ctrl+, / F11 standard shortcuts.
- Screen-reader hints (egui has limited support; full screen-reader integration is Phase 4 polish).

### 18.4 Internationalization (planned, Phase 4)

Mirror mobile's catalogues:

| Locale | File | Strings (target) |
|---|---|---|
| English (default) | `assets/i18n/en.ftl` | ~216 |
| Spanish | `assets/i18n/es.ftl` | ~156 |
| French | `assets/i18n/fr.ftl` | ~156 |
| German | `assets/i18n/de.ftl` | ~156 |

`fluent-rs` is the recommended runtime; `gettext` is the alternative.

### 18.5 Layouts and dialogs

There are no `.xml` layout files — egui is procedural. The dialog list (when fully built) mirrors mobile, named by purpose: `AuthenticationDialog`, `QuickConnectDialog`, `HostKeyChangedDialog`, `NewHostKeyDialog`, `SshConnectionErrorDialog` (with the centralised `Copy` button per Issue #167), `SyncPasswordDialog`, `ConflictResolutionDialog`, `AddHypervisorDialog`, `LocalForwardDialog`, `RemoteForwardDialog`, `DynamicForwardDialog`, `EditIdentityDialog`, `EditGroupDialog`, `EditSnippetDialog`, `BulkEditDialog`, `ChmodDialog`, `PairingDialog`.

### 18.6 Adapters / list widgets

egui equivalents of mobile's `RecyclerView` adapters: `connection_list_widget`, `grouped_connection_list_widget`, `tab_pager_widget`, `identity_list_widget`, `file_list_widget`, `transfer_list_widget`, `tunnel_list_widget`, `snippet_list_widget`, `macro_list_widget`, `audit_log_widget`, `transcript_list_widget`, `hypervisor_list_widget`, `vnc_host_list_widget`, `cloud_account_list_widget`. Each is a `fn render(&mut self, ui: &mut egui::Ui, items: &[T])` that returns user actions.

---

## 19. Notifications, system tray, CLI mode

### 19.1 Native notifications

`src/notifications/` (planned). Cross-platform via `notify-rust` (Linux: D-Bus / freedesktop, macOS: NSUserNotification, Windows: WinRT). Categories mirror mobile's notification channels:

| Category | Use |
|---|---|
| `connection` | connect / disconnect / error events |
| `file_transfer` | SFTP progress (ongoing), completion summary |
| `monitoring` | host availability state change |
| `error` | actionable errors |

User-controllable via Settings → General. Mobile's persistent foreground-service notification has no desktop equivalent; the **system tray** does that job.

### 19.2 System tray

`src/notifications/tray.rs` (planned, Phase 3.3) uses `tray-icon` crate. Menu:

```
TabSSH
├── Open window
├── Connect to ▶ (recent / favourites submenu)
├── Active connections (N)
├── ───
├── Settings…
├── Pair phone…
├── About
└── Quit
```

Auto-launch on login: `.desktop` autostart file on Linux, LaunchAgent plist on macOS, Startup folder shortcut on Windows. User-toggleable in Settings → General.

### 19.3 CLI mode

`src/bin/` already exists. Plan: `tabssh user@host` (or with port: `tabssh user@host:port`) invocations from the shell open a new tab in the running GUI instance (via a Unix socket / Windows named pipe in `~/.config/tabssh/control.sock`). If no instance is running, launch one.

`tabssh --connect <profile-name>` opens by saved-profile name. `tabssh --version`, `--help` standard.

---

## 20. Build and release infrastructure

### 20.1 Docker

`docker/Dockerfile` — base `rustlang/rust:nightly-bookworm`. Adds GUI dev libs + musl tooling. Build the image with `make docker` (or `docker build -t tabssh-builder -f docker/Dockerfile .`).

### 20.2 Makefile

| Target | Effect | Output |
|---|---|---|
| `help` | Print targets | stdout |
| `build` | Docker `cargo build --release` | `./binaries/tabssh-{os}-{arch}` |
| `release` | Docker release builds for every target | `./releases/tabssh-{os}-{arch}` + `checksums.txt` + source archive |
| `test` | Docker `cargo test` | stdout |
| `docker` | Build the builder image (multi-arch via buildx) | `tabssh-builder:latest`, `:{version}`, `:{commit}`, `:{YYMM}` |
| `clean` | Remove `binaries/`, `releases/`, `target/` | — |

The Docker run wrapper bind-mounts the repo to `/workspace`, sets `CARGO_TARGET_DIR=/workspace/target`, uses `--network=host` for crates.io fetches.

### 20.3 GitHub Actions

`.github/workflows/`:

| Workflow | Trigger | Job |
|---|---|---|
| `ci.yml` | push / PR | `make docker` then `cargo check` + `cargo test` + `cargo clippy -- -D warnings` + `cargo fmt --check` + `cargo audit` |
| `release.yml` | tag `v*` | Build all 11 binary variants, generate SHA-256 checksums, generate release notes, create GitHub Release |
| `development.yml` | push to `develop` | Debug build; upload artifact |

Desktop signs via OS-native code-signing (Apple developer certificate for `.dmg`, Authenticode for `.msi`) — out of scope for v0.1.0.

### 20.4 Unified binary naming schema (shared with mobile)

Mobile renames APKs to `tabssh-android-{arm64|arm|amd64|x86|universal}.apk`. Desktop names binaries `tabssh-{os}-{arch}` where `{arch}` is one of `amd64` (= x86_64) / `arm64` (= aarch64). This is the same simplified-tag schema across the two repos so a release page can list both side-by-side.

### 20.5 Cross-compilation targets

```
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-unknown-freebsd
rustup target add aarch64-unknown-freebsd
rustup target add x86_64-unknown-openbsd
rustup target add x86_64-unknown-netbsd
```

(BSD targets require `cross` or appropriate sysroots — see `docs/` once written.)

### 20.6 Scripts

`scripts/`:
- `scripts/build/build-all.sh` — call `cargo build` for every supported target
- `scripts/release/release.sh` — build, checksum, source-archive, release-notes
- `scripts/dev-shell.sh` — interactive Docker shell for manual cargo invocations

---

## 21. Module map

| Module (`src/...`) | Responsibility |
|---|---|
| `main.rs` | Entry point — runtime + window |
| `lib.rs` | Crate-level re-exports |
| `app.rs` | Top-level egui `App` impl + manager wiring |
| `ssh::connection` | `SshConnection`, `SshClientHandler`, host-key verification |
| `ssh::session_manager` | `SessionManager`, listener registry |
| `ssh::active_session` | `ActiveSession` async task + mpsc command/event channels |
| `ssh::auth` | Auth-method enums + prompt flows |
| `ssh::forwarding` | Local / remote / dynamic port forwarding |
| `ssh::portknock` | `PortKnocker` — TCP/UDP knock sequence |
| `ssh::x11_proxy` | `X11Proxy` — X11 forwarding bridge |
| `ssh::mosh` | `MoshHandoff` — Mosh bootstrap over SSH |
| `ssh::mosh_client` | Pure-Rust Mosh UDP client |
| `ssh::config_parser` | `~/.ssh/config` reader |
| `ssh::bulk_import` | `BulkImportParser` — CSV/JSON/PuTTY/Terraform |
| `sftp::client` | `SftpClient` over a russh channel |
| `sftp::browser` | `SftpBrowser` UI state |
| `sftp::operations` | High-level transfer operations |
| `sftp::transfer` | `TransferManager` queue |
| `terminal::parser` | `vte`-driven escape parser |
| `terminal::emulator` | `TerminalEmulator` (parser + buffer wrapper) |
| `terminal::buffer` | Grid + scrollback + cursor + alternate screen |
| `terminal::cell` | `Cell` + `Attrs` |
| `terminal::renderer` | egui canvas painter |
| `terminal::recorder` | `SessionRecorder` — timing log writer |
| `storage::database` | rusqlite handle + schema bootstrapping |
| `storage::sessions` | `tab_sessions` access |
| `storage::settings` | TOML-backed settings |
| `crypto::keychain` | OS keychain wrapper (`KeyringStore`) + password-storage levels |
| `crypto::keys` | `KeyParser` + `KeyGenerator` via `ssh-key` |
| `crypto::encryption` | `SyncEncryptor` — AES-256-GCM + PBKDF2 |
| `config::themes` | `Theme`, `ThemeManager`, `BuiltInThemes` |
| `platform::linux` / `macos` / `windows` / `bsd` | Per-OS adapters |
| `ui::main_window` | Top-level chrome |
| `ui::tab_manager` / `tab` | Tab state |
| `ui::keyboard` | Key-event → ANSI translation |
| `ui::search` | Find-in-scrollback |
| `ui::components` / `dialogs` / `screens` | Reusable widgets and screens |
| `utils::logging` / `errors` | Cross-cutting utilities |

Planned modules (all under `src/`):
- `pairing/{payload,encrypt,qr}` — QR pairing sender (Phase 2.8)
- `sync/{encryption,collector,applier,merge,watcher,scheduler,metadata}` — cloud sync (Phase 2.9)
- `hypervisor/{proxmox,xcpng,xen_orchestra,vmware,oci,libvirt,console}` — VM management (Phase 2.5)
- `cloud/{aws,gcp,azure,digitalocean,hetzner}` — cloud host import (Phase 2.6)
- `vnc/{client,renderer}` — direct VNC / RFB 3.8 (Phase 2.6)
- `monitoring/{metrics,availability}` — performance + background monitoring (Phase 2.7)
- `audit/{manager}` — audit log (Phase 3.5)
- `backup/{exporter,importer,validator}` — encrypted backup (Phase 3.1)
- `notifications/{native,tray}` — system integration (Phase 3.3)

---

## 22. Known stubs and limitations

These exist in source or design but are **not** wired into a working user-facing flow. Treat them as roadmap items — see `TODO.AI.md` for ordered work.

- **Build is broken.** 50 errors as of 2026-05-01. `src/sftp/client.rs` (russh-sftp 2.1 API drift, 42 errors) is the dominant blocker; `src/ssh/forwarding.rs` (Handle clone + stream/channel ownership, 5 errors), `src/sftp/browser.rs` (FileType not Copy, 3 errors), and `src/ssh/connection.rs` / `active_session.rs` (russh 0.40 `check_server_key` shape, 3 errors) round out the ssh path. See `TODO.AI.md` Phase 0 for the breakdown.
- **SFTP code exists but doesn't compile.** Once Phase 0 is done, the dual-pane browser + transfer manager are ~80% in place.
- **Port forwarding is structurally complete but has the same russh 0.40 issues.** The rewrite uses `tokio::io::copy_bidirectional` and `Arc<Handle<H>>` — see uncommitted local edits or `TODO.AI.md` Phase 0.
- **Keyboard-interactive auth, agent forwarding, OpenSSH user certificates** — not yet wired.
- **Universal SSH key parser, in-app key generation** — placeholders; `ssh-key` crate not yet a dependency.
- **`~/.ssh/config` direct read** — `config_parser.rs` exists but doesn't yet read in-place from the user's home; that's the desktop-only advantage to land.
- **23 built-in themes** — only 1 default theme defined; mobile's `BuiltInThemes.kt` not ported into `assets/themes/*.json`.
- **All 50-key `Settings` table** — present at the type level but most are stubs in the UI.
- **System tray, auto-launch, CLI mode** — not started.
- **Hypervisor management** — entire `hypervisor/` subtree not started.
- **Cloud host import** — entire `cloud/` subtree not started; spec documented in §12.
- **VNC direct client** — entire `vnc/` subtree not started; spec documented in §13.
- **Performance monitoring** — entire `monitoring/metrics.rs` not started; spec documented in §14.
- **Background monitoring** — entire `monitoring/availability.rs` not started; spec documented in §15.
- **Session recording** — entire `terminal/recorder.rs` not started; spec documented in §16.
- **Audit log** — entire `audit/` subtree not started; spec documented in §17.
- **Cloud sync** — entire `sync/` subtree not started; wire format documented in §9.
- **QR pairing** — entire `pairing/` subtree not started; spec documented in §24.
- **Mosh** — TODO.AI.md Phase 4. Pure-Rust client preferred over wrapping `mosh-client`.
- **X11 forwarding** — TODO.AI.md Phase 4. `x11rb` crate for display-server connect; `X11Proxy` skeleton exists in §5.9.
- **No tests except `tests/ssh_config_test.rs`** — 0% coverage in practice.
- **No installers / packages** — distribution work not started.
- **Crash reporter** — `panic::set_hook` not yet wired.

---

## 23. Editing guidelines for AI agents

When modifying this codebase, follow these rules:

1. **Don't reimplement what's there.** `russh` owns the SSH protocol; `vte` + `alacritty_terminal` own escape parsing; `rusqlite` owns persistence; `keyring` owns OS-keychain transport; `notify` owns filesystem events; `reqwest` owns HTTP client; `hmac`/`sha2` own SigV4 building blocks. New features should compose these — not replace them.
2. **Database changes must ship a migration.** Add a numbered SQL file in `src/storage/migrations/`, bump the schema version, never destructive-drop. The migration file is checked into the repo and tested against an empty DB and the previous-version DB.
3. **Sync surface is opinionated.** Anything user-visible and persisted that is not in `SyncDataCollector` won't sync. If you add a new entity, decide whether to sync it and update `SyncDataCollector` / `SyncDataApplier` / the wire-format coverage matrix in §9.5. Cross-platform sync interop with mobile is a hard requirement, not a nice-to-have. The sync package has exactly 10 entity-type fields — adding an 11th requires a mobile-side change first.
4. **Crypto stays at the boundary.** Don't add ad-hoc password storage — use `KeyringStore`. Don't add ad-hoc key parsing — use the `ssh-key` crate via `crypto::keys`. Don't add custom AES code — use `crypto::encryption` (which wraps `aes-gcm`). Don't add custom KDFs — use `crypto::encryption` for PBKDF2 (sync/backup) and `crypto::pairing` for Argon2id (QR pairing).
5. **Composition over inheritance.** Rust has no inheritance; that's the point. New screens are structs with `render(&mut self, ui: &mut egui::Ui)` impls. Share behaviour via trait impls or composition, never via "extends".
6. **Use the existing notification categories.** Don't create new ones for one-off events. Four categories: `connection`, `file_transfer`, `monitoring`, `error`.
7. **Keep MSRV at 1.75.** New dependencies must respect it. Nightly features are allowed only inside `#[cfg(nightly)]` and only when there's no stable equivalent; the release build path uses nightly base image but with `--edition 2021` features only.
8. **Reproducible builds.** Don't introduce non-deterministic generated code. Don't add network-fetching `build.rs` scripts (other than for embedding the git commit ID). `SOURCE_DATE_EPOCH` should be honoured by the Docker builder.
9. **Prefer `tokio::sync` primitives.** `tokio::sync::watch` for "current value with notification", `tokio::sync::mpsc` for command queues, `tokio::sync::RwLock` for shared mutable state. Don't introduce alternative async runtimes (async-std, smol). Cross-thread egui updates use `egui::Context::request_repaint()`.
10. **When you change architecture, update this file.** When you add a target / script / policy / build instruction, update `TODO.AI.md`.
11. **Mobile is the protocol reference.** When designing a sync entity, QR pairing payload, backup format, port-knock JSON, or cloud account token format, the android sibling's existing implementation is the spec. Round-trip with the mobile codebase via test vectors before marking the feature done.
12. **Audit log is opt-in.** Do not log events if `audit_log_enabled` is false. The `AuditLogManager::log_event` method must be a zero-cost no-op when disabled.
13. **Cloud tokens never touch the DB.** `CloudAccount` rows store metadata only (`provider`, `enabled`, `last_refresh_at`, `last_count`). The actual API tokens live in the OS keyring.

---

## 24. QR pairing — desktop sender

**Status:** Mobile side shipped 2026-04-28. Desktop is the **sender**. Wire format is fixed (mobile won't change). Same content lives at `../android/AI.md` §18 — that's the canonical spec; this section covers the desktop-side implementation only.

### 24.1 Goal

Add an existing TabSSH connection from desktop to a phone without retyping. Desktop renders an encrypted QR + 6-digit code; phone scans, enters the code, imports.

Use cases:
- New phone, no existing sync set up.
- "Just got this server working on my laptop, want it on my phone."
- Set up a colleague's phone for shared infra fast.

### 24.2 Flow

1. User opens TabSSH desktop → menu → File → **Pair Phone…**
2. Desktop shows a modal with a checklist of current connection profiles. User picks which to send.
3. On confirm, desktop renders a QR + 6-digit code + 60-second countdown.
4. User opens TabSSH on phone → drawer → **Pair from desktop…**
5. Phone scans the QR, prompts for the 6-digit code.
6. Phone shows preview: *"Import N connections from {device_label}?"*.
7. User confirms → connections appear in the phone's list.

### 24.3 Non-goals (v1)

- Bidirectional pairing (mobile → desktop). Direction matters because desktop has filesystem write access for `~/.ssh/authorized_keys`; phones don't.
- Continuous sync. Use the cloud-sync system for that (§9).
- Private key transfer. Public-key fingerprint + comment only.
- Multi-frame animated QR. Single-frame in v1.

### 24.4 Data model

`QrPayload`:
```
{
  version: u8 = 1,
  salt: [u8; 16],
  nonce: [u8; 12],
  ciphertext: bytes,
}
```
CBOR-encoded (`ciborium`), then base64-encoded, then rendered as a QR in byte mode.

`PairingPayload` (encrypted):
```
{
  version: u8 = 1,
  expires_at: u64,                  // unix seconds, ~60s after generation
  device_label: Option<String>,     // e.g. "Alice's Linux desktop"
  connections: [ConnectionProfile], // subset, no secrets
  groups: [Group],                  // optional, only those referenced
  identities: [Identity],           // optional, only those referenced
}
```

`ConnectionProfile` is a slim view: `name`, `host`, `port`, `username`, `protocol`, `auth_type`, optional `ssh_key_public` (OpenSSH-format public-key line) + `ssh_key_fingerprint`, plus cosmetic / behavioural fields (`color_tag`, `group_name`, `identity_name`, `terminal_type`, `compression`, `keep_alive`, `env_vars`, `post_connect_script`, `use_mosh`, `agent_forwarding`, `x11_forwarding`, jump-host config). **No password, no private key.**

### 24.5 Encryption

Desktop generates:
- 6-digit numeric code (cryptographically random — `rand::Rng::gen_range(0..1_000_000)`)
- 16-byte random salt
- 12-byte random nonce

Derive a 32-byte symmetric key using Argon2id (matching android's BouncyCastle params exactly):
```rust
let params = argon2::Params::new(
    64 * 1024,  // ARGON2_MEMORY_KIB = 65536
    3,          // ARGON2_ITERATIONS = 3
    1,          // ARGON2_PARALLELISM = 1
    Some(32),
)?;
let argon2 = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
let mut key = [0u8; 32];
argon2.hash_password_into(code.as_bytes(), &salt, &mut key)?;
```

Encrypt the CBOR-encoded `PairingPayload`:
```rust
let cipher = aes_gcm::Aes256Gcm::new(&key.into());
let ciphertext = cipher.encrypt(&nonce.into(), &payload_bytes[..])?;
```

Build `QrPayload { version: 1, salt, nonce, ciphertext }`, CBOR-encode it, base64-encode that, render with `qrcodegen` at ECC level **L** in **byte mode** at 256×256.

### 24.6 Capacity ceiling

QR Code byte mode at ECC-L: 2,953 bytes max. Typical encrypted-payload sizes:

| Content | Bytes (after AES-GCM + base64) |
|---|---|
| 1 connection, no key | ~250 |
| 1 connection + Ed25519 public key | ~400 |
| 5 connections, no keys | ~700 |
| 5 connections + 5 Ed25519 public keys | ~1,400 |
| 10 connections + RSA-4096 public keys | ~2,800 (close to ceiling) |

**Cap v1 at 10 connections per QR.**

### 24.7 Argon2id parameters justification

`m=64 MiB, t=3, p=1` matches mobile's BouncyCastle setting (`ARGON2_MEMORY_KIB = 64 * 1024`, `ARGON2_ITERATIONS = 3`, `ARGON2_PARALLELISM = 1`):
- Brute-force cost: 1M codes × ~1s/derivation = ~12 days on a single core. With a 60s TTL, the QR is gone before any meaningful fraction can be tried.
- Legitimate cost on mid-range hardware: ~1s. Acceptable for a one-off operation.

### 24.8 UI

`PairingDialog` (`src/ui/pairing_dialog.rs`) — egui state machine:
```
Idle → Selecting → Generating → Active → Expired
```
Active layout:
- 256×256 QR centred
- 6-digit code in large monospace below
- 60s countdown bar
- Per-profile checklist showing what's being sent
- Cancel / Generate-new buttons

### 24.9 Implementation TODO

See `TODO.AI.md` Phase 2.8. Order:
1. Add `Cargo.toml` deps: `qrcodegen`, `ciborium`, `argon2`, `aes-gcm`, `rand`, `base64`.
2. `src/pairing/payload.rs` — `PairingPayload` + slim `ConnectionProfile` + CBOR encoder via `ciborium`.
3. `src/pairing/encrypt.rs` — code/salt/nonce generation, Argon2id, AES-GCM, `QrPayload` CBOR.
4. `src/pairing/qr.rs` — render `QrPayload` (base64) → QR bitmap → egui texture.
5. `src/ui/pairing_dialog.rs` — state machine + countdown + render.
6. Wire menu entry: File → Pair Phone…
7. Round-trip test vectors (commit them — mobile reuses).
8. Run mobile-side `ImportFromQrActivity` against our QRs to verify wire compatibility.

### 24.10 Reference

- `qrcodegen` Rust crate — pure Rust QR encoding
- `ciborium` Rust crate — CBOR codec
- `argon2` Rust crate — RFC 9106 Argon2id
- `aes-gcm` Rust crate — RustCrypto AES-GCM
- ZXing on the mobile side (`com.journeyapps:zxing-android-embedded`) — pure-Java QR scanner
- BouncyCastle `bcprov-jdk18on` on mobile — provides Argon2id
- ISO/IEC 18004:2015 — QR Code spec
- RFC 9106 — Argon2 KDF parameters

---

*End of AI.md. For build commands and day-to-day workflows see the project tracker. For the live work list see `TODO.AI.md`. For mobile-side spec see `../android/AI.md`.*
