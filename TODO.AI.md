# TODO.AI.md

## Spec clarification needed

- **AI.md Directory Naming conflict (resolved for now, keep singular):** the
  generic "Directory Naming" rule (AI.md line 1199) states all Rust module
  directories must be plural, but AI.md's own concrete Project Layout
  example for this project (lines 339-345, repeated at 731-742) shows
  `platform/` and `ui/` as singular, matching the current repo structure.
  User decision: treat the project-specific example as authoritative;
  `src/ssh/`, `src/sftp/`, `src/crypto/`, `src/platform/`, `src/storage/`,
  `src/terminal/`, `src/ui/` stay singular. If AI.md is ever revised to
  resolve this contradiction explicitly, re-check this decision.

## Pending security wiring (found during audit 2026-08-01)

- **HIGH — Host-key verification is not wired in the live connect path.**
  `src/ssh/active_session.rs` `SessionHandler::check_server_key()` and
  `src/ssh/connection.rs` `SshClientHandler::check_server_key()` both log the
  fingerprint and unconditionally return `Ok(true)`, so every real SSH/SFTP
  connection accepts any server host key with no known-hosts comparison. The
  correct TOFU logic in `connection.rs` `verify_host_key` is `#[allow(dead_code)]`
  and never called, and `Settings.strict_host_key_checking` (default `true`) is
  never consulted. Phase 1.1: wire `check_server_key` to `verify_host_key`
  against the `known_hosts` table, prompt on first-seen, reject on mismatch.
- **TOFU auto-accepts new hosts without prompting.** Even the (currently dead)
  `verify_host_key` stores a first-seen key and returns `Ok(true)` with a
  `// should prompt user` comment. When wired, surface a confirmation dialog
  instead of silent trust-on-first-use.
- **Non-constant-time fingerprint comparison** (`connection.rs`
  `known_key.fingerprint == key_info.fingerprint`). Host-key fingerprints are
  not secret so this is not exploitable today, but the PIN/pairing-token
  comparisons the spec requires must use `subtle::ConstantTimeEq`. No `subtle`
  dependency exists yet — add it when the sync/pairing paths land.
- **"Save password in keychain" checkbox is unwired.** The `save_password`
  field / checkbox in `src/ui/screens/connection_editor.rs` never calls
  `KeychainManager::store_password`. Nothing leaks (credentials are simply not
  persisted), but the feature does not work. Wire it into the save flow.

## Pending runtime modes (found during audit 2026-08-01)

- **TUI and CLI connection modes are not implemented (PART 3 / PART 7).** The
  universal flags (`--help`/`-h`, `--version`/`-v`, `--debug`, `--color`) and
  GUI/TUI/CLI auto-detection (`src/cli.rs`) are now implemented, and `--ui gui`
  launches the GUI. But `--ui tui`/`--ui cli` and headless/remote-shell
  detection currently exit with a "not yet available" message. The
  README-documented direct-connect forms (`tabssh user@host`, `--connect NAME`,
  `-p`, `-i`) are parsed by neither yet — implement the connection-intent args
  plus a real TUI and CLI SSH path, then un-gate them and update README §CLI.

## Pending attribution / license surface (found during audit 2026-08-01)

- **LICENSE.md third-party region is an unpopulated placeholder.** The
  `<!-- GENERATED -->` region of `LICENSE.md` still reads "*Run `cargo about
  generate about.hbs` to regenerate this section.*" instead of the real
  cargo-about output, so vendored-C and transitive crates (rusqlite/bundled
  SQLite, ring, etc.) carry no attribution text. `about.toml` and `about.hbs`
  exist and are correct; the generation step has simply never been run.
  `cargo-about` is NOT present in the local sandbox image (`cargo about` →
  "no such command: about"), although AI.md PART 10/11 require it to be
  pre-installed in `casjaysdev/rust:latest`. Regenerate the region wherever
  cargo-about is available (real CI or a fixed image), then commit.
- **No cargo-about attribution-drift CI gate.** `ci.yml` runs the
  `cargo deny check` gate but not the `cargo about generate about.hbs` +
  `sed`/`diff` drift check that AI.md PART 10 "Suggested CI Steps" / PART 12
  Quality Checklist (line 1938) mandate. Add it once the image reliably
  ships cargo-about — adding it before that would make every CI run red.
- **User-visible licenses surface is missing (PART 11 line 1857-1861, PART 12
  line 1922).** There is no CLI `--licenses`/`--credits` flag, no GUI
  "About -> Open Source Licenses" entry reading the embedded `LICENSE.md`.
  Wire this together with the CLI/TUI connection work above, since CLI mode
  is not yet implemented and the flag should print the same `include_str!`ed
  blob the GUI About screen shows.

## CI failure (found post-push 2026-08-30, unrelated to the triggering commit)

- **`vuln-scan` job fails on RUSTSEC-2026-0257 (`webbrowser` 1.2.1).**
  `cargo audit` in CI run
  https://github.com/tabssh/desktop/actions/runs/33314492262 (triggered by
  commit `008fac749da4`, a docs-only `IDEA.md` fix — confirmed unrelated)
  fails with exit code 1 on "Unix `BROWSER` handling allows browser argument
  injection", disclosed 2026-07-29, fixed upstream in `webbrowser` >=1.2.2.
  Bump the `webbrowser` dependency (direct or transitive — check
  `Cargo.lock`) to >=1.2.2 and re-run `cargo audit` clean.

## Follow-up from bootstrap session

- `make build`/`make test` fail in sandboxed environments because
  `casjaysdev/rust:latest`'s shared entrypoint tries to auto-configure an
  SMTP relay before running the passed command, and that fails without
  outbound network access. Verified `cargo build`/`cargo test` work cleanly
  when bypassing the entrypoint (`--entrypoint /bin/sh -c '...'`). Confirm
  on a real machine/CI whether this also occurs there; if so, add an
  entrypoint override to `DOCKER_RUN` in the Makefile.
