# Project Audit — TabSSH Desktop

Started: 2026-06-12
Mode: report-only (user requested no fixes)

Toolchain check: `docker run rust:bookworm cargo check --lib --target x86_64-unknown-linux-gnu --keep-going`
Result: **1 hard error**, 18 warnings (lib). Bin/test targets not type-checked because the lib gate fails first.

The AI.md §22 note "50 errors as of 2026-05-01" no longer reflects reality — `src/sftp/client.rs` has been rewritten against the russh-sftp 2.3.0 API (resolved version, not 2.1.1 as the spec claims) and the forwarding rewrite using `Arc<Handle<H>>` + `tokio::io::copy_bidirectional` has landed. The dominant remaining blocker is the russh 0.40.2 `Handler::check_server_key` shape — and the wider "concatenated-string" damage that did not exist when the 50-error note was written.

---

## Pass 1: Build errors (BUILD_ERROR)

- [ ] `src/ssh/active_session.rs:45` — `Handler::check_server_key` impl uses `&mut self -> Result<bool, _>`, but russh 0.40.2 trait (russh-0.40.2/src/client/mod.rs:1373) requires `self -> Result<(Self, bool), _>`. **E0195 lifetime mismatch under `#[async_trait]`.** This is the only hard error currently surfaced by `cargo check --lib`.
- [ ] `src/bin/ssh_test.rs:27` — same `check_server_key` shape mismatch; not surfaced because the lib fails first, but will fail identically once the lib compiles. Same fix.
- [ ] `src/ssh/connection.rs` — `SshClientHandler::check_server_key` also needs verification with the same russh 0.40.2 signature; not surfaced yet for the same reason.

## Pass 2: Structural / module-tree problems (STRUCTURAL)

Files exist on disk but are **not** declared in any `mod.rs`. They are unreachable, untested, and not counted by `cargo check`. AI.md §21 explicitly lists most of these as required modules.

- [ ] `src/terminal/buffer.rs` — orphan; AI.md §21 lists `terminal::buffer` as required (Grid + scrollback + cursor + alt screen).
- [ ] `src/terminal/cell.rs` — orphan; AI.md §21 lists `terminal::cell` as required (`Cell` + `Attrs`).
- [ ] `src/terminal/parser.rs` — orphan; AI.md §21 lists `terminal::parser` as required (`vte`-driven escape parser).
- [ ] `src/terminal/renderer.rs` — orphan; AI.md §21 lists `terminal::renderer` as required (egui canvas painter).
- [ ] `src/ui/main_window.rs` — orphan; AI.md §21 lists `ui::main_window` as required (top-level chrome).
- [ ] `src/ui/tab.rs` — orphan; AI.md §21 lists `ui::tab` as required (tab state).
- [ ] `src/ui/tab_manager.rs` — orphan; AI.md §21 lists `ui::tab_manager` as required.
- [ ] `src/ui/screens/connection_editor.rs` — orphan; referenced by §5/§24 connection-editor flow.
- [ ] `src/ui/screens/connection_manager.rs` — orphan; duplicates `connection_list` purpose — decide which to keep.
- [ ] `src/ui/screens/settings.rs` — orphan; duplicates `settings_screen.rs` — decide which to keep.
- [ ] `src/ui/screens/sftp_browser.rs` — orphan; duplicates `sftp_browser_ui.rs` — decide which to keep.
- [ ] `src/ui/screens/terminal_view.rs` — orphan; required for the terminal pane.
- [ ] `src/ui/mod.rs:5` — `// pub mod dialogs;  // TODO: Create dialogs module` — dialogs subtree missing entirely.
- [ ] `src/lib.rs` re-exports `terminal::{TerminalEmulator, VtParser}` and `config::{Settings, Theme}` — but `config::Settings` does not exist; `Settings` lives in `storage::settings` (also re-exported from `storage::`). `config/mod.rs` defines a different `Settings` type. Two `Settings` types is a design collision.
- [ ] AI.md §21 lists multiple modules that have no file at all: `ssh::portknock`, `ssh::x11_proxy`, `ssh::mosh`, `ssh::mosh_client`, `ssh::bulk_import`, `terminal::recorder`, `crypto::keys`, `crypto::encryption`. Spec drift — they are documented as known stubs in §22 but the spec/module-map should mark them as Phase-N rather than current.

## Pass 3: API drift (API_DRIFT)

- [ ] **russh 0.40.2** — `client::Handler::check_server_key` takes `self` (consumed) and returns `Result<(Self, bool), _>`. The code uses `&mut self -> Result<bool, _>` (cf. `src/ssh/active_session.rs:45`, `src/bin/ssh_test.rs:27`). Pattern is wrong in every handler impl — must thread `self` through.
- [ ] **russh-sftp** — `Cargo.toml` pins `russh-sftp = "2.0"` but `Cargo.lock` resolves to **2.3.0**, and `src/sftp/client.rs` is written against the 2.3 `SftpSession` API (`SftpSession::new(stream)`, `read_dir`, `open`, `create`, `metadata().await`). Spec says 2.1.1. Pin and AI.md should agree with the actual resolved version.
- [ ] `src/ssh/connection.rs:320` — `let mut channel = jump_conn.handle.channel_open_direct_tcpip(...)` then channel is unused (warning); the jump-host implementation never actually tunnels — it returns the jump connection as a placeholder (line 332 comment confirms). Logic stub, not API drift, but worth flagging because it is masked by `unused_variables`.
- [ ] `src/ssh/forwarding.rs` rewrite uses `Arc<Handle<H>>` and `copy_bidirectional` — looks correct against 0.40.2 but cannot be verified until `active_session.rs` compiles. Needs a re-check after Pass 1 fix.

## Pass 4: Code quality (QUALITY)

Whitespace-strip damage — log/error messages with all spaces removed; user-visible:
- [ ] `src/main.rs:18` — `"StartingTabSSHDesktopv{}"`.
- [ ] `src/main.rs:52` — `"Failedtorunapplication:{}"`.
- [ ] `src/app.rs:23` — `"Failedtoinitializeappstate:{}"`.
- [ ] `src/app.rs:43` — `"Newtab"`.
- [ ] `src/ssh/connection.rs:327` — `"Establishedtunnelthroughjumphostto{}:{}"`.
- [ ] `src/platform/linux.rs:4` — `"Linuxplatforminitialization"`.
- [ ] `src/platform/macos.rs:4` — `"macOSplatforminitialization"`.
- [ ] `src/platform/windows.rs:4` — `"Windowsplatforminitialization"`.
- [ ] `src/platform/bsd.rs:4` — `"BSDplatforminitialization"`.
- [ ] `src/ui/screens/sftp_browser_ui.rs:131,142` — `"Download:{}"`, `"Delete:{}"`.
- [ ] `src/ui/screens/forwarding_screen.rs:116` — `"SOCKSproxyon:{}"`.
- [ ] `src/sftp/operations.rs:20,67` — `"Listingdirectory:{}"`, `"Createdirectory:{}"`.
- [ ] `src/config/themes.rs:124` — `"Failedtoparsebundledtheme:{}"`.
- [ ] `src/utils/errors.rs:43-52` — 10 `Display` strings ("Connectionfailed", "Authenticationfailed", "Hostkeyerror", "Filetransfererror", "Portforwardingerror", "Databaseerror", "IOerror", "Parseerror", "Configurationerror", "Error"). These are user-facing error messages.

Stubs and placeholders in production:
- [ ] `src/sftp/operations.rs:19,31,44,52,59,66,73,80` — 8 methods are `TODO: Implement with russh SFTP` stubs returning fabricated success; the actual implementation lives in `sftp/client.rs`. `SftpOperations` should either delegate to `SftpClient` or be deleted.
- [ ] `src/ui/screens/connection_manager.rs:165,193,245` — TODO markers (and the file is orphan).
- [ ] `src/ui/screens/settings.rs:589,640,665,671,677,700,724,730,743` — 9 TODO markers for unimplemented dialogs (and the file is orphan).
- [ ] `src/ui/screens/connection_editor.rs:238` — TODO (file is orphan).
- [ ] `src/ui/mod.rs:5` — `// pub mod dialogs;  // TODO: Create dialogs module`.
- [ ] `src/main.rs:4` — `#![allow(dead_code, unused_variables, unused_imports)] // TODO: Remove after fixing compilation errors` — global crate-wide silencer of three real diagnostic categories.

Warnings (all 18 from current `cargo check --lib`):
- [ ] `src/ssh/config_parser.rs:3` unused import `anyhow`.
- [ ] `src/ssh/config_parser.rs:5` unused import `PathBuf`.
- [ ] `src/sftp/browser.rs:3` unused import `anyhow::Result`.
- [ ] `src/sftp/operations.rs:4` unused import `PathBuf`.
- [ ] `src/sftp/transfer.rs:3` unused import `anyhow::Result`.
- [ ] `src/sftp/transfer.rs:4` unused import `Path`.
- [ ] `src/terminal/emulator.rs:3` unused imports `AnsiColor`, `CellStyle`.
- [ ] `src/ui/keyboard.rs:3` unused import `Modifiers`.
- [ ] `src/ui/screens/settings_screen.rs:4` unused imports `BellStyle`, `CursorStyle`.
- [ ] `src/ssh/connection.rs:320` `let mut channel` — `mut` not needed (and `channel` unused — see logic stub above).
- [ ] `src/ssh/connection.rs:295,296` unused `target_user`, `target_creds` — jump-host stub.
- [ ] `src/sftp/operations.rs:79` unused `path` parameter on `stat` — operations stub.
- [ ] `src/terminal/emulator.rs:21` unused `cols`, `rows` — emulator constructor ignores its dimensions.
- [ ] `src/ui/screens/settings_screen.rs:19` unused `ctx`.
- [ ] `src/ui/screens/sftp_browser_ui.rs:33` unused `ctx`.

Other quality:
- [ ] `src/app.rs:19` — `let mut fonts = egui::FontDefinitions::default();` declared `mut` but never modified before being passed to `set_fonts`.
- [ ] `src/app.rs:21` — `eprintln!` used for fatal error; should be `log::error!` + propagate.
- [ ] `src/main.rs:4` — global `#![allow]` of `dead_code`/`unused_variables`/`unused_imports` is hiding real problems across the entire binary.
- [ ] Several `#![allow(dead_code)]` at module level in `src/ssh/mod.rs:3`, `src/sftp/mod.rs:3` — masks any future drift.

## Pass 5: Tests (TEST)

- [ ] `tests/` contains an entire integration tree (`tests/integration/*`, `tests/unit/*`, `tests/common/mod.rs`) that is **not** wired up. Only `tests/ssh_config_test.rs` is declared in `Cargo.toml` (`[[test]] name = "ssh_config"`). The integration and unit subdirectories are unreachable by `cargo test`.
- [ ] `tests/integration/` and `tests/unit/` use a non-standard layout — Cargo runs `tests/*.rs` as separate crates; subdirectories require an explicit `[[test]]` entry per file or a single dispatcher `tests/integration.rs` + `mod` declarations.
- [ ] `tests/common/mod.rs` exists but no `tests/*.rs` dispatcher consumes it.
- [ ] `tests/ssh_config_test.rs` will not run today — it imports `tabssh::ssh::SshConfigParser`, which requires the lib to compile (blocked by Pass 1).
- [ ] No tests exist for: terminal vte parsing, sftp client, port forwarding, jump host, keychain, theme loader, settings persistence, database schema bootstrap. AI.md §21 lists all of these as required modules.

## Pass 6: Spec / rules compliance (SPEC)

- [ ] `Cargo.toml:52` `lto = true` — rust convention requires `lto = "fat"`.
- [ ] `Cargo.toml:54` `strip = true` — rust convention requires `strip = "symbols"`.
- [ ] `rust-toolchain.toml` missing at repo root — required by rust conventions.
- [ ] `deny.toml` missing — required by rust conventions; `cargo-deny check` cannot run.
- [ ] `LICENSE.md` exists but has no third-party attribution section regenerated from `cargo about` (Cargo.lock changed 2026-06-12).
- [ ] `Cargo.toml:13` pins `russh-sftp = "2.0"` while lock resolves 2.3.0; either widen the pin or pin exact.
- [ ] AI.md §22 says "50 errors as of 2026-05-01" — out of date; only 1 hard error remains. Spec should be updated post-fix, not now.
- [ ] AI.md §21 module map references `ui::dialogs` as part of "components / dialogs / screens" but the subtree is absent.
- [ ] `PLAN.md` exists (human-owned) — fine.
- [ ] `TODO.AI.md` exists with extensive content — fine, but Phase 0 entries should be reconciled after current build state is re-baselined.
- [ ] No `CHANGELOG.md` / `SUMMARY.md` / `NOTES.md` / `REPORT.md` / `ANALYSIS.md` / `COMPLIANCE.md` at root — good.
- [ ] No `.env` files committed — good.
- [ ] No `config/`, `data/`, `logs/`, `tmp/`, `build/`, `dist/`, `vendor/`, `node_modules/` at root — good.
- [ ] `docker/Dockerfile` present — good. (Spec asks for `docker/Dockerfile.build` for the `:build` image; current single file may or may not satisfy that — verify.)

## Notes

- All warnings cited above are from `cargo check --lib --target x86_64-unknown-linux-gnu --keep-going` on commit-HEAD inside `rust:bookworm`. The user-requested target (`x86_64-unknown-linux-musl`, set by `.cargo/config.toml` default) cannot type-check `async-recursion` proc-macro under musl in the cargo registry path; this is an environment quirk, not a project bug.
- No additional errors will become visible until Pass 1's single E0195 is resolved; expect a second wave of errors from bin/test targets and from `src/ssh/connection.rs::check_server_key` once the lib compiles. Re-run cargo check after the fix to surface the next layer.
