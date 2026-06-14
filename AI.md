# {PROJECT_NAME} Specification

**Name**: {project_name}

**About this file:** `TEMPLATE.md` is the master template. When applied to a project, this file is copied (or symlinked) into the project as `AI.md`. Throughout this document, all references to `AI.md` refer to that resulting file in a real project.

**Note:** `{PROJECT_NAME}` and `{project_name}` in this file are reference tokens, not setup-time text replacements. Their values are resolved from `IDEA.md ## Project variables` while `AI.md` remains read-only.

---

# PROJECT DESCRIPTION

**See `IDEA.md` for project-specific details.**

---

# SOURCE OF TRUTH AND IDEA.md PRECEDENCE

**See `IDEA.md` for features, data models, and business rules.**

IDEA.md is the project PLAN. AI.md (this file) is the SOURCE OF TRUTH.

| File | Role | Update When |
|------|------|-------------|
| **AI.md** | SOURCE OF TRUTH - implementation rules (readonly template copy) | No — use SPEC.md for project-specific rule overrides |
| **SPEC.md** | Project-specific rule overrides — created only when a rule must contradict the template or global. May be empty. SPEC.md wins over AI.md. | When a project rule must differ from the template or global |
| **IDEA.md** | PROJECT PLAN - must follow AI.md | Features change, project variables change |

**Rule hierarchy:** SPEC.md > AI.md > global CLAUDE.md. If SPEC.md and AI.md conflict, SPEC.md wins — that is its purpose.
**Rule:** If AI.md and IDEA.md conflict, AI.md wins. Fix IDEA.md.

## IDEA.md Required Layout

**Every IDEA.md MUST have exactly these three top-level sections, in this order. For a fillable template plus worked examples (Notes / Feeds / dotctl), see PART 13 → "IDEA.md REFERENCE".**

```markdown
## Project description

(Full project description — what the project is, who uses it, what problem it solves.)

## Project variables

(All project variables in `key: value` form. Required keys at minimum: `project_name`,
`project_org`, `internal_name`, `internal_org`. Add more as the project needs — `app_name`,
`official_site`, `maintainer_name`, `maintainer_email`, `crate_name`, etc.)

Example:

    project_name:  notes
    project_org:   casjay
    internal_name: notes        # FROZEN — set once at first-time setup, never edit
    internal_org:  casjay       # FROZEN — set once at first-time setup, never edit
    app_name:      Notes
    crate_name:    notes
    official_site: https://notes.example.com

## Business logic

(Full business spec — the WHAT, not the HOW. Features, data models, user flows,
permission rules, business invariants, platform targets, input modes, accessibility,
security assumptions, and any exceptions.)
```

**Rules for `## Project variables`:**
- One variable per line: `key: value`
- Keys are **lower_snake_case** only
- The setup flow renders `{KEY_UPPER}` automatically by uppercasing the lowercase key
- Never guess values: use commands and existing files
- If a placeholder referenced by AI.md has no entry in `## Project variables`, setup MUST stop and ask instead of inventing a value

**Rules for `## Business logic`:**
- It MUST define the actual product scope for THIS project - not generic boilerplate
- It MUST state which app surfaces exist: GUI, TUI, CLI, or a subset
- It MUST define user flows, stored data, trust boundaries, abuse cases, and platform constraints
- If a security-sensitive choice is intentionally allowed, the reason MUST be documented there

## Migrating Existing `CLAUDE.md` Into `IDEA.md`

**If a repository already has a pre-template `CLAUDE.md` or `.claude/CLAUDE.md` with real project details, those project details MUST be migrated into `IDEA.md`.**

**What belongs in `IDEA.md`:**
- project description / elevator pitch
- project-specific terminology
- project variables that can be expressed as `key: value`
- business logic, roles, flows, constraints, trust boundaries, abuse cases, and security exceptions

**What does NOT belong in `IDEA.md`:**
- generic Claude/Copilot usage instructions
- loader boilerplate whose job is only to point at `AI.md`
- duplicated global implementation rules that already live in `AI.md`
- stale code snippets, one-off notes, or tool chatter with no business/spec value

**Migration rules:**
1. Read existing `CLAUDE.md` and `.claude/CLAUDE.md` first - never overwrite blindly
2. Extract valid project-specific content and reorganize it into the required `IDEA.md` layout
3. Normalize discovered variables into lower_snake_case `key: value` entries
4. If `internal_name` cannot be proven, initialize it to `project_name` on first migration and treat it as frozen after that. Do the same for `internal_org` ← `project_org`.
5. If statements from `CLAUDE.md` or `.claude/CLAUDE.md` conflict with `AI.md`, `AI.md` wins
6. After migration, keep root `CLAUDE.md` and/or `.claude/CLAUDE.md` only as short efficient loaders and keep the real plan/spec in `IDEA.md`
7. Never silently discard meaningful project-specific content; migrate it, trim it, or explicitly ask where it belongs

---

# PART 0: CRITICAL RULES - READ FIRST

## THIS IS A STRICT SPECIFICATION - NOT GUIDELINES

- Every item in this specification MUST be followed exactly unless explicitly marked optional
- This is not a suggestion document
- There are no silent exceptions
- If the spec says X, do X - not “improved X”
- If something seems wrong, follow it and flag it; do not silently rewrite intent

## ⚠️ CRITICAL: File Paths and Project Root

- All paths are relative to the project root unless explicitly noted
- Do not scatter top-level files unnecessarily
- Runtime-generated files are not committed
- AI must not move the project root or invent sibling repositories

## ⚠️ CRITICAL: AI.md is the Source of Truth

- `AI.md` is read-only during routine work
- `IDEA.md` is where project-specific values and product rules live
- Loader files (`CLAUDE.md`, `.claude/CLAUDE.md`) stay short and point back to `AI.md`
- If a loader file and `AI.md` disagree, `AI.md` wins

## ⚠️ CRITICAL: Keep Documentation in Sync

Update these when their subject changes:
- `IDEA.md` when features or variables change
- `README.md` when install, usage, or packaging changes
- `LICENSE.md` when dependencies or attribution changes
- CI/CD docs or scripts when release mechanics change

## ⚠️ CRITICAL: One Coherent Product

This template defines **one Rust application** with shared core logic and up to three presentation layers:
- GUI (preferred when available)
- TUI (fallback for interactive terminals)
- CLI (fallback for non-interactive/plain execution)

Single-process, single binary. The app may make outbound network calls to remote services it consumes; it does not host any.

## ⚠️ CRITICAL: No Host Toolchain or Binary Execution

The Rust toolchain and any compiled artifact from this project MUST NOT run on the host machine.

- Never invoke `cargo`, `rustc`, `rustfmt`, `clippy`, `cargo test`, `cargo run`, `cargo doc`, `cargo build`, `cargo install`, or any project binary from `target/` directly on the host
- Every build, test, lint, format-check, doc-gen, run, install, package, and manual debug session executes inside a project-provided Docker container
- The host's role is limited to editing source files, version control, and orchestrating Docker
- CI and local commands documented in this spec are illustrative shapes; the **actual invocation** is always Docker-wrapped (see PART 5 → "Docker Rule")
- If a contributor's environment cannot run Docker, they cannot build/test this project — that is intentional, not a bug

This rule has no opt-out. There is no "just this once" exception for `cargo test` on the host.

## ⚠️ CRITICAL: X11 AND Wayland Are Both Required

If this project ships a GUI surface, it MUST support **both** X11 and Wayland as first-class display backends.

- Wayland-only GUIs are not acceptable
- X11-only GUIs are not acceptable
- The chosen GUI stack/toolkit must be one that natively supports both, or the app must integrate both backends
- Display detection at runtime must consider `WAYLAND_DISPLAY` AND `DISPLAY` and pick the appropriate backend
- GUI smoke testing inside Docker MUST be runnable against both an X11 socket and a Wayland socket forwarded from the host (see PART 5 → "Docker Rule")
- IDEA.md may declare GUI out of scope, but it may NOT declare "X11 only" or "Wayland only"
- Reconciling X11/Wayland with the static-binary rule (see "Single Static Binary" below): use display crates that avoid **link-time** C dependencies. For X11, the preferred option is `x11rb` (pure-Rust, talks to the X server's Unix socket directly with no `libX11` involvement); `x11-dl` is also acceptable because it `dlopen`s `libX11` lazily at runtime instead of link-time. For Wayland, use `wayland-client` / `wayland-rs` with the `dlopen` feature, which removes the link-time dependency on `libwayland-client.so` by loading it lazily at runtime. In all of these cases the binary has no link-time C dependency for display I/O — that is the rule. The Rust GUI ecosystem (e.g., `winit`, and toolkits built on it) typically relies on `x11-dl` + `wayland-client+dlopen`; that path is compliant.

## ⚠️ CRITICAL: Rust-Only Application

This project's source code is **exclusively Rust**.

- All application code, library code, build automation, and test code in this repository is written in Rust
- No C, C++, Objective-C, Swift, Go, Python, JavaScript, TypeScript, or shell-script source files contribute to the produced binary
- `build.rs`, `xtask/`, and any task automation are written in Rust — not Make, not Bash, not Python
- Small `docker/rootfs/usr/local/bin/entrypoint.sh` and `docker/` shell helpers are tolerated because they orchestrate the container, not the application; they MUST NOT contain application logic
- Third-party crates that internally vendor C code (e.g., compression, crypto, SQLite, GUI) are allowed only when (a) no pure-Rust equivalent is viable, (b) the C code is statically linked into the final binary, (c) it does NOT require a system C lib at runtime (no `*-sys`-style dynamic linkage to system-installed `.so`/`.dylib`/`.dll`), and (d) the dependency is documented in `IDEA.md` and `LICENSE.md`. The Pure-Rust Library Stack in PART 5 pre-grants this exception for `ring` only — small, audited, ubiquitous, and effectively unavoidable for crypto performance — which still requires LICENSE.md attribution but does not need a per-project IDEA.md write-up. Larger vendored-C dependencies like `rusqlite` with `bundled` remain a per-project IDEA.md exception.
- **Prefer pure-Rust crates whenever a viable one exists.** Pure-Rust crates are what make the single-static-binary rule and cross-platform GUI (Windows / macOS / Linux / BSD) achievable in practice — every `*-sys` crate dragged in becomes a portability and build-system tax. See PART 5 → "Pure-Rust Library Stack" for the recommended crate-by-capability list.
- Never introduce a build that requires a system C/C++ toolchain on the user's machine — the project Docker image is the only required build environment

## ⚠️ CRITICAL: Single Static Binary

The deliverable for each supported target is **one statically linked binary** that runs without external runtime dependencies on the user's system beyond the kernel and (where applicable) display server sockets.

**Required build outputs per target:**

| Target | Linkage | Notes |
|--------|---------|-------|
| `x86_64-unknown-linux-musl` | fully static (musl libc) | Default Linux release target |
| `aarch64-unknown-linux-musl` | fully static (musl libc) | Default ARM64 Linux release target |
| `x86_64-unknown-freebsd` / `x86_64-unknown-netbsd` / `x86_64-unknown-openbsd` (and aarch64 equivalents) | native BSD libc, statically linked where the platform supports it | BSD targets use the platform's own libc — they are NOT musl; opt-in per IDEA.md |
| `x86_64-pc-windows-msvc` | static CRT (`+crt-static`) | Single `.exe` with no MSVC runtime DLL dependency |
| `aarch64-pc-windows-msvc` | static CRT (`+crt-static`) | Single `.exe` |
| `x86_64-apple-darwin` / `aarch64-apple-darwin` | system frameworks only (Apple does not allow static libSystem) | "Static" here means: no third-party dynamic libraries; only Apple-provided frameworks resolved at runtime |

**Rules:**
- No `glibc` runtime dependency on Linux release artifacts — musl by default. BSD release artifacts use the platform's native libc and are statically linked where the platform allows.
- No third-party `.so` / `.dylib` / `.dll` shipped alongside the binary
- No `LD_LIBRARY_PATH`, `DYLD_LIBRARY_PATH`, or wrapper-script tricks to find runtime libs
- `cargo build --release` inside the Docker image MUST produce a binary that passes a "no unexpected dynamic deps" check (`ldd`, `otool -L`, `dumpbin /dependents`) appropriate to the target
- X11 and Wayland are runtime-discovered display sockets, not link-time dependencies — see "X11 AND Wayland Are Both Required"
- Plugin systems, `dlopen` of arbitrary user code, and runtime extension loading from disk are forbidden unless IDEA.md explicitly defines a hardened plugin contract

## ⚠️ CRITICAL: Self-Contained Assets

The single binary contains **everything the app needs to function**. The user is not required to install fonts, themes, icons, templates, schemas, locales, or any other support file separately.

- Embed assets at build time using `include_bytes!`, `include_str!`, `rust-embed`, or an equivalent compile-time embedding pattern
- Fonts, icons, theme data, default config, JSON/YAML schemas, SQL migrations, web assets (if any), localization catalogs, default templates, and licenses-to-display all live inside the binary
- The `assets/` directory in the repo is a **build-time source tree** for embedding — it is not a runtime install target
- The app may **read** user-provided config and data from per-user paths (PART 4 path table), and may **write** cache/state there, but it must run with full default functionality when those paths are empty
- Network/CDN fetches at first run to "download missing assets" are forbidden
- A new install on an air-gapped machine MUST work end-to-end with only the binary present

## Licensing & Features

| Rule | Description |
|------|-------------|
| **MIT License** | All project code is MIT licensed unless IDEA.md explicitly states an additional compatible license policy |
| **3rd party attribution** | All third-party licenses are listed in `LICENSE.md`. Generation, allowlist/denylist, IDEA.md exception path, and the user-visible licenses surface live in PART 11 → "License Compliance" |
| **GPL / AGPL / LGPL denied by default** | Static linking would relicense the distributed binary away from MIT. Allowed only via a documented IDEA.md exception (PART 11 → "License Compliance") |
| **Free & open source** | No paid tiers, enterprise gating, or artificial feature segmentation |
| **No premium features** | GUI, TUI, and CLI surfaces expose the same core product capabilities where applicable |
| **No activation gates** | No license keys, phone-home unlocks, or paywalled code paths |

**NEVER implement:**
- upgrade/paywall prompts for core functionality
- hidden features enabled by payment tier
- telemetry-based licensing enforcement
- artificial limits used for monetization

---

# PART 1: PROJECT FILES & GOVERNANCE

## Project Files

| File | Purpose | Update When |
|------|---------|-------------|
| **AI.md** | Implementation spec (HOW) - SOURCE OF TRUTH, readonly template copy | No — use SPEC.md for project-specific rule overrides |
| **SPEC.md** | Project-specific rule overrides (optional, may be empty) | When a project rule must contradict the template or global |
| **IDEA.md** | Project plan (WHAT) | Features or variables change |
| **TODO.AI.md** | Task tracking (AI-owned) | Tasks added/completed |
| **TODO.md** | Task tracking (human-owned) | AI may mark done; never delete/empty |
| **PLAN.AI.md** | Implementation plan (AI-owned) | Planning new work |
| **PLAN.md** | Implementation plan (human-owned) | AI may mark done; never rewrite wholesale |
| **README.md** | User-facing install/usage docs | Usage changes |
| **LICENSE.md** | Project + dependency licenses | Dependency set changes |
| **release.txt** | Canonical release version when present | Release version changes |
| **site.txt** | Optional official site/homepage URL | Official site changes |
| **deny.toml** | `cargo-deny` license / advisory / bans / sources policy (PART 11 → "License Compliance") | Policy or allow/deny set changes |
| **about.toml** | `cargo-about` configuration (allowlist of accepted licenses for attribution generation) | Accepted-licenses set changes |
| **about.hbs** | `cargo-about` Handlebars template controlling the format of the generated `LICENSE.md` GENERATED region | Output-format changes |
| **Makefile** | Build, test, run targets for local development. Convenience wrappers around the Docker-wrapped commands. **Never called from CI workflow `run:` steps** (PART 10 → "CI/CD Rules") | Local-dev targets change |
| **.dockerignore** | Build-context exclusions; Rust-specific entries: `target/`. See `dockerfile_conventions.md` → ".dockerignore" for the full standard entry set (version control, `.env`, build artifacts, `volumes/`, OS files, markdown/LICENSE). `docker/`, `src/`, `Cargo.toml`, `Cargo.lock`, `build.rs`, `release.txt` are NEVER excluded. | Build-context surface changes |
| **renovate.json** | Single Renovate config covering Cargo deps, GitHub Actions SHAs, and Docker digests across all five providers. Dependabot forbidden (PART 9 → "Dependency Governance") | Update-policy changes |

## Mandatory Compliance Schedule

| When | Action | Purpose |
|------|--------|---------|
| Before each task | Read only the spec parts relevant to what you are about to implement — do not pre-load speculatively | Prevent token waste |
| Every 3-5 changes | Stop and verify against spec | Catch drift early |
| Before task completion | Full compliance check | Ensure correctness |
| When uncertain about a spec requirement | Read that specific section — never guess, never rely on prior-session memory | Accuracy without waste |

## Self-Validation Loop

**AI MUST verify its own work with real tools before reporting a task as done. Do not rely on "the code looks right."**

**This rule applies to EVERY change type covered by this template — library/API logic, GUI/TUI/CLI binaries, single-static-binary build, asset embedding, Docker, CI/CD, configuration, documentation, security — not only one category.** Whatever you touched, you verify.

Getting code correct on the first try is much harder than iterating with feedback. Close the loop every time. All execution goes through the project's containerised targets — never bare host cargo.

| Change type | How to verify |
|-------------|---------------|
| Library / API logic | Run the project's test target inside the container; exercise the API directly; compare output against expected |
| Behavior-preserving refactor | Diff outputs of old vs. new path on representative inputs (don't trust that the diff "looks right") |
| CLI binary | Run the binary in the container; exercise relevant flags including `--help`/`--version`; check stdout, stderr, and exit code |
| TUI binary | Run in the container; verify rendering, keyboard input, and screen redraw on resize/exit |
| GUI binary | Run in the container; verify the rendered window under BOTH X11 AND Wayland forwarding (per the X11/Wayland mandate); confirm input events reach the app |
| Single static binary requirement | Confirm the artifact is a single self-contained file and that it runs on a clean container with no extra runtime install |
| Asset embedding | Confirm assets are loaded from the binary itself (not from a host path); test on a container without the source tree mounted |
| Performance change | Measure before AND after — don't assume parallelism, caching, or "cleaner" code is faster |
| Bug fix | Reproduce the bug FIRST so you have a failing signal, then verify the fix makes it disappear; add a regression test where feasible |
| Configuration / settings | Start the binary with the new config; verify defaults; verify validation rejects bad input with a useful error |
| Docker / container build | Build and verify each image variant as appropriate to the change: `:devel` (debug binary) for logic changes, `:latest` (release binary) for release verification. Run the container; smoke-test the binary inside it; for GUI, verify display forwarding still works. |
| CI/CD workflow | Run the workflow on a branch (or equivalent dry-run); verify each job's exit status, not just YAML validity |
| Logging / error paths | Trigger the error path; verify the log line/structured event was emitted with expected fields |
| Security-sensitive change (auth, crypto, input validation, plugin/dlopen contracts) | Test both the success path AND attempted bypass paths; never assume a guard works without exercising it |
| Documentation / README | Render markdown locally; verify links, code samples, and example commands actually work |
| Type / lint / build correctness | The project's containerised check, clippy, and build targets — green across all |

**Iteration rules:**
- A failed check is data, not failure — adjust and re-run until green
- Never report "done" while any verification is still red
- If a check reveals the change is wrong in a way that can't be patched, revert and re-plan; do not paper over a failing check
- When verification is genuinely impossible in this environment (no display, no DB, no network): say so explicitly. List what was checked and what could not be, so the user knows where to look

**Reference:** based on published guidance about AI coding agent self-validation (Eivind Kjosbakken, Towards Data Science, 2026) — when an AI agent is given verification tools (output diffing, browser/display, test runners) and allowed to iterate, one-shot success rate, run length, and task complexity all improve substantially.

## Loader Files

| Tool | Primary Loader | Alternate Loader | Personal Override |
|------|----------------|------------------|------------------|
| Claude Code | `CLAUDE.md` | `.claude/CLAUDE.md` | `CLAUDE.local.md` |

**Loader rule:** loader files stay short. Long-form product content belongs in `IDEA.md`; long-form implementation policy belongs in `AI.md`.

---

# PART 2: RUST APPLICATION MODEL

## Product Model

This template targets a **single-binary, fully self-contained Rust application** that may expose:
- a native GUI
- a terminal UI (TUI)
- a plain CLI

The application may make outbound network calls to consume remote services it depends on (APIs, databases, object stores, update endpoints, etc.).

**Distribution model:** one statically linked binary per supported target. Everything the app needs at runtime — UI assets, fonts, icons, default config, schemas, templates, locales — is embedded inside that binary. See PART 0 → "Single Static Binary" and "Self-Contained Assets."

## Architectural Rule

Use **one shared core application layer** and thin presentation adapters:

```text
src/
├── main.rs                 # auto-detects GUI/TUI/CLI mode unless explicitly overridden
├── assets.rs               # compile-time asset embedding (rust-embed / include_bytes!)
├── app/                    # shared domain/application logic
├── config/                 # configuration loading + defaults
├── platform/               # OS/platform integration
├── ui/
│   ├── gui/                # optional native GUI
│   ├── tui/                # optional terminal UI
│   └── cli/                # optional plain CLI commands/output
├── state/                  # app state and persistence adapters
└── support/                # logging, errors, utilities
```

(See PART 5 → "Project Layout" for the full repository tree, including `docker/`, `assets/`, `packaging/`, `xtask/`, etc.)

**Rules:**
- Core behavior MUST live in shared modules, not be duplicated across GUI/TUI/CLI
- UI layers are adapters over the same business rules
- Optional `src/bin/*.rs` utilities are allowed only for real auxiliary tools, not as a substitute for clean architecture

## Binary Model

| Binary | Status | Purpose |
|--------|--------|---------|
| `{project_name}` | REQUIRED | Primary application binary — single statically linked artifact |
| `src/bin/*` helpers | DISCOURAGED | Permitted only when IDEA.md justifies a real auxiliary tool; each helper is itself a single static binary |

**Binary naming rules:**

Distribution artifact names follow the schema:

```
{project_name}-{platform}-{arch}{.ext}
```

| Token | Allowed values |
|-------|----------------|
| `{platform}` | `linux`, `windows`, `macos`, `freebsd`, `netbsd`, `openbsd` (lowercase, normalized OS name — never the full Rust target triple) |
| `{arch}` | `x86_64`, `aarch64`, `armv7`, `i686`, `riscv64`, … (the architecture portion of the Rust target triple) |
| `{.ext}` | `.exe` on Windows; empty everywhere else |

**Normalization rules:**
- Strip the `-musl` libc suffix from the artifact name. `x86_64-unknown-linux-musl` → `linux-x86_64`, NOT `linux-x86_64-musl`. Static linkage is the project default (PART 0 → "Single Static Binary"), so calling it out in the filename is redundant.
- Strip the vendor token (`unknown`, `pc`, `apple`) from the artifact name.
- Strip the ABI token (`gnu`, `msvc`, `eabihf`, …) from the artifact name unless multiple ABIs of the same `{platform}-{arch}` ship in the same release; in that rare case append `-{abi}` after `{arch}`.
- `apple-darwin` maps to `macos`. `pc-windows-msvc` maps to `windows`. `unknown-linux-musl` maps to `linux`.

**Worked examples:**

| Rust target triple | Artifact name |
|--------------------|---------------|
| `x86_64-unknown-linux-musl` | `{project_name}-linux-x86_64` |
| `aarch64-unknown-linux-musl` | `{project_name}-linux-aarch64` |
| `armv7-unknown-linux-musleabihf` | `{project_name}-linux-armv7` |
| `x86_64-pc-windows-msvc` | `{project_name}-windows-x86_64.exe` |
| `aarch64-pc-windows-msvc` | `{project_name}-windows-aarch64.exe` |
| `x86_64-apple-darwin` | `{project_name}-macos-x86_64` |
| `aarch64-apple-darwin` | `{project_name}-macos-aarch64` |
| `x86_64-unknown-freebsd` | `{project_name}-freebsd-x86_64` |

**Other rules:**
- Local (in-tree) primary binary name: `{project_name}` (no platform/arch suffix during local development inside the Docker image)
- If optional helper binaries exist, use `{project_name}-{tool}` for the in-tree name and `{project_name}-{tool}-{platform}-{arch}{.ext}` for distribution
- Checksum files mirror the artifact name plus `.sha256` (e.g., `{project_name}-linux-x86_64.sha256`)

**Single-binary rule:** the default user experience is "download one file, run it." The primary binary MUST be self-sufficient (PART 0 → "Self-Contained Assets"). Helper binaries are not a substitute for putting features into the primary binary; if a feature can live behind a CLI subcommand of the primary binary, it MUST.

## GUI/TUI/CLI Capability Rule

- A project may implement one, two, or all three surfaces
- If GUI exists, it is the preferred interactive experience on capable local desktops
- If TUI exists, it is the preferred interactive fallback for capable terminals
- CLI is the universal fallback and the default for automation/non-interactive use
- IDEA.md MUST declare which surfaces are actually in scope

---

# PART 3: RUNTIME MODE SELECTION

## Selection Priority

**Automatic runtime selection priority is:**
1. **GUI**
2. **TUI**
3. **CLI**

**Override priority is:**
1. explicit CLI flag / subcommand (`--ui gui|tui|cli`, `gui`, `tui`, `cli`, etc.)
2. config file value
3. environment override
4. automatic detection using the priority above

## Smart Detect Rules

### GUI

Choose GUI when **all** are true:
- GUI support is compiled and enabled for the project
- the process is **not** running in SSH/MOSH/remote-shell style contexts
- a desktop/session display stack is available
- the invocation is interactive enough for windowed UX

**Treat these as remote-shell / non-local GUI blockers:**
- `SSH_CONNECTION`, `SSH_CLIENT`, `SSH_TTY`
- `MOSH_IP`, `MOSH_KEY`
- explicit headless environment from config or flag

**Treat these as positive GUI/display signals (platform-appropriate):**
- `WAYLAND_DISPLAY` — Wayland session (MUST be supported by the app on Linux/BSD)
- `DISPLAY` — X11 / XWayland session (MUST be supported by the app on Linux/BSD)
- a local Windows desktop session
- a local macOS Aqua/session launch

**Backend selection on Linux/BSD when both signals are present:**
- Prefer Wayland when `WAYLAND_DISPLAY` is set; if connecting to that Wayland socket fails at runtime, fall back to X11 instead of erroring out
- Fall back to X11 (`DISPLAY`) when `WAYLAND_DISPLAY` is unset or the user/config explicitly requests X11
- Both backends MUST be exercised in tests (PART 0 → "X11 AND Wayland Are Both Required")

### TUI

Choose TUI when GUI is not selected and **all** are true:
- TUI support is implemented
- stdin and stdout are TTYs
- `TERM` is not `dumb`
- output is interactive (not piped for machine consumption)
- the environment is not clearly no-format/no-interaction

**Avoid TUI when any of these are true:**
- `TERM=dumb`
- stdout or stdin is not a TTY
- `CI=true` or similar automation context
- `NO_COLOR=1` or another plain-output requirement **when the TUI depends on color/alternate-screen/cursor control**
- explicit `--plain`, `--json`, `--quiet`, or script-oriented modes

### CLI

Choose CLI when:
- GUI and TUI are unavailable or inappropriate
- the app is running non-interactively
- output is piped/redirected
- machine-readable output is requested
- automation or plain-text mode is requested

CLI is the **required** fallback and the default for non-interactive work.

## Reference Detection Logic

```rust
pub enum UiMode {
    Gui,
    Tui,
    Cli,
}

pub fn detect_ui_mode(env: &Env, caps: &Capabilities) -> UiMode {
    if let Some(forced) = env.forced_ui_mode() {
        return forced;
    }

    let remote_shell = env.has_any(&[
        "SSH_CONNECTION",
        "SSH_CLIENT",
        "SSH_TTY",
        "MOSH_IP",
        "MOSH_KEY",
    ]);

    let display_available = env.has("WAYLAND_DISPLAY")
        || env.has("DISPLAY")
        || caps.local_windows_session
        || caps.local_macos_session;

    if caps.gui_supported && !remote_shell && display_available && caps.interactive_launch {
        return UiMode::Gui;
    }

    let plain_or_noninteractive = !caps.stdin_tty
        || !caps.stdout_tty
        || env.is("TERM", "dumb")
        || env.truthy("CI")
        || (env.truthy("NO_COLOR") && caps.tui_requires_formatting)
        || caps.machine_output_requested;

    if caps.tui_supported && !plain_or_noninteractive {
        return UiMode::Tui;
    }

    UiMode::Cli
}
```

## Behavior Rules by Mode

| Mode | Primary UX | Good For | Avoid |
|------|------------|----------|-------|
| GUI | native windows/dialogs/desktop integration | local desktop use | headless/SSH automation |
| TUI | structured terminal app | interactive terminal workflows | `TERM=dumb`, plain-output contexts |
| CLI | plain commands/text/json | automation, scripts, CI, pipes | none |

---

# PART 4: PRIVILEGE ESCALATION & SYSTEM INTEGRATION

## Core Rule

Privilege escalation support is allowed, but it is **optional** and must be used **only when the user explicitly requests it**.

## NEVER Do

- Never auto-run `sudo`, `pkexec`, UAC elevation, or similar escalation just because a path is protected
- Never silently switch from user-scope install to system-scope install
- Never require elevation for normal app launch, per-user config, or per-user updates
- Never block core app usage behind root/admin requirements unless IDEA.md explicitly defines a genuinely privileged tool

## Allowed Uses (Explicit Request Only)

- system-wide install into protected directories
- system-wide uninstall or cleanup
- privileged file associations / shell integration / desktop registration when explicitly requested
- service registration only if the project truly implements a background service and IDEA.md says so

## Default Scope Rule

Default to **user scope**:
- user config
- user data
- user cache
- per-user desktop integration
- user-local updates/installations

## Path Rule

Prefer platform-standard user directories:

**On-disk paths use the frozen pair `{internal_org}` and `{internal_name}` only — never the mutable `{project_org}` / `{project_name}`. This is the rule that protects user data across project/org renames.**

| Purpose | Linux / BSD | macOS | Windows |
|---------|-------------|-------|---------|
| Config | `~/.config/{internal_org}/{internal_name}/` | `~/Library/Application Support/{internal_name}/config/` | `%AppData%\\{internal_org}\\{internal_name}\\config\\` |
| Data | `~/.local/share/{internal_org}/{internal_name}/` | `~/Library/Application Support/{internal_name}/data/` | `%LocalAppData%\\{internal_org}\\{internal_name}\\data\\` |
| Cache | `~/.cache/{internal_org}/{internal_name}/` | `~/Library/Caches/{internal_name}/` | `%LocalAppData%\\{internal_org}\\{internal_name}\\cache\\` |
| Logs | `~/.local/state/{internal_org}/{internal_name}/logs/` | `~/Library/Logs/{internal_name}/` | `%LocalAppData%\\{internal_org}\\{internal_name}\\logs\\` |

**Rule:** Both `{internal_name}` and `{internal_org}` anchor on-disk identifiers and stable OS-registered names (Bundle IDs, package IDs, dbus names, keychain entries, updater channels). A rename of `{project_name}` or `{project_org}` MUST NOT silently move user data or change those identifiers.

---

# PART 5: RUST TOOLCHAIN, BUILD, AND PACKAGING

## Toolchain Rules

| Rule | Description |
|------|-------------|
| Rust edition | Use the current stable Rust edition declared in `Cargo.toml` |
| Toolchain channel | Stable by default; toolchain version pinned in `rust-toolchain.toml` |
| Targets | Static targets pre-installed in the Docker image: `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, Windows MSVC targets with `+crt-static`, Apple Silicon + x86_64 macOS. BSD targets (`x86_64-unknown-{freebsd,netbsd,openbsd}` and aarch64 equivalents) are opt-in — added to the image only when IDEA.md declares them in scope (PART 0 → "Single Static Binary") |
| Formatting | `rustfmt` is required; default `max_width = 100` — do not override without a `rustfmt.toml` |
| Linting | `clippy` is required |
| Testing | `cargo test` is required |
| Docs | `cargo doc --no-deps` is required for public APIs/libraries inside the app workspace |
| Source language | Rust only (PART 0 → "Rust-Only Application") |

## Pure-Rust Library Stack

This is the recommended starting point for satisfying common application needs with crates that do **not** drag in `*-sys` C dependencies. Pure-Rust crates are what make a single static binary that works on Windows, macOS, Linux, and BSD achievable; every C-linked crate becomes a portability tax across that target matrix.

**Rule:** when a capability below is needed, prefer the pure-Rust option. Only deviate when (a) the pure-Rust option is not viable for the project's actual requirements, AND (b) the deviation is documented per the Rust-Only Application rule (PART 0 → "Rust-Only Application"). Note: PART 0 pre-grants the vendored-C exception for `ring` only — it requires LICENSE.md attribution but no per-project IDEA.md write-up. All other deviations (e.g., `rusqlite + bundled`, `zstd` / `zstd-safe`) require both an IDEA.md exception and LICENSE.md attribution.

| Capability | Preferred (pure Rust) | Avoid (drags in C) |
|------------|-----------------------|--------------------|
| TLS | `rustls` | `openssl`, `native-tls` (when it picks OpenSSL) |
| HTTP client | `reqwest` with `rustls-tls` (no `default-features`), `ureq` with `rustls` | anything pulling `openssl-sys` |
| Crypto | `RustCrypto/*` (`sha2`, `aes-gcm`, `ed25519-dalek`, …); `ring` is acceptable (asm + small C, statically linked, widely audited) | system OpenSSL via `openssl-sys` |
| Compression | `flate2` with `rust_backend` feature, `miniz_oxide`, `zune-inflate`; for zstd decode, `ruzstd` (pure Rust). For zstd encode there is no pure-Rust option today — `zstd` / `zstd-safe` (vendors C) requires an IDEA.md exception per PART 0 → "Rust-Only Application", same as `rusqlite + bundled` | `flate2` default backend with `libz-sys`, system zlib |
| Serialization | `serde` + `serde_json` / `toml` / `ron` / `postcard` / `bincode` | — |
| Async runtime | `tokio` (pure Rust), `smol`, `async-std` | — |
| CLI parsing | `clap` (no system deps), `argh`, `lexopt` | — |
| Logging / tracing | `tracing` + `tracing-subscriber`, `log` + `env_logger` | — |
| Error handling | `thiserror`, `anyhow`, `eyre` | — |
| Date / time | `time`, `jiff`, `chrono` | — |
| Locale / i18n | `icu4x` (pure-Rust ICU rewrite) | system ICU via `icu-sys` |
| Filesystem watching | `notify` (pure-Rust backends), `walkdir` | — |
| Config files | `figment`, `config`, `confy` | — |
| Window + input (GUI) | `winit` (cross-platform; on Linux uses `x11-dl` + `wayland-client` with `dlopen` — both are runtime-loaded, no link-time C dep, see PART 0 → "X11 AND Wayland Are Both Required") | `glfw-sys`, `sdl2-sys` |
| Renderer (GUI) | `wgpu` (talks Vulkan / Metal / DX12 / GL via runtime loaders, no C-link) | hand-rolled `gl-sys` / `vulkan-sys` link-time deps |
| GUI toolkit | `egui`, `iced`, `slint`, `floem`, `dioxus` (native renderer mode) — all pure Rust on top of `winit` + `wgpu` | `gtk-rs`, `qt-*`, `fltk-rs`, `tauri` (uses platform webview) |
| Font rendering | `fontdue`, `swash`, `ttf-parser`, `cosmic-text` | `freetype-sys`, `harfbuzz-sys` |
| Image decoding | `image`, `kamadak-exif`, `zune-image` | `libjpeg-sys`, `libpng-sys` |
| TUI | `ratatui` + `crossterm` (pure Rust on Windows / Unix) | `ncurses-rs` (links system ncurses) |
| Clipboard | `arboard` (pure-Rust on Linux via `x11rb` + Wayland data-device protocol; native APIs on macOS / Windows) | `clipboard` crate variants that link `libxcb` / GTK |
| Desktop notifications | `notify-rust` (pure-Rust D-Bus on Linux/BSD; native APIs on macOS / Windows) | `libnotify-sys` |
| File / save / open dialogs | `rfd` configured with the pure-Rust XDG portal backend on Linux | GTK-based `rfd` backend, `tinyfiledialogs-sys` |
| Theme detection | `dark-light` (pure Rust) | system polling that links GUI C libs |
| Secrets storage | `secret-service` (Linux/BSD freedesktop Secret Service via D-Bus), `security-framework` (Apple), `windows`/`windows-sys` (Windows) | `libsecret`, `libgnome-keyring`, `kwallet`, `keytar` |
| Local SQL | accept `rusqlite` with the `bundled` feature (statically vendors SQLite C, IDEA.md exception); evaluate `limbo` once stable for a fully Rust SQLite-compatible engine | `libsqlite3-sys` linking system SQLite |
| Networking primitives | `tokio` / `async-std` / std `net` types | — |
| Process / IPC | `interprocess` (cross-platform Unix-socket / named-pipe abstraction in pure Rust) | C-linked IPC bindings |
| UUID / hashing helpers | `uuid`, `blake3`, `xxhash-rust`, `ahash` | — |
| Display protocol (Linux/BSD) | `x11rb` (pure-Rust X11) **or** `x11-dl` (dlopen-style); `wayland-client` / `wayland-rs` with the `dlopen` feature (no link-time `libwayland-client`) | `xlib-sys`, `x11` (link-time-loaded variants only) |

This list is not exhaustive; treat it as the starting point. When introducing a new dependency:

1. Search crates.io for the pure-Rust option first
2. Check the dependency tree (`cargo tree`) for any transitive `*-sys` crate
3. If the only viable option pulls in C, document the exception in `IDEA.md` and confirm it can be statically linked into the final binary

## Cargo Commands

**All cargo invocations execute inside the project Docker container — never on the host.** The table below shows the *logical* command; the **actual** invocation is wrapped (e.g., `docker compose run --rm dev <cmd>` or `docker run --rm -it --name "{project_name}-XXXX" -v "$PWD":/work -w /work <image> <cmd>`). See "Docker Rule" below.

| Logical Command | Purpose |
|-----------------|---------|
| `cargo fmt --all` | Format code |
| `cargo fmt --all --check` | Formatting verification |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Lint enforcement |
| `cargo test --workspace --all-features` | Test suite |
| `cargo build` | Debug build |
| `cargo build --release` | Release build |
| `cargo doc --workspace --no-deps` | Documentation build |
| `cargo run -- [args]` | Execution (in container; with X11/Wayland forwarding if GUI) |
| `cargo install --path .` | In-container install for packaging/CLI testing |

**Examples of the actual wrapped form:**

```bash
# Format check
docker compose run --rm dev cargo fmt --all --check

# Lint
docker compose run --rm dev cargo clippy --workspace --all-targets --all-features -- -D warnings

# Tests
docker compose run --rm dev cargo test --workspace --all-features

# Release build (artifacts land in a mounted target/ inside the container)
docker compose run --rm dev cargo build --release

# Run with display forwarding via the `gui` compose service, which mounts
# the X11 / Wayland sockets and exports DISPLAY / WAYLAND_DISPLAY (see
# "Docker Rule" → "X11 and Wayland Forwarding" for socket/env wiring).
docker compose run --rm gui cargo run -- --ui gui
```

Bare `cargo …` invocations on the host are forbidden by PART 0 → "No Host Toolchain or Binary Execution."

## Build Rules

- **Pure Rust by default** — every dependency is pure Rust unless a specific exception is documented in `IDEA.md` and the C code is statically linked into the final binary (PART 0 → "Rust-Only Application")
- Release builds use `cargo build --release` against a static target (e.g., `--target x86_64-unknown-linux-musl`) inside the Docker image
- The final artifact MUST be a single statically linked binary per target (PART 0 → "Single Static Binary")
- Use `RUSTFLAGS="-C target-feature=+crt-static"` for Windows MSVC targets; use musl targets for Linux; for BSDs, use the platform's native target triple and statically link where the platform allows
- All runtime assets are embedded at compile time; the repo's `assets/` directory is build-time input only (PART 0 → "Self-Contained Assets")
- A static-binary self-check runs as part of CI (`ldd` on Linux musl artifacts must show "not a dynamic executable" or only kernel-vDSO; `otool -L` on macOS must show only Apple-provided frameworks; `dumpbin /dependents` on Windows static-CRT must show no MSVC runtime DLLs)
- If a workspace is used, keep one top-level `Cargo.toml` as the build entry point
- Recommended release profile in `Cargo.toml`:

```toml
[profile.release]
opt-level     = 3
lto           = "fat"
codegen-units = 1
strip         = "symbols"
panic         = "abort"
```

- Required `.cargo/config.toml` snippets for static linking (Bootstrap Checklist enforces presence):

```toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```

## Project Layout

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml     # pinned stable toolchain + targets
deny.toml               # cargo-deny license/advisory/bans/sources policy
about.toml              # cargo-about config (accepted-licenses allowlist)
about.hbs               # cargo-about Handlebars template for the generated attribution section
.cargo/
└── config.toml         # static-linking rustflags per target
src/
├── main.rs
├── app/
├── assets.rs           # compile-time asset embedding (rust-embed / include_bytes!)
├── config/
├── platform/
├── state/
├── support/
└── ui/
    ├── gui/
    ├── tui/
    └── cli/
assets/                 # BUILD-TIME ONLY: source tree embedded into the binary at compile time
docker/                 # REQUIRED: Dockerfile, docker-compose.yml, rootfs/usr/local/bin/entrypoint.sh, README.md
packaging/              # installer/manifests/bundle metadata
scripts/                # optional helper scripts that wrap Docker invocations (host-side wrappers); MUST NOT contain application logic
xtask/                  # Rust-based build/release automation crate (executed inside Docker)
tests/                  # integration tests
benches/                # optional benchmarks
examples/               # optional focused examples
```

The `assets/` directory in the repo holds source files (fonts, icons, default themes, schemas, locales, default config). They are embedded into the binary at compile time. The directory is **never** shipped or installed alongside the binary on a user's machine.

## Naming Rules

| Item | Rule | Example |
|------|------|---------|
| Files/modules | `snake_case` | `config_loader.rs` |
| Types/traits | `PascalCase` | `AppConfig`, `UiMode` |
| Functions | `snake_case` | `detect_ui_mode` |
| Constants/statics | `SCREAMING_SNAKE_CASE` | `DEFAULT_THEME` |
| Cargo package name (`[package].name`) | lowercase, hyphenated | `notes-app` |
| Rust crate identifier (used in `use`, `extern crate`) | lowercase, underscored — Cargo auto-converts hyphens | `notes_app` |

## Release Artifacts

- Primary binary follows the naming schema `{project_name}-{platform}-{arch}{.ext}` defined in PART 2 → "Binary Model" — single statically linked file, `-musl`/vendor/ABI tokens stripped
- Each release MUST publish artifacts for at minimum: `{project_name}-linux-x86_64`, `{project_name}-linux-aarch64`, `{project_name}-windows-x86_64.exe`, `{project_name}-windows-aarch64.exe`, `{project_name}-macos-x86_64`, `{project_name}-macos-aarch64` (subset acceptable only when IDEA.md narrows platform scope)
- A static-linkage verification step is part of release: `ldd` / `otool -L` / `dumpbin /dependents` output is captured and checked against an allowlist (kernel vDSO, Apple system frameworks, Windows kernel32/user32 etc.) — anything outside the allowlist fails the release
- No companion files (no `.so`, `.dylib`, `.dll`, no asset bundles, no font directories) ship next to the binary
- Include SHA-256 checksums for every published artifact, named `{artifact}.sha256`
- Include release notes that describe actual changes
- Include an SBOM (always — generated via `cargo-cyclonedx`; see PART 10 → "Suggested CI Steps" for the invocation, PART 11 → "Required Tooling" for the tool pin). Include provenance/attestation via `actions/attest-build-provenance` when the release platform supports it; always set `provenance: false` on `docker/build-push-action` steps
- If GUI packaging exists (MSI, DMG, AppImage, deb, rpm, etc.), the package wraps the same single static binary plus desktop integration metadata; package metadata lives in `packaging/`

## Docker Rule

Docker is **REQUIRED**. The project MUST ship a working Docker setup, and every toolchain command and every produced binary runs inside it (PART 0 → "No Host Toolchain or Binary Execution").

### Required Docker Assets

Place Docker assets under `docker/`:

```text
docker/
├── Dockerfile                              # production runtime image — two-stage (builder + minimal Alpine/Debian); tagged :latest
├── Dockerfile.dev                          # devel image — same as release but binary runs in debug mode; tagged :devel   (project-specific)
├── rootfs/                                 # build-time filesystem overlay copied into image at /   (project-specific)
│   └── usr/local/bin/entrypoint.sh         # sets non-root UID/GID, prepares cache/target dirs; called by tini → entrypoint.sh → app
├── docker-compose.yml                      # production/human runtime — image: ghcr.io/{org}/{name}:latest
├── docker-compose.dev.yml                  # human development — image: ghcr.io/{org}/{name}:devel
├── docker-compose.test.yml                 # automated testing (AI-usable) — builds from Dockerfile, ephemeral tmpfs, named bridge net
└── README.md                               # how to build the image, run tests, run GUI with display forwarding
```

All three compose files live under `docker/` (per `dockerfile_conventions.md` → "Docker Compose / File locations"). A top-level `docker-compose.yml` symlink or shim is allowed for ergonomics, but the source of truth lives under `docker/`. AI MUST only run `docker-compose.test.yml`; `docker-compose.yml` and `docker-compose.dev.yml` are human-only.

### Toolchain Image

**All Rust projects use `casjaysdev/rust:latest` — never create `docker/Dockerfile.build` or `build-toolchain.yml` for Rust.** The maintained image includes everything any Rust project needs:

- Latest stable toolchain + nightly (minimal profile with miri and rust-src)
- Components: `rustfmt`, `clippy`, `rust-src`, `rust-analyzer`, `llvm-tools-preview`
- `cargo-binstall` for fast tool installation without source compilation
- C/C++ toolchain: `build-base`, `musl-dev`, `clang`, `lld`, `llvm`, `cmake`, `gdb`
- Cross-compile linker: `mingw-w64-gcc` (Windows GNU), `zig` (cargo-zigbuild), `binaryen` (WASM optimisation)
- Cross-compile targets: musl Linux (x86_64, aarch64, i686, armv7, riscv64), glibc Linux (x86_64, aarch64, i686, armv7, arm, riscv64, ppc64le, s390x), Windows GNU (x86_64, i686, aarch64), macOS (x86_64, aarch64), FreeBSD, WebAssembly (wasm32-unknown-unknown, wasip1, wasip2, emscripten), embedded ARM/RISC-V, Android (aarch64)
- `cargo-audit`, `cargo-deny`, `cargo-tarpaulin`, `cargo-llvm-cov`, `grcov` — security and coverage
- `cargo-nextest`, `cargo-make`, `just` — testing and task runners
- `cargo-zigbuild`, `cross`, `cargo-ndk` — cross-compilation runners
- `sccache` — compiler cache
- `cargo-release`, `cargo-dist`, `cargo-deb` — release tooling
- `cargo-edit`, `cargo-watch`, `cargo-outdated`, `cargo-update`, `cargo-expand` — development workflow
- `cargo-semver-checks`, `cargo-msrv`, `cargo-machete`, `cargo-udeps` — API and dependency analysis
- `cargo-fuzz`, `cargo-mutants`, `cargo-careful` — testing depth
- `wasm-pack`, `wasm-tools`, `wasm-bindgen-cli`, `trunk` — WebAssembly toolchain
- `cargo-bloat`, `cargo-asm`, `cargo-binutils`, `hyperfine`, `flamegraph`, `samply` — performance and binary analysis
- `cbindgen` — C header generation from Rust
- `mdbook`, `mdbook-toc`, `cargo-public-api`, `cargo-spellcheck` — documentation
- `typos-cli`, `taplo-cli`, `dprint`, `cargo-sort` — code quality
- `sqlx-cli`, `sea-orm-cli` — database tooling
- `tokei` — code statistics
- `cargo-geiger` — unsafe code audit
- `cargo-chef` — layer caching for Docker builds
- `cargo-criterion`, `cargo-hack`, `cargo-minimal-versions` — testing and compatibility
- `probe-rs` — embedded debugging
- `bacon` — background cargo runner

CI workflows reference this image directly: `container: image: casjaysdev/rust:latest`. No `cargo install`, no `rustup` in CI, no `ensure-build-image` job, no `build-toolchain.yml`.

### OCI Annotations (No LABEL Policy)

All image metadata is applied as **OCI annotations at build time** — never as `LABEL` blocks in any Dockerfile. Labels attach to per-platform image layers; multiarch manifest indexes do not inherit labels, so they appear missing on multiarch pulls. Annotations attach to the manifest index and are visible across all platforms.

- No `LABEL` blocks in `docker/Dockerfile` or `docker/Dockerfile.dev`
- GitHub Actions: use `docker/metadata-action@030e881283bb7a6894de51c315a6bfe6a94e05cf  # v6.0.0` with an `annotations:` input listing the required OCI keys
- `docker/build-push-action@bcafcacb16a39f128d818304e6c9c0c18556b85f  # v7.1.0` MUST set `annotations: ${{ steps.meta.outputs.annotations }}`, `labels: ""` to suppress label output, AND `provenance: false` to prevent a spurious `unknown/unknown` platform entry in the manifest list (use `actions/attest-build-provenance` for release binary attestation instead)

See `dockerfile_conventions.md` → "OCI Annotations" for the full required annotation set (`org.opencontainers.image.{title,description,url,source,documentation,vendor,authors,vcs-type,version,revision,created,licenses,...}`).

### Container Runtime Rules

Every production image MUST satisfy:

- **Startup chain `tini → entrypoint.sh → app`** — `ENTRYPOINT [ "tini", "-p", "SIGTERM", "--", "/usr/local/bin/entrypoint.sh" ]`. Never override `ENTRYPOINT` or `CMD` to bypass `tini` or the entrypoint shim. All startup customization goes in `docker/rootfs/usr/local/bin/entrypoint.sh`, which MUST end with `exec "$@"` to preserve PID 1 signal handling.
- **`STOPSIGNAL SIGTERM`** (or `SIGRTMIN+3` for s6-based images) for graceful shutdown
- **`HEALTHCHECK`** — every production image declares a `HEALTHCHECK` that exits non-zero when the binary is unhealthy
- **Non-root `USER`** — containers MUST NOT run as root. Create a non-root user/group in the Dockerfile and switch to it via `USER` before `ENTRYPOINT`. `entrypoint.sh` may remap UID/GID at runtime to match host ownership of mounted volumes. Exceptions (privileged port binding, device access, etc.) MUST be documented in `IDEA.md`.

### Mandatory `docker run` Naming Convention

Every `docker run` invocation in this project (CI, scripts, docs, examples) MUST use:

```bash
docker run --rm -it \
  --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" \
  ...
```

- `--rm` — self-remove on exit (no orphaned containers)
- `-it` — interactive-capable for log streaming and signal handling
- `--name "${PROJECT_NAME}-XXXX"` — traceable name; `XXXX` is the 8-char lowercase-alphanumeric random suffix produced by `tr -dc 'a-z0-9' </dev/urandom | head -c8` (Makefile form: `$$(tr -dc 'a-z0-9' </dev/urandom | head -c8)`)

There is no opt-out. Unnamed or persistent build/test containers are a spec violation.

### Portability Rule

No hardcoded org, project name, official site, or registry value may appear in any Dockerfile, workflow, or Makefile. Use build-time variables:

| Context | Reference |
|---------|-----------|
| Dockerfile | `ARG PROJECT_ORG` / `ARG PROJECT_NAME` (passed via `--build-arg`) |
| GitHub Actions | `${{ github.repository_owner }}` / `${{ github.event.repository.name }}` / `${{ github.event.repository.html_url }}` |
| GitLab CI | `$CI_REGISTRY_IMAGE`, `$CI_PROJECT_NAMESPACE`, `$CI_PROJECT_NAME` |
| Gitea / Forgejo | provider-supplied equivalents to the GitHub variables |
| Jenkinsfile | `${env.JOB_NAME}`, `${env.GIT_URL}` (parse org/name) |
| Makefile | `PROJECT_ORG ?= $(shell git remote get-url origin \| ...)` |

This rule ensures every workflow, Dockerfile, and Makefile keeps working after a fork without editing values.

### Build-Time vs Runtime Linkage of Display Libraries

By default `casjaysdev/rust:latest` carries no GUI-stack C dev libraries at link time. Whether the image ships them or not, the produced release binary MUST NOT have a **link-time** dependency on them. Display-related C libraries reach the binary only via runtime `dlopen`, on demand, when a real GUI session is started.

- **X11**: prefer the pure-Rust `x11rb` crate (no `libX11` involvement at any stage). `x11-dl` is **also acceptable** — it uses runtime `dlopen` and contributes no link-time C dep. Do **not** use `x11`, `xlib-sys`, or any other crate that resolves libX11 symbols at link time.
- **Wayland**: use `wayland-client` / `wayland-rs` with the `dlopen` feature enabled (or an equivalent runtime-loading mode). The binary then opens `libwayland-client.so` lazily at runtime if Wayland is detected, but contains no link-time dependency on it.
- **OpenGL / EGL / Vulkan**: when in scope, use Rust loaders that resolve symbols at runtime (e.g., `libloading`, or `glow` / `ash` configured for runtime loading) so the binary stays portable across hosts with different GPU stacks.
- After every release build, the static-linkage check (`ldd`, `otool -L`, `dumpbin /dependents`) MUST confirm there is no link-time dependency on `libX11` / `libwayland-client` / `libGL` / `libEGL` / `libVulkan`. With `x11rb`, `libX11` will not appear in the runtime `dlopen` set either; with `x11-dl` it will appear only when an X11 session is actually used.

### X11 and Wayland Forwarding (Mandatory for GUI/Display Testing)

GUI and display-aware test runs use the `gui` compose service (or equivalent `docker run` flags). Both X11 and Wayland forwarding MUST be supported; the spec does not pick one — the container detects what the host provides and forwards accordingly.

**X11 forwarding (host running Xorg or XWayland):**

```bash
xhost +SI:localuser:$(id -un)        # grant access to current user only; revoke when done

docker run --rm -it \
  --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" \
  -e DISPLAY="$DISPLAY" \
  -e XAUTHORITY=/tmp/.docker.xauth \
  -v /tmp/.X11-unix:/tmp/.X11-unix:rw \
  -v "$XAUTHORITY":/tmp/.docker.xauth:ro \
  --device /dev/dri \
  -v "$PWD":/work -w /work \
  "$PROJECT_IMAGE" cargo run -- --ui gui

xhost -SI:localuser:$(id -un)        # revoke after the session
```

**Wayland forwarding (host running a Wayland compositor):**

```bash
docker run --rm -it \
  --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" \
  -e WAYLAND_DISPLAY="$WAYLAND_DISPLAY" \
  -e XDG_RUNTIME_DIR=/tmp/xdg \
  -e QT_QPA_PLATFORM=wayland \
  -e GDK_BACKEND=wayland \
  -v "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY":"/tmp/xdg/$WAYLAND_DISPLAY" \
  --device /dev/dri \
  -v "$PWD":/work -w /work \
  "$PROJECT_IMAGE" cargo run -- --ui gui
```

**Rules:**
- Both forwarding modes MUST be documented and working — pick one based on what the host session exposes
- `xhost +` (wide open) is forbidden; use scoped grants (`+SI:localuser:…`) and revoke after
- Do not run the container as root just to make sockets work; map UID/GID instead
- GPU access (`/dev/dri`) is allowed when the GUI stack benefits from it; keep it opt-in via a compose profile when not always needed
- Sound/MIDI/input device forwarding is allowed only when IDEA.md justifies it
- Audio (PulseAudio/PipeWire) forwarding follows the same scoped-socket pattern when needed

### Forbidden

- Running cargo, rustc, or any built binary on the host
- Treating Docker as a "CI-only" or "release-only" concern
- Recommending `cargo install` on the host to "just try it quickly"
- `--privileged`, `--net=host` outside of clearly justified, documented profiles
- `xhost +` without a scoped target

---

# PART 6: VERSION, SITE, AND BUILD METADATA

## `release.txt` (Canonical Version When Present)

- `release.txt` is a single-line canonical version source when present
- It overrides derived/tag-based defaults and should stay in sync with package metadata
- Use semantic versions unless the project intentionally uses another documented scheme
- If `release.txt` exists, CI/CD and release scripts must use it rather than inventing a version

## `site.txt` (Optional Official Site)

- Optional single-line URL file in project root
- Use it for the official site, homepage, docs root, support page, or update landing page
- Never guess it
- If `site.txt` exists, it wins over `IDEA.md`, README, environment variables, and CI secrets
- If `site.txt` and `IDEA.md` disagree, `site.txt` wins; update IDEA.md to match or remove the stale variable

## Metadata Priority Rules

### Version
1. `release.txt` (when present)
2. `Cargo.toml` package version
3. explicit documented fallback for bootstrap/dev builds

### Official Site
1. `site.txt` (when present)
2. `IDEA.md ## Project variables` (`official_site`)
3. explicit environment/CI override used only where documented
4. empty

## Build Metadata

Embed at build time when practical:
- version
- commit ID
- build date
- official site (optional)

**Example `build.rs` pattern:**
```rust
use std::{fs, path::Path};

fn main() {
    // Re-run this build script when either metadata file changes, otherwise
    // bumping release.txt / site.txt would not invalidate the embedded constants.
    println!("cargo:rerun-if-changed=release.txt");
    println!("cargo:rerun-if-changed=site.txt");

    if Path::new("release.txt").exists() {
        let version = fs::read_to_string("release.txt").unwrap();
        println!("cargo:rustc-env=APP_VERSION={}", version.trim());
    }

    if Path::new("site.txt").exists() {
        let site = fs::read_to_string("site.txt").unwrap();
        println!("cargo:rustc-env=APP_OFFICIAL_SITE={}", site.trim());
    }
}
```

**Example runtime constants:**
```rust
pub const VERSION: &str = option_env!("APP_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
pub const OFFICIAL_SITE: &str = option_env!("APP_OFFICIAL_SITE").unwrap_or("");
pub const COMMIT_ID: &str = option_env!("APP_COMMIT_ID").unwrap_or("N/A");
pub const BUILD_DATE: &str = option_env!("APP_BUILD_DATE").unwrap_or("N/A");
```

---

# PART 7: RUNTIME DETECTION, CONFIGURATION, AND OUTPUT

## Runtime Detection Rules

All machine-dependent settings MUST be detected at runtime on the target machine, never copied from the developer machine.

| Setting | Detection Method | NEVER Do |
|---------|------------------|----------|
| OS / architecture | `std::env::consts`, target info | hardcode dev machine values |
| Home/config/data dirs | platform APIs / env vars | hardcode absolute dev paths |
| Locale | `LANG`, platform locale APIs | assume English-only unless documented |
| Theme preference | OS theme APIs / config | assume dark/light universally |
| Terminal capability | TTY + TERM + negotiated features | assume ANSI/alt-screen support |
| Display stack | X11 (`DISPLAY`) **and** Wayland (`WAYLAND_DISPLAY`) detection — both backends supported, runtime-selected | assume GUI exists; ship X11-only or Wayland-only |
| CPU / memory | runtime detection if needed | tune only for dev hardware |

## Configuration Rules

- Config files store user choices, not auto-detected machine facts
- Defaults must be sensible for a fresh per-user install
- Support config file + environment variable + CLI override layering
- CLI override wins over env; env wins over config; config wins over defaults
- Secrets must not be stored in world-readable files

## Standard CLI Flags

### Universal Flags (ALL Binaries)

| Flag | Short | Values | Description |
|------|-------|--------|-------------|
| `--help` | `-h` | — | Show help and exit 0 |
| `--version` | `-v` | — | Show version and exit 0 |
| `--debug` | — | — | Enable debug output |
| `--color` | — | `auto` (default) / `yes` / `no` | Color output — `auto`: TTY detect; `yes`: force on; `no`: force off and remove emojis |

- `--color auto` and `--color=auto` (space and `=` forms) must both work — `clap` handles this natively; no extra parsing needed.
- **No escalation** — `--help` and `--version` must never be gated behind privilege checks; never call `nix::unistd::getuid()` or any capability check before help/version output.

## Output Rules

- Respect `NO_COLOR`
- Respect `TERM=dumb`
- Respect non-TTY stdout/stderr
- Provide machine-readable output when the app advertises it (`--json`, `--plain`, etc.)
- Help/version output must show the **actual invoked binary name**
- **No escalation** — help at every level (main, subcommand, nested) must never call `sudo`, require root/admin, or check privilege state; exit immediately with the help text.

## NO_COLOR Support

When `NO_COLOR` is set and non-empty, disable ANSI color output. If the TUI depends on richer formatting, fall back to CLI/plain output instead of forcing a degraded pseudo-TUI.

| Condition | GUI | TUI | CLI |
|-----------|-----|-----|-----|
| local desktop with display | allowed | fallback | fallback |
| SSH with TTY | no auto-GUI | allowed if terminal capable | fallback |
| `TERM=dumb` | no auto-GUI | avoid | use CLI |
| `NO_COLOR=1` | GUI unaffected unless project says otherwise | avoid color-dependent TUI | prefer CLI/plain |
| stdout piped | avoid GUI | avoid TUI | use CLI |

---

# PART 8: TESTING, QUALITY, AND DEBUGGING

## Required Quality Gates

All gates execute inside the project Docker container (PART 5 → "Docker Rule"). The "Logical Command" column shows the cargo invocation; the actual command is Docker-wrapped.

| Gate | Logical Command | Wrapped Form (example) |
|------|-----------------|------------------------|
| Formatting | `cargo fmt --all --check` | `docker compose run --rm dev cargo fmt --all --check` |
| Linting | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | `docker compose run --rm dev cargo clippy …` |
| Tests | `cargo test --workspace --all-features` | `docker compose run --rm dev cargo test …` |
| Docs | `cargo doc --workspace --no-deps` | `docker compose run --rm dev cargo doc …` |
| Licenses + advisories + bans + sources | `cargo deny check licenses advisories bans sources` | `docker compose run --rm dev cargo deny check …` |
| Attribution drift | `cargo about generate about.hbs` (output diffed against the GENERATED region of `LICENSE.md`) | see PART 10 → "Suggested CI Steps" |
| GUI smoke (X11) | `cargo run -- --ui gui` against an X11 socket | see PART 5 → "X11 and Wayland Forwarding" |
| GUI smoke (Wayland) | `cargo run -- --ui gui` against a Wayland socket | see PART 5 → "X11 and Wayland Forwarding" |

## Coverage Gate

`ci.yml` MUST enforce a minimum test coverage threshold. The threshold is declared in `IDEA.md ## Business logic` (free-form prose — e.g., "minimum test coverage: 75%"). If `IDEA.md` does not specify a value, the **default is 60%**.

- Use `cargo tarpaulin` (preferred for Linux musl) or `cargo llvm-cov` (works on more targets) — both pre-installed in `casjaysdev/rust:latest`
- CI MUST fail when coverage drops below the threshold; a passing build with uncovered code is a silent regression
- Coverage is computed against `cargo test --workspace --all-features` output, run inside the toolchain image

```bash
# Example (Docker-wrapped) — fails if total coverage < threshold
THRESHOLD="${COVERAGE_MIN:-60}"
docker run --rm -it \
  --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" \
  -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
  cargo tarpaulin --workspace --all-features --fail-under "$THRESHOLD"
```

## Testing Rules

- Core business logic must have unit tests
- Integration tests go in `tests/`
- UI-specific code should keep as much logic as possible outside the UI layer so it is unit-testable
- GUI/TUI smoke tests are required when those surfaces exist
- Snapshot/golden tests are allowed for stable text output and rendering state, but must be intentionally reviewed
- Fixes must include a test when the behavior is testable

## Debug Rules

- Release builds default to user-friendly error messages
- Debug/development mode may include verbose logs and backtraces
- Do not expose sensitive data in logs
- Panic/backtrace behavior must be intentional and documented
- GUI/TUI debug tooling must not leak into normal production UX by default

## Directory Naming

**Plural** — all directories use plural names (`handlers/`, `models/`, `routes/`, `utils/`). Rust module directories follow the same rule — `src/handlers/mod.rs` not `src/handler/mod.rs`. Tooling dirs are also plural (`scripts/`, `tests/`, `completions/`).

---

## Performance Rules

- Optimize based on measured behavior, not guesses
- Scale caches/worker counts from runtime capabilities when relevant
- Keep minimal-system defaults sensible
- Avoid unnecessary allocations and duplicate state copies in hot paths

---

# PART 9: SECURITY & PRIVACY

## Security-First Design

- Least privilege by default
- Per-user storage by default
- Explicit consent for networked, destructive, or privileged operations
- Validate all untrusted input
- Prefer safe Rust and audited crates
- Use `rustls` for TLS. On macOS and Windows, the platform-native TLS APIs (`security-framework` on Apple, Schannel via `windows`/`windows-sys` on Windows) are also acceptable since they reach the OS through Apple frameworks / Windows system DLLs that are already in our allowed runtime set. On Linux/BSD, do **not** fall back to GnuTLS or system OpenSSL — they would require a `*-sys` exception per PART 0, and `rustls` covers the use case
- Use `secrecy` for in-memory secret handling
- For OS-level secret storage, use pure-Rust crates that speak the platform protocol directly: `secret-service` (Linux/BSD freedesktop Secret Service over D-Bus), the `security-framework` Rust binding to Apple frameworks (counts as an Apple-frameworks dependency, not third-party C), and the `windows` / `windows-sys` crates for Windows DPAPI / Credential Manager. Do **not** link to `libsecret` / `libgnome-keyring` / `kwallet` directly — those would be `*-sys` C deps and would need an IDEA.md exception per PART 0 → "Rust-Only Application"

## Secrets and Sensitive Data

- Never log raw secrets
- Redact tokens, passwords, API keys, auth headers, and personal data from logs and diagnostics
- Config files containing secrets must use restrictive permissions
- Export/import features must clearly label sensitive content

## Update / Download Safety

If the app downloads updates or remote content:

- require explicit user intent or documented auto-update policy
- verify integrity/signatures where supported
- never run downloaded code implicitly
- document trust assumptions in IDEA.md

Plugin downloads are an additional case and apply only when IDEA.md defines a hardened plugin contract per PART 0 → "Single Static Binary"; the four rules above apply to them as well.

## Dependency Governance

- Keep dependencies minimal
- Remove unused crates promptly
- **Renovate is the only supported dependency-update tool** — covers Cargo deps, GitHub Actions SHAs, and Docker image digests from a single `renovate.json` at the repo root. Works on GitHub, GitLab, Gitea, Forgejo, and Bitbucket.
- **Dependabot is forbidden** — GitHub-only, duplicates Renovate's work on GitHub, and cannot serve the other four providers. Never enable both.
- Public repos MUST ship `renovate.json` so Cargo / Actions / Docker updates land as PRs automatically; Renovate uses `pinDigests: true` to keep all `uses:` lines pinned to immutable SHAs
- Renovate only updates the SHA; the **runtime-still-supported** verification (e.g., node24 vs deprecated runtimes) remains a manual check on every SHA bump (PART 10 → "Third-party Action Pinning")
- Security advisories are blockers until triaged

## Telemetry Rule

No hidden telemetry. Any analytics, crash reporting, or update pings must be documented and controllable.

---

# PART 10: CI/CD, RELEASES, AND AUTOMATION

## CI/CD Rules

| Rule | Description |
|------|-------------|
| Explicit commands | CI must run explicit Cargo commands, not hide core release logic behind undocumented wrappers |
| Least privilege | CI tokens/permissions default to read-only |
| Pinned actions | Third-party actions pinned to full commit SHAs; verify runtime and maintenance status on every SHA update |
| No unsafe fork secrets | Fork PRs do not receive secrets or publish permissions |
| Version precedence | `release.txt` wins when present |
| Site precedence | `site.txt` wins when present |
| Verifiable outputs | Releases publish checksums and an SBOM (always); provenance/attestation when the platform supports it |
| No Makefile in CI | Workflow `run:` steps invoke explicit commands with all environment variables inlined — never `make {target}`. The Makefile is for local developer convenience only. CI MUST NOT depend on Makefile targets that could drift silently. |
| Portability | No hardcoded org, project name, official site, or registry value anywhere in workflows. Use `${{ github.repository_owner }}` / `${{ github.event.repository.name }}` (and provider equivalents). Workflows must keep working after a fork without editing values. |
| Renovate only | `renovate.json` at repo root is the only supported dependency-update tool — it covers GitHub Actions SHAs, Docker image digests, Cargo deps, and works across all five providers from a single config. Dependabot is **forbidden** (GitHub-only; duplicates Renovate on GitHub; cannot serve the other four providers). |
| `act` pre-commit validation | Before committing any change to `.github/workflows/*.yml`, run `act --list -W {file}` on each changed file. Fix all errors before committing. The `validate-workflows.sh` PreToolUse hook enforces this automatically. |
| Concurrency groups | Every push/PR workflow declares `concurrency: { group: ${{ github.workflow }}-${{ github.ref }}, cancel-in-progress: true }`. Release workflows use `cancel-in-progress: false` so a release in flight is never cancelled by a follow-up push. |
| Artifact retention | Every `actions/upload-artifact` step sets `retention-days: 7` (or shorter) — no infinite retention of build outputs. |

## Workflow Permissions

Set `contents: read` at the workflow level as the read-only baseline. Grant write permissions only on the specific job that performs the release or publish step — never workflow-wide.

| Permission | Scope | Why |
|------------|-------|-----|
| `contents: read` | All jobs (baseline) | Checkout |
| `contents: write` | Release job only | Create GitHub release, upload assets |
| `packages: write` | Release job only | Push images to `ghcr.io` |
| `id-token: write` | Release job only | OIDC token for Sigstore/cosign artifact signing |
| `attestations: write` | Release job only | GitHub artifact attestation (SBOM, provenance) |

```yaml
# Workflow-level: read-only baseline
permissions:
  contents: read

jobs:
  build:
    # Inherits read-only — no overrides needed
    runs-on: ubuntu-latest
    ...

  release:
    needs: build
    permissions:
      contents: write      # create GitHub release + upload assets
      packages: write      # push to ghcr.io
      id-token: write      # OIDC token for cosign signing
      attestations: write  # GitHub artifact attestations (SBOM, provenance)
    ...
```

Third-party registry publishing uses repository secrets, not GitHub token permissions (e.g. `CARGO_REGISTRY_TOKEN` for crates.io).

## Third-party Action Pinning

Every external action (`uses: owner/action@...`) MUST be pinned to a full commit SHA — never a mutable tag or branch:

```yaml
# Wrong — tag can silently change or be deleted
- uses: actions/checkout@v4

# Correct — SHA is immutable
- uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd  # v6.0.2
```

**When updating a pinned SHA**, verify three things:

1. **Action is still maintained** — check the upstream repo is not archived, deprecated, or abandoned
2. **Runtime is still supported** — open the action's `action.yml` at the new SHA and check `runs.using`; if it names a runtime that GitHub has deprecated or scheduled for removal, the action will silently fail after that date. Example: `node20` is removed from GitHub-hosted runners on **2026-09-16** — any action still on `node20` must be updated to a SHA where it has migrated to `node24` — all common `actions/*` and `docker/*` actions have already done so
3. **No supply-chain change** — skim the diff between the old and new SHA; unexpected new dependencies, changed entrypoints, or network calls added to setup steps are red flags

**Renovate** (`renovate.json`) is the only supported dependency update tool — it updates GitHub Actions SHAs, Docker image digests, and Cargo deps across all providers in a single config (see `cicd_conventions.md`). Renovate does not do the runtime verification — that is always a manual check.

### Provider CLIs and Local Runner

**Internet access is available and must be used** — never say "I don't have internet access". Fetch docs, versions, READMEs, and any fact that changes over time rather than guessing from training data.

**User-provided URLs:** always fetch with `\curl -q -LSs {url}` — never WebFetch. WebFetch is for AI-initiated lookups only.

**Tool preference for lookups:** `WebSearch` (open-ended query) → `WebFetch` (known URL, no pipe) → `\curl -q -LSs` (pipe to `jq`/`grep`/file) → provider CLI for provider API ops.

**Provider CLIs** (prefer over raw `curl` when installed):
- `gh` — GitHub (Apache-2.0)
- `glab` — GitLab (MIT)
- `tea` — Gitea and Forgejo (MIT, compatible API)

**`act`** (nektos/act, MIT) — run GitHub Actions locally with `act -j {job}` to verify before pushing. Not a CI replacement — always let real CI run.

## Minimum Public Repo Workflows

Every project ships workflow files for all five CI/CD providers. Same gates, different syntax — no vendor lock-in.

**Workflow creation order — not all workflows carry the same risk:**
1. **Security-only workflows** (secret scan, SHA/digest policy, dependency audit) — no build dependency; safe to add anytime
2. **`ci.yml` and `release.yml`** — add **last**, only after all code is complete, `make test` passes, and the lint gate is clean; these trigger a full build on push and will fail immediately if the code is not ready

Rust projects never have `build-toolchain.yml` — `casjaysdev/rust:latest` is maintained externally and needs no per-project rebuild workflow.

| Provider | Workflow location | Syntax |
|----------|------------------|--------|
| GitHub | `.github/workflows/ci.yml` / `release.yml` | GitHub Actions |
| GitLab | `.gitlab-ci.yml` (stages: build, test, security, release) | GitLab CI |
| Gitea | `.gitea/workflows/ci.yml` / `release.yml` | GitHub Actions (act runner) |
| Forgejo | `.forgejo/workflows/ci.yml` / `release.yml` | GitHub Actions (act runner) |
| Jenkins | `Jenkinsfile` (parallel stages: Build / Test / Security / Release) | Declarative Pipeline |

**Workflow purposes:**

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `ci.yml` | push + pull_request + weekly schedule (security jobs only on schedule) | `cargo fmt --check`, `cargo clippy`, `cargo test`, coverage gate, `cargo build --release`, truffleHog secret scan, workflow-policy SHA check, `cargo audit`, Trivy image scan |
| `release.yml` | tag push (`v*`) + workflow_dispatch | Build statically linked artifacts for the supported target matrix, sign, upload to GitHub Releases, publish SBOM and checksums |

In addition to the workflows, every repository ships:

- `renovate.json` — single dependency-update config covering GitHub Actions SHAs, Docker digests, Cargo deps; works on all five providers (see PART 9 → "Dependency Governance")

**`security` job conditionality (applies to all providers):**
- Secret scan (truffleHog) — always runs; full git history required
- Workflow policy (SHA/digest pinning check) — always runs
- `vuln-scan` (cargo audit) — conditional on `Cargo.lock` present
- `image-scan` (Trivy) — conditional on Dockerfile present; runs after image build

**GitHub Actions job ordering (`needs:`):**
- `ci.yml`: `lint` and `test` run in parallel → `build` needs: test → `upload-artifacts` needs: build; security jobs (`secret-scan`, `workflow-policy`, `vuln-scan`, `image-scan`) run in parallel with each other
- `release.yml`: `build` → `release` (needs: build); release job always re-runs its own build inline
- Cross-workflow ordering via branch protection; never `workflow_run`

**GitLab CI**: security jobs run in the `security` stage (parallel by default). Release stage triggered by `$CI_COMMIT_TAG`.

**Jenkins**: `Security` stage uses `parallel {}`. `Release` stage uses `when { tag 'v*' }`.

CI MUST fail on all providers when tests, coverage gates, secret scans, dependency checks, or release validation fail. Never accept a weaker gate on one provider than another.

### Security Jobs in `ci.yml` Example (GitHub / Gitea / Forgejo)

truffleHog is the mandatory secret scanner — **never** `gitleaks` (requires a commercial license for org repos). Trivy is the mandatory image scanner. Both pin to full commit SHAs. These jobs live in `ci.yml` and also run on the weekly schedule trigger.

```yaml
name: CI

on:
  push:
  pull_request:
  schedule:
    - cron: '0 6 * * 1'   # weekly Monday 06:00 UTC

permissions:
  contents: read

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  secret-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd  # v6.0.2
        with:
          fetch-depth: 0   # required: truffleHog needs full history

      - name: TruffleHog secret scan
        uses: trufflesecurity/trufflehog@b634fb72d9901a4f942e5b8e4ef5f7ec59c97e7c  # v3.88.2
        with:
          base: ${{ github.event.before }}   # NEVER use default_branch — it resolves to HEAD post-push and skips the scan
          head: ${{ github.sha }}
          extra_args: --only-verified

  workflow-policy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd  # v6.0.2
      - name: Verify all third-party actions are pinned to a 40-char SHA
        run: |
          set -eo pipefail
          bad=$(grep -RhnE '^\s*uses:\s*[^@]+@(v?[0-9]|main|master)' .github/ .gitea/ .forgejo/ 2>/dev/null || true)
          if [[ -n "$bad" ]]; then
            echo "::error::Unpinned actions found (must be 40-char SHAs):"
            echo "$bad"
            exit 1
          fi

  vuln-scan:
    runs-on: ubuntu-latest
    if: ${{ hashFiles('Cargo.lock') != '' }}
    steps:
      - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd  # v6.0.2
      - name: cargo audit (inside :build image)
        run: |
          IMAGE="ghcr.io/${{ github.repository_owner }}/${{ github.event.repository.name }}:build"
          docker run --rm -i \
            --name "${{ github.event.repository.name }}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" \
            -v "$PWD":/work -w /work "$IMAGE" cargo audit

  image-scan:
    runs-on: ubuntu-latest
    if: ${{ hashFiles('docker/Dockerfile') != '' }}
    steps:
      - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd  # v6.0.2
      - uses: docker/setup-buildx-action@4d04d5d9486b7bd6fa91e7baf45bbb4f8b9deedd  # v4.0.0
      - name: Build local image for scanning
        run: |
          docker build -f docker/Dockerfile -t scan-target:ci .
      - name: Trivy image scan
        uses: aquasecurity/trivy-action@76071ef0d7ec797419534a183b498b4d6366cf37  # v0.70.0
        with:
          image-ref: scan-target:ci
          severity: CRITICAL,HIGH
          exit-code: '1'
```

**Per-provider notes:**

- GitLab: `secret-scan` runs as a docker job using `image: trufflesecurity/trufflehog:latest` with `GIT_DEPTH: 0`; `image-scan` uses `image: aquasecurity/trivy:0.70.0`.
- Jenkins: `Security` stage uses `parallel {}`; truffleHog and Trivy each run via `docker.image(...).inside { ... }`.
- All providers: same gates, same severities, same exit conditions — no weaker subset on any provider.

## Suggested CI Steps

All Rust CI jobs run inside `casjaysdev/rust:latest`. Never `cargo install` or `rustup` in a workflow `run:` step — every tool is already in the image. No `ensure-build-image` pre-flight, no `build-toolchain.yml`.

Canonical job pattern for `ci.yml` / `release.yml`:

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    container:
      image: casjaysdev/rust:latest
    steps:
      - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd  # v6.0.2
      - run: make build
```

### Required Concurrency and Retention Headers

Every push/PR workflow (`ci.yml`) MUST declare:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

Release workflows (`release.yml`) MUST use `cancel-in-progress: false` so a release build already in flight is not killed by a follow-up push.

Every `actions/upload-artifact` step MUST set a finite `retention-days`:

```yaml
- uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a  # v7.0.1
  with:
    name: {project_name}-${{ matrix.target }}
    path: dist/
    retention-days: 7   # release-job artifacts may use up to 30; build-job CI artifacts use 7
```

```bash
# Prepare output directory for release artifacts (binaries, checksums, SBOM)
mkdir -p dist

# Run gates inside the image (casjaysdev/rust:latest — all tools pre-installed)
docker run --rm -it --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" cargo fmt --all --check
docker run --rm -it --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" cargo clippy --workspace --all-targets --all-features -- -D warnings
docker run --rm -it --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" cargo test --workspace --all-features
docker run --rm -it --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" cargo doc --workspace --no-deps

# License + advisory enforcement (PART 11 → "License Compliance")
docker run --rm -it --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
  cargo deny check licenses advisories bans sources

# Attribution-drift check: regenerate the GENERATED region and compare to committed.
# The `>` redirection runs on the host shell, capturing the container's stdout
# into a host-side file (the cwd is bind-mounted at /work, so this is intentional).
docker run --rm -it --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
  cargo about generate about.hbs > LICENSE.generated.md
sed -n '/<!-- GENERATED:/,$p' LICENSE.md > LICENSE.committed-generated.md
diff LICENSE.committed-generated.md LICENSE.generated.md

# Build statically linked release binaries (one per supported target).
# `cargo --target` keeps the full Rust target triple. The published artifact
# is renamed to {project_name}-{platform}-{arch}{.ext} with -musl / vendor / ABI
# tokens stripped (see PART 2 → "Binary Model").
for TARGET in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  docker run --rm -it \
    --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" \
    -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
    cargo build --release --target "$TARGET"

  # Map triple → artifact name: x86_64-unknown-linux-musl → linux-x86_64
  case "$TARGET" in
    x86_64-unknown-linux-musl)   ARTIFACT="{project_name}-linux-x86_64" ;;
    aarch64-unknown-linux-musl)  ARTIFACT="{project_name}-linux-aarch64" ;;
  esac

  cp "target/$TARGET/release/{project_name}" "dist/$ARTIFACT"
  sha256sum "dist/$ARTIFACT" > "dist/$ARTIFACT.sha256"

  # Verify static linkage for this target — fails the build if unexpected
  # dynamic deps appear. Use the appropriate inspector per target family:
  #   Linux musl     → ldd  (expect "not a dynamic executable" / "statically linked")
  #   Apple darwin   → otool -L (expect only Apple-provided frameworks)
  #   Windows MSVC   → dumpbin /dependents (expect no MSVC runtime DLLs)
  docker run --rm -it --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
    sh -c "ldd target/$TARGET/release/{project_name} 2>&1 | grep -qE 'not a dynamic executable|statically linked'"
done

# Generate the SBOM (CycloneDX) — published alongside the release artifacts.
# `cargo cyclonedx --format json` writes `bom.json` next to Cargo.toml; rename
# into dist/ for inclusion in the release.
docker run --rm -it --name "${PROJECT_NAME}-$(tr -dc 'a-z0-9' </dev/urandom | head -c8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
  cargo cyclonedx --format json
cp bom.json "dist/{project_name}-bom.json"
```

For GUI smoke tests in CI, use a virtual X server (e.g., `Xvfb`) and a headless Wayland compositor (e.g., `cage`, `weston --backend=headless`) **inside** the container or as a sidecar service — both backends MUST be exercised, not just one.

## Workflow Error Messaging

Use `::error::` workflow commands for validation failures — they appear as red annotations on the Actions summary page, not just buried in logs:

```bash
echo "::error::Tag 'foo' does not exist in this repository"
echo "::error file=.github/workflows/release.yml,line=12::message tied to a source location"
```

Always write messages so a developer reading only the step name + message understands what failed and what to do next.

## Release Pre-flight Validation

The GitHub Releases API returns HTTP 422 `"tag_name is not a valid tag"` when the tag does not exist at API call time or is malformed. The correct fix is for the **release job to own the tag** — delete it if it exists, then recreate it at the current HEAD. This ensures the tag always exists and points to the right commit, and makes the workflow idempotent.

The `release` job already has `contents: write` to push assets — this covers tag push as well.

```yaml
- uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd  # v6.0.2
  with:
    fetch-depth: 0   # required: full history needed to inspect and push tags

- name: Ensure release tag
  run: |
    ref="${{ github.ref }}"
    if [[ "$ref" != refs/tags/* ]]; then
      echo "::error::release.yml triggered on non-tag ref '$ref'. Releases require a tag push (refs/tags/v...)."
      exit 1
    fi
    tag="${ref#refs/tags/}"
    if printf '%s' "$tag" | grep -qP '[[:space:][:cntrl:]]'; then
      echo "::error::Tag '$tag' contains whitespace or control characters and is not a valid GitHub tag name."
      exit 1
    fi
    # Delete existing tag (local + remote) then recreate at current HEAD
    git tag -d "$tag" 2>/dev/null || true
    git push origin ":refs/tags/$tag" 2>/dev/null || true
    git tag "$tag"
    git push origin "refs/tags/$tag"
    echo "Tag '$tag' ensured at $(git rev-parse HEAD)"
```

## Release Integrity

Tagged/manual releases should publish:
- binaries/packages
- SHA-256 checksum file
- release notes
- SBOM (`CycloneDX` from `cargo-cyclonedx`; `SPDX JSON` is acceptable if a project chooses that format instead) — always
- provenance / attestation — when the release platform supports it

If signing or attestation is required but keys/permissions are unavailable, stop and ask instead of faking compliance.

---

# PART 11: DOCUMENTATION & LICENSE

## README Minimum Sections

- Badges (linked — see Badges rule below)
- About
- Features
- Installation
- Usage
- GUI/TUI/CLI mode behavior (GUI section MUST cover both X11 and Wayland)
- Configuration
- Development (MUST document the Docker-based workflow — no host cargo invocations)
- Testing (MUST show Docker-wrapped commands; GUI tests show X11 and Wayland forwarding)
- License

### Badges

**Every badge MUST be a linked badge** — `[![alt](image_url)](link_url)` is the only valid form. A bare `![alt](image_url)` with no wrapping link is never acceptable. Each badge links to the resource it represents:

| Badge | Links to |
|-------|----------|
| CI / build status | CI runs page — detect platform: `.github/workflows/*.yml` → GitHub Actions; `.gitea/workflows/*.yml` → Gitea/Forgejo; `.gitlab-ci.yml` → GitLab CI; `Jenkinsfile` → Jenkins |
| Release / version | Releases page |
| License | `LICENSE.md` |
| Docs | Documentation site URL |

Never use a GitHub Actions badge for a GitLab or Gitea project — the CI badge must match the actual hosting platform.

**Badge Templates by Platform:**

```markdown
# GitHub Actions
[![CI](https://github.com/{project_org}/{project_name}/actions/workflows/ci.yml/badge.svg)](https://github.com/{project_org}/{project_name}/actions/workflows/ci.yml)

# Gitea/Forgejo Actions
[![CI](https://git.example.com/{project_org}/{project_name}/actions/workflows/ci.yml/badge.svg)](https://git.example.com/{project_org}/{project_name}/actions)

# GitLab CI
[![Build](https://gitlab.com/{project_org}/{project_name}/badges/main/pipeline.svg)](https://gitlab.com/{project_org}/{project_name}/-/pipelines)

# Jenkins
[![Build](https://jenkins.example.com/buildStatus/icon?job={project_org}/{project_name})](https://jenkins.example.com/job/{project_org}/job/{project_name}/)
```

**Release/License/Docs badges also adapt to platform:**

```markdown
# GitHub
[![Release](https://img.shields.io/github/v/release/{project_org}/{project_name})](https://github.com/{project_org}/{project_name}/releases)
[![License](https://img.shields.io/github/license/{project_org}/{project_name})](LICENSE.md)
[![Docs](https://readthedocs.org/projects/{RTD_PROJECT}/badge/?version=latest)](https://{RTD_URL})

# GitLab
[![Release](https://gitlab.com/{project_org}/{project_name}/-/badges/release.svg)](https://gitlab.com/{project_org}/{project_name}/-/releases)
[![License](https://img.shields.io/github/license/{project_org}/{project_name})](LICENSE.md)
[![Docs](https://readthedocs.org/projects/{RTD_PROJECT}/badge/?version=latest)](https://{RTD_URL})

# Gitea/Forgejo (use shields.io with custom endpoint or static badge)
[![Release](https://img.shields.io/badge/dynamic/json?url=https://git.example.com/api/{api_version}/repos/{project_org}/{project_name}/releases/latest&query=$.tag_name&label=release)](https://git.example.com/{project_org}/{project_name}/releases)
[![License](https://img.shields.io/github/license/{project_org}/{project_name})](LICENSE.md)
[![Docs](https://readthedocs.org/projects/{RTD_PROJECT}/badge/?version=latest)](https://{RTD_URL})

# {RTD_PROJECT} and {RTD_URL} - Use one of:
#   {project_org}-{project_name} / {project_org}-{project_name}.readthedocs.io
#   {project_name} / {project_name}.readthedocs.io
#   Custom project name / {custom_rtd_address}
```

**License badge — use the GitHub API endpoint, not a static badge:**

```markdown
# ✅ CORRECT - GitHub can detect license
[![License](https://img.shields.io/github/license/{project_org}/{project_name})](LICENSE.md)

# ❌ WRONG - Static badge, GitHub cannot detect
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
```

**Docs badge — only add when documentation is actually deployed:**

| Docs Platform | Badge |
|---------------|-------|
| ReadTheDocs | `[![Docs](https://readthedocs.org/projects/{project_org}-{project_name}/badge/?version=latest)](https://{project_org}-{project_name}.readthedocs.io)` |
| GitHub Pages | `[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://{project_org}.github.io/{project_name})` |
| Self-hosted | `[![Docs](https://img.shields.io/badge/docs-online-blue)]({docs_url})` |
| None | **Do not add docs badge** |

## LICENSE.md Requirements

`LICENSE.md` is the canonical attribution document. It MUST contain:

1. **Project license text** — verbatim copy of the MIT license (or whatever IDEA.md declares), with the project's copyright line
2. **Third-party dependency attributions** — for every direct and transitive crate in `Cargo.lock` whose license requires attribution (MIT, BSD-2/3-Clause, Apache-2.0, ISC, MPL-2.0, Zlib, BSL-1.0, OpenSSL, Unicode-DFS-2016, etc. — i.e., everything except true public domain). Public-domain / CC0-1.0 crates SHOULD still be listed for transparency
3. **Verbatim NOTICE / COPYING / LICENSE preservation** — preserve upstream notice files verbatim whenever the upstream license requires it. Two common triggers: (a) crates that vendor C code (e.g., `ring`, `rusqlite` with `bundled`) — preserve the upstream C project's NOTICE / COPYING / LICENSE alongside the crate's own license; (b) Apache-2.0 dependencies that ship a NOTICE file — preserve verbatim
4. **Embedded asset notices** — fonts, icons, theme data, sample data, locale catalogs etc. that are embedded in the binary; each shipped asset's license/attribution as required by its source
5. **Generation banner** — a header comment recording which tool generated the third-party section and from which `Cargo.lock` revision, so reviewers can tell at a glance whether the file is current

**Rule:** `LICENSE.md` is generated, not hand-written, for the third-party section (see "License Compliance" below). The project license text and upstream NOTICE preservation pieces are hand-written and MUST be merged with the generated section, not overwritten by it.

## License Compliance

License compliance is enforced by tooling and gated in CI; it is not left to manual review.

### Required Tooling

| Tool | Purpose | Config file |
|------|---------|-------------|
| `cargo-deny` | Enforce the license allowlist / denylist; flag advisories; flag duplicate transitive crates | `deny.toml` (project root) |
| `cargo-about` | Generate the third-party attribution section of `LICENSE.md` from `Cargo.lock` | `about.toml` (accepted-licenses config) **and** `about.hbs` (Handlebars output template), both at project root |
| `cargo-cyclonedx` | Generate the SBOM (CycloneDX format) referenced in PART 5 → "Release Artifacts" and PART 10 → "Release Integrity" | (CLI flags; no separate config file) |

All three tools run inside `casjaysdev/rust:latest` — `rustfmt`, `clippy`, `cargo-audit`, `cargo-deny`, `cargo-tarpaulin`, and `cargo-llvm-cov` are pre-installed. Never install them in CI workflow `run:` steps.

### License Allowlist (default)

| Allowed | Notes |
|---------|-------|
| `MIT` | Project default; baseline |
| `BSD-2-Clause`, `BSD-3-Clause` | Permissive; require attribution |
| `Apache-2.0`, `Apache-2.0 WITH LLVM-exception` | Require NOTICE preservation when one is shipped |
| `ISC` | Permissive; require attribution |
| `MPL-2.0` | File-level copyleft only — compatible with static linking into MIT binaries |
| `Zlib`, `BSL-1.0`, `Unicode-DFS-2016`, `Unicode-3.0` | Permissive |
| `CC0-1.0`, `Unlicense` | Public-domain equivalents |
| `OpenSSL` | For `ring` and similar |

Compound SPDX expressions like `MIT OR Apache-2.0` (the most common dual-license pattern in the Rust ecosystem) are accepted automatically: `cargo-deny` satisfies the allowlist if **any** alternative in the expression is allowed. No special handling required in `deny.toml`.

### License Denylist (default — deny without IDEA.md exception)

| Denied | Reason |
|--------|--------|
| `GPL-2.0-*`, `GPL-3.0-*` | Strong copyleft; static linking forces the entire distributed binary to GPL, silently relicensing it away from MIT |
| `AGPL-1.0-*`, `AGPL-3.0-*` | Strong copyleft + network-use clause; even non-distributed deployments incur source-release obligations |
| `LGPL-2.0-*`, `LGPL-2.1-*`, `LGPL-3.0-*` | LGPL §6 under static linking requires shipping object files / a written relink offer alongside the binary, breaking the single-binary deliverable |
| `SSPL-1.0` | Service-source copyleft; treated like AGPL for our purposes |
| `Commons-Clause`, `BUSL-*`, `Elastic-2.0`, `Confluent-Community-*` | Source-available with commercial-use restrictions; not OSI-approved; incompatible with the "Free & open source" rule (PART 0 → "Licensing & Features") |

### IDEA.md Exception Path

A project MAY use a denylisted crate only if **all** of:

1. `IDEA.md` adds a `## License exceptions` subsection naming the crate, the upstream license, and the rationale
2. The exception explicitly accepts the consequence — e.g., "this binary is distributed under GPL-3.0; the project's MIT claim applies only to the source we author, not the published binary" — and `IDEA.md ## Project variables` adds (or updates) the variable `distribution_license` accordingly. `distribution_license` is an exception-only variable: it is not part of the required-keys list (see "IDEA.md Required Layout" → Project variables rules) and is present only in projects that have taken a license exception
3. README, `LICENSE.md`, and any user-visible "About" surface are updated to reflect the actual distribution license, not just the source license
4. `deny.toml` is updated with a scoped allow entry for the specific crate + version, not a blanket category unblock

There is no other path to use a denylisted crate. Adding it to `deny.toml` without going through these steps is a spec violation.

### Required `Cargo.toml` Metadata

Every workspace member's `Cargo.toml` MUST include:

```toml
[package]
edition      = "2024"       # current stable Rust edition declared in Cargo.toml (PART 5 → "Toolchain Rules")
rust-version = "1.XX"       # MSRV pin matching rust-toolchain.toml (PART 5 → "Toolchain Image")
license      = "MIT"        # or the SPDX expression for the IDEA.md-declared license
authors      = ["…"]
repository   = "…"
homepage     = "…"          # follows the official-site precedence chain in PART 6 → "Metadata Priority Rules / Official Site": site.txt > IDEA.md `official_site` > documented env override > empty
description  = "…"
```

The `license` field uses an SPDX expression. Do not set `license-file`; `LICENSE.md` is the canonical attribution document for this project, not a Cargo `license-file` target. A non-SPDX project license requires an IDEA.md exception that documents how the project license is declared and shipped.

### Generation Pattern

`LICENSE.md` has two regions:

```markdown
<!-- HAND-WRITTEN: project license, upstream NOTICE files, embedded asset notices -->

# Project License

(MIT text here)

# Embedded Assets

(font / icon / locale attributions here)

# Upstream NOTICE Files

(verbatim NOTICE / COPYING from any vendored C dependency)

<!-- GENERATED: third-party crate attributions -->
<!-- Generated by cargo-about from Cargo.lock @ <commit> on <date> — do not edit by hand -->

# Third-Party Crate Attributions

(cargo-about output here)
```

The generated region is regenerated on every dependency change. The hand-written region is preserved across regenerations.

### User-Visible Attribution Surface

Embedded license data MUST be reachable by the user at runtime, not just shipped in the source repo:

- **CLI**: `--licenses` (alias: `--credits`) prints the full embedded `LICENSE.md` to stdout
- **TUI**: a "Licenses" / "Credits" entry in the help / about screen, scrollable
- **GUI**: an "About → Open Source Licenses" entry that opens a scrollable view of the full text

All three surfaces read the same `LICENSE.md` blob embedded at compile time via `include_str!` (PART 0 → "Self-Contained Assets"). The version, commit ID, and build date (PART 6) are shown alongside.

### CI Gate (mandatory)

The canonical CI invocation lives in PART 10 → "Suggested CI Steps" (the `cargo deny check …` command and the `cargo about generate about.hbs` + `sed`/`diff` drift check). This section defines the **policy**; PART 10 owns the script template. Do not duplicate the commands here when editing — keep PART 10 as the single source.

Drift between `Cargo.lock` and the generated section of `LICENSE.md` is a CI failure, not a warning. The lockfile is the source of truth for what licenses we ship, and `LICENSE.md` MUST match it.

## Documentation Style

- Keep docs precise and operational
- Prefer command snippets that actually match the Rust project layout
- If GUI/TUI/CLI differ in behavior, document the differences explicitly

---

# PART 12: CHECKLISTS

## Bootstrap Checklist

- [ ] `IDEA.md` exists
- [ ] `IDEA.md` has `## Project description`
- [ ] `IDEA.md` has `## Project variables`
- [ ] `IDEA.md` has `## Business logic`
- [ ] `project_name`, `project_org`, `internal_name`, and `internal_org` exist
- [ ] If pre-template `CLAUDE.md` or `.claude/CLAUDE.md` existed, project-specific content was migrated into IDEA.md
- [ ] `CLAUDE.md` / `.claude/CLAUDE.md` are short loaders, not duplicate specs
- [ ] `release.txt` exists if the project is using explicit release versioning
- [ ] `site.txt` exists only if there is a real official site URL
- [ ] `docker/Dockerfile` (always, when project ships a container), `docker/Dockerfile.dev` (project-specific — when a debug-mode image is shipped), `docker/docker-compose.yml`, `docker/docker-compose.dev.yml`, `docker/docker-compose.test.yml`, and `docker/rootfs/usr/local/bin/entrypoint.sh` exist as needed; `Dockerfile` is the runtime image
- [ ] `.dockerignore` exists at project root with Rust-specific entries (`target/`, plus the standard exclusions from `dockerfile_conventions.md` → ".dockerignore")
- [ ] `docker/Dockerfile.build` does not exist — Rust projects always use `casjaysdev/rust:latest` directly; no custom toolchain image is ever needed
- [ ] `ci.yml` and `release.yml` use `container: image: casjaysdev/rust:latest` — no `ensure-build-image` pre-flight, no `build-toolchain.yml`
- [ ] If IDEA.md documents a `*-sys` exception requiring system dev libs at build time, only the minimum set needed by that crate is added to the image — by default the image carries no GUI-stack C dev libs (PART 0 → "Rust-Only Application")
- [ ] X11 forwarding sample command is documented and works against a real Xorg/XWayland session
- [ ] Wayland forwarding sample command is documented and works against a real Wayland compositor
- [ ] `rust-toolchain.toml` pins the toolchain; `.cargo/config.toml` pins static-link rustflags
- [ ] `Cargo.toml` release profile uses `lto = "fat"`, `codegen-units = 1`, `strip = "symbols"`, `panic = "abort"`
- [ ] Repo has `assets/` (build-time source) and a Rust embedding module (`include_bytes!` / `rust-embed`) wiring it into the binary
- [ ] No source files in any language other than Rust contribute to the binary (small Docker shell helpers excepted)
- [ ] `deny.toml` exists at project root with the spec's default allowlist / denylist (PART 11 → "License Compliance")
- [ ] `about.toml` and `about.hbs` exist at project root and `cargo-about` produces the third-party attribution section
- [ ] `cargo-deny`, `cargo-about`, and `cargo-cyclonedx` are pre-installed in the Docker image
- [ ] `Cargo.toml` for every workspace member sets `[package].license`, `authors`, `repository`, `description`
- [ ] `LICENSE.md` has the hand-written region (project license, upstream NOTICE files, embedded asset notices) AND the generated region (cargo-about output) clearly delimited

## Implementation Checklist

- [ ] Core logic is shared across GUI/TUI/CLI
- [ ] Runtime mode auto-detect follows GUI → TUI → CLI priority
- [ ] GUI is not auto-selected in SSH/MOSH/remote-shell contexts
- [ ] If GUI is in scope, both X11 and Wayland backends are supported and runtime-selected
- [ ] Display I/O has no **link-time** C dependency: X11 via `x11rb` (pure Rust) or `x11-dl` (dlopen); Wayland via `wayland-client` / `wayland-rs` with the `dlopen` feature
- [ ] TUI is avoided for `TERM=dumb`, non-TTY, and plain-output contexts
- [ ] CLI works for non-interactive/scripted use
- [ ] Privilege escalation is optional and explicit-request only
- [ ] Config/data/cache/log paths use per-user defaults unless system scope was explicitly requested
- [ ] All assets (fonts, icons, themes, default config, schemas, locales) are embedded at compile time
- [ ] Dependencies are pure Rust where a viable pure-Rust crate exists (PART 5 → "Pure-Rust Library Stack"); each `*-sys` crate is justified in IDEA.md and statically linked
- [ ] `cargo tree` was reviewed — no surprise transitive `*-sys` dependencies
- [ ] No GPL / AGPL / LGPL / SSPL / BUSL / source-available dep was added without an IDEA.md `## License exceptions` entry and an updated `distribution_license` (PART 11 → "License Compliance")
- [ ] User-visible licenses surface exists: CLI `--licenses`, TUI "Licenses" entry, GUI "About → Open Source Licenses" — all reading the same embedded `LICENSE.md`
- [ ] App runs end-to-end from the binary alone on an air-gapped machine — no first-run downloads
- [ ] No plugin or runtime extension loading from disk unless IDEA.md defines a hardened plugin contract
- [ ] `release.txt` / `site.txt` precedence is preserved
- [ ] Docs and examples use Cargo/Rust terminology — wrapped in Docker invocations
- [ ] No build/test/run instructions tell the user to invoke cargo on the host

## Quality Checklist

All gates run inside the project Docker image — never on the host.

- [ ] `cargo fmt --all --check` (Docker-wrapped)
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` (Docker-wrapped)
- [ ] `cargo test --workspace --all-features` (Docker-wrapped)
- [ ] `cargo doc --workspace --no-deps` (Docker-wrapped)
- [ ] `cargo deny check licenses advisories bans sources` passes (Docker-wrapped)
- [ ] `cargo about generate` output matches the generated region of `LICENSE.md` (Docker-wrapped diff check)
- [ ] If GUI is in scope: smoke test exercised under both X11 and Wayland forwarding
- [ ] New behavior includes tests where practical

## Security Checklist

- [ ] No hidden telemetry
- [ ] No secrets in logs
- [ ] No automatic privilege escalation
- [ ] No unsafe downloaded-code execution
- [ ] Dependency/license/security checks remain enabled

## Release Checklist

- [ ] Version comes from `release.txt` when present
- [ ] Official site comes from `site.txt` when present
- [ ] Release notes match actual changes
- [ ] Checksums (SHA-256) are published for every artifact
- [ ] Each release artifact is a single statically linked binary — no companion `.so` / `.dylib` / `.dll` / asset bundle
- [ ] Static-linkage check (`ldd` / `otool -L` / `dumpbin /dependents`) was run and recorded for every published target
- [ ] `LICENSE.md` regenerated and committed if `Cargo.lock` changed; CI license-drift check is green
- [ ] Distributed binary's actual license matches what README and "About" claim (no silent GPL/LGPL relicensing via a transitive crate)
- [ ] Artifact filenames follow `{project_name}-{platform}-{arch}{.ext}` with `-musl`/vendor/ABI tokens stripped
- [ ] Artifacts cover the supported target matrix declared in IDEA.md (defaults: `{project_name}-linux-{x86_64,aarch64}`, `{project_name}-windows-{x86_64,aarch64}.exe`, `{project_name}-macos-{x86_64,aarch64}`)
- [ ] SBOM generated via `cargo-cyclonedx` and published with the release; provenance/attestation included when the platform supports it
- [ ] Packaging metadata matches supported GUI/TUI/CLI surfaces

## Red Flags (Stop and Ask Human)

- required project variables cannot be proven
- migration would discard meaningful project-specific content
- privileged/system-wide action is needed but was not explicitly requested
- licensing or security policy conflict cannot be resolved from the spec
- requested behavior contradicts documented product scope in IDEA.md
- a proposed dependency would force a non-Rust source file into the build, dynamic linking into the release binary, or a runtime asset alongside the binary
- a feature request implies shipping multiple files, an installer-only flow, or a first-run download
- only one of X11 / Wayland can be supported by the chosen GUI stack
- a proposed dependency is GPL / AGPL / LGPL / SSPL / BUSL / source-available — flag the license-relicensing implication before adding (PART 11 → "License Compliance")

## Success Criteria

A compliant Rust project created from this template:
- is driven by `IDEA.md` project variables while `AI.md` stays read-only
- preserves the governance/documentation discipline of the original template
- models a single-binary, fully self-contained Rust application built around GUI / TUI / CLI surfaces
- ships exclusively Rust source code (small Docker shell helpers excepted)
- produces one statically linked binary per target with all assets embedded
- runs end-to-end from the binary alone on an air-gapped machine
- supports both X11 and Wayland on Linux/BSD when GUI is in scope
- builds, tests, and runs only inside the project's Docker image — never on the host
- auto-selects GUI, then TUI, then CLI using environment-aware detection
- uses optional explicit privilege escalation only when requested
- uses Cargo/Rust tooling consistently across development, testing, docs, and CI/CD

---

# PART 13: IDEA.md REFERENCE

**NEVER modify this PART. Read-only reference for IDEA.md structure.**

The canonical layout, variable rules, immutability constraints, and migration procedure are defined at the top of this file under "IDEA.md Required Layout" (the section preceding PART 0). PART 13 here is the worked-example reference — every example below uses the same three-section structure.

---

## IDEA.md Structure

Every IDEA.md has exactly three top-level sections, in this order:

1. `## Project description` — free-form prose: what the project is, who uses it, what problem it solves
2. `## Project variables` — `key: value` lines that provide the canonical values AI.md resolves for `project_name`, `project_org`, `internal_name`, `internal_org`, etc.
3. `## Business logic` — features, data models, user flows, platform constraints, security assumptions (WHAT, not HOW)

See "IDEA.md Required Layout" at the top of this file for the authoritative rules: variable-key naming, the immutable `internal_name` / `internal_org` rule, the missing-value setup flow, and the migration procedure for legacy `CLAUDE.md` files.

---

## IDEA.md Template

```markdown
## Project description

{Brief description of what this project does, its primary users, and what problem it
solves. Free-form prose, 1–3 paragraphs.}

## Project variables

project_name:     {project_name}
project_org:      {project_org}
internal_name:    {project_name}        # FROZEN — equals project_name on first install, never changes
internal_org:     {project_org}         # FROZEN — equals project_org on first install, never changes
app_name:         {App Display Name}
crate_name:       {project_name}
official_site:    {full URL with scheme, e.g., https://example.com — or empty}
maintainer_name:  {Maintainer Name}
maintainer_email: {maintainer@example.com}

## Business logic

**Target users:**
- {User type 1}
- {User type 2}

**Surfaces (declare which apply — see PART 2 → "GUI/TUI/CLI Capability Rule"):**
- GUI: {yes / no — and on which platforms}
- TUI: {yes / no}
- CLI: {yes / no}

**Features:**
- **{Feature category 1}**: {brief description}
- **{Feature category 2}**: {brief description}

**Data models:**
- {Entity}: {fields and meaning, where stored}

**User flows:**
- {Flow}: {step 1 → step 2 → outcome}

**Business rules:**
- {Validation, constraint, invariant}

**Platform constraints:**
- {OS support, hardware requirements, display-server requirements}

**Outbound network use (if any — see PART 9 → "Security-First Design"):**
- {Remote service consumed, purpose, auth model}

**Stored data location (per-user — see PART 4 → "Path Rule"):**
- {What is stored at the per-user paths}

**License exceptions (only if a denylisted or vendored-C dep is used — see PART 11 → "License Compliance"):**
- {crate, upstream license, rationale, `distribution_license` consequence if applicable}
```

**Rules for the example contents above:**

- No implementation details — describe behavior, not algorithms or libraries. AI.md PARTs 0–11 define HOW; PART 12 verifies compliance.
- This template targets GUI / TUI / CLI applications (PART 0 → "One Coherent Product").
- Cross-reference AI.md PARTs by number for any pattern that already exists there (Docker → PART 5, security → PART 9, license exceptions → PART 11, etc.).
- `internal_name` and `internal_org` are immutable after first set (see "IDEA.md Required Layout" → Project variables rules).

---

## IDEA.md Examples

**Example 1: Notes (GUI + CLI, fully offline)**

```markdown
## Project description

A local-first notes application with a native GUI for daily use and a CLI for scripting.
Notes are stored on the user's machine — no cloud sync, no telemetry, fully offline.

## Project variables

project_name:     notes
project_org:      casjay
internal_name:    notes
internal_org:     casjay
app_name:         Notes
crate_name:       notes
official_site:    https://notes.example.com
maintainer_name:  Jane Doe
maintainer_email: jane@example.com

## Business logic

**Target users:**
- Individual users wanting a fast, offline notes app
- Power users scripting note creation / search via shell

**Surfaces:**
- GUI: yes (Linux X11 + Wayland, macOS, Windows)
- TUI: no
- CLI: yes (`notes new`, `notes search`, `notes ls`)

**Features:**
- **Note management**: create, edit, archive, delete with markdown support
- **Search**: full-text search across all notes
- **Organization**: tag-based grouping; no folders
- **Export**: dump notes to a directory of `.md` files

**Data models:**
- Note: id, title, body (markdown), tags[], created_at, updated_at

**User flows:**
- Quick capture: Ctrl+N opens compose → write → Ctrl+S saves → return to list
- Search: Ctrl+K opens overlay → type → live-filter results

**Business rules:**
- Archive is reversible; delete is permanent
- All edits are local; no network
- Title can be empty; body cannot

**Platform constraints:**
- GUI requires X11 or Wayland on Linux/BSD (PART 0 → "X11 AND Wayland Are Both Required")
- No internet access required; works fully offline (PART 0 → "Self-Contained Assets")

**Outbound network use:** none

**Stored data location (per-user):**
- Notes database: `~/.local/share/casjay/notes/notes.db`
- Config: `~/.config/casjay/notes/config.toml`
- Cache: `~/.cache/casjay/notes/`

**License exceptions:**
- `rusqlite` with `bundled` feature for the local notes database. Pure-Rust alternative (`limbo`) is not yet stable for production; statically vendored SQLite C is acceptable per PART 0 → "Rust-Only Application" exception path. Distribution license remains MIT.
```

**Example 2: Feeds (TUI + CLI, consumes a remote API)**

```markdown
## Project description

A terminal-first RSS / Atom feed reader. Subscribes to feeds, fetches new items in the
background, and presents them in a keyboard-driven TUI. A `feeds` CLI exposes the same
operations for automation.

## Project variables

project_name:     feeds
project_org:      casjay
internal_name:    feeds
internal_org:     casjay
app_name:         Feeds
crate_name:       feeds
official_site:
maintainer_name:  Jane Doe
maintainer_email: jane@example.com

## Business logic

**Target users:**
- Terminal-first users who follow many feeds
- Anyone wanting offline-friendly RSS reading without a hosted service

**Surfaces:**
- GUI: no
- TUI: yes (primary)
- CLI: yes (`feeds add`, `feeds sync`, `feeds export`)

**Features:**
- **Subscription**: add / remove / rename feeds; OPML import / export
- **Sync**: fetch all feeds in parallel; respect HTTP cache headers
- **Reading**: keyboard navigation; mark read / unread; star items
- **Filtering**: by feed, by tag, by unread state

**Data models:**
- Feed: id, url, title, last_etag, last_modified, last_synced
- Item: id, feed_id, guid, title, body, published_at, read, starred

**User flows:**
- Launch → background sync starts → pick a feed → arrow keys to navigate → Enter to read
- `feeds sync` from cron pulls new items without launching the TUI

**Business rules:**
- Items are deduplicated by GUID within a feed
- Sync respects ETag / Last-Modified to avoid re-fetching unchanged feeds
- Network failures degrade gracefully (read-only mode over cached items)

**Platform constraints:**
- TUI requires a TTY (PART 3 → "Smart Detect Rules"); for headless cron use the `feeds sync` CLI subcommand

**Outbound network use:**
- HTTPS GET to feed URLs the user adds. TLS via `rustls` (PART 9 → "Security-First Design"). No telemetry, no tracking, no third-party analytics.

**Stored data location (per-user):**
- Feed / item database: `~/.local/share/casjay/feeds/feeds.db`
- Config: `~/.config/casjay/feeds/config.toml`
- Cache (article bodies, images): `~/.cache/casjay/feeds/`

**License exceptions:**
- `rusqlite` with `bundled` feature for the local feeds database. Same rationale as the Notes example. Distribution license remains MIT.
```

**Example 3: dotctl (CLI only, manages dotfiles)**

```markdown
## Project description

A small CLI for managing user dotfiles: stow / unstow symlinks, version-pin individual
files, and verify that what is on disk matches what the user committed.

## Project variables

project_name:     dotctl
project_org:      casjay
internal_name:    dotctl
internal_org:     casjay
app_name:         dotctl
crate_name:       dotctl
official_site:
maintainer_name:  Jane Doe
maintainer_email: jane@example.com

## Business logic

**Target users:**
- Developers managing dotfiles across multiple machines

**Surfaces:**
- GUI: no
- TUI: no
- CLI: yes (`dotctl link`, `dotctl unlink`, `dotctl status`, `dotctl diff`)

**Features:**
- **Link / unlink**: create or remove symlinks from a dotfiles repo into the user's home
- **Status**: report which managed files match, differ, or are missing
- **Diff**: show the diff between repo and on-disk versions
- **Atomic apply**: link operations are dry-run-able and reversible

**Data models:**
- ManagedFile: source_path (in repo), target_path (in home), state (linked / unlinked / divergent)

**User flows:**
- `dotctl status` shows the current state matrix
- `dotctl link --dry-run` previews what would change; `dotctl link` applies it

**Business rules:**
- Never overwrite a regular file without explicit `--force`
- Symlinks are created relative to the dotfiles repo so the home directory remains portable

**Platform constraints:**
- POSIX-style filesystem operations; Windows support uses junction-style links via the standard library

**Outbound network use:** none

**Stored data location (per-user — PART 4 → "Path Rule"):**
- Config: `~/.config/casjay/dotctl/config.toml`
- State (last known link map): `~/.local/share/casjay/dotctl/state.json`

**License exceptions:** none
```
