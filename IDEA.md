## Project description

TabSSH Desktop is a cross-platform SSH/SFTP client for Windows, Linux, macOS, and BSD systems. Built as a single static Rust binary with no runtime dependencies, it gives developers and sysadmins browser-style tabbed terminal sessions, integrated SFTP, port forwarding, and hypervisor management. It targets feature parity with the TabSSH Android sibling app where desktop constraints allow, adding desktop-native conveniences such as direct `~/.ssh/` access, system tray, and CLI invocation from the shell.

## Project variables

project_name: desktop
project_org: tabssh
internal_name: tabssh
internal_org: tabssh
display_name: TabSSH Desktop
crate_name: tabssh
android_sibling: ../android
license: MIT
repo: https://github.com/tabssh/desktop

## Business logic

### Must have

**Core SSH**
- SSH2 protocol: password, public key, keyboard-interactive, and OpenSSH user certificate authentication
- Universal SSH key support: OpenSSH, PEM (PKCS#1/PKCS#8), PuTTY v2/v3; RSA, ECDSA (P-256/P-384/P-521), Ed25519, DSA
- In-app SSH key generation for all supported types
- Browser-style tabbed interface — multiple independent SSH sessions, including multiple tabs to the same host
- Host key verification with TOFU and MITM detection
- Always-on keepalive (60s interval, count-max 3); no per-profile toggle

**Terminal emulation**
- Full VT100/VT220/xterm emulation
- 256-color and 24-bit true color
- UTF-8 and Unicode
- Configurable scrollback (default 10,000 lines)
- Mouse support (SGR mode), alternate screen, title escape sequences (OSC 0/2)

**SFTP**
- Dual-pane SFTP browser
- Remote file editor (open → edit → save back)
- chmod editor
- SCP fallback when SFTP subsystem is unavailable
- Resumable transfers, batch transfers, per-transfer progress tracking

**Port forwarding and tunnels**
- Local (-L), remote (-R), dynamic SOCKS5 (-D) port forwarding
- Background tunnels that outlive the originating terminal tab
- ProxyJump / jump-host cascading
- Bind-to-all-interfaces toggle per forward

**SSH config and key management**
- Read `~/.ssh/config` directly in-place — no import step required (desktop-native advantage)
- SSH config round-trip export
- Bulk import: CSV, JSON, PuTTY key lists
- OpenSSH user certificate authentication

**Credential security**
- OS-keychain-backed credential storage: Linux Secret Service, macOS Keychain, Windows Credential Manager
- Four storage levels: Never, SessionOnly, Encrypted (default), Biometric
- Master-password wrapping mode (optional)
- Auto-lock on idle; configurable TTL (default 24 hours)
- Passwords and private keys must never be stored in plaintext on disk or in the database

**Themes and UI**
- 23 built-in terminal themes (byte-compatible with the Android sibling's catalogue)
- GUI theme editor; import/export theme JSON
- Per-host color tags
- Workspaces (named tab groups)
- Command palette (Ctrl+K), quick switcher (Ctrl+J), history palette (Ctrl+R)
- Split-pane terminals
- Broadcast input and cluster commands with live streaming
- Snippets library with `{?variable}` prompt-style placeholders
- Identity abstraction (reusable credential profiles)
- Connection groups/folders (hierarchical)
- Find/search in scrollback
- Recordable macros (raw byte sequences, distinct from snippets)

**Hypervisor management**
- Proxmox VE, XCP-ng, Xen Orchestra (REST + WebSocket live updates), VMware ESXi/vCenter
- VM serial console via hypervisor API — works without VM network (OS install, VMs without network)
- Power management, snapshots, backup jobs (Xen Orchestra)

**Cloud host import**
- DigitalOcean, Hetzner, Linode, Vultr
- Tokens stored in the OS keychain; never in the database

**Data management**
- Encrypted ZIP backup and restore — byte-compatible with Android sibling's `BackupManager` (BACKUP_VERSION = 1)
- Cloud sync via filesystem watch and the user's existing sync app (Nextcloud, syncthing, rclone, etc.)
  - Wire format byte-compatible with Android sibling's `TABSSH_SYNC_V2`
  - Three-way merge with conflict resolution UI
  - Per-entity sync toggles; debounced sync-on-change
- Bulk import (CSV, JSON, PuTTY); migration from PuTTY registry/file

**Mobile interoperability**
- QR pairing: desktop is the sender, phone scans and imports connections
  - Wire format is fixed by the Android sibling (shipped 2026-04-28): CBOR + AES-256-GCM + Argon2id (m=64MiB, t=3, p=1) + 6-digit code + 60-second TTL
  - Must produce QRs that `ImportFromQrActivity` on Android can decode and import without modification
  - Must pass the shared test vectors committed in both repos
- Sync blobs must be interchangeable with the Android sibling (same TABSSH_SYNC_V2 format, same KDF parameters: PBKDF2-HMAC-SHA256 100k iterations)
- SQLite schema tracks Android sibling's Room schema (currently v26); desktop versions numbered independently

**Desktop-native features (no mobile equivalent)**
- Read `~/.ssh/` directly — keys and config without an import step
- System tray with connect-from-tray submenu and auto-launch on login
- CLI mode: `tabssh user@host` and `tabssh --connect <profile>` invocation from the shell
- Multiple windows
- Native package distribution: `.deb`, `.rpm`, AUR PKGBUILD, AppImage, `.dmg`, Homebrew formula, MSI, WinGet manifest, Scoop, FreeBSD pkg/ports, OpenBSD packages, NetBSD pkgsrc

**Security**
- No telemetry unless explicitly opted in by the user
- No feature gating — all features available without license or subscription
- Must work offline (cloud features are additive; offline is the baseline)
- No plaintext secrets in memory dumps, on disk, or in the database
- Parameterized queries everywhere; constant-time comparisons for secrets

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

### Must be compatible with

- Android sibling TABSSH_SYNC_V2 wire format (AES-256-GCM + PBKDF2-HMAC-SHA256 100k iterations)
- Android sibling QR pairing payload (CBOR + AES-256-GCM + Argon2id m=64MiB t=3 p=1 + base64 byte-mode QR)
- Android sibling backup ZIP format (BACKUP_VERSION = 1, same JSON entity shapes)
- OpenSSH private key format v1 (binary), PEM PKCS#1/PKCS#8, public-key lines (`ssh-rsa …` etc.)
- PuTTY key format v2 and v3 (.ppk files)
- RFC 4251–4254 (SSH2 protocol)
- SHA-256 host key fingerprints + emoji visual fingerprints matching the Android sibling's format
- POSIX/XDG directory conventions on Linux/BSD; `~/Library/` paths on macOS; `%APPDATA%` on Windows

### Platform targets

| OS | Architectures | Binary names |
|----|--------------|--------------|
| Linux (musl static) | amd64, arm64 | `tabssh-linux-amd64`, `tabssh-linux-arm64` |
| macOS | amd64, arm64 | `tabssh-macos-amd64`, `tabssh-macos-arm64` |
| Windows | amd64, arm64 | `tabssh-windows-amd64.exe`, `tabssh-windows-arm64.exe` |
| FreeBSD | amd64, arm64 | `tabssh-freebsd-amd64`, `tabssh-freebsd-arm64` |
| OpenBSD | amd64, arm64 | `tabssh-openbsd-amd64`, `tabssh-openbsd-arm64` |
| NetBSD | amd64 | `tabssh-netbsd-amd64` |

Total: 11 binary variants. Binary naming uses OS names `linux/macos/windows/freebsd/openbsd/netbsd` and arch names `amd64`/`arm64` (not `darwin`, not `x86_64`/`aarch64`).
