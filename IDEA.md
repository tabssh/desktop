## Project description

TabSSH Desktop is a cross-platform SSH/SFTP/VNC client for Windows, Linux, macOS, and BSD systems. Built as a single static Rust binary with no runtime dependencies, it gives developers and sysadmins browser-style tabbed terminal sessions, integrated SFTP, port forwarding, hypervisor management, background host monitoring, and VNC access. It targets feature parity with the TabSSH Android sibling app where desktop constraints allow, adding desktop-native conveniences such as direct `~/.ssh/` access, system tray, and CLI invocation from the shell.

## Project variables

project_name:     desktop
project_org:      tabssh
# FROZEN — set once at first-time setup, never edit
internal_name:    tabssh
# FROZEN — set once at first-time setup, never edit
internal_org:     tabssh
app_name:         TabSSH Desktop
crate_name:       tabssh
official_site:
maintainer_name:  casjay
maintainer_email: casjay@yahoo.com
android_sibling:  ../android
license:          MIT
repo:             https://github.com/tabssh/desktop

## Business logic

**Target users:**
- Developers and sysadmins managing fleets of Linux/BSD servers
- DevOps engineers who need SSH, SFTP, port forwarding, hypervisor consoles, and host monitoring in one tool
- Power users migrating from PuTTY, SecureCRT, or the TabSSH Android app to a desktop client

**Surfaces (PART 2 → "GUI/TUI/CLI Capability Rule"):**
- GUI: yes — egui-based native GUI; primary interactive surface on Linux X11+Wayland, macOS, and Windows
- TUI: no
- CLI: yes — `tabssh user@host` and `tabssh --connect <profile>` for shell invocation

**Outbound network use (PART 9 → "Security-First Design"):**
- SSH/Telnet to user-configured hosts (RFC 4251–4254, RFC 854)
- SFTP/SCP subsystem over the same SSH channel
- Hypervisor REST/XML-RPC/WebSocket APIs (Proxmox, XCP-ng, Xen Orchestra, ESXi/vCenter, OCI, libvirt/QEMU) — TLS via `rustls`; no system OpenSSL
- Cloud provider REST APIs (DigitalOcean, Hetzner, Linode, Vultr, AWS EC2, GCE, Azure) — credential stored in OS keychain, never in DB
- Mosh UDP/SSP to remote `mosh-server` on user-configured hosts
- No telemetry; no analytics; no CDN asset fetches at runtime

**Stored data location (per-user — PART 4 → "Path Rule"):**
- Config: `~/.config/tabssh/tabssh/config.toml` (Linux/BSD) · `~/Library/Application Support/tabssh/config/config.toml` (macOS) · `%AppData%\tabssh\tabssh\config\config.toml` (Windows)
- Data/DB: `~/.local/share/tabssh/tabssh/tabssh.db` (Linux/BSD) · `~/Library/Application Support/tabssh/data/tabssh.db` (macOS) · `%LocalAppData%\tabssh\tabssh\data\tabssh.db` (Windows)
- Cache: `~/.cache/tabssh/tabssh/` (Linux/BSD) · `~/Library/Caches/tabssh/` (macOS) · `%LocalAppData%\tabssh\tabssh\cache\` (Windows)
- Logs: `~/.local/state/tabssh/tabssh/logs/` (Linux/BSD) · `~/Library/Logs/tabssh/` (macOS) · `%LocalAppData%\tabssh\tabssh\logs\` (Windows)
- Credentials: OS keychain only (Linux Secret Service, macOS Keychain, Windows Credential Manager) — never in the DB or on disk in plaintext

**License exceptions (PART 0 → "Rust-Only Application", PART 5 → "Pure-Rust Library Stack"):**
- `rusqlite` with `bundled` feature: statically vendors SQLite C for the local encrypted database. No viable pure-Rust production alternative exists today (`limbo` is pre-alpha). The C code is statically linked into the final binary and does not require a system SQLite at runtime. Distribution license remains MIT.
- `ring`: pre-granted by PART 5 (small, audited, ubiquitous; requires LICENSE.md attribution only).

---

### Must have

**Core SSH**
- SSH2 protocol: password, public key, keyboard-interactive, and OpenSSH user certificate authentication
- Universal SSH key support: OpenSSH v1 format, PEM (PKCS#1/PKCS#8), PuTTY v2/v3; RSA (2048/3072/4096), ECDSA (P-256/P-384/P-521), Ed25519, DSA
- In-app SSH key generation for all supported types; output: PEM (optionally passphrase-encrypted) + OpenSSH public key
- SHA-256 fingerprints and visual emoji fingerprints matching the Android sibling's format
- Browser-style tabbed interface — multiple independent SSH sessions, including multiple tabs to the same host; each tab owns an independent shell channel (SSH multiplexing: sibling tabs survive when one shell exits)
- Host key verification with TOFU and MITM detection; trust levels: UNKNOWN / ACCEPTED / VERIFIED
- Always-on keepalive (60s interval, count-max 3); no per-profile toggle
- Authentication priority: public key → password → keyboard-interactive
- Auto-reconnect: up to 3 retries with 5s delay for transient network errors; never retries auth failures
- Per-connection environment variables
- Agent forwarding

**Telnet**
- RFC 854 Telnet protocol (ECHO/SGA/TERMINAL-TYPE/NAWS negotiation; same `ConnectionProfile` model as SSH; protocol field distinguishes SSH vs Telnet)

**Terminal emulation**
- Full VT100/VT220/xterm emulation
- 256-color and 24-bit true color
- UTF-8 and Unicode
- Configurable scrollback (default 10,000 lines)
- Mouse support (SGR mode), alternate screen, title escape sequences (OSC 0/2)
- URL detection (Ctrl+Click → open in browser; Ctrl+Right-Click → Open/Copy dialog)
- Find/search in scrollback (case-insensitive, highlight matching rows, scroll to first hit)
- Session recording and replay (transcript format interchangeable with Android sibling)

**Tmux/Screen/Zellij auto-launch + post-connect script**
- Per-profile `multiplexerMode`: `OFF` / `AUTO_ATTACH` / `CREATE_NEW` / `ASK`; multiplexer type (tmux/screen/zellij) from global preference; session name from `multiplexerSessionName`
- `postConnectScript` executed after multiplexer attach, lines starting with `#` skipped
- Multiplexer command sent first, post-connect script after; both injected as if typed by the user ~500ms after bridge wires up

**SFTP**
- Dual-pane SFTP browser
- Remote file editor: download → edit in-place → upload; binary-file guard (null-byte scan in first 8 KB); files ≤ 1 MiB only
- chmod editor (rwx checkboxes per owner/group/other; live octal display)
- SCP fallback when SFTP subsystem is unavailable (speaks `scp -t` wire protocol over exec channel)
- Resumable transfers, batch transfers, per-transfer progress tracking; default buffer 32 KB

**Port forwarding and tunnels**
- Local (-L), remote (-R), dynamic SOCKS5 (-D) port forwarding
- Background tunnels that outlive the originating terminal tab
- ProxyJump / jump-host cascading
- Bind-to-all-interfaces toggle per forward
- HTTP/SOCKS4/SOCKS5 proxy support for SSH connections

**Port knocking**
- Optional pre-connect port knock sequence per profile: knock host, port list (JSON array), delay ms between knocks
- Executed before SSH/Telnet connect; failure is non-fatal (knock is best-effort)

**SSH config and key management**
- Read `~/.ssh/config` directly in-place — no import step required (desktop-native advantage)
- Parsed fields: `Host`, `HostName`, `User`, `Port`, `IdentityFile`, `ProxyJump`, `ProxyCommand`, `Compression`, `ServerAliveInterval`, `ConnectTimeout`, `Ciphers`, `Macs`
- SSH config round-trip export
- Bulk import — auto-detect format from a single file or text blob:
  - CSV (columns: `host`, `port`, `username`, `name`, `auth_type`, `group`)
  - JSON (array of objects with same field names as CSV)
  - PuTTY `.reg` (Windows Registry Editor format, Session entries)
  - Terraform `.tf` (extracts `public_ip`/`public_dns`, connection block fields from `aws_instance` and `google_compute_instance` resources)
- OpenSSH user certificate authentication (`*-cert.pub`)

**Credential security**
- OS-keychain-backed credential storage: Linux Secret Service, macOS Keychain, Windows Credential Manager — single `keyring` crate API
- Four storage levels: Never, SessionOnly, Encrypted (default), Biometric — Biometric tier unlocks through the OS's own factor: Windows Hello (`windows-rs` credential prompt), macOS Touch ID (`LocalAuthentication` via Keychain access control), Linux `polkit`/`fprintd` where available; on platforms/hardware without a biometric factor the tier falls back to the OS keychain's own auth prompt (password/PIN), never to plaintext
- Cipher: AES/GCM/NoPadding, 12-byte IV, 128-bit tag; hardware-backed where OS supports it
- Auto-lock on idle; configurable TTL (default 24 hours)
- Passwords and private keys must never be stored in plaintext on disk or in the database
- Clipboard auto-clear for sensitive pastes: configurable delay (seconds; 0 = disabled); suppress clipboard preview on platforms that support it
- In-memory password lifecycle: zero credential after auth failure; zero when connection tab closes

**App lock**
- Optional PIN code app lock; SHA-256 hash of PIN stored in preferences (never plaintext)
- `MAX_ATTEMPTS = 5`: after 5 failed verify attempts, lock is reset and a new PIN must be set
- Auto-lock when app is backgrounded longer than configurable timeout (default 300s)
- Screenshot/screen-capture prevention on PIN and auth screens, configurable: on Windows via `SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)`, on macOS via `NSWindow.sharingType = .none`; Linux/Wayland has no equivalent OS primitive, so the setting is a no-op there and the UI must say so rather than silently claim protection

**Themes and UI**
- 23 built-in terminal themes (byte-compatible with the Android sibling's `BuiltInThemes.kt` catalogue): System Default / Dark / Light, Dracula, Solarized Dark/Light, Nord, One Dark, Monokai, Gruvbox Dark/Light, Tomorrow Night, GitHub Light, Atom One Dark, Material Dark, Tokyo Night/Light, Catppuccin Mocha, Rose Pine, Everforest, Kanagawa, Night Owl, Cobalt2
- Theme structure: id, name, author, isDark/isBuiltIn, terminal foreground/background/cursor/selection/highlight, 16-entry ANSI palette, UI overrides
- GUI theme editor; WCAG AA/AAA contrast validation; color-blindness simulation (protanopia, deuteranopia, tritanopia); import/export theme JSON (VS Code theme format compatible)
- Per-host color tags; per-connection terminal font size override
- Workspaces (named tab groups; each workspace maps to a set of connection profile IDs)
- Command palette (Ctrl+K), quick switcher (Ctrl+J), history palette (Ctrl+R)
- Split-pane terminals
- Broadcast input and cluster commands with live streaming output per target
- Snippets library with `{?variable}` prompt-style placeholders; category, tags, usage count, favorites
- Identity abstraction (reusable credential profiles: username + auth type + key/password; referenced by connections)
- Connection groups/folders — hierarchical, collapsible, group types (user / system auto-groups)
- Recordable macros (raw byte sequences including escape codes, paste payloads, modifier-composed keys; distinct from snippets; replay verbatim)
- Active Sessions strip: top-of-window list of running tabs with terminal title (OSC 0/2) + connection-state indicator; click to focus

**Accessibility**
- Screen-reader support via `egui`'s AccessKit integration (Windows Narrator, macOS VoiceOver, Linux Orca/AT-SPI) — the desktop analogue of the Android sibling's TalkBack requirement; every interactive widget exposes an accessible name/role
- Full keyboard navigation — every action reachable via the command palette (Ctrl+K) or a bound shortcut, no mouse-only affordances
- High-contrast mode and large-text/UI-scale mode, in addition to the WCAG AA/AAA theme contrast validation already listed under Themes and UI
- Dark/light/auto per OS preference

**Performance monitoring**
- Per-connection SSH-exec metrics: CPU (`/proc/stat` delta), memory (`/proc/meminfo`), disk (`df -h /`), network (`/proc/net/dev` delta), load average (`/proc/loadavg`), platform info (`uname -a` + `/etc/os-release`, cached per session)
- `PerformanceMetrics`: timestamp, cpuUsage, memoryUsage, diskUsage, networkStats, loadAverage, platformInfo
- Rolling history: 60 data points at 5-second intervals per connection (5 minutes)
- Draggable real-time overlay HUD (CPU/mem/network/load) visible inside any terminal tab
- Multi-host dashboard: parallel polling against multiple connections; grouped by user-defined groups
- Two connect paths: interactive (starts system tray notification path) vs monitoring (no notification, reuses live session if available)

**Background host monitoring**
- Two-tier model:
  - Availability (always-on background): TCP connect to `host:port`, 5s timeout, battery-aware; processes all enabled monitor slots in one sweep
  - Performance (opt-in per host): SSH `MetricsCollector` — only runs if a live session already exists; never opens SSH sessions from background
- Per-host `MonitorSlot` config: `enabled`, `alertOnDown`, `alertOnRecovery`, `cpuThreshold`, `memoryThreshold`, `diskThreshold`, `loadThreshold` (nullable = disabled), `enablePerformanceChecks`, `checkIntervalMinutes`, `alertCooldownMinutes`
- State tracking: `isCurrentlyDown`, `consecutiveFailures`, `lastCheckedAt`, `lastSeenUp`, `lastNotifiedDownAt`
- Desktop equivalent of WorkManager periodic scheduling: in-process tokio interval task or OS task scheduler
- Notifications: host down, host recovered, threshold breach; per-channel notification cooldown

**Hypervisor management**

*Proxmox VE (REST):*
- Auth: `POST /api2/json/access/ticket` (returns authTicket + CSRFPreventionToken); realm `user@pam` / `user@pve`
- List nodes, list VMs (`/cluster/resources?type=vm`), power actions (start/stop/shutdown/reboot/reset)
- Serial console: Termproxy (`POST .../termproxy` → WS ticket); graphical VNC proxy
- Optional SSL verification bypass; TOFU TLS pinning per `pinned_cert_sha256`

*XCP-ng / XenServer (XML-RPC):*
- Auth: `session.login_with_password`; detects HTML response → falls back to Xen Orchestra client
- VM enumerate, start/stop/reboot operations; console WebSocket from `console.get_location`

*Xen Orchestra (REST + WebSocket):*
- Auto-detects API version: `/rest/v6` → `/rest/v5` → `/rest/v0`
- Auth: Bearer token from POST to authentication_tokens endpoint
- ~26 REST methods: VMs (list/start/stop/restart/reset/suspend/resume), snapshots (list/create/delete/revert), backup jobs (list/run/runs/details), pools, hosts
- WebSocket real-time events: vm.started/stopped/suspended/restarted/created/deleted, snapshot.created/deleted, backup.completed; live-updates indicator in UI
- Console: `getConsoleWebSocketUrl(vmId)`

*VMware ESXi / vCenter (REST):*
- Auth: `POST /api/session` with HTTP Basic → session ID cookie
- VM list (`GET /api/vcenter/vm`), power actions (`POST .../power`)
- Detects vCenter vs standalone ESXi by probing `/api/vcenter/datacenter`
- Console: VMware WebMKS/VMRC is proprietary — SSH into guest VM is the supported path

*Oracle Cloud Infrastructure (OCI) — REST + RSA-SHA256 signed requests:*
- Auth model: API key only (PKCS#1/PKCS#8/encrypted PEM; RSA-SHA256 cavage HTTP signatures draft-cavage-http-signatures-08)
- Onboarding: import `~/.oci/config` (INI with DEFAULT + named sections), import private key PEM; fingerprint round-trip validation before save; session-token profiles rejected
- Operations: validateCredentials, listInstances(compartmentOcid), getInstance, instanceAction (START/STOP/SOFTSTOP/RESET/SOFTRESET), getInstancePublicIp via VNIC walk
- Endpoints: Identity `https://identity.<region>.oci.oraclecloud.com`, Compute/Networking `https://iaas.<region>.oraclecloud.com`; 34 commercial regions pre-seeded, free-text allowed
- Secrets: PEM key + passphrase stored in OS keychain under `oci_private_key_{id}` / `oci_passphrase_{id}`; never in DB
- Out of scope v1: Instance Console Connection, compartment browser (paste OCID), pagination, identity domains

*Libvirt / QEMU (SSH-tunneled control plane):*
- Auth: SSH key (`sshIdentityId`) or password to the hypervisor host
- VM enumeration: `virsh list --all` over SSH exec, parsed into VM records
- VNC discovery: `virsh vncdisplay <domain>` returns `:N`; tunnel VNC stream through direct-tcpip to `127.0.0.1:(5900+N)` — no VNC port exposure needed on the hypervisor
- SSH fallback: `virsh domifaddr` discovers VM IP; builds a `ConnectionProfile` with ProxyJump pointing at the hypervisor

*Console infrastructure (all hypervisors):*
- `HypervisorConsoleManager` routes to per-protocol `ConsoleWebSocketClient`
- Proxmox Termproxy text framing: `"0:LEN:MSG"` (data), `"1:COLS:ROWS:"` (resize)
- Proxmox VNC: RFB 3.8 client; ServerFence (type 248) and inline ENC_FENCE (-312 / 0xFFFFFEB8) handling required before first FramebufferUpdate
- All console streams wired to the same terminal emulator as SSH sessions

- VM serial console via hypervisor API — works without VM network (OS install, VMs without network)
- Power management, snapshots, backup jobs (Xen Orchestra)
- Reusable `HypervisorAccount` credentials (separate from per-profile inline credentials)

**Cloud host import**
- DigitalOcean, Hetzner, Linode, Vultr, AWS EC2, Google Cloud Compute, Azure VMs, Oracle Cloud Infrastructure (OCI) — same 8-provider surface as the Android sibling's cloud provider management; OCI instance import reuses the API-key auth/endpoints defined under Hypervisor management (OCI), it is not a second credential model
- On import: connect to provider REST API, enumerate instances, create `ConnectionProfile` rows with public IP, username, port
- Tokens stored in the OS keychain under `cloud_token_{accountId}`; never in the database
- `CloudAccount` row stores only metadata (provider, enabled, lastRefreshAt, lastCount)
- Both the row and token are synced: row in sync payload, token in the AES-GCM secrets map as `cloud_token_{id}`

**VNC hosts (direct VNC, no SSH)**
- CRUD `VncHost` entries (name, host, port, identityId, color tag)
- `VncIdentity` — reusable VNC credentials; password stored in OS keychain under `vnc_identity_{id}`, never in DB
- RFB 3.8 client: same implementation as the hypervisor console path

**Data management**
- Encrypted backup and restore — BACKUP_VERSION = 3, wire format v2: each entity file `{"v":2,"items":[...]}`
  - Entity files: connections, keys, themes, certificates, host_keys, identities, connection_groups, snippets, hypervisors, hypervisor_accounts, workspaces, cloud_accounts, macros, monitor_slots, vnc_hosts, vnc_identities, preferences, secrets
  - `secrets.json`: all credentials exported (user controls whether to encrypt the backup file overall)
  - Excluded from backup: tab_sessions (runtime), sync_state (per-device), audit_log (exported separately)
  - Old v1 ZIP backups remain restorable
- Cloud sync via filesystem watch and the user's existing sync app (Nextcloud, syncthing, rclone, etc.)
  - Wire format byte-compatible with Android sibling's `TABSSH_SYNC_V2`
  - Header layout: `[0..32)` magic (`TABSSH_SYNC_V2` + zero padding) · `[32..64)` PBKDF2 salt · `[64..76)` GCM IV · `[76…]` ciphertext (GZIP-compressed JSON `SyncDataPackage`); GCM 128-bit auth tag appended by cipher
  - KDF: PBKDF2-HMAC-SHA256, 100,000 iterations, 256-bit key
  - Three-way merge with conflict resolution UI for connections, stored_keys, themes, host_keys
  - Last-write-wins for workspaces, snippets, identities, connection_groups, cloud_accounts, macros, monitor_slots, vnc_hosts, vnc_identities, trusted_certificates, hypervisor_profiles, hypervisor_accounts
  - Per-entity sync toggles; 30s debounced sync-on-change
  - Synced entities: connections, stored_keys, themes, host_keys, preferences, workspaces, snippets, identities, connection_groups, cloud_accounts, macros, monitor_slots, vnc_hosts, vnc_identities, trusted_certificates, hypervisor_profiles, hypervisor_accounts
  - NOT synced: tab_sessions, audit_log, sync_state
  - Cloud tokens and VNC identity passwords transferred inside the AES-GCM secrets map (never in the sync JSON plaintext)
- Bulk import: CSV, JSON, PuTTY registry/file, Terraform `.tf`
  - `ParseResult`: format detected, `List<ConnectionProfile>`, `List<String>` warnings

**Audit log**
- Optional per-connection audit log: connect/disconnect events, SSH commands (opt-in), terminal output per command (opt-in)
- Rolling size cleanup: configurable max size MB (default 100) and max age days (default 30)
- Disabled by default; master on/off preference
- Exported separately from backup (per-device security trail, not synced)
- Viewer UI: searchable log, filter by connection/session/date range

**Mosh**
- Native Rust Mosh client implementation — no cross-compiled native binary required (unlike Android)
- `use_mosh` flag per `ConnectionProfile`; switches the connection path in the tab
- Handoff via SSH exec channel (`mosh-server` on remote, parse `MOSH CONNECT <port> <key>` response), then UDP/SSP/AES-128-OCB transport

**X11 forwarding**
- X11 proxy binds ephemeral localhost port; X11 channels routed through it
- Desktop: forward to the user's existing display server (DISPLAY env var, Wayland XWayland, etc.)
- `x11_forwarding` flag per `ConnectionProfile`; non-fatal warning if no X server found

**Mobile interoperability**
- QR pairing: desktop is the sender, phone scans and imports connections
  - Wire format (fixed by Android sibling — do not change):
    - `QrPayload` CBOR: `{ version: u8 = 1, salt: [u8;16], nonce: [u8;12], ciphertext: bytes }` → base64-encoded (standard or URL-safe, with or without padding) → byte-mode QR
    - `ciphertext` = AES-256-GCM(key, nonce, CBOR(PairingPayload)); key = Argon2id(password=6-digit-code, salt, m=64MiB, t=3, p=1)
    - Argon2id: `Params::new(64*1024, 3, 1, None)` — m in KiB (64×1024 = 65536)
    - `PairingPayload` CBOR: `{ version: u8=1, expires_at: u64, device_label: Option<String>, connections: [ConnectionProfile], groups: [Group], identities: [Identity] }`
    - `ConnectionProfile` in payload: no password, no private key; public-key fingerprint + OpenSSH public-key line only; includes cosmetic and behavioral fields (color_tag, env_vars, multiplexer fields, post_connect_script, use_mosh, agent_forwarding, x11_forwarding, proxy_host/port/username/auth_type)
  - QR at ECC level L, 256×256 monochrome; cap v1 at 10 connections per QR (fits within 2,953 byte binary-mode ceiling)
  - Must produce QRs that `ImportFromQrActivity` on Android can decode and import without modification
  - Must pass shared test vectors committed in both repos
  - 6-digit code: displayed separately beside the QR; 60-second TTL; countdown in UI
  - Desktop UI: File → Pair Phone… → state machine: Idle → Selecting → Generating → Active → Expired
  - Crates required: `qrcodegen`, `ciborium`, `argon2`, `aes-gcm`, `rand`, `base64`
- Sync blobs must be interchangeable with the Android sibling (same TABSSH_SYNC_V2 format, same KDF)
- Backup entity shapes must be deserializable by Android's `BackupImporter`
- SQLite schema tracks Android sibling's Room schema (currently v37, migration chain v1→v37 in `../android/AI.md §8.4`); desktop schema versions are independent integers but must carry equivalent fields

**Desktop-native features (no mobile equivalent)**
- Read `~/.ssh/` directly — keys and config without an import step (Linux/BSD/macOS: `~/.ssh/`; Windows: `%USERPROFILE%\.ssh\`)
- System tray with connect-from-tray submenu and auto-launch on login
- CLI mode: `tabssh user@host` and `tabssh --connect <profile>` invocation from the shell
- Multiple windows; workspace-per-window layout option
- Build commit-id marker: log `## binary built from: <commit> ##` once per commit-id change; resolved at build time via `build.rs`, fall back to `release.txt`
- Native package distribution: `.deb`, `.rpm`, AUR PKGBUILD, AppImage, `.dmg`, Homebrew formula, MSI, WinGet manifest, Scoop, FreeBSD pkg/ports, OpenBSD packages, NetBSD pkgsrc

**Security**
- No telemetry unless explicitly opted in by the user
- No feature gating — all features available without license or subscription
- Must work offline (cloud features are additive; offline is the baseline)
- No plaintext secrets in memory dumps, on disk, or in the database
- Parameterized queries everywhere; constant-time comparisons for secrets
- Crash reporter via `panic::set_hook`; UI-thread freeze detection optional

### Must not

- Ship with a JVM, Android SDK, or any non-OS runtime dependency; binary must be fully static
- Store credentials or private keys in plaintext anywhere
- Enable telemetry without explicit user opt-in
- Gate any feature behind a subscription or license key
- Port mobile-only features that have no desktop equivalent:
  - SAF document URIs (desktop reads files directly)
  - Android foreground service notification (desktop uses system tray)
  - On-screen keyboard customisation
  - Swipe gestures, pinch-zoom, volume key bindings
  - Tasker integration
  - ANR watchdog (use `panic::set_hook` crash reporting instead)
  - Foldable/book-mode layouts (desktop windows are freely resizable)
  - Android home-screen widgets

### Must be compatible with

- Android sibling TABSSH_SYNC_V2 wire format (AES-256-GCM + PBKDF2-HMAC-SHA256 100k iterations; header layout per `../android/AI.md §9.1`)
- Android sibling QR pairing payload (CBOR + AES-256-GCM + Argon2id m=64MiB t=3 p=1 + base64 byte-mode QR; `PairingDecryptor.kt` is authoritative)
- Android sibling backup format BACKUP_VERSION = 3, wire format v2 (`{"v":2,"items":[...]}` per entity file)
- OpenSSH private key v1 format, PEM PKCS#1/PKCS#8, and standard OpenSSH public-key lines
- PuTTY key format v2 and v3 (.ppk files)
- RFC 4251–4254 (SSH2 protocol)
- RFC 854 (Telnet) + ECHO/SGA/TERMINAL-TYPE/NAWS options
- RFB 3.8 (VNC); ServerFence (type 248) and inline ENC_FENCE (-312) mandatory
- SHA-256 host key fingerprints + emoji visual fingerprints matching the Android sibling's format
- POSIX/XDG directory conventions on Linux/BSD; `~/Library/` paths on macOS; `%APPDATA%` on Windows
- OCI HTTP signatures (draft-cavage-http-signatures-08, RSA-SHA256): signing string format, `Authorization: Signature version="1",keyId="…"` header
- Terraform `.tf` bulk import: `aws_instance` and `google_compute_instance` block extraction

### Platform targets

| OS | Architectures | Binary names |
|----|--------------|--------------|
| Linux (musl static) | amd64, arm64 | `tabssh-linux-amd64`, `tabssh-linux-arm64` |
| macOS | amd64, arm64 | `tabssh-darwin-amd64`, `tabssh-darwin-arm64` |
| Windows | amd64, arm64 | `tabssh-windows-amd64.exe`, `tabssh-windows-arm64.exe` |
| FreeBSD | amd64, arm64 | `tabssh-freebsd-amd64`, `tabssh-freebsd-arm64` |
| OpenBSD | amd64, arm64 | `tabssh-openbsd-amd64`, `tabssh-openbsd-arm64` |
| NetBSD | amd64 | `tabssh-netbsd-amd64` |

Total: 11 binary variants. Binary naming uses OS names `linux/darwin/windows/freebsd/openbsd/netbsd` and arch names `amd64`/`arm64` (Go-style, per AI.md → PART 2 "Binary Model" — not `macos`, not `x86_64`/`aarch64`).
