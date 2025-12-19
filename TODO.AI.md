# TabSSH Desktop - AI Assistant TODO List

**Last Updated:** 2025-12-19  
**Current Status:** 50% Complete (Functional MVP)  
**Target:** 100% Complete (Feature Parity with Android App)

**🎯 Goal:** Build a complete desktop SSH client matching `../android/` feature set

---

## 📊 Progress Overview

```
Phase 1: Foundation               ████████████████████ 100% ✅ COMPLETE
Phase 2: Core Features            ████████████████████ 90%  🚧 MOSTLY DONE
Phase 3: Advanced SSH             ████░░░░░░░░░░░░░░░░ 20%  ❌ TODO
Phase 4: UI Polish                ███░░░░░░░░░░░░░░░░░ 15%  ❌ TODO
Phase 5: Platform Integration     ░░░░░░░░░░░░░░░░░░░░ 0%   ❌ TODO
Phase 6: Testing & Quality        ░░░░░░░░░░░░░░░░░░░░ 0%   ❌ TODO

Overall: ██████████░░░░░░░░░░ 50%
```

---

## 🎯 Phase 2: Core Features (90% → 100%)

### High Priority - Complete Remaining 10%

#### 2.1 Host Key Verification - Database Integration
**Status:** 🚧 Basic implementation done, DB storage pending  
**Priority:** HIGH  
**Time:** ~4 hours

**Files to modify:**
- `src/ssh/connection.rs` - Store verified host keys
- `src/storage/database.rs` - Add host key queries
- `src/ssh/active_session.rs` - Check known hosts before connecting

**Tasks:**
```rust
// TODO: In src/ssh/connection.rs
pub async fn verify_host_key(
    host: &str,
    port: u16,
    key: &key::PublicKey,
    database: &Database,
) -> Result<bool> {
    // 1. Calculate fingerprint
    // 2. Check known_hosts table
    // 3. If unknown, show dialog and store if accepted
    // 4. If known, verify match (MITM detection)
    // 5. Update last_seen timestamp
}
```

**Implementation steps:**
1. Add `HostKey` struct in `src/ssh/mod.rs`
2. Implement database queries in `src/storage/database.rs`
3. Add verification dialog in `src/ui/screens/connection_manager.rs`
4. Wire up in `src/ssh/active_session.rs`

---

#### 2.2 Session Persistence - Database Integration
**Status:** 🚧 DB schema ready, integration pending  
**Priority:** MEDIUM  
**Time:** ~6 hours

**Files to modify:**
- `src/app.rs` - Save/restore session state
- `src/storage/database.rs` - Add session persistence queries
- `src/ui/tab_manager.rs` - Serialize tab state

**Tasks:**
```rust
// TODO: In src/storage/database.rs
pub fn save_session(
    &self,
    tab_id: Uuid,
    host: &str,
    user: &str,
    port: u16,
    scrollback: &[String],
) -> Result<()> {
    // Save active session data
}

pub fn restore_sessions(&self) -> Result<Vec<SessionData>> {
    // Load all saved sessions
}
```

**Implementation steps:**
1. Create `SessionData` struct
2. Add save/restore methods to database
3. Implement in app shutdown/startup
4. Add "Restore previous sessions" dialog

---

## 🚀 Phase 3: Advanced SSH (20% → 100%)

**Time Estimate:** ~15-20 days  
**Priority:** HIGH - Critical for feature parity

### 3.1 SFTP Browser Implementation
**Status:** ❌ Stub only (5%)  
**Priority:** CRITICAL  
**Time:** ~5-7 days

**Files to create:**
- `src/sftp/browser.rs` - File browser UI component
- `src/sftp/transfer.rs` - Upload/download with progress
- `src/sftp/operations.rs` - File operations (copy, delete, rename, mkdir)

**Files to modify:**
- `src/sftp/client.rs` - Expand SFTP client functionality
- `src/ui/screens/sftp_browser.rs` - Complete UI implementation
- `src/app.rs` - Add SFTP view integration

**Reference:** `../android/app/src/main/java/io/github/tabssh/sftp/`

**Features to implement:**
```
✅ Connect to SFTP server
✅ List directory contents
✅ Navigate directories (cd, up, home)
✅ Download files with progress bar
✅ Upload files with progress bar
✅ Delete files/directories
✅ Rename files/directories
✅ Create directories
✅ Copy files
✅ Show file permissions
✅ Show file sizes/dates
✅ Drag-and-drop support
✅ Multi-file selection
✅ Transfer queue
✅ Resume interrupted transfers
```

**Implementation checklist:**
- [ ] `SftpBrowser` struct with file list
- [ ] `FileItem` struct (name, size, permissions, modified_date)
- [ ] `TransferManager` for upload/download queue
- [ ] Progress tracking (`TransferProgress` struct)
- [ ] UI components (file list, toolbar, transfer panel)
- [ ] File operations (download, upload, delete, rename, mkdir)
- [ ] Drag-and-drop from OS file manager
- [ ] Keyboard shortcuts (F5 refresh, Del delete, F2 rename)
- [ ] Context menu (right-click operations)
- [ ] Error handling and retry logic

---

### 3.2 Port Forwarding Implementation
**Status:** ❌ Not implemented  
**Priority:** HIGH  
**Time:** ~3-4 days

**Files to create:**
- `src/ssh/forwarding.rs` - Port forwarding manager
- `src/ssh/tunnel.rs` - Tunnel management
- `src/ui/screens/port_forwarding.rs` - UI for managing forwards

**Files to modify:**
- `src/ssh/connection.rs` - Add forwarding methods
- `src/app.rs` - Add port forwarding view

**Reference:** `../android/app/src/main/java/io/github/tabssh/ssh/forwarding/`

**Features to implement:**
```
✅ Local port forwarding (ssh -L)
✅ Remote port forwarding (ssh -R)
✅ Dynamic port forwarding / SOCKS proxy (ssh -D)
✅ Multiple forwards per connection
✅ Add/remove forwards dynamically
✅ Status indicators (active/inactive)
✅ Port availability checking
✅ Auto-reconnect on failure
```

**Implementation checklist:**
- [ ] `PortForward` struct (local_port, remote_host, remote_port, type)
- [ ] `ForwardingManager` to track all forwards
- [ ] Local forward implementation (bind local port)
- [ ] Remote forward implementation (request remote forward)
- [ ] Dynamic forward implementation (SOCKS5 proxy)
- [ ] UI for adding/editing forwards
- [ ] Status monitoring (connection count, bytes transferred)
- [ ] Persistence (save forwards with connection profile)
- [ ] Error handling and logging

---

### 3.3 SSH Config Parser
**Status:** ❌ Not implemented  
**Priority:** HIGH  
**Time:** ~2-3 days

**Files to create:**
- `src/ssh/config_parser.rs` - Parse ~/.ssh/config
- `src/ssh/config.rs` - Config data structures

**Files to modify:**
- `src/ui/screens/connection_manager.rs` - Import button
- `src/storage/database.rs` - Bulk insert connections

**Reference:** `../android/app/src/main/java/io/github/tabssh/ssh/config/`

**Features to implement:**
```
✅ Parse ~/.ssh/config format
✅ Support all common directives:
   - Host
   - HostName
   - Port
   - User
   - IdentityFile
   - ProxyCommand
   - ProxyJump
   - LocalForward
   - RemoteForward
   - DynamicForward
✅ Import all hosts at once
✅ Handle wildcards (Host *)
✅ Handle includes (Include ~/.ssh/config.d/*)
✅ Preview before import
✅ Detect duplicates
```

**Implementation checklist:**
- [ ] `SSHConfig` struct
- [ ] `HostConfig` struct
- [ ] Parser using regex or nom crate
- [ ] Handle multi-line values
- [ ] Import dialog with preview
- [ ] Conflict resolution (if host exists)
- [ ] Test with real ~/.ssh/config files

---

### 3.4 Jump Host / ProxyJump Support
**Status:** ❌ Not implemented  
**Priority:** MEDIUM  
**Time:** ~2-3 days

**Files to modify:**
- `src/ssh/connection.rs` - Multi-hop connection logic
- `src/ssh/ConnectionConfig` - Add jump host fields
- `src/ui/screens/connection_editor.rs` - Jump host UI

**Reference:** `../android/app/src/main/java/io/github/tabssh/ssh/connection/`

**Features to implement:**
```
✅ ProxyJump directive support
✅ ProxyCommand support
✅ Multiple jump hosts (A → B → C → Target)
✅ Jump host authentication
✅ Show connection chain status
```

**Implementation checklist:**
- [ ] Add `jump_host: Option<String>` to ConnectionConfig
- [ ] Implement connection chaining
- [ ] UI for configuring jump host
- [ ] Visual indicator of connection chain
- [ ] Test multi-hop scenarios

---

## 🎨 Phase 4: UI Polish (15% → 100%)

**Time Estimate:** ~10-12 days  
**Priority:** MEDIUM

### 4.1 Theme System Implementation
**Status:** 🚧 Structure exists (10%)  
**Priority:** MEDIUM  
**Time:** ~4-5 days

**Files to modify:**
- `src/config/themes.rs` - Expand theme definitions
- `src/terminal/renderer.rs` - Apply theme colors
- `src/ui/components.rs` - Themed UI components

**Themes to implement (match Android):**
```
1. ✅ Dark (default) - Current implementation
2. ❌ Dracula
3. ❌ Solarized Light
4. ❌ Solarized Dark
5. ❌ Nord
6. ❌ Monokai
7. ❌ One Dark
8. ❌ Tokyo Night
9. ❌ Gruvbox
10. ❌ High Contrast (accessibility)
```

**Theme definition format (JSON):**
```json
{
  "name": "Dracula",
  "background": "#282a36",
  "foreground": "#f8f8f2",
  "cursor": "#f8f8f2",
  "selection": "#44475a",
  "black": "#000000",
  "red": "#ff5555",
  "green": "#50fa7b",
  "yellow": "#f1fa8c",
  "blue": "#bd93f9",
  "magenta": "#ff79c6",
  "cyan": "#8be9fd",
  "white": "#bbbbbb",
  "bright_black": "#555555",
  "bright_red": "#ff6e6e",
  "bright_green": "#69ff94",
  "bright_yellow": "#ffffa5",
  "bright_blue": "#d6acff",
  "bright_magenta": "#ff92df",
  "bright_cyan": "#a4ffff",
  "bright_white": "#ffffff"
}
```

**Implementation checklist:**
- [ ] Create theme JSON files in `assets/themes/`
- [ ] Theme parser and validator
- [ ] Theme import/export functionality
- [ ] Theme selector in settings
- [ ] Apply theme to terminal renderer
- [ ] Apply theme to UI components
- [ ] Live theme preview
- [ ] Theme hot-reload (no restart required)

---

### 4.2 Custom Fonts Support
**Status:** ❌ Not implemented  
**Priority:** MEDIUM  
**Time:** ~2-3 days

**Files to modify:**
- `src/terminal/renderer.rs` - Font loading and rendering
- `src/ui/screens/settings.rs` - Font selector

**Fonts to support (match Android):**
```
- Cascadia Code (default)
- Fira Code
- JetBrains Mono
- Source Code Pro
- Hack
- Inconsolata
- DejaVu Sans Mono
- Ubuntu Mono
- Custom TTF/OTF import
```

**Implementation checklist:**
- [ ] Bundle popular programming fonts
- [ ] Font loading system
- [ ] Font size adjustment (8-32pt)
- [ ] Font weight selection (light, regular, bold)
- [ ] Ligature support (for Fira Code, etc.)
- [ ] Font preview in settings
- [ ] Custom font import (.ttf, .otf)
- [ ] Font metrics calculation

---

### 4.3 Settings Persistence
**Status:** ❌ Not implemented  
**Priority:** MEDIUM  
**Time:** ~2 days

**Files to modify:**
- `src/storage/database.rs` - Settings table operations
- `src/ui/screens/settings.rs` - Load/save settings

**Settings categories:**
```
General:
  - Default shell
  - Auto-connect on startup
  - Restore previous sessions
  - Notification preferences

Terminal:
  - Font family
  - Font size
  - Scrollback lines
  - Cursor style (block, beam, underline)
  - Cursor blink
  - Bell style (visual, audio, none)

Theme:
  - Selected theme
  - Custom theme path

Connection:
  - Default port (22)
  - Connection timeout (30s)
  - Keepalive interval (60s)
  - Compression enabled

Security:
  - Auto-lock timeout
  - Remember passwords
  - Host key checking

Advanced:
  - Log level
  - Log file path
```

**Implementation checklist:**
- [ ] Settings data structure
- [ ] Load settings on startup
- [ ] Save settings on change
- [ ] Settings validation
- [ ] Default values
- [ ] Settings export/import

---

### 4.4 Context Menus & Keyboard Shortcuts
**Status:** 🚧 Basic shortcuts done (Ctrl+T, W, Tab)  
**Priority:** MEDIUM  
**Time:** ~1-2 days

**Context menus to add:**
- [ ] Terminal right-click menu (Copy, Paste, Clear, Select All)
- [ ] Connection list right-click menu (Connect, Edit, Delete, Duplicate)
- [ ] Tab right-click menu (Close, Close Others, Close All)
- [ ] SFTP browser right-click menu (Download, Upload, Delete, Rename)

**Additional keyboard shortcuts:**
```
Global:
  Ctrl+Shift+T - Reopen closed tab
  Ctrl+Shift+N - New window
  Ctrl+, - Settings
  Ctrl+Q - Quit
  
Terminal:
  Ctrl+Shift+C - Copy
  Ctrl+Shift+V - Paste
  Ctrl+Shift+F - Find in terminal
  Ctrl+Shift+K - Clear terminal
  Ctrl++ - Increase font size
  Ctrl+- - Decrease font size
  Ctrl+0 - Reset font size
  
Tab Management:
  Ctrl+PgUp - Previous tab
  Ctrl+PgDn - Next tab
  Alt+1-9 - Switch to tab N
  
SFTP:
  F5 - Refresh file list
  Del - Delete selected
  F2 - Rename selected
  Ctrl+A - Select all
```

**Implementation checklist:**
- [ ] Add context menu framework
- [ ] Implement all context menus
- [ ] Add missing keyboard shortcuts
- [ ] Show shortcuts in tooltips
- [ ] Create keyboard shortcuts help screen

---

### 4.5 Drag-and-Drop Support
**Status:** ❌ Not implemented  
**Priority:** LOW  
**Time:** ~1 day

**Features:**
- [ ] Drag files from OS → SFTP browser (upload)
- [ ] Drag files from SFTP browser → OS (download)
- [ ] Drag connection from list → new window
- [ ] Drag tabs to reorder

---

### 4.6 Search in Terminal
**Status:** ❌ Not implemented  
**Priority:** LOW  
**Time:** ~1-2 days

**Features:**
- [ ] Search box (Ctrl+F)
- [ ] Highlight matches
- [ ] Navigate matches (F3/Shift+F3)
- [ ] Case-sensitive option
- [ ] Regex support
- [ ] Search in scrollback

---

## 🔧 Phase 5: Platform Integration (0% → 100%)

**Time Estimate:** ~8-10 days  
**Priority:** MEDIUM

### 5.1 OS Keychain Integration
**Status:** ❌ Not implemented  
**Priority:** MEDIUM  
**Time:** ~3-4 days

**Platforms:**
- [ ] macOS Keychain (security-framework crate)
- [ ] Windows Credential Manager (windows crate)
- [ ] Linux Secret Service (secret-service crate)
- [ ] BSD: Encrypted file storage with OS permissions

**Implementation:**
```rust
// src/crypto/keychain.rs
pub trait KeychainBackend {
    fn store_password(&self, service: &str, account: &str, password: &str) -> Result<()>;
    fn retrieve_password(&self, service: &str, account: &str) -> Result<String>;
    fn delete_password(&self, service: &str, account: &str) -> Result<()>;
}
```

**Implementation checklist:**
- [ ] Platform-specific backends
- [ ] Unified API
- [ ] Fallback to encrypted file
- [ ] Migration from plaintext
- [ ] Test on all platforms

---

### 5.2 Cross-Platform Builds
**Status:** ❌ Only Linux works  
**Priority:** MEDIUM  
**Time:** ~2-3 days

**Targets to support:**
```
Linux:
  ✅ x86_64-unknown-linux-musl (working)
  ❌ aarch64-unknown-linux-musl

macOS:
  ❌ x86_64-apple-darwin
  ❌ aarch64-apple-darwin

Windows:
  ❌ x86_64-pc-windows-msvc
  ❌ aarch64-pc-windows-msvc

BSD:
  ❌ x86_64-unknown-freebsd
  ❌ aarch64-unknown-freebsd
  ❌ x86_64-unknown-openbsd
  ❌ x86_64-unknown-netbsd
```

**Implementation checklist:**
- [ ] Set up cross-compilation environment
- [ ] Update Makefile for all targets
- [ ] Test builds on each platform
- [ ] Solve platform-specific issues
- [ ] Generate platform-specific binaries

---

### 5.3 Platform-Specific Installers
**Status:** ❌ Not implemented  
**Priority:** MEDIUM  
**Time:** ~2-3 days

**Installer formats:**
```
macOS:
  - .dmg - Drag-and-drop installer
  - Homebrew formula
  
Windows:
  - .msi - Windows Installer
  - WinGet manifest
  - Chocolatey package
  
Linux:
  - .deb - Debian/Ubuntu
  - .rpm - Fedora/RHEL
  - AppImage - Universal
  - Flatpak
  - Snap
  
BSD:
  - FreeBSD pkg
  - OpenBSD package
```

**Implementation checklist:**
- [ ] cargo-bundle for .app bundle
- [ ] cargo-deb for .deb packages
- [ ] cargo-wix for .msi installer
- [ ] AppImage build script
- [ ] Flatpak manifest
- [ ] Package manager manifests

---

### 5.4 System Tray Integration
**Status:** ❌ Not implemented  
**Priority:** LOW  
**Time:** ~1 day

**Features:**
- [ ] System tray icon
- [ ] Show active connections
- [ ] Quick connect menu
- [ ] Minimize to tray
- [ ] Restore from tray

---

### 5.5 Auto-Update Mechanism
**Status:** ❌ Not implemented  
**Priority:** LOW  
**Time:** ~1-2 days

**Features:**
- [ ] Check for updates on startup
- [ ] Download updates
- [ ] Install updates (with restart)
- [ ] Update notification
- [ ] Release notes display

---

## 🧪 Phase 6: Testing & Quality (0% → 100%)

**Time Estimate:** ~10-15 days  
**Priority:** HIGH (Quality)

### 6.1 Test Suite Implementation
**Status:** ❌ ZERO tests (0%)  
**Priority:** CRITICAL  
**Time:** ~5-7 days

**Test files to create:**
```
tests/unit/
  - ssh_connection_test.rs
  - terminal_emulator_test.rs
  - ansi_parser_test.rs
  - config_parser_test.rs
  - theme_parser_test.rs
  - crypto_test.rs
  - storage_test.rs
  
tests/integration/
  - connection_flow_test.rs
  - sftp_operations_test.rs
  - port_forwarding_test.rs
  - session_persistence_test.rs
  - settings_test.rs
```

**Test coverage goals:**
- [ ] Unit tests: 70%+ coverage
- [ ] Integration tests: 50%+ coverage
- [ ] SSH connection tests (mock SSH server)
- [ ] Terminal emulation tests
- [ ] ANSI parser tests
- [ ] Database tests
- [ ] UI tests (where possible)

---

### 6.2 Performance Optimization
**Status:** ❌ Not optimized  
**Priority:** MEDIUM  
**Time:** ~2-3 days

**Areas to optimize:**
- [ ] Terminal rendering (60 FPS target)
- [ ] ANSI parsing (lazy evaluation)
- [ ] Database queries (indexes, caching)
- [ ] Memory usage (profile and reduce)
- [ ] Binary size (strip unused code)
- [ ] Startup time (<500ms target)

**Profiling tools:**
- [ ] cargo-flamegraph for CPU profiling
- [ ] valgrind for memory profiling
- [ ] hyperfine for benchmarking

---

### 6.3 Security Audit
**Status:** ❌ Not audited  
**Priority:** HIGH  
**Time:** ~2-3 days

**Security checklist:**
- [ ] No hardcoded secrets
- [ ] Secure random number generation
- [ ] Proper input validation
- [ ] SQL injection prevention
- [ ] Command injection prevention
- [ ] Secure credential storage
- [ ] Memory clearing for sensitive data
- [ ] Host key verification always enabled
- [ ] TLS/SSL best practices
- [ ] Dependency vulnerability scan (cargo-audit)

---

### 6.4 Documentation
**Status:** 🚧 Technical docs done (40%)  
**Priority:** MEDIUM  
**Time:** ~2-3 days

**Documentation to create:**
- [ ] User manual (usage, features)
- [ ] Installation guide (all platforms)
- [ ] Configuration guide
- [ ] Troubleshooting guide
- [ ] Security best practices
- [ ] FAQ
- [ ] Screenshots and demos
- [ ] Video tutorials (optional)

---

### 6.5 Accessibility Features
**Status:** ❌ Not implemented  
**Priority:** LOW  
**Time:** ~2 days

**Features from Android app:**
- [ ] Screen reader support
- [ ] High contrast mode
- [ ] Large text support (8-32pt)
- [ ] Keyboard-only navigation
- [ ] ARIA labels for UI elements

---

### 6.6 Multi-Language Support (i18n)
**Status:** ❌ English only  
**Priority:** LOW  
**Time:** ~2-3 days

**Languages (from Android):**
- [ ] English (default)
- [ ] Spanish
- [ ] French
- [ ] German
- [ ] Chinese (Simplified)
- [ ] Japanese

**Implementation:**
- [ ] Use fluent-rs or i18n crate
- [ ] Extract all strings
- [ ] Create translation files
- [ ] Language selector in settings
- [ ] RTL support (for Arabic, Hebrew)

---

## 🚀 Advanced Features (Future)

These are lower priority, implement after Phase 6:

### Mosh Protocol Support
**Status:** ❌ Not implemented  
**Time:** ~3-4 days

**Features:**
- [ ] Mosh client implementation
- [ ] Roaming support (IP changes)
- [ ] Local echo
- [ ] Automatic reconnection

---

### X11 Forwarding
**Status:** ❌ Not implemented  
**Time:** ~2-3 days

**Features:**
- [ ] X11 socket forwarding
- [ ] DISPLAY variable handling
- [ ] Xauth cookie management
- [ ] Test with graphical apps

---

### Backup & Restore
**Status:** ❌ Not implemented  
**Time:** ~1-2 days

**Features:**
- [ ] Export all connections (JSON)
- [ ] Export all settings
- [ ] Export all SSH keys
- [ ] Import from backup file
- [ ] Encrypted backup option

---

### Connection Statistics
**Status:** ❌ Not implemented  
**Time:** ~1 day

**Features:**
- [ ] Track connection count
- [ ] Track total connection time
- [ ] Track data transferred
- [ ] Display "Connected X times"
- [ ] Last connected timestamp

---

### Cloud Sync
**Status:** ❌ Not implemented  
**Time:** ~3-4 days

**Features:**
- [ ] Google Drive sync (optional)
- [ ] WebDAV sync (degoogled devices)
- [ ] Encrypted sync
- [ ] Conflict resolution

---

## 📝 Implementation Notes

### Development Workflow

1. **Before starting a task:**
   - Read relevant code in `src/`
   - Check Android reference in `../android/`
   - Review related files in TODO

2. **While implementing:**
   - Write tests first (TDD where possible)
   - Follow Rust best practices
   - Use `cargo clippy` for linting
   - Run `cargo fmt` for formatting

3. **After completing a task:**
   - Update this TODO (mark as done)
   - Update CLAUDE.md if needed
   - Run `cargo test`
   - Run `cargo check`
   - Test manually

### Priority Levels

- **CRITICAL:** Blocks other work, must do now
- **HIGH:** Important for feature completeness
- **MEDIUM:** Nice to have, improves experience
- **LOW:** Optional, can defer

### Time Estimates

- Small task: ~1 day (4-8 hours)
- Medium task: ~2-3 days (8-24 hours)
- Large task: ~4-7 days (24-56 hours)

### Testing Requirements

For each feature:
1. Unit tests for core logic
2. Integration test for end-to-end flow
3. Manual testing on Linux (minimum)
4. Manual testing on other platforms (if time permits)

---

## 📊 Summary

### By Phase
- **Phase 2:** 2 tasks remaining (~10 hours)
- **Phase 3:** 4 major tasks (~15-20 days)
- **Phase 4:** 6 tasks (~10-12 days)
- **Phase 5:** 5 tasks (~8-10 days)
- **Phase 6:** 6 tasks (~10-15 days)

### By Priority
- **CRITICAL:** 2 tasks
- **HIGH:** 12 tasks
- **MEDIUM:** 18 tasks
- **LOW:** 10 tasks

### Total Remaining Work
- **Estimated time:** ~45-60 days
- **Current progress:** 50%
- **Target:** 100% (feature parity with Android)

---

## 🎯 Next Actions

**Immediate (This Week):**
1. Complete Phase 2 (host key verification + session persistence)
2. Start Phase 3 (SFTP browser - highest priority)

**Short-term (This Month):**
3. Complete SFTP browser
4. Implement port forwarding
5. Add SSH config parser

**Medium-term (Next 2 Months):**
6. Complete Phase 3 & 4 (Advanced SSH + UI Polish)
7. Start Phase 5 (Platform integration)
8. Begin test suite

**Long-term (3+ Months):**
9. Complete Phase 6 (Testing & Quality)
10. Advanced features (Mosh, X11, etc.)
11. 1.0 Release!

---

**Last Updated:** 2025-12-19  
**Maintained for:** AI Development Assistants  
**Format:** Comprehensive technical TODO with implementation details
