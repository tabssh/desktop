# {PROJECT_NAME} Specification

**Name**: {project_name}

**About this file:** `AI.md` is the complete, authoritative specification for this project. Throughout this document, all references to `AI.md` refer to this file.

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
| **AI.md** | SOURCE OF TRUTH - implementation rules (read-only) | No — use SPEC.md for project-specific rule overrides |
| **SPEC.md** | Project-specific rule overrides — created only when a rule must contradict this specification or global. May be empty. SPEC.md wins over AI.md. | When a project rule must differ from this specification or global |
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
`official_site` (only when the user supplies it — never invented), `maintainer_name`, `maintainer_email`, `crate_name`, etc.)

Example:

    project_name:  notes
    project_org:   casjay
    # STABLE — set at first-time setup; change only on an explicit rename or fork reset
    internal_name: notes
    # STABLE — set at first-time setup; change only on an explicit rename or fork reset
    internal_org:  casjay
    app_name:      Notes
    crate_name:    notes
    # official_site is OPTIONAL: add it only when the user gives the real URL; omit the key otherwise

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
- It MUST state which app surfaces exist: GUI, TUI, CLI, or a subset — and whether the project is an RFC protocol daemon (PART 14) or a local client/server (IPC) split (PART 2 → "Local Client/Server (IPC) Mode")
- It MUST define user flows, stored data, trust boundaries, abuse cases, and platform constraints
- If a security-sensitive choice is intentionally allowed, the reason MUST be documented there

## Migrating Existing `CLAUDE.md` Into `IDEA.md`

**If a repository already has a pre-existing `CLAUDE.md` or `.claude/CLAUDE.md` with real project details, those project details MUST be migrated into `IDEA.md`.**

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
4. If `internal_name` cannot be proven, initialize it to `project_name` on first migration and treat it as stable after that. Do the same for `internal_org` ← `project_org`.
5. If statements from `CLAUDE.md` or `.claude/CLAUDE.md` conflict with `AI.md`, `AI.md` wins
6. After migration, keep root `CLAUDE.md` and/or `.claude/CLAUDE.md` only as short efficient loaders and keep the real plan/spec in `IDEA.md`, and mirror the final root `CLAUDE.md` content into `AGENTS.md` as a real file copy (never a symlink)
7. Never silently discard meaningful project-specific content; migrate it, trim it, or explicitly ask where it belongs

---

# PART 0: CRITICAL RULES - READ FIRST

## THIS IS A STRICT SPECIFICATION - NOT GUIDELINES

- Every item in this specification MUST be followed exactly unless explicitly marked optional
- This is not a suggestion document
- There are no silent exceptions
- If the spec says X, do X - not "improved X"
- If something seems wrong, follow it and flag it; do not silently rewrite intent

## Attribution

**AI operates on behalf of the user in a Senior Developer / UI-UX Designer capacity.**

## ⚠️ CRITICAL: File Paths and Project Root

- All paths are relative to the project root unless explicitly noted
- Do not scatter top-level files unnecessarily
- Runtime-generated files are not committed
- AI must not move the project root or invent sibling repositories
- **Never edit files outside the project root to work around a problem inside it** — no host system files (`/etc/*`, shell rc files, systemd units), no sibling/other repositories, no global tool configs. A failing build, missing dependency, or broken tool inside the project is fixed in the project's own code/config/Docker image, never by reaching outside it. The only exception is an external path the user explicitly names for that specific task

## ⚠️ CRITICAL: AI.md is the Source of Truth

- `AI.md` is read-only during routine work
- `IDEA.md` is where project-specific values and product rules live
- Loader files (`CLAUDE.md`, `.claude/CLAUDE.md`) stay short and point back to `AI.md`
- If a loader file and `AI.md` disagree, `AI.md` wins
- `.claude/memory/` holds durable, project-specific knowledge discovered during
  development — decisions, gotchas, conventions unique to this codebase — distinct
  from `AI.md` (spec) and the loader files above. Committed to the repo, not
  gitignored. One markdown file per topic, YAML frontmatter (`name`, `description`,
  `type: project`), indexed by `.claude/memory/MEMORY.md`, read on demand. Same
  credential-masking rule as everywhere else. `~/.claude/**` (global) stays
  read-only, deployed only via `claudemgr/config`'s `install.sh`; `.claude/memory/`
  here is read/write in this repo directly

## ⚠️ CRITICAL: Keep Documentation in Sync

Update these when their subject changes:
- `IDEA.md` when features or variables change
- `README.md` when install, usage, or packaging changes
- `LICENSE.md` when dependencies or attribution changes
- CI/CD docs or scripts when release mechanics change

## ⚠️ CRITICAL: One Coherent Product

This specification defines **one Rust application** with shared core logic and up to three presentation layers:
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
- `Makefile` and `scripts/` are used for build automation — they orchestrate Docker invocations but MUST NOT contain application logic; Rust-side build logic lives in `build.rs` and the optional `xtask/` crate
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
| `x86_64-pc-windows-gnu` | static CRT via `-C target-feature=+crt-static` | Single `.exe`; `casjaysdev/rust:latest` does not ship an MSVC toolchain — `*-pc-windows-msvc` is NOT buildable, use `-gnu`/`-gnullvm` |
| `aarch64-pc-windows-gnullvm` | static CRT via `-C target-feature=+crt-static` | Single `.exe`; no MSVC toolchain in the image — use `-gnullvm`, not `-msvc` |
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
- The app may **read** user-provided config and data from per-user paths (PART 4 → "Path Rule"), and may **write** cache/state there, but it must run with full default functionality when those paths are empty
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
| **AI.md** | Implementation spec (HOW) - SOURCE OF TRUTH, read-only | No — use SPEC.md for project-specific rule overrides |
| **SPEC.md** | Project-specific rule overrides (optional, may be empty) | When a project rule must contradict this specification or global |
| **IDEA.md** | Project plan (WHAT) | Features or variables change |
| **TODO.AI.md** | Task tracking (AI-owned) | Tasks added/completed |
| **TODO.md** | Task tracking (human-owned) | AI may mark done; never delete/empty |
| **PLAN.AI.md** | Implementation plan (AI-owned) | Planning new work |
| **PLAN.md** | Implementation plan (human-owned) | AI may mark done; never rewrite wholesale |
| **README.md** | User-facing install/usage docs | Usage changes |
| **LICENSE.md** | Project + dependency licenses | Dependency set changes |
| **release.txt** | Optional local-build version (Makefile only; empty or missing = devel rule) | Local-build version changes |
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

**Hook-enforced:** `spec-guard.sh` blocks Edit/Write on project files until AI.md/SPEC.md has been Read this session; the gate re-arms after every compaction.

## Self-Validation Loop

**AI MUST verify its own work with real tools before reporting a task as done. Do not rely on "the code looks right."**

**This rule applies to EVERY change type covered by this specification — library/API logic, GUI/TUI/CLI binaries, single-static-binary build, asset embedding, Docker, CI/CD, configuration, documentation, security — not only one category.** Whatever you touched, you verify.

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

**AGENTS.md (REQUIRED, root):** Many non-Claude coding agents/tools read `AGENTS.md` at the project root instead of `CLAUDE.md`. Generate/reconcile `AGENTS.md` as an exact copy of root `CLAUDE.md`'s final content — same loader text, kept in sync whenever `CLAUDE.md` changes. Always a real file copy, never a symlink (symlinks are not portable across all platforms/filesystems).

**Loader rule:** loader files stay short. Long-form product content belongs in `IDEA.md`; long-form implementation policy belongs in `AI.md`. The same loader content is also mirrored into `AGENTS.md` at the project root as a real file copy — never a symlink.

## Commit workflow (required on every commit)

1. `git status --porcelain` + `git diff --stat` — see exactly what changed.
2. **Test gate** — `make test` (the Rust tests, run in Docker) must pass; never skip it to save time.
3. **Lint gate** — the `rust-lint` Agent, spawned through the Agent tool (never a shell command). Only NEW findings block; pre-existing findings are logged to `TODO.AI.md` before moving on. Never commit with a failing test or a new lint violation.
4. Write `{dir}/.git/COMMIT_MESS` from that output — every changed or untracked file needs a `- path: change` bullet (a `- dir/: …` bullet covers the files beneath it); never from memory.
5. **Doc-sync line** — for whichever of `IDEA.md` / `README.md` exists at the project root, add `- IDEA.md: updated (<what changed in Business Logic>)` or `- IDEA.md: N/A — no user-facing change` (the same for `README.md`); only Business Logic changes count as "updated".
6. Re-read `COMMIT_MESS` against the diff; rewrite if anything is missing or wrong.
7. `gitcommit --dir {dir} all` — the only commit path; never bare `git commit` or `git push` (they bypass signing and the unified commit+push wrapper), never `-m`/`--message`. Call it as plain `gitcommit` with no path prefix and never read its script. `gitcommit --dir {dir} spec` commits only markdown spec files.

**Format:** `{emoji} Title (≤64 chars) {emoji}` + blank line + body + `- path: change` bullets. Emoji map: ✨ feat · 🐛 fix · 📝 docs · 🎨 style · ♻️ refactor · ⚡ perf · ✅ test · 🔧 chore · 🔒 security · 🗑️ remove · 🚀 deploy · 📦 deps. No bare `@` handles (they notify and link a GitHub user); no AI attribution trailers; never invent a role label such as "operator decision" for who made a change — state the fact plainly ("removed at the user's request") or leave the attribution out.

**Scope:** a commit's scope is the resolved unit of work, not "one fix" by default — decide the scope BEFORE fixing, in this order (first match wins): **(1)** user states or implies a single commit → everything for that request in one commit, overriding every rule below (never the gates); **(2)** ad hoc "fix X and anything else you find" → one commit for the whole request; **(3)** multi-file bug fixes → one commit per group of files actually coupled by dependency/root cause, split when files are independent; **(4)** findings-based work (audits, reviews, numbered fix-lists) → one commit per finding by default, batched only when genuinely inseparable; **(5)** feature work → one commit for the entire feature plus directly-related bugs, never split per part. Never commit a half-finished or inconsistent state. Unrelated bugs found mid-feature go to `TODO.AI.md`, except app-breaking bugs, which must be fixed immediately.

**Findings-based work:** before writing `COMMIT_MESS`, re-list every finding by its original number and confirm with a grep or diff that its fix is present in the working tree. A finding without a matching diff hunk is not fixed: do not list it as done, and either resolve it or say plainly that it is deferred.

**Who commits:** only the main session runs `gitcommit` or writes `COMMIT_MESS`. Agents and subagents never commit or push, never write `COMMIT_MESS`, and never run `make`, tests, lint or any gate — they edit their scoped files and report back; the main session reviews the full diff, runs the gates once and commits.

**Push behavior:** push is immediate and irreversible, and `gitcommit` creates the remote repository when it does not exist yet. To skip pushing, `touch .no_push` (confirm with the user first). If the push fails offline, run `gitcommit --dir {dir} push` later — do not recreate `COMMIT_MESS`.

**Hook blocks and overrides:** a hook that blocks a commit (test/lint gate, doc-sync, `COMMIT_MESS` coverage) is reported to the user and never bypassed automatically. The `TEST_LINT_GATE_OVERRIDE=1` and `DOC_SYNC_GATE_OVERRIDE=1` prefixes exist only for a user who explicitly directs the bypass after confirming the underlying check passed.

**`TODO.AI.md` completion:** a committed TODO file must be true at the moment of the commit. (1) Implement the task; (2) delete its item from `TODO.AI.md` in that same change, before `COMMIT_MESS` is written, so the commit never leaves finished work listed; (3) run `gitcommit`, and if it fails put the item back and fix the cause (an item never disappears while its work is uncommitted or failing); (4) a partly done item is rewritten to the remaining work, never left unchanged; (5) before every commit re-read `TODO.AI.md`, `TODO.md` and `PLAN.AI.md` and confirm nothing this commit completes is still open and nothing is listed done that is not implemented; (6) the commit that deletes the LAST item uses the title `✅ all todo items have been completed ✅` and a body that summarizes what was accomplished. The human-owned `TODO.md` is never emptied or truncated by AI — only individual items are marked done in place.

---

# PART 2: RUST APPLICATION MODEL

## Product Model

This specification targets a **single-binary, fully self-contained Rust application** that may expose:
- a native GUI
- a terminal UI (TUI)
- a plain CLI
- an **RFC protocol server (daemon)** — conditional; only when IDEA.md declares it (PART 14)
- a **local client/server (IPC) split** — conditional; only when IDEA.md declares it (see "Local Client/Server (IPC) Mode" below)

The application may make outbound network calls to consume remote services it depends on (APIs, databases, object stores, update endpoints, etc.).

**Network listening sockets (TCP/UDP) are allowed only in daemon mode** — a project that accepts inbound network connections MUST satisfy PART 14; otherwise the application opens no network listening sockets. Unix domain sockets and named pipes are local IPC, not network listening — they are governed by "Local Client/Server (IPC) Mode" below, not by PART 14.

**Distribution model:** one statically linked binary per supported target. Everything the app needs at runtime — UI assets, fonts, icons, default config, schemas, templates, locales — is embedded inside that binary. See PART 0 → "Single Static Binary" and "Self-Contained Assets."

## Local Client/Server (IPC) Mode

Some applications split into a thin client and a long-lived local server the way tmux, `nvim --listen`, and atuin's daemon do. This is allowed for any surface when IDEA.md `## Business logic` declares the client/server architecture — it does NOT trigger PART 14, which governs network protocol daemons only.

**Socket placement & permissions:**
- All sockets live in `$XDG_RUNTIME_DIR/{project_name}/` (fallback: the per-user state dir, PART 4 → "Path Rule"); directory mode `0700`, sockets `0600` — never world-accessible, never in shared `/tmp` paths
- Private wire protocols are allowed on IPC sockets — PART 14's "never invent wire protocols" rule applies to network sockets only
- No TCP listener by default, not even loopback — a TCP listener changes the trust boundary and requires its own explicit IDEA.md declaration

**Main socket + named servers:**
- The first server started is the **main server** and owns the **main socket**: `$XDG_RUNTIME_DIR/{project_name}/{project_name}.sock`
- The user may start additional **named servers**; each gets its own socket: `$XDG_RUNTIME_DIR/{project_name}/{server_name}.sock` (server names validated against `[a-z0-9._-]+`; no path separators, never colliding with the main socket name)
- **The main server is aware of all servers** — it keeps a registry of every named server; named servers announce themselves to the main socket on startup and deregister on clean shutdown
- Clients enumerate all running servers through the main socket (e.g. a `{project_name} servers` / `list-servers` command)
- **Registry self-heal:** on main-server start and on every listing, the registry reconciles against a scan of the socket directory — each socket is connect-tested, and dead/stale entries are dropped from the registry and their socket files unlinked
- A missing main server never blocks a named server from starting — the named server retries registration in the background and is picked up by the main server's next reconcile scan

**Advanced capabilities (optional — each one used only when IDEA.md `## Business logic` declares it):**
- **Session state awareness** — the server may track live per-session/per-client state: working directory, foreground command, environment snapshot, window/pane layout, whatever the product needs; clients query it over the IPC socket
- **Save/restore** — full server state (sessions, layout, per-session working directory, running-command metadata) serializes to the per-user state dir (PART 4 → "Path Rule") as `0600` files, written atomically (temp file + rename); an explicit `save` / `restore` command pair
- **Continuous persistence (continuum-style)** — periodic autosave on a configurable interval plus save on clean shutdown, and restore-on-start reattaches the previous state; autosave runs in the background and never blocks the serving path
- **Restore never re-executes commands by default** — a default restore recreates sessions, layout, and working directories only; re-launching previously running programs happens only when the user has explicitly enabled it (per program or per profile), mirroring tmux-resurrect's opt-in model
- **Privacy:** tracked state (working directories, command lines, environment) is sensitive data — stored only in per-user `0600` files, never logged at default verbosity, never transmitted off-machine, and excluded from any diagnostics output unless the user explicitly opts in

**Trust boundary:** the IPC surface trusts same-UID local processes only — access control is the `0700`/`0600` filesystem permissions above; peer-credential checks (`SO_PEERCRED` on Linux, `getpeereid` on BSD/macOS) are the defense-in-depth verification. IDEA.md's **Trust boundaries & abuse cases** block MUST name this boundary (abuse case: another local user or process attempting to reach a socket → refused by permissions and peer check).

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

(See PART 5 → "Project Layout" for the full repository tree, including `docker/`, `assets/`, `packaging/`, `Makefile`, `scripts/`, `xtask/`, etc.)

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
| `{platform}` | `linux`, `windows`, `darwin`, `freebsd`, `netbsd`, `openbsd` (lowercase, normalized OS name — never the full Rust target triple; macOS is always `darwin`, never `macos`) |
| `{arch}` | `amd64`, `arm64`, `armv7`, `i686`, `riscv64`, … (normalized architecture name — `x86_64` maps to `amd64`, `aarch64` maps to `arm64`; other arches keep the triple's arch token) |
| `{.ext}` | `.exe` on Windows; empty everywhere else |

**Normalization rules:**
- Strip the `-musl` libc suffix from the artifact name. `x86_64-unknown-linux-musl` → `linux-amd64`, NOT `linux-amd64-musl`. Static linkage is the project default (PART 0 → "Single Static Binary"), so calling it out in the filename is redundant.
- Strip the vendor token (`unknown`, `pc`, `apple`) from the artifact name.
- Strip the ABI token (`gnu`, `msvc`, `eabihf`, …) from the artifact name unless multiple ABIs of the same `{platform}-{arch}` ship in the same release; in that rare case append `-{abi}` after `{arch}`.
- `apple-darwin` maps to `darwin`. `pc-windows-gnu` / `pc-windows-gnullvm` maps to `windows`. `unknown-linux-musl` maps to `linux`. `x86_64` maps to `amd64`; `aarch64` maps to `arm64` — artifact names use Go-style OS/arch terms so Go and Rust releases are named identically.

**Worked examples:**

| Rust target triple | Artifact name |
|--------------------|---------------|
| `x86_64-unknown-linux-musl` | `{project_name}-linux-amd64` |
| `aarch64-unknown-linux-musl` | `{project_name}-linux-arm64` |
| `armv7-unknown-linux-musleabihf` | `{project_name}-linux-armv7` |
| `x86_64-pc-windows-gnu` | `{project_name}-windows-amd64.exe` |
| `aarch64-pc-windows-gnullvm` | `{project_name}-windows-arm64.exe` |
| `x86_64-apple-darwin` | `{project_name}-darwin-amd64` |
| `aarch64-apple-darwin` | `{project_name}-darwin-arm64` |
| `x86_64-unknown-freebsd` | `{project_name}-freebsd-amd64` |

**Other rules:**
- Local (in-tree) primary binary name: `{project_name}` (no platform/arch suffix during local development inside the Docker image)
- If optional helper binaries exist, use `{project_name}-{tool}` for the in-tree name and `{project_name}-{tool}-{platform}-{arch}{.ext}` for distribution
- Checksums are published as two aggregate files per release — `sha256.txt` and `sha512.txt` — covering every artifact in standard `sha256sum`/`sha512sum` format (`{hash}  {filename}` per line); never per-artifact sidecar files

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

**On-disk paths use the stable pair `{internal_org}` and `{internal_name}` only — never the mutable `{project_org}` / `{project_name}`. This is the rule that protects user data across project/org renames.**

| Purpose | Linux / BSD | macOS | Windows |
|---------|-------------|-------|---------|
| Config | `~/.config/{internal_org}/{internal_name}/` | `~/Library/Application Support/{internal_org}/{internal_name}/config/` | `%AppData%\\{internal_org}\\{internal_name}\\config\\` |
| Data | `~/.local/share/{internal_org}/{internal_name}/` | `~/Library/Application Support/{internal_org}/{internal_name}/data/` | `%LocalAppData%\\{internal_org}\\{internal_name}\\data\\` |
| Cache | `~/.cache/{internal_org}/{internal_name}/` | `~/Library/Caches/{internal_org}/{internal_name}/` | `%LocalAppData%\\{internal_org}\\{internal_name}\\cache\\` |
| Logs | `~/.local/state/{internal_org}/{internal_name}/logs/` | `~/Library/Logs/{internal_org}/{internal_name}/` | `%LocalAppData%\\{internal_org}\\{internal_name}\\logs\\` |

**Rule:** Both `{internal_name}` and `{internal_org}` anchor on-disk identifiers and stable OS-registered names (Bundle IDs, package IDs, dbus names, keychain entries, updater channels). A rename of `{project_name}` or `{project_org}` MUST NOT silently move user data or change those identifiers.

---

# PART 5: RUST TOOLCHAIN, BUILD, AND PACKAGING

## Toolchain Rules

| Rule | Description |
|------|-------------|
| Rust edition | Use the current stable Rust edition declared in `Cargo.toml` |
| Toolchain channel | Stable by default; toolchain version pinned in `rust-toolchain.toml` |
| Targets | Static targets pre-installed in the Docker image: `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, Windows GNU targets (`x86_64-pc-windows-gnu`, `aarch64-pc-windows-gnullvm`) with `+crt-static` — the image ships no MSVC toolchain, so `*-pc-windows-msvc` is NOT buildable — Apple Silicon + x86_64 macOS. `x86_64-unknown-freebsd` is pre-installed; other BSD targets (`netbsd`, `openbsd`, aarch64 equivalents) are opt-in via `rustup target add` when IDEA.md declares them in scope (PART 0 → "Single Static Binary") |
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

**All cargo invocations execute inside the project Docker container — never on the host.** The table below shows the *logical* command; the **actual** invocation is wrapped via the `CARGO_DOCKER` macro (see "Build Metadata" in PART 6 for the full macro definition). See "Docker Rule" below.

| Logical Command | Purpose |
|-----------------|---------|
| `cargo fmt --all` | Format code |
| `cargo fmt --all --check` | Formatting verification |
| `cargo clippy -- -D warnings` | Lint enforcement |
| `cargo test --lib --no-fail-fast` | Test suite |
| `cargo build` | Debug build |
| `cargo build --release` | Release build |
| `cargo doc --workspace --no-deps` | Documentation build |
| `cargo run -- [args]` | Execution (in container; with X11/Wayland forwarding if GUI) |
| `cargo install --path .` | In-container install for packaging/CLI testing |

**Examples of the actual wrapped form (using the `CARGO_DOCKER` macro from the Makefile):**

```bash
# Format check
$(CARGO_DOCKER) cargo fmt --all --check

# Lint
$(CARGO_DOCKER) cargo clippy --workspace --all-targets --all-features -- -D warnings

# Tests
$(CARGO_DOCKER) cargo test --workspace --all-features

# Release build (artifacts land in binaries/ via the mounted workspace)
$(CARGO_DOCKER) \
  sh -c "cargo build --release --target x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/{project_name} \
      binaries/{project_name}-linux-amd64"

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
- Use `RUSTFLAGS="-C target-feature=+crt-static"` for Windows GNU targets (`x86_64-pc-windows-gnu`, `aarch64-pc-windows-gnullvm`) — the image has no MSVC toolchain, so `*-pc-windows-msvc` is not an option; use musl targets for Linux; for BSDs, use the platform's native target triple and statically link where the platform allows
- All runtime assets are embedded at compile time; the repo's `assets/` directory is build-time input only (PART 0 → "Self-Contained Assets")
- A static-binary self-check runs as part of CI (`ldd` on Linux musl artifacts must show "not a dynamic executable" or only kernel-vDSO; `otool -L` on macOS must show only Apple-provided frameworks; `dumpbin /dependents` on Windows GNU/static-CRT builds must show no MinGW runtime DLLs)
- If a workspace is used, keep one top-level `Cargo.toml` as the build entry point
- Recommended release profile in `Cargo.toml`:

```toml
[profile.release]
opt-level     = "z"
lto           = true
codegen-units = 1
strip         = true
panic         = "abort"
```

- Required `.cargo/config.toml` snippets for static linking (Bootstrap Checklist enforces presence):

```toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "target-feature=+crt-static"]

[target.aarch64-pc-windows-gnullvm]
rustflags = ["-C", "target-feature=+crt-static"]
```

## Build Metadata

Build metadata (version, commit ID, build epoch, official site) is embedded at compile time by `build.rs`, which maps the `VERSION`, `COMMIT_ID` and `BUILD_EPOCH` build-environment variables (plus `site.txt`) to the `APP_*` constants `option_env!()` reads. The build date is never embedded — `build_date()` derives it from the embedded epoch at runtime (see PART 6 → "Build Metadata" for the full `build.rs` pattern and runtime constants).

**Full release build command (logical form — run inside the Docker container, not on the host):**
```bash
# BUILD_EPOCH is the single captured time source - captured once per build
BUILD_EPOCH="$(date -u +%s)"

# VERSION comes from the version rule (CI release = pushed tag with leading v stripped; local = Makefile)
VERSION="${VERSION}" \
COMMIT_ID="$(git rev-parse --short=7 HEAD)" \
BUILD_EPOCH="${BUILD_EPOCH}" \
cargo build --release --target x86_64-unknown-linux-musl

cp target/x86_64-unknown-linux-musl/release/{project_name} \
  binaries/{project_name}-linux-amd64
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
deps/                   # OPTIONAL: committed, project-specific support files
                         # not part of build/release output (e.g. scripts or
                         # Dockerfiles for building a dependency) — never a
                         # cache or temp/output dir
docker/                 # REQUIRED: Dockerfile, docker-compose.yml, rootfs/usr/local/bin/entrypoint.sh, README.md
Makefile                # build targets: build, release, docker, test, dev, clean
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

**Operator-side install (the fourth exception).** `--service --install` (PART 23) and the packages and installers (PART 24) place the binary at the service path (`/usr/local/bin/{project_name}`) on the machine that will run it. That is the operator's action on their own system: AI sessions and development workflows never run it on the development machine, and the rules above stay in force there.

## Release Artifacts

- Primary binary follows the naming schema `{project_name}-{platform}-{arch}{.ext}` defined in PART 2 → "Binary Model" — single statically linked file, `-musl`/vendor/ABI tokens stripped
- Each release MUST publish artifacts for at minimum: `{project_name}-linux-amd64`, `{project_name}-linux-arm64`, `{project_name}-windows-amd64.exe`, `{project_name}-windows-arm64.exe`, `{project_name}-darwin-amd64`, `{project_name}-darwin-arm64` (subset acceptable only when IDEA.md narrows platform scope)
- A static-linkage verification step is part of release: `ldd` / `otool -L` / `dumpbin /dependents` output is captured and checked against an allowlist (kernel vDSO, Apple system frameworks, Windows kernel32/user32 etc.) — anything outside the allowlist fails the release
- No companion files (no `.so`, `.dylib`, `.dll`, no asset bundles, no font directories) ship next to the binary
- Include two aggregate checksum files covering every published artifact — `sha256.txt` (SHA-256) and `sha512.txt` (SHA-512), standard `{hash}  {filename}` format — uploaded as release assets; never per-artifact sidecar files
- Include release notes that describe actual changes
- Include an SBOM (always — generated via `cargo-cyclonedx`; see PART 10 → "Suggested CI Steps" for the invocation, PART 11 → "Required Tooling" for the tool pin). Include provenance/attestation via `actions/attest-build-provenance` when the release platform supports it; always set `provenance: false` on `docker/build-push-action` steps
- If GUI packaging exists (MSI, DMG, AppImage, deb, rpm, etc.), the package wraps the same single static binary plus desktop integration metadata; package metadata lives in `packaging/`

## Docker Rule

Docker is **REQUIRED**. The project MUST ship a working Docker setup, and every toolchain command and every produced binary runs inside it (PART 0 → "No Host Toolchain or Binary Execution").

### Required Docker Assets

Place Docker assets under `docker/`:

```text
docker/
├── Dockerfile              # production runtime image — two-stage (builder + minimal Alpine); tagged :latest
├── Dockerfile.dev          # devel image — same as release but binary runs in debug mode; tagged :devel   (project-specific)
├── rootfs/                 # build-time filesystem overlay copied into image at /   (project-specific)
│   └── usr/local/bin/entrypoint.sh   # prepares cache dirs; user creation and privilege drop happen in the binary; called by tini → entrypoint.sh → app
├── docker-compose.yml      # production compose — HUMAN USE ONLY
├── docker-compose.dev.yml  # development compose — runs `:devel` image in debug mode; HUMAN USE ONLY
├── docker-compose.test.yml # automated test compose — AI prefers the tests/ scripts over running this directly
└── README.md               # how to build the image, run tests, run GUI with display forwarding
```

`docker-compose.yml` always lives under `docker/` — never at the repo root. See `dockerfile_conventions.md` → "Docker Compose" for the canonical layout: `docker-compose.yml` and `docker-compose.dev.yml` are human-only, and AI's preferred interface for `docker-compose.test.yml` is the project's `tests/` scripts rather than a direct invocation.

### Toolchain Image

**All Rust projects use `casjaysdev/rust:latest` — never create `docker/Dockerfile.build` or `build-toolchain.yml` for Rust.** The maintained image includes everything any Rust project needs:

- Latest stable toolchain + nightly (minimal profile with miri and rust-src)
- Components: `rustfmt`, `clippy`, `rust-src`, `rust-analyzer`, `llvm-tools-preview`
- `cargo-binstall` for fast tool installation without source compilation
- C/C++ toolchain: `build-base`, `musl-dev`, `clang`, `clang-dev`, `llvm-dev`, `lld`, `cmake`, `make`, `perl`, `openssl-dev`, `openssl-libs-static`, `pkgconf`, `gdb`
- Cross-compile linker: `mingw-w64-gcc` (Windows GNU), `zig` (cargo-zigbuild), `binaryen` (WASM optimisation)
- `perf` — Linux performance counters, required by `flamegraph`/`samply` for profiling
- Cross-compile targets: musl Linux (x86_64, aarch64, i686, armv7, riscv64), glibc Linux (x86_64, aarch64, i686, armv7, arm, riscv64, ppc64le, s390x), Windows GNU (x86_64, i686, aarch64), macOS (x86_64, aarch64), FreeBSD, WebAssembly (wasm32-unknown-unknown, wasip1, wasip2, emscripten), embedded ARM/RISC-V, Android (aarch64)
- `cargo-audit`, `cargo-deny`, `cargo-tarpaulin`, `cargo-llvm-cov`, `grcov` — security and coverage
- `cargo-nextest`, `cargo-make`, `just` — testing and task runners
- `cargo-zigbuild`, `cross`, `cargo-ndk` — cross-compilation runners
- `sccache` — compiler cache
- `cargo-release`, `cargo-dist`, `cargo-deb` — release tooling
- `cargo-generate` — scaffold new projects from templates
- `flip-link` — zero-cost stack overflow protection for embedded targets
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
- GitHub Actions: use `docker/metadata-action@dc802804100637a589fabce1cb79ff13a1411302  # v6.2.0` with an `annotations:` input listing the required OCI keys
- `docker/build-push-action@c3c9e263c25d99ce0380d002d59b67737d91b0dc  # v7.4.0` MUST set `annotations: ${{ steps.meta.outputs.annotations }}`, `labels: ""` to suppress label output, AND `provenance: false` to prevent a spurious `unknown/unknown` platform entry in the manifest list (use `actions/attest-build-provenance` for release binary attestation instead)

See `dockerfile_conventions.md` → "OCI Annotations" for the full required annotation set (`org.opencontainers.image.{title,description,url,source,documentation,vendor,authors,vcs-type,version,revision,created,licenses,...}`).

### Build-Time Metadata Args

`docker/Dockerfile` declares `ARG VERSION` (default `devel`) and `ARG BUILD_EPOCH` in the builder stage and exports them as environment variables for the `cargo build` step, so `build.rs` embeds them (PART 6 → "Build Metadata"). `ARG BUILD_DATE` is declared only where it feeds the `org.opencontainers.image.created` value — it is never passed to the cargo build. The caller captures `BUILD_EPOCH` once, derives `BUILD_DATE` from it, and passes both:

```bash
# Captured ONCE; BUILD_DATE is derived from it, never independently captured
BUILD_EPOCH="$(date -u +%s)"
BUILD_DATE="$(date -u -d "@${BUILD_EPOCH}" +%Y-%m-%dT%H:%M:%SZ)"
docker build -f docker/Dockerfile \
  --build-arg VERSION="$VERSION" \
  --build-arg BUILD_EPOCH="$BUILD_EPOCH" \
  --build-arg BUILD_DATE="$BUILD_DATE" \
  ...
```

### Container Runtime Rules

Every production image MUST satisfy:

- **Startup chain `tini → entrypoint.sh → app`** — `ENTRYPOINT [ "tini", "-p", "SIGTERM", "--", "/usr/local/bin/entrypoint.sh" ]`. Never override `ENTRYPOINT` or `CMD` to bypass `tini` or the entrypoint shim. All startup customization goes in `docker/rootfs/usr/local/bin/entrypoint.sh`, which MUST end with `exec "$@"` to preserve PID 1 signal handling.
- **`STOPSIGNAL SIGTERM`** (or `SIGRTMIN+3` for s6-based images) for graceful shutdown
- **`HEALTHCHECK`** — every production image declares a `HEALTHCHECK` that exits non-zero when the binary is unhealthy
- **Privilege drop, not Dockerfile users** — containers start as root with NO `USER` directive and no user/group creation in the Dockerfile. The binary itself creates its dedicated user/group, creates its directories, sets permissions, then drops privileges once initialization completes. `entrypoint.sh` may export UID/GID env vars so the binary can match host ownership of mounted volumes. Running permanently as root (never dropping) is the exception and MUST be justified in `IDEA.md`.

### Mandatory `docker run` Naming Convention

Every `docker run` invocation in this project (CI, scripts, docs, examples) MUST use:

```bash
PROJECT_NAME="{project_name}"
PROJECT_IMAGE="casjaysdev/rust:latest"

docker run --rm \
  --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" \
  ...
```

`PROJECT_NAME` and `PROJECT_IMAGE` are shell variables set once (e.g., in the Makefile or the calling script) before any of the `docker run` examples below — `PROJECT_NAME` resolves to the project's `{project_name}` and `PROJECT_IMAGE` to the toolchain image selected under PART 5 (normally `casjaysdev/rust:latest`).

- `--rm` — self-remove on exit (no orphaned containers)
- `--no-healthcheck` — on toolchain and one-shot runs, so the image's service `HEALTHCHECK` never polls them
- `-it` — ONLY for an interactive run with a real TTY (log streaming, signal handling); batch, script and CI runs never use it, because `docker run -it` fails without a TTY
- `--name "${PROJECT_NAME}-XXXX"` — traceable name; `XXXX` is the 8-char lowercase-alphanumeric random suffix produced by `head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8` (Makefile form: `$$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)`)

There is no opt-out. Unnamed or persistent build/test containers are a spec violation.

**Container discipline (keep usage low):**

- Steps that belong together run in ONE container (`sh -c "a && b && c"`), never a container per step (format, then lint, then one package's tests)
- The test gate (`make test`) runs ONCE per finished change set; a failure is fixed, then the full gate runs again
- At most ONE toolchain container per repository at a time; sub-agents and forks never start containers
- Every run is bounded: the Makefile prefix wraps `docker run` in `timeout -k 30 $(DOCKER_TIMEOUT)`, and a script or ad-hoc run does the same, so a hung step cannot hold a container for hours
- Any script that starts a container, sidecar, network or volume removes it in a `trap cleanup EXIT INT TERM` handler registered before the first one starts, so a failed or interrupted run leaves nothing behind
- A leftover container is found with `docker ps -a --filter "name=^${PROJECT_NAME}-"` and removed by its exact name, never by pattern and never with `prune`
- Before reporting a task done, that listing is empty

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

### CI/CD Workflow Pattern

All Rust CI jobs run inside `casjaysdev/rust:latest`. Never `apk add`, `cargo install`, or `rustup` in a workflow `run:` step — every tool is already in the image. No `ensure-build-image` pre-flight, no `build-toolchain.yml`.

Canonical job pattern for `ci.yml` / `release.yml`:

```yaml
jobs:
  build:
    runs-on: ubuntu-26.04
    container:
      image: casjaysdev/rust:latest
      options: "--user 0:0"
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
      - name: Capture build epoch
        run: |
          # Captured ONCE per job - every other time value derives from this
          BUILD_EPOCH=$(date -u +%s)
          echo "BUILD_EPOCH=$BUILD_EPOCH" >> $GITHUB_ENV
      - run: cargo build --release
        env:
          BUILD_EPOCH: ${{ env.BUILD_EPOCH }}
```

### X11 and Wayland Forwarding (Mandatory for GUI/Display Testing)

GUI and display-aware test runs use the `gui` compose service (or equivalent `docker run` flags). Both X11 and Wayland forwarding MUST be supported; the spec does not pick one — the container detects what the host provides and forwards accordingly.

**X11 forwarding (host running Xorg or XWayland):**

```bash
# grant access to current user only; revoke when done
xhost +SI:localuser:$(id -un)

docker run --rm \
  --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" \
  -e DISPLAY="$DISPLAY" \
  -e XAUTHORITY=/tmp/.docker.xauth \
  -v /tmp/.X11-unix:/tmp/.X11-unix:rw \
  -v "$XAUTHORITY":/tmp/.docker.xauth:ro \
  --device /dev/dri \
  -v "$PWD":/work -w /work \
  "$PROJECT_IMAGE" cargo run -- --ui gui

# revoke after the session
xhost -SI:localuser:$(id -un)
```

**Wayland forwarding (host running a Wayland compositor):**

```bash
docker run --rm \
  --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" \
  -e WAYLAND_DISPLAY="$WAYLAND_DISPLAY" \
  -e XDG_RUNTIME_DIR=/tmp/xdg \
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

## `release.txt` (Local-Build Version, Optional)

- `release.txt` is a single-line version for local Makefile builds when present and non-empty
- It overrides the devel rule for local builds and should stay in sync with package metadata
- Use semantic versions unless the project intentionally uses another documented scheme
- Only the Makefile reads it, for local builds; CI/CD workflows derive VERSION from the pushed tag (leading `v` stripped) and never read it
- If missing or empty, the Makefile uses the devel rule (short commit id)

## `site.txt` (Optional Official Site)

- Optional single-line URL file in project root
- Use it for the official site, homepage, docs root, support page, or update landing page
- Never guess it, never derive it from the project name, org, FQDN or a git remote, and never write an `example.com` or placeholder host into it or into `official_site`; unset means unset (no key, no file, empty `OFFICIAL_SITE`)
- If `site.txt` exists, it wins over `IDEA.md`, README, environment variables, and CI secrets
- If `site.txt` and `IDEA.md` disagree, `site.txt` wins; update IDEA.md to match or remove the stale variable

## Metadata Priority Rules

### Version
1. CI release: pushed tag with leading `v` stripped (CI never reads `release.txt`)
2. Local builds: `VERSION` environment variable (user override only; AI never sets it)
3. `release.txt` (Makefile only, when present and non-empty)
4. explicit documented fallback (`devel`)

### Official Site
1. `site.txt` (when present)
2. `IDEA.md ## Project variables` (`official_site`)
3. explicit environment/CI override used only where documented
4. empty

## Build Metadata

Embed at build time when practical:
- version
- commit ID
- build epoch (Unix seconds, UTC — the build date is derived from it at runtime)
- official site (optional)

**Single captured time source — `BUILD_EPOCH`:** every build path (local script, Makefile, Dockerfile, CI job) captures the build time exactly once, as Unix seconds UTC, and derives everything else from it:

```bash
# Captured ONCE per build; every other time value derives from this
BUILD_EPOCH="$(date -u +%s)"
```

**Makefile pattern for build metadata and the Docker build macro** (the derived `BUILD_DATE` line sits immediately after the capture):

```makefile
PROJECTNAME := {project_name}
PROJECTORG  := {project_org}
BINDIR      := binaries
RELDIR      := releases
TARGET      ?= x86_64-unknown-linux-musl

# Local builds only: VERSION env (user override, AI never sets it) > non-empty release.txt > devel rule (short commit id, "devel" outside git)
VERSION     ?= $(shell v=; [ -r release.txt ] && v=$$(tr -d '[:space:]' < release.txt); if [ -n "$$v" ]; then echo "$$v"; else git rev-parse --short=7 HEAD 2>/dev/null || echo devel; fi)
COMMIT_ID   := $(shell git rev-parse --short=7 HEAD 2>/dev/null || echo "N/A")
# BUILD_EPOCH is the single captured time source - captured once per build
BUILD_EPOCH := $(shell date -u +%s)
# Derived from BUILD_EPOCH - used only for the Docker OCI created annotation; never passed to cargo
BUILD_DATE  := $(shell date -u -d @$(BUILD_EPOCH) +"%Y-%m-%dT%H:%M:%SZ")

CARGO_CACHE ?= $(HOME)/.cargo/registry
CARGO_GIT   ?= $(HOME)/.cargo/git

DOCKER_MEM  ?= 4g
DOCKER_CPUS ?= 2

# CPU weight under contention (Docker's default is 1024): a build never starves the host's other work
DOCKER_CPU_SHARES ?= 256

# Hard ceiling in seconds for ONE container run, so a hung step can never hold a container for hours (raise it for a very long cross-compile)
DOCKER_TIMEOUT ?= 3600

# COMMIT_ID and BUILD_EPOCH are exported into the build environment so build.rs embeds them
CARGO_DOCKER := timeout -k 30 $(DOCKER_TIMEOUT) docker run --rm --no-healthcheck \
	--name $(PROJECTNAME)-$$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8) \
	--memory=$(DOCKER_MEM) --cpus=$(DOCKER_CPUS) --cpu-shares=$(DOCKER_CPU_SHARES) \
	-v $(PWD):/app \
	-v $(CARGO_CACHE):/usr/local/cargo/registry \
	-v $(CARGO_GIT):/usr/local/cargo/git \
	-w /app \
	-e VERSION=$(VERSION) \
	-e COMMIT_ID=$(COMMIT_ID) \
	-e BUILD_EPOCH=$(BUILD_EPOCH) \
	casjaysdev/rust:latest

TARGETS          ?= x86_64-unknown-linux-musl aarch64-unknown-linux-musl x86_64-pc-windows-gnu aarch64-pc-windows-gnullvm x86_64-apple-darwin aarch64-apple-darwin
DOCKER_PLATFORMS ?= linux/amd64,linux/arm64
REGISTRY         ?= ghcr.io/$(PROJECTORG)/$(PROJECTNAME)

.PHONY: build release docker test dev clean

build:
	@mkdir -p $(BINDIR)
	$(CARGO_DOCKER) cargo build --release --target $(TARGET)
	@cp target/$(TARGET)/release/$(PROJECTNAME) $(BINDIR)/$(PROJECTNAME)

test:
	@$(CARGO_DOCKER) sh -c " \
		mkdir -p \"\$${TMPDIR:-/tmp}/$(PROJECTORG)\" && \
		COVDIR=\$$(mktemp -d \"\$${TMPDIR:-/tmp}/$(PROJECTORG)/$(PROJECTNAME)-XXXXXX\") && \
		cargo tarpaulin --workspace --all-features --out Xml \
			--output-dir \$$COVDIR --fail-under 60 && \
		echo \"Tests complete - Coverage >= 60% required ✓\""

release:
	@mkdir -p $(BINDIR) $(RELDIR)
	@for target in $(TARGETS); do \
		echo "Building $$target..."; \
		$(CARGO_DOCKER) cargo build --release --target $$target || exit 1; \
		case $$target in \
			x86_64-unknown-linux-musl)  OUT=$(PROJECTNAME)-linux-amd64 ;; \
			aarch64-unknown-linux-musl) OUT=$(PROJECTNAME)-linux-arm64 ;; \
			x86_64-pc-windows-gnu)      OUT=$(PROJECTNAME)-windows-amd64.exe ;; \
			aarch64-pc-windows-gnullvm) OUT=$(PROJECTNAME)-windows-arm64.exe ;; \
			x86_64-apple-darwin)        OUT=$(PROJECTNAME)-darwin-amd64 ;; \
			aarch64-apple-darwin)       OUT=$(PROJECTNAME)-darwin-arm64 ;; \
		esac; \
		BIN=$(PROJECTNAME); case $$target in *windows*) BIN=$(PROJECTNAME).exe ;; esac; \
		cp target/$$target/release/$$BIN $(BINDIR)/$$OUT; \
	done
	@cp $(BINDIR)/$(PROJECTNAME)-* $(RELDIR)/
	@echo "$(VERSION)" > $(RELDIR)/version.txt
	@cd $(RELDIR) && FILES="$$(ls)" && sha256sum $$FILES > sha256.txt && sha512sum $$FILES > sha512.txt

docker:
	docker buildx build \
		--platform $(DOCKER_PLATFORMS) \
		--build-arg VERSION=$(VERSION) \
		--build-arg COMMIT_ID=$(COMMIT_ID) \
		--build-arg BUILD_DATE=$(BUILD_DATE) \
		--build-arg BUILD_EPOCH=$(BUILD_EPOCH) \
		-t $(REGISTRY):$(VERSION) -t $(REGISTRY):latest \
		-f docker/Dockerfile .

dev: build
	docker run --rm -it \
		--name $(PROJECTNAME)-dev-$$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8) \
		-v $(PWD):/app -w /app \
		casjaysdev/rust:latest \
		$(BINDIR)/$(PROJECTNAME)

clean:
	rm -rf $(BINDIR) $(RELDIR)
```

`BUILD_DATE` is never independently captured — it exists only where a Docker build needs the `org.opencontainers.image.created` label (`--build-arg BUILD_DATE=`), derived as `BUILD_DATE="$(date -u -d "@${BUILD_EPOCH}" +%Y-%m-%dT%H:%M:%SZ)"`. The app itself never receives `BUILD_DATE`; it derives the date from the embedded epoch via `build_date()` below.

**Example `build.rs` pattern** — the build environment exports `VERSION`, `COMMIT_ID` and `BUILD_EPOCH` (CI: per the version rule; local: the Makefile); `build.rs` maps them (plus `site.txt`) to the `APP_*` variables `option_env!()` reads:

```rust
use std::{env, fs, path::Path};

fn main() {
    // Re-run this build script when site.txt changes, otherwise bumping it
    // would not invalidate the embedded constant.
    println!("cargo:rerun-if-changed=site.txt");

    if Path::new("site.txt").exists() {
        let site = fs::read_to_string("site.txt").unwrap();
        println!("cargo:rustc-env=APP_OFFICIAL_SITE={}", site.trim());
    }

    // Map build-environment variables to the APP_* names option_env!() reads.
    // BUILD_DATE is deliberately NOT mapped - the app derives it from BUILD_EPOCH.
    for (src, dst) in [("VERSION", "APP_VERSION"), ("COMMIT_ID", "APP_COMMIT_ID"), ("BUILD_EPOCH", "APP_BUILD_EPOCH")] {
        println!("cargo:rerun-if-env-changed={}", src);
        if let Ok(val) = env::var(src) {
            println!("cargo:rustc-env={}={}", dst, val.trim());
        }
    }
}
```

**Example runtime constants** (`build_date()` uses `chrono` — an approved pure-Rust date/time crate, PART 5 → "Pure-Rust Library Stack"; add `chrono = "0.4"` to `[dependencies]`):
```rust
pub const VERSION: &str = option_env!("APP_VERSION").unwrap_or("devel");
pub const OFFICIAL_SITE: &str = option_env!("APP_OFFICIAL_SITE").unwrap_or("");
pub const COMMIT_ID: &str = option_env!("APP_COMMIT_ID").unwrap_or("N/A");
pub const BUILD_EPOCH: &str = option_env!("APP_BUILD_EPOCH").unwrap_or("0");

// Build date derived from BUILD_EPOCH (RFC 3339 UTC); "N/A" when unset
pub fn build_date() -> String {
    match BUILD_EPOCH.parse::<i64>() {
        Ok(n) if n > 0 => chrono::DateTime::from_timestamp(n, 0)
            .map(|t| t.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .unwrap_or_else(|| "N/A".into()),
        _ => "N/A".into(),
    }
}
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
| Theme preference | `theme` config key (default `dark`); OS theme APIs only when it is `auto` | assume dark/light universally |
| Terminal capability | TTY + TERM + negotiated features | assume ANSI/alt-screen support |
| Display stack | X11 (`DISPLAY`) **and** Wayland (`WAYLAND_DISPLAY`) detection — both backends supported, runtime-selected | assume GUI exists; ship X11-only or Wayland-only |
| CPU / memory | runtime detection if needed | tune only for dev hardware |

## Configuration Rules

- Config files store user choices, not auto-detected machine facts
- Defaults must be sensible for a fresh per-user install
- Support config file + environment variable + CLI override layering
- CLI override wins over env; env wins over config; config wins over defaults
- Secrets must not be stored in world-readable files

## Logging & Log Rotation

Applications write `app.log` and `error.log` to the platform log directory. Rotation is built in — no external logrotate needed. Same `rotate`/`keep` schema as the SERVER specification.

### Rotation Options

| Option | Description |
|--------|-------------|
| `never` | Never rotate |
| `daily` | Rotate daily |
| `weekly` | Rotate weekly |
| `monthly` | Rotate monthly |
| `yearly` | Rotate yearly |
| `NMB` | Rotate at N megabytes (e.g., `50MB`) |
| `NGB` | Rotate at N gigabytes (e.g., `1GB`) |
| Combined | Time + size, whichever first (e.g., `weekly,50MB`) |

### Retention Options

| Option | Description |
|--------|-------------|
| `none` | Do not keep old logs (delete after rotation) |
| `N` | Keep N old log files |
| `Nd` | Keep logs for N days |
| `Nw` | Keep logs for N weeks |
| `Nm` | Keep logs for N months |
| `forever` | Keep forever (no automatic deletion) |

### Configuration

```yaml
logging:
  # Global log level: debug, info, warn, error
  level: warn

  # All log types share these options:
  #   filename: name of log file
  #   format: output format (text, json)
  #   rotate: rotation policy
  #   keep: retention policy
  #   compress: compress rotated logs (only useful if keep > 0)

  app:
    filename: app.log
    format: text
    rotate: weekly,50MB
    keep: none
    compress: false

  error:
    filename: error.log
    format: text
    rotate: weekly,50MB
    keep: none
    compress: false
```

### Defaults

| Log Type | Rotation | Keep |
|----------|----------|------|
| app.log | weekly,50MB | none |
| error.log | weekly,50MB | none |

- `weekly,50MB` = rotate on weekly OR 50MB, whichever comes first
- `keep: none` = do not retain old logs (default) — deleted immediately after rotation

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

## Human-Readable Values (User-Facing Output)

**Every value shown on a user-facing surface — GUI windows and dialogs, TUI screens, and pretty CLI output — MUST be human-readable. Raw machine values belong to machine-readable output (`--json`, `--plain`) and log files only.**

| Kind | Rule | Examples |
|------|------|----------|
| **Durations** | Largest fitting unit, at most two units, correct singular/plural: <60 s → seconds · ≥60 s → minutes · ≥60 min → hours · ≥24 h → days | `1 second` · `45 seconds` · `3 minutes` · `2 minutes 5 seconds` · `2 hours` · `1 hour 30 minutes` · `3 days 4 hours` |
| **Sizes** | 1024 boundaries, full unit names, singular/plural, at most one decimal (drop `.0`): bytes → kilobytes → megabytes → gigabytes → terabytes | `1 byte` · `512 bytes` · `1 kilobyte` · `2.5 megabytes` · `5 gigabytes` · `1.2 terabytes` |
| **Counts** | Locale-aware thousands separators | `12,847` |
| **Timestamps** | Display format `%B %d, %Y at %H:%M:%S %Z` (zero-padded day) | `January 05, 2026 at 14:03:07 UTC` |

| Rule | Detail |
|------|--------|
| **Shared helpers** | One implementation: `format::duration()` / `format::size()` / `format::count()` in the shared/common formatting module — never per-surface ad-hoc formatting |
| **i18n** | Unit names go through translation keys (`format.seconds_one`, `format.seconds_other`, …) with per-language plural rules — never hardcoded English unit strings |
| **Raw value preserved** | GUI/TUI surfaces MAY carry the machine value in a tooltip or detail view; the visible text is always the human form |
| **Machine surfaces unchanged** | `--json`/`--plain` output and log files keep raw base units (seconds, bytes) — formatting is a presentation concern only |

## NO_COLOR Support

When `NO_COLOR` is set and non-empty, disable ANSI color output. If the TUI depends on richer formatting, fall back to CLI/plain output instead of forcing a degraded pseudo-TUI.

| Condition | GUI | TUI | CLI |
|-----------|-----|-----|-----|
| local desktop with display | allowed | fallback | fallback |
| SSH with TTY | no auto-GUI | allowed if terminal capable | fallback |
| `TERM=dumb` | no auto-GUI | avoid | use CLI |
| `NO_COLOR=1` | GUI unaffected unless project says otherwise | avoid color-dependent TUI | prefer CLI/plain |
| stdout piped | avoid GUI | avoid TUI | use CLI |

### Color Enablement Precedence

```rust
/// Return true if color output should be used
pub fn color_enabled(force_color: Option<bool>) -> bool {
    // 1. CLI flag overrides everything
    if let Some(forced) = force_color {
        return forced;
    }
    // 2. Config file (if applicable)
    if let Some(cfg) = get_config() {
        if cfg.output.color_set {
            return cfg.output.color;
        }
    }
    // 3. NO_COLOR env var (non-empty = disable)
    if std::env::var("NO_COLOR").map(|v| !v.is_empty()).unwrap_or(false) {
        return false;
    }
    // 4. Auto-detect: TTY + TERM support
    if !atty::is(atty::Stream::Stdout) {
        return false;
    }
    if std::env::var("TERM").map(|v| v == "dumb").unwrap_or(false) {
        return false;
    }
    true
}
```

CLI and TUI output MUST gate on `color_enabled(None)` — never a separate
ad hoc `NO_COLOR` check. `ratatui`/`crossterm` do NOT auto-detect
`NO_COLOR` — check `color_enabled(None)` before constructing styles and
fall back to `Style::default()` (no `fg`/`bg`) when it returns `false`.
Raw ANSI escapes in the CLI path are likewise not NO_COLOR-aware by
themselves and must check `color_enabled()` explicitly.

## Theme System (GUI / TUI / CLI)

**Default: dark. Always.** Every GUI and TUI supports three modes:

| Mode | Behaviour |
|------|-----------|
| `dark` | Dark background, light text — the default |
| `light` | Light background, dark text |
| `auto` | Follows the OS preference |

Rules:

- The mode is the config key `theme` (`dark` default, `light`, `auto`) in the user config file. The GUI/TUI settings screen has one theme control that cycles dark, light, auto and writes the key. There is no `--theme` flag: the standard flag set (see "Standard CLI Flags") is unchanged, and `--color` controls color output only (`auto`/`yes`/`no`), never light versus dark.
- The user's choice always wins over OS detection and persists across restarts.
- `auto` resolves through OS theme detection (see "Theme detection") and re-resolves on live OS changes without a restart where the OS API supports it. Where no preference can be read (most terminals, BSD without a GUI), `auto` resolves to dark.
- The mode is resolved once into a single `Theme` value that is passed down the UI tree. Widgets never read the OS theme and never hold color values themselves.
- `NO_COLOR`, `TERM=dumb`, `--color no` or a non-TTY stream strip all color: output is plain text, the theme has no visual effect, and the `theme` setting is left untouched.
- Colors come only from the design tokens (`~/.claude/memory/ui_ux_conventions.md` → Design Token System) and only in the pre-checked token pairings (WCAG 2.1 AA, 4.5:1 minimum). The OS or toolkit high-contrast mode is honored where it exposes one.

### Theme struct (GUI and true-color TUI)

```rust
// Theme is built once from the design tokens for the resolved mode and passed through the UI tree.
// It is the only place hex values live; widgets read fields, never literals.
pub struct Theme {
    pub bg: Color, pub bg_subtle: Color, pub bg_elevated: Color,
    pub bg_overlay: Color, pub bg_inset: Color,
    pub fg: Color, pub fg_muted: Color, pub fg_subtle: Color,
    pub fg_disabled: Color, pub fg_on_accent: Color, pub fg_link: Color,
    pub border_subtle: Color, pub border: Color, pub border_strong: Color,
    pub accent: Color, pub accent_hover: Color, pub accent_pressed: Color,
    pub accent_subtle: Color, pub accent_fg: Color,
    pub success: StatusColors, pub warning: StatusColors,
    pub error: StatusColors, pub info: StatusColors,
}

// StatusColors carries the three token roles every status color needs.
pub struct StatusColors { pub fg: Color, pub bg: Color, pub border: Color }
```

How each surface uses it:

- **GUI:** the toolkit theme is built from the resolved `Theme`, never from the toolkit's own OS-driven default; the accent and the light/dark mode come from the resolved mode.
- **TUI:** true-color terminals (`COLORTERM` = `truecolor` or `24bit`) use the `Theme`; any other terminal uses the 16-color ANSI mapping below. Plain CLI output uses the ANSI mapping only.

## Color Palette (TUI/CLI)

**This is a single native binary — there is no Web CSS palette. Hex values exist only in the `Theme` built from the design tokens; 16-color terminals and CLI output map the same roles to ANSI indices instead:**

| Role | Dark ANSI | Light ANSI |
|------|-----------|------------|
| `foreground` | `BrightWhite` | `Black` |
| `muted` | `White` | `DarkGray` |
| `subtle` / `disabled` | `BrightBlack` | `BrightBlack` |
| `on_accent` | `Black` | `White` |
| `primary` / `accent` | `BrightMagenta` | `Blue` |
| `secondary` / `success` | `BrightGreen` | `Green` |
| `warning` | `BrightYellow` | `Yellow` |
| `error` | `BrightRed` | `Red` |
| `info` | `BrightBlue` | `Blue` |

```rust
// TerminalPalette holds ANSI 16-color indices (0-15) for TUI/CLI.
// ratatui::style::Color::Indexed() and the ESC[38;5;{n}m escape both
// accept these indices directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalPalette {
    pub foreground: String,
    pub muted: String,
    pub subtle: String,
    pub disabled: String,
    pub on_accent: String,
    pub primary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
    pub border: String,
}

pub fn terminal_palette_dark() -> TerminalPalette {
    TerminalPalette {
        foreground: "15".into(), muted: "7".into(), subtle: "8".into(),
        disabled: "8".into(), on_accent: "0".into(), primary: "13".into(),
        success: "10".into(), warning: "11".into(), error: "9".into(),
        info: "12".into(), border: "13".into(),
    }
}

pub fn terminal_palette_light() -> TerminalPalette {
    TerminalPalette {
        foreground: "0".into(), muted: "8".into(), subtle: "8".into(),
        disabled: "8".into(), on_accent: "15".into(), primary: "4".into(),
        success: "2".into(), warning: "3".into(), error: "1".into(),
        info: "4".into(), border: "4".into(),
    }
}
```

---

## Reuse Before Creating

**Before writing new code for anything — a function, a variable/constant, or a UI component/style — check whether an equivalent already exists in the project and reuse or extend it. Only create something new when nothing existing covers the need.** This is a general project-wide rule; it governs every artifact type and is not restricted to any one feature.

### Functions

Before writing a new function, search for an existing one with the same or similar behavior — in the same module, in `helpers.rs`, in existing handlers/validators/detection logic (e.g. the UI-mode detection and theme-detection functions above) — and call or extend it instead of re-implementing the logic. Two near-identical functions that differ only in a hardcoded value are a sign the existing function should take that value as a parameter instead of being copy-pasted.

### Variables & Constants

Before adding a new constant or config key, check existing constants/enum variants (e.g. `UiMode::Tui`/`UiMode::Gui`/`UiMode::Cli`) and the config schema for one that already means the same thing. Two names for the same underlying value is a bug waiting to happen, not two separate settings.

### Components

Before building a new TUI view, GUI widget, or CLI output helper, check for an existing one and reuse or extend it instead of building a near-duplicate with a different name.

### Styling (TUI/CLI/GUI — not CSS)

**This is a single native binary — there is no Web CSS to reuse.** Styling reuse instead means: never invent a second color palette, a second set of semantic role names, or a second theme-detection mechanism alongside the ones already defined above.

- **TUI/CLI:** always style through the existing `Theme` (true-color) or `TerminalPalette` struct (16-color/CLI) and its semantic fields (`foreground`, `muted`, `subtle`, `disabled`, `on_accent`, `primary`, `success`, `warning`, `error`, `info`, `border`) and the existing `terminal_palette_dark()`/`terminal_palette_light()` functions — never a new hardcoded ANSI index or a second palette struct. New TUI/CLI output reuses `ratatui::style::Color::Indexed()` built from the palette; a genuinely new style still derives from a `TerminalPalette` field, never a literal color value.
- **GUI:** never invent a custom color palette or a literal hex value at a widget call site: read the `Theme` passed down the UI tree (hex values live only in the `Theme` built from the design tokens, see Theme System above). A genuinely new semantic color is added to the token-built `Theme`, not inlined.

```rust
// CORRECT — new TUI element styled from the existing palette, no new
// hardcoded ANSI index or duplicate palette struct
let style = Style::default().fg(Color::Indexed(palette.warning.parse().unwrap()));
```

```rust
// WRONG — hardcodes a new ANSI index instead of reusing the existing
// TerminalPalette.warning field
let style = Style::default().fg(Color::Indexed(11));
```

This rule governs the entire spec — it is not restricted to the Color Palette section, and it is not restricted to whatever the reader might assume is the "main" UI surface (TUI vs. GUI vs. CLI). Every section describing a function, variable, or UI surface inherits this rule automatically; sections do not need to restate it.

---

# PART 8: TESTING, QUALITY, AND DEBUGGING

## Required Quality Gates

All gates execute inside the project Docker container (PART 5 → "Docker Rule"). The "Logical Command" column shows the cargo invocation; the actual command is Docker-wrapped.

| Gate | Logical Command | Wrapped Form (example) |
|------|-----------------|------------------------|
| Formatting | `cargo fmt --all --check` | `$(CARGO_DOCKER) cargo fmt --all --check` |
| Linting | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | `$(CARGO_DOCKER) cargo clippy …` |
| Tests | `cargo test --workspace --all-features` | `$(CARGO_DOCKER) cargo test …` |
| Docs | `cargo doc --workspace --no-deps` | `$(CARGO_DOCKER) cargo doc …` |
| Vulnerability scan | `cargo audit` | `$(CARGO_DOCKER) cargo audit` |
| Licenses + advisories + bans + sources | `cargo deny check licenses advisories bans sources` | `$(CARGO_DOCKER) cargo deny check …` |
| Attribution drift | `cargo about generate about.hbs` (output diffed against the GENERATED region of `LICENSE.md`) | see PART 10 → "Suggested CI Steps" |
| Coverage | `cargo tarpaulin --workspace --all-features --fail-under …` | see "Coverage Gate" below |
| GUI smoke (X11) | `cargo run -- --ui gui` against an X11 socket | see PART 5 → "X11 and Wayland Forwarding" |
| GUI smoke (Wayland) | `cargo run -- --ui gui` against a Wayland socket | see PART 5 → "X11 and Wayland Forwarding" |

## Coverage Gate

`ci.yml` MUST enforce a minimum test coverage threshold. The threshold is declared in `IDEA.md ## Business logic` (free-form prose — e.g., "minimum test coverage: 75%"). If `IDEA.md` does not specify a value, the **default is 60%**. The `test:` Makefile target already enforces a hardcoded 60% floor (PART 6 → "Build Metadata"); when `IDEA.md` declares a different threshold, update that hardcoded value to match — the Makefile check and the CI gate below must never disagree.

- Use `cargo tarpaulin` (preferred for Linux musl) or `cargo llvm-cov` (works on more targets) — both pre-installed in `casjaysdev/rust:latest`
- CI MUST fail when coverage drops below the threshold; a passing build with uncovered code is a silent regression
- Coverage is computed against `cargo test --workspace --all-features` output, run inside the toolchain image

```bash
# Example (Docker-wrapped) — fails if total coverage < threshold
THRESHOLD="${COVERAGE_MIN:-60}"
docker run --rm \
  --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" \
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

**Output streams and exit codes (MUST):** data and requested output go to stdout; every error message, usage error, warning and `SKIP:` reason goes to stderr, never stdout, and every non-zero exit is preceded by one line on stderr saying what failed. Exit codes follow one standard set: `0` success (`--help`, `--version`, a graceful shutdown included) · `1` general/runtime error (includes a recovered panic and a failed check) · `2` usage error (bad flags/args, matches Go's own `flag` package and Rust's `clap`) · `65` bad input data (`EX_DATAERR`) · `66` input or resource not found (`EX_NOINPUT`) · `69` a required service or capability is unavailable (`EX_UNAVAILABLE`) · `74` I/O error (`EX_IOERR`) · `75` temporary failure (`EX_TEMPFAIL`) · `77` permission denied or authentication failed (`EX_NOPERM`) · `78` invalid configuration (`EX_CONFIG`). `126`, `127` and `128+n` belong to the shell and are never returned on purpose. Use the most specific code and document every code above `2` in `--help`; never invent another one.

**Panic messaging under `panic = "abort"` (MUST):** this template's release profile sets `panic = "abort"` (see Build Rules) — correct for a short-lived CLI/GUI/TUI binary, but it means a panic terminates the process immediately via `abort()` and CANNOT be caught/continued with `catch_unwind`. To still avoid a raw Rust backtrace reaching a normal user, install a custom panic hook at the top of `main()` that prints a clean, user-facing message before the unavoidable abort happens:

```rust
fn main() -> std::process::ExitCode {
    std::panic::set_hook(Box::new(|info| {
        if debug_mode() {
            eprintln!("panic: {info}\n\n{}", std::backtrace::Backtrace::force_capture());
        } else {
            eprintln!("error: {info}");
        }
    }));
    run()
}
```

Note the process still terminates via the platform's abort signal (exit code `134`/`SIGABRT` on Unix, not a controlled `1`) once the hook returns — the hook only controls what's printed, not the final exit code. Do not rely on this path for expected/recoverable errors; return `Result` from fallible functions and map errors to `ExitCode::from(1)` in `main()` instead of panicking for anything the program can reasonably anticipate.

**Signal handling outside daemon mode (MUST):** a plain CLI/GUI/TUI binary (i.e., not running in PART 14's RFC daemon mode) MUST still install a `Ctrl+C`/`SIGTERM` handler (via `tokio::signal::ctrl_c()` for async binaries, or the `ctrlc` crate for sync ones) around its main operation so an interrupt during a long-running command (a download, a build, an IPC session) triggers cleanup (temp files, open IPC sockets/handles, partial output) before exiting `1` — never leave the process to terminate mid-write via the platform default. Daemon mode's own richer signal contract (SIGHUP reload, graceful drain) is defined separately in PART 14 and takes precedence when that mode is active.

## Directory Naming

**Plural** — all directories use plural names (`handlers/`, `models/`, `routes/`, `utils/`). Rust module directories follow the same rule — `src/handlers/mod.rs` not `src/handler/mod.rs`. Tooling dirs are also plural (`scripts/`, `tests/`, `completions/`).

## Performance Rules

- Optimize based on measured behavior, not guesses
- Scale caches/worker counts from runtime capabilities when relevant
- Keep minimal-system defaults sensible
- Avoid unnecessary allocations and duplicate state copies in hot paths
- Never spawn unbounded tasks/threads — always cap with a semaphore, worker pool, or cancellation

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
- **Renovate is the only supported dependency-update tool** — covers Cargo deps, GitHub Actions SHAs, and Docker image digests from a single `renovate.json` at the repo root. Works on GitHub, GitLab, Gitea, Forgejo, and Jenkins.
- **Dependabot is forbidden** — GitHub-only, duplicates Renovate's work on GitHub, and cannot serve the other four providers. Never enable both.
- Public repos MUST ship `renovate.json` so Cargo / Actions / Docker updates land as PRs automatically; Renovate uses `pinDigests: true` to keep all `uses:` lines pinned to immutable SHAs
- Renovate only updates the SHA; the **runtime-still-supported** verification (e.g., node24 vs deprecated runtimes) remains a manual check on every SHA bump (PART 10 → "Third-party Action Pinning")
- Security advisories are blockers until triaged
- Run `cargo audit` as part of CI to catch known vulnerabilities

## Telemetry Rule

No hidden telemetry. Any analytics, crash reporting, or update pings must be documented and controllable.

---

# PART 10: CI/CD, RELEASES, AND AUTOMATION

## CI/CD Rules

| Rule | Description |
|------|-------------|
| Explicit commands | CI must run explicit `cargo` / `make` commands, not hide core release logic behind undocumented wrappers |
| Least privilege | CI tokens/permissions default to read-only |
| Pinned actions | Third-party actions pinned to full commit SHAs; verify runtime and maintenance status on every SHA update |
| No unsafe fork secrets | Fork PRs do not receive secrets or publish permissions |
| Secret scanning | truffleHog on push, PR and weekly: `verified` and `unknown` block range scans, `verified` blocks full-history scans; exclusions only in `.github/trufflehog-exclude.txt`; doc examples use `{user}:{password}` placeholders (see Secret Scan Job) |
| Version precedence | CI release = pushed tag with `v` stripped; CI never reads `release.txt`; local Makefile builds = `VERSION` env (user override, AI never sets it) > non-empty `release.txt` > devel rule (short commit id, `devel` outside git) |
| **Generated text is fully expanded** | Release notes, release names and descriptions, asset names and links, labels and any other text a workflow generates MUST contain real values, never a literal `${VAR}`, `${{ … }}` or a spec placeholder such as `{project_name}`. Shell: build a file with an unquoted heredoc or `printf` (a quoted heredoc is only for text that must stay literal). GitHub/Gitea/Forgejo YAML values use `${{ env.X }}`; shell `${X}` is not expanded there. GitLab keywords outside `script:` (`release:` name, description and assets, `artifacts:paths`) expand only `$VAR` / `${VAR}`, never `${VAR:0:7}` or `${VAR#v}`: compute the value in an upstream job and pass it with a dotenv report. Jenkins: Groovy `"…"` strings interpolate, `'…'` strings do not. Any workflow that builds its own notes or description file runs the guard step below before publishing |
| Site precedence | `site.txt` wins when present |
| Verifiable outputs | Releases publish aggregate `sha256.txt` / `sha512.txt` checksum files and an SBOM (always); provenance/attestation when the platform supports it |
| Single build time source | Every build job — on every provider (GitHub Actions, Gitea/Forgejo, GitLab CI, Jenkins) — captures `BUILD_EPOCH="$(date -u +%s)"` exactly once and exports it to the build environment so `build.rs` embeds it (PART 6). `BUILD_DATE` is never independently captured; where a Docker build needs the OCI `image.created` label, derive it (`date -u -d "@${BUILD_EPOCH}" +%Y-%m-%dT%H:%M:%SZ`) and pass both via `--build-arg BUILD_EPOCH=` / `--build-arg BUILD_DATE=` |
| No Makefile in CI | Workflow `run:` steps invoke explicit commands with all environment variables inlined — never `make {target}`. The Makefile is for local developer convenience only. CI MUST NOT depend on Makefile targets that could drift silently. |
| Portability | No hardcoded org, project name, official site, or registry value anywhere in workflows. Use `${{ github.repository_owner }}` / `${{ github.event.repository.name }}` (and provider equivalents). Workflows must keep working after a fork without editing values. |
| Renovate only | `renovate.json` at repo root is the only supported dependency-update tool — it covers GitHub Actions SHAs, Docker image digests, Cargo deps, and works across all five providers from a single config. Dependabot is **forbidden** (GitHub-only; duplicates Renovate on GitHub; cannot serve the other four providers). |
| `act` pre-commit validation | Before committing any change to `.github/workflows/*.yml`, run `act --list -W {file}` on each changed file. Fix all errors before committing. The `validate-workflows.sh` PreToolUse hook enforces this automatically. |
| Concurrency groups | Every push/PR workflow declares `concurrency: { group: ${{ github.workflow }}-${{ github.ref }}, cancel-in-progress: true }`. Release workflows also use `cancel-in-progress: true` — a newer tag push supersedes the in-flight release build. |
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
    runs-on: ubuntu-26.04
    ...

  release:
    needs: build
    permissions:
      # create GitHub release + upload assets
      contents: write
      # push to ghcr.io
      packages: write
      # OIDC token for cosign signing
      id-token: write
      # GitHub artifact attestations (SBOM, provenance)
      attestations: write
    ...
```

Third-party registry publishing uses repository secrets, not GitHub token permissions (e.g. `CARGO_REGISTRY_TOKEN` for crates.io).

## Third-party Action Pinning

Every external action (`uses: owner/action@...`) MUST be pinned to a full commit SHA — never a mutable tag or branch:

```yaml
# Wrong — tag can silently change or be deleted
- uses: actions/checkout@v4

# Correct — SHA is immutable
- uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
```

**When updating a pinned SHA**, verify three things:

1. **Action is still maintained** — check the upstream repo is not archived, deprecated, or abandoned
2. **Runtime is still supported** — open the action's `action.yml` at the new SHA and check `runs.using`; if it names a runtime that GitHub has deprecated or scheduled for removal, the action will silently fail after that date. Example: `node20` is removed from GitHub-hosted runners on **2026-09-23** — any action still on `node20` must be updated to a SHA where it has migrated to `node24` — all common `actions/*` and `docker/*` actions have already done so
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

**Container-job user rule:** every `container:` job MUST set `options: "--user 0:0"`. The runner (and actions/checkout's post-job cleanup) execs into the job container — e.g. `cat /etc/*release` for OS diagnostics — as a user the image's `/etc/passwd` may not define, which fails or flakes the job after all real work already passed, wasting the entire run. Numeric `0:0` needs no `/etc/passwd` lookup at all, so it is immune regardless of the image's user table.

**GitHub Actions job ordering (`needs:`):**

- `ci.yml`: `lint` and `test` run in parallel → `build` needs: lint and test; coverage collection and the 60% enforcement are steps of the `test` job, never a separate job; security jobs (`secret-scan`, `workflow-policy`, `vuln-scan`, `image-scan`) run in parallel with each other
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
    # weekly Monday 06:00 UTC
    - cron: '0 6 * * 1'
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}-${{ github.event_name }}
  cancel-in-progress: true

jobs:
  secret-scan:
    runs-on: ubuntu-26.04
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
        with:
          fetch-depth: 0
      - name: Pick the scan scope
        id: scope
        run: |
          ZERO=0000000000000000000000000000000000000000
          if [ "${{ github.event_name }}" = "pull_request" ]; then
            echo "base=${{ github.event.pull_request.base.sha }}" >> "$GITHUB_OUTPUT"
            echo "head=${{ github.event.pull_request.head.sha }}" >> "$GITHUB_OUTPUT"
            echo "results=verified,unknown" >> "$GITHUB_OUTPUT"
          elif [ "${{ github.event_name }}" = "push" ] && [ "${{ github.event.before }}" != "$ZERO" ]; then
            echo "base=${{ github.event.before }}" >> "$GITHUB_OUTPUT"
            echo "head=${{ github.event.after }}" >> "$GITHUB_OUTPUT"
            echo "results=verified,unknown" >> "$GITHUB_OUTPUT"
          else
            echo "results=verified" >> "$GITHUB_OUTPUT"
          fi
      - uses: trufflesecurity/trufflehog@b2b0a92070f206ab7b5a1105d82a7f2f48d92341  # v3.99.0
        with:
          base: ${{ steps.scope.outputs.base }}
          head: ${{ steps.scope.outputs.head }}
          extra_args: --results=${{ steps.scope.outputs.results }} --exclude-paths=.github/trufflehog-exclude.txt

  workflow-policy:
    runs-on: ubuntu-26.04
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
      - name: Verify all third-party actions are pinned to a 40-char SHA
        run: |
          set -eo pipefail
          bad=$(grep -RnE '^[[:space:]]*uses:' .github/ .gitea/ .forgejo/ 2>/dev/null | grep -vE '@[0-9a-f]{40}([[:space:]]|$)' || true)
          if [[ -n "$bad" ]]; then
            echo "::error::Unpinned actions found (must be 40-char SHAs):"
            echo "$bad"
            exit 1
          fi

  vuln-scan:
    runs-on: ubuntu-26.04
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
      # step-level gate: hashFiles() is not valid in a job-level if
      - name: cargo audit (inside casjaysdev/rust:latest)
        if: hashFiles('Cargo.lock') != ''
        run: |
          IMAGE="casjaysdev/rust:latest"
          docker run --rm -i \
            --name "$(basename "$GITHUB_REPOSITORY")-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" \
            -v "$PWD":/work -w /work "$IMAGE" cargo audit

  image-scan:
    runs-on: ubuntu-26.04
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
      # step-level gates: hashFiles() is not valid in a job-level if
      - uses: docker/setup-buildx-action@f87e5991a6d7451dcb8d9637bfbc97413f497069  # v4.4.1
        if: hashFiles('docker/Dockerfile') != ''
      - name: Build local image for scanning
        if: hashFiles('docker/Dockerfile') != ''
        run: |
          docker build -f docker/Dockerfile -t scan-target:ci .
      - name: Trivy image scan
        if: hashFiles('docker/Dockerfile') != ''
        uses: aquasecurity/trivy-action@ed142fd0673e97e23eac54620cfb913e5ce36c25  # v0.36.0
        with:
          image-ref: scan-target:ci
          severity: CRITICAL,HIGH
          exit-code: '1'
```

**Per-provider notes:**

- GitLab: `secret-scan` runs as a docker job using `image: trufflesecurity/trufflehog:latest` with `GIT_DEPTH: 0`; `image-scan` uses `image: aquasec/trivy:0.70.0`.
- Jenkins: `Security` stage uses `parallel {}`; truffleHog and Trivy each run via `docker.image(...).inside { ... }`.
- All providers: same gates, same severities, same exit conditions — no weaker subset on any provider.

### Secret Scan Job (truffleHog)

**`secret-scan` in `ci.yml` is a host-runner job (no `container:`, no `needs`) that runs `trufflesecurity/trufflehog`, pinned by SHA like every action, with `fetch-depth: 0` so the whole history is available. What it scans, and what blocks, depends on the event:**

| Event | Scope | Results that FAIL the job |
|-------|-------|---------------------------|
| Pull request | `base.sha` to `head.sha` | `verified` and `unknown` |
| Push to an existing branch | `event.before` to `event.after` | `verified` and `unknown` |
| New-branch push (`before` is all zeros), weekly schedule, manual run | The full history | `verified` only |

| Rule | Detail |
|------|--------|
| **Flag** | `--results=verified,unknown` or `--results=verified`, always with `--exclude-paths=.github/trufflehog-exclude.txt`. The old `--only-verified` flag is not used. `unknown` means verification was attempted and could not complete; `unverified` (checked and found inactive) is reported and does not fail the job |
| **Never `default_branch`** | `base` is never `repository.default_branch`: after a push it equals the pushed commit and truffleHog skips the scan ("BASE and HEAD commits are the same") |
| **Documentation examples** | `AI.md`, `IDEA.md`, `README.md` and `docs/` never contain a credential-shaped string. Connection strings use `{user}:{password}` placeholders; a flagged example is rewritten, not allowlisted, because a copied spec file is read-only in the projects that receive it |
| **Excluded paths** | `.github/trufflehog-exclude.txt` (the same path on every provider), passed as `--exclude-paths=.github/trufflehog-exclude.txt`. One regex per line, no comments (it is a data file). It lists ONLY files that are not production code: `^Cargo\.lock$` (dependency hashes), `(^|/)tests/fixtures/` (test fixtures) and `^AI\.md$` (the read-only copy of this specification, which carries example values; the template repository scans the original itself). Source, test code, docs, configuration and the exclude file itself are never listed. Adding a path needs a reason in the commit message. Everything else is handled by the placeholder rule or the allowlist below |
| **Test fixtures** | Fixtures live ONLY in `tests/fixtures/` (the one directory the exclude file lists). Their values are obviously fake (`EXAMPLE`, `xxxx`, `{placeholder}`) and never a real credential, because that directory is not scanned. A test that needs a credential-shaped string builds it at run time in the test code, which IS scanned |
| **Allowlist** | Only a reviewed true false positive in project code or a test fixture, with truffleHog's inline `trufflehog:ignore` comment on that line. There is no root config file for it |
| **Other providers** | GitLab and Jenkins run the same contract in Docker: `trufflehog git file://. --since-commit HEAD~1 --results=verified,unknown --exclude-paths=.github/trufflehog-exclude.txt --fail` for a range, `trufflehog git file://. --results=verified --exclude-paths=.github/trufflehog-exclude.txt --fail` for the full history |

## Post-Push CI Verification

`act --list` and a local `cargo test` pass only prove the workflow's syntax/job graph is valid and the code works in the local environment — they are not the real CI build. Every push to `main` triggers a real CI run on the provider's infrastructure with real secrets, real matrix jobs, and the real `casjaysdev/rust:latest` toolchain image; any of those can fail even when every local check passed. Treating "local checks passed" as equivalent to "the build is green" is itself a bug.

After every push, check the triggered run's status:
- **GitHub**: `gh run list --branch {branch} --commit {sha} --limit 1` then `gh run watch {run-id}` (or `gh run view {run-id} --json status,conclusion`)
- **GitLab**: `curl -qsSf -H "PRIVATE-TOKEN: $GITLAB_TOKEN" ".../repository/commits/{sha}/statuses"`
- **Gitea / Forgejo**: `curl -qsSf -H "Authorization: token $TOKEN" ".../commits/{sha}/status"`
- **Jenkins**: poll the job's `lastBuild/api/json` for `result`

Build failed → this is a bug, not a note for later; diagnose the root cause and fix it with a follow-up commit — never leave the default branch red. Build pending/running → the task is not done yet; wait and re-check. No CI config in the project → this step is a no-op.

## Suggested CI Steps

CI runs every Rust step inside `casjaysdev/rust:latest`. CI MUST NOT install a Rust toolchain on the runner and call `cargo` directly — and MUST NOT run quality-gate commands inside the runtime image (`docker/Dockerfile`), which contains only the final binary. Use `casjaysdev/rust:latest` for all build, test, lint, and doc steps.

**Single time source (all providers — GitHub Actions, Gitea/Forgejo, GitLab CI, Jenkins):** every CI job that builds binaries or images captures `BUILD_EPOCH="$(date -u +%s)"` exactly once (GitHub / Gitea / Forgejo: `echo "BUILD_EPOCH=${BUILD_EPOCH}" >> "$GITHUB_ENV"`; GitLab CI and Jenkins: a shell/environment variable set at the top of the stage). Every `cargo build` step exports `BUILD_EPOCH` in its environment so `build.rs` embeds it, every image build passes `--build-arg BUILD_EPOCH="${BUILD_EPOCH}"`, and `BUILD_DATE` is only ever derived from it — `BUILD_DATE="$(date -u -d "@${BUILD_EPOCH}" +%Y-%m-%dT%H:%M:%SZ)"` — where an OCI `org.opencontainers.image.created` annotation needs it. `BUILD_DATE` is never independently captured and never consumed by the cargo build.

### Required Concurrency and Retention Headers

Every push/PR workflow (`ci.yml`) MUST declare:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

Release workflows (`release.yml`) MUST use `cancel-in-progress: true` — a newer tag push supersedes the in-flight release build, so the superseded run should be cancelled rather than left to finish.

Every `actions/upload-artifact` step MUST set a finite `retention-days`:

```yaml
- uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a  # v7.0.1
  with:
    name: {project_name}-${{ matrix.target }}
    path: binaries/
    # release-job artifacts may use up to 30; build-job CI artifacts use 7
    retention-days: 7
```

```bash
# Prepare output directory for release artifacts (binaries, checksums, SBOM)
mkdir -p binaries

# Captured ONCE per job - every other time value derives from this (PART 6)
BUILD_EPOCH="$(date -u +%s)"

# Run gates inside the image (casjaysdev/rust:latest — all tools pre-installed)
docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" cargo fmt --check
docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" cargo clippy -- -D warnings
docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" cargo test --workspace --all-features
docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" cargo doc --workspace --no-deps

# License + advisory enforcement (PART 11 → "License Compliance")
docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
  cargo deny check licenses advisories bans sources

# Attribution-drift check: regenerate the GENERATED region and compare to committed.
# The `>` redirection runs on the host shell, capturing the container's stdout
# into a host-side file (the cwd is bind-mounted at /work, so this is intentional).
docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
  cargo about generate about.hbs > LICENSE.generated.md
sed -n '/<!-- GENERATED:/,$p' LICENSE.md > LICENSE.committed-generated.md
diff LICENSE.committed-generated.md LICENSE.generated.md

# VERSION comes from the CI version rule (release = pushed tag with leading v stripped); never from release.txt
# Build statically linked release binaries (one per supported target).
# `cargo --target` keeps the full Rust target triple. The published artifact
# is renamed to {project_name}-{platform}-{arch}{.ext} with -musl / vendor / ABI
# tokens stripped (see PART 2 → "Binary Model").
for TARGET in x86_64-unknown-linux-musl aarch64-unknown-linux-musl \
              x86_64-pc-windows-gnu aarch64-pc-windows-gnullvm \
              x86_64-apple-darwin aarch64-apple-darwin; do
  docker run --rm \
    --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" \
    -e VERSION="$VERSION" \
    -e BUILD_EPOCH="$BUILD_EPOCH" \
    -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
    cargo build --release --target "$TARGET"

  # Map triple → artifact name: x86_64-unknown-linux-musl → linux-amd64
  case "$TARGET" in
    x86_64-unknown-linux-musl)     ARTIFACT="{project_name}-linux-amd64" ;;
    aarch64-unknown-linux-musl)    ARTIFACT="{project_name}-linux-arm64" ;;
    x86_64-pc-windows-gnu)         ARTIFACT="{project_name}-windows-amd64.exe" ;;
    aarch64-pc-windows-gnullvm)    ARTIFACT="{project_name}-windows-arm64.exe" ;;
    x86_64-apple-darwin)           ARTIFACT="{project_name}-darwin-amd64" ;;
    aarch64-apple-darwin)          ARTIFACT="{project_name}-darwin-arm64" ;;
  esac

  # Windows binaries carry the .exe extension in the build output; others do not.
  case "$TARGET" in
    *-pc-windows-*) BIN="{project_name}.exe" ;;
    *)              BIN="{project_name}" ;;
  esac
  cp "target/$TARGET/release/$BIN" "binaries/$ARTIFACT"

  # Verify static/expected linkage for this target — fails the build if
  # unexpected dynamic deps appear. Use the appropriate inspector per target family:
  #   Linux musl     → ldd  (expect "not a dynamic executable" / "statically linked")
  #   Apple darwin   → otool -L (expect only Apple-provided frameworks)
  #   Windows GNU    → dumpbin /dependents (expect no MinGW runtime DLLs; image has no MSVC toolchain)
  case "$TARGET" in
    *-unknown-linux-musl)
      docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
        sh -c "ldd target/$TARGET/release/{project_name} 2>&1 | grep -qE 'not a dynamic executable|statically linked'"
      ;;
    *-apple-darwin)
      docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
        sh -c "otool -L target/$TARGET/release/{project_name}"
      ;;
    *-pc-windows-*)
      docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
        sh -c "dumpbin /dependents target/$TARGET/release/{project_name}.exe"
      ;;
  esac
done

# Generate the SBOM (CycloneDX) — published alongside the release artifacts.
# `cargo cyclonedx --format json` writes `bom.json` next to Cargo.toml; rename
# into binaries/ for inclusion in the release.
docker run --rm --name "${PROJECT_NAME}-$(head -c6 /dev/urandom | od -An -tx1 | tr -d ' \n' | cut -c1-8)" -v "$PWD":/work -w /work "$PROJECT_IMAGE" \
  cargo cyclonedx --format json
cp bom.json "binaries/{project_name}-bom.json"

# Aggregate checksums over every release asset, generated last so the SBOM is
# included. The file list is captured first so the checksum files never hash
# themselves. Both files are uploaded as release assets.
WORKDIR="$PWD"
cd "$WORKDIR/binaries"
FILES="$(ls)"
sha256sum $FILES > sha256.txt
sha512sum $FILES > sha512.txt
cd "$WORKDIR"
```

For GUI smoke tests in CI, use a virtual X server (e.g., `Xvfb`) and a headless Wayland compositor (e.g., `cage`, `weston --backend=headless`) **inside** the container or as a sidecar service — both backends MUST be exercised, not just one.

## Workflow Error Messaging

Use `::error::` workflow commands for validation failures — they appear as red annotations on the Actions summary page, not just buried in logs:

```bash
echo "::error::Tag 'foo' does not exist in this repository"
echo "::error file=.github/workflows/release.yml,line=12::message tied to a source location"
```

Always write messages so a developer reading only the step name + message understands what failed and what to do next.

**Generated-text guard.** A workflow that builds its own notes or description file checks it before publishing, so an unexpanded variable can never reach a release:

```yaml
- name: Guard generated text
  run: |
    if grep -nE '\$\{|\{\{|\{(project_name|project_org|version)\}' RELEASE_NOTES.md; then
      echo "::error::RELEASE_NOTES.md still contains an unexpanded variable or placeholder. Build it with an unquoted heredoc or printf so shell variables expand."
      exit 1
    fi
```

## Release Pre-flight Validation

The GitHub Releases API returns HTTP 422 `"tag_name is not a valid tag"` when the tag does not exist at API call time or is malformed. The correct fix is for the **release job to own the tag** — delete it if it exists, then recreate it at the current HEAD. This ensures the tag always exists and points to the right commit, and makes the workflow idempotent.

The `release` job already has `contents: write` to push assets — this covers tag push as well.

```yaml
- uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
  with:
    # required: full history needed to inspect and push tags
    fetch-depth: 0

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
- aggregate `sha256.txt` and `sha512.txt` checksum files covering every asset
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
[![Docs](https://readthedocs.org/projects/{rtd_project}/badge/?version=latest)](https://{rtd_url})

# GitLab
[![Release](https://gitlab.com/{project_org}/{project_name}/-/badges/release.svg)](https://gitlab.com/{project_org}/{project_name}/-/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Docs](https://readthedocs.org/projects/{rtd_project}/badge/?version=latest)](https://{rtd_url})

# Gitea/Forgejo (use shields.io with custom endpoint or static badge)
[![Release](https://img.shields.io/badge/dynamic/json?url=https://git.example.com/api/v1/repos/{project_org}/{project_name}/releases/latest&query=$.tag_name&label=release)](https://git.example.com/{project_org}/{project_name}/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Docs](https://readthedocs.org/projects/{rtd_project}/badge/?version=latest)](https://{rtd_url})

# {rtd_project} and {rtd_url} - Use one of:
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

## Embedded Datasets and Content (Provenance Before Embedding)

**Any third-party dataset or content a project embeds or ships (word lists, quote or joke collections, place and postal data, images, fonts, icons, translations, GeoIP-style databases) needs documented provenance BEFORE it is embedded.**

| Required | Rule |
|----------|------|
| **Source** | The exact origin: URL or publisher, version, and retrieval date |
| **Terms** | The license or terms of use. They MUST allow redistribution inside a project released under the project's license, checked against the project license and its dependencies' licenses |
| **Attribution** | The notice the terms require (author, license, link, changes made) |
| **Record** | In the project's third-party notices (`LICENSE.md`, "Third-Party Licenses") and on the attribution surface (`/server/about`, or the README when the project has no web page) |
| **Per-record provenance** | When records come from several sources, every record or file carries a `source` key that names its entry in the third-party notices |
| **Unknown or unusable terms** | Content with no stated license, a license that forbids redistribution, a site that forbids scraping, or an unknown origin is NOT embedded. The project asks the user instead of guessing |
| **Generated content** | It counts as the project's own only when the user says so, and is never presented with an invented source |

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

1. `IDEA.md` adds (or extends) the `**License exceptions:**` block under `## Business logic`, naming the crate, the upstream license, and the rationale (IDEA.md keeps exactly three top-level sections — this is a block, never a new `##` section)
2. The exception explicitly accepts the consequence — e.g., "this binary is distributed under GPL-3.0; the project's MIT claim applies only to the source we author, not the published binary" — and `IDEA.md ## Project variables` adds (or updates) the variable `distribution_license` accordingly. `distribution_license` is an exception-only variable: it is not part of the required-keys list (see "IDEA.md Required Layout" → Project variables rules) and is present only in projects that have taken a license exception
3. README, `LICENSE.md`, and any user-visible "About" surface are updated to reflect the actual distribution license, not just the source license
4. `deny.toml` is updated with a scoped allow entry for the specific crate + version, not a blanket category unblock

There is no other path to use a denylisted crate. Adding it to `deny.toml` without going through these steps is a spec violation.

### Required `Cargo.toml` Metadata

Every workspace member's `Cargo.toml` MUST include:

```toml
[package]
# current stable Rust edition declared in Cargo.toml (PART 5 → "Toolchain Rules")
edition      = "2024"
# MSRV pin matching rust-toolchain.toml (PART 5 → "Toolchain Image")
rust-version = "1.XX"
# or the SPDX expression for the IDEA.md-declared license
license      = "MIT"
authors      = ["…"]
repository   = "…"
# follows the official-site precedence chain in PART 6 → "Metadata Priority Rules / Official Site": site.txt > IDEA.md `official_site` > documented env override > empty
homepage     = "…"
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

All three surfaces read the same `LICENSE.md` blob embedded at compile time via `include_str!` (PART 0 → "Self-Contained Assets"). The version, commit ID, and build date (derived from `BUILD_EPOCH` via `build_date()`, PART 6) are shown alongside.

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
- [ ] If a pre-existing `CLAUDE.md` or `.claude/CLAUDE.md` existed, project-specific content was migrated into IDEA.md
- [ ] `CLAUDE.md` / `.claude/CLAUDE.md` are short loaders, not duplicate specs
- [ ] `AGENTS.md` exists at project root as an exact copy of `CLAUDE.md` (real file, not a symlink)
- [ ] `.claude/memory/` directory exists (with an empty `MEMORY.md` index if no entries yet) and is committed, not gitignored
- [ ] `release.txt` is read only by the Makefile for local builds
- [ ] `site.txt` exists only if there is a real official site URL
- [ ] `docker/Dockerfile` (always, when project ships a container), `docker/Dockerfile.dev` (project-specific — when a debug-mode image is shipped), `docker/docker-compose.yml`, `docker/docker-compose.dev.yml`, `docker/docker-compose.test.yml`, and `docker/rootfs/usr/local/bin/entrypoint.sh` exist as needed; `Dockerfile` is the runtime image
- [ ] `.dockerignore` exists at project root with Rust-specific entries (`target/`, plus the standard exclusions from `dockerfile_conventions.md` → ".dockerignore")
- [ ] `docker/Dockerfile.build` does not exist — Rust projects always use `casjaysdev/rust:latest` directly; no custom toolchain image is ever needed
- [ ] If IDEA.md documents a `*-sys` exception requiring system dev libs at build time, only the minimum set needed by that crate is added to the image — by default the image carries no GUI-stack C dev libs (PART 0 → "Rust-Only Application")
- [ ] X11 forwarding sample command is documented and works against a real Xorg/XWayland session
- [ ] Wayland forwarding sample command is documented and works against a real Wayland compositor
- [ ] `rust-toolchain.toml` pins the toolchain; `.cargo/config.toml` pins static-link rustflags
- [ ] `Cargo.toml` release profile uses `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`
- [ ] Repo has `assets/` (build-time source) and a Rust embedding module (`include_bytes!` / `rust-embed`) wiring it into the binary
- [ ] No source files in any language other than Rust contribute to the binary (small Docker shell helpers excepted)
- [ ] `Makefile` exists with at minimum `build`, `test`, `dev`, `clean` targets
- [ ] `deny.toml` exists at project root with the spec's default allowlist / denylist (PART 11 → "License Compliance")
- [ ] `about.toml` and `about.hbs` exist at project root and `cargo-about` produces the third-party attribution section
- [ ] `cargo-deny`, `cargo-about`, and `cargo-cyclonedx` are pre-installed in the Docker image
- [ ] `Cargo.toml` for every workspace member sets `[package].license`, `authors`, `repository`, `description`
- [ ] `LICENSE.md` has the hand-written region (project license, upstream NOTICE files, embedded asset notices) AND the generated region (cargo-about output) clearly delimited

## Implementation Checklist

- [ ] `ci.yml` and `release.yml` use `container: image: casjaysdev/rust:latest` — no `ensure-build-image` pre-flight, no `build-toolchain.yml`
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
- [ ] No GPL / AGPL / LGPL / SSPL / BUSL / source-available dep was added without an IDEA.md `**License exceptions:**` entry and an updated `distribution_license` (PART 11 → "License Compliance")
- [ ] User-visible licenses surface exists: CLI `--licenses`, TUI "Licenses" entry, GUI "About → Open Source Licenses" — all reading the same embedded `LICENSE.md`
- [ ] App runs end-to-end from the binary alone on an air-gapped machine — no first-run downloads
- [ ] No plugin or runtime extension loading from disk unless IDEA.md defines a hardened plugin contract
- [ ] `release.txt` / `site.txt` precedence is preserved
- [ ] Docs and examples use Cargo/Rust terminology — wrapped in Docker invocations
- [ ] No build/test/run instructions tell the user to invoke cargo on the host
- [ ] If IDEA.md declares an RFC protocol daemon: the PART 14 daemon checklist passes
- [ ] If IDEA.md declares a local client/server (IPC) split: sockets live in `$XDG_RUNTIME_DIR/{project_name}/` with `0700`/`0600` modes, the main-socket registry model is implemented, and no TCP listener exists (PART 2 → "Local Client/Server (IPC) Mode")

## Quality Checklist

All gates run inside the project Docker image — never on the host.

**Commit-time lint gate:** running `cargo clippy` directly satisfies the quality bar
shown here, but does NOT satisfy `gitcommit`'s pre-commit lint gate —
`lint-agent-mark.sh` only records the gate as passed when the `rust-lint` agent
itself reports a clean result. Invoke the `rust-lint` agent before running
`gitcommit`; a raw `cargo clippy` invocation leaves the commit permanently blocked.

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
- [ ] `.github/trufflehog-exclude.txt` exists and lists only `AI.md`, `Cargo.lock` and the fixtures directory `tests/fixtures/` (plus any other non-production file, with a reason in its commit)
- [ ] No automatic privilege escalation
- [ ] No unsafe downloaded-code execution
- [ ] Dependency/license/security checks remain enabled

## Release Checklist

- [ ] CI version comes from the pushed tag (`v` stripped); only the Makefile reads `release.txt`
- [ ] Official site comes from `site.txt` when present
- [ ] Release notes match actual changes
- [ ] Aggregate `sha256.txt` and `sha512.txt` checksum files covering every artifact are published as release assets
- [ ] Each release artifact is a single statically linked binary — no companion `.so` / `.dylib` / `.dll` / asset bundle
- [ ] Static-linkage check (`ldd` / `otool -L` / `dumpbin /dependents`) was run and recorded for every published target
- [ ] `LICENSE.md` regenerated and committed if `Cargo.lock` changed; CI license-drift check is green
- [ ] Distributed binary's actual license matches what README and "About" claim (no silent GPL/LGPL relicensing via a transitive crate)
- [ ] Artifact filenames follow `{project_name}-{platform}-{arch}{.ext}` with `-musl`/vendor/ABI tokens stripped
- [ ] Artifacts cover the supported target matrix declared in IDEA.md (defaults: `{project_name}-linux-{amd64,arm64}`, `{project_name}-windows-{amd64,arm64}.exe`, `{project_name}-darwin-{amd64,arm64}`)
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
- a proposed GUI toolkit only supports X11 or only supports Wayland — not both
- a proposed dependency is GPL / AGPL / LGPL / SSPL / BUSL / source-available — flag the license-relicensing implication before adding (PART 11 → "License Compliance")

## Success Criteria

A compliant Rust project following this specification:
- is driven by `IDEA.md` project variables while `AI.md` stays read-only
- preserves the governance/documentation discipline of this specification
- models a single-binary, fully self-contained Rust application built around GUI / TUI / CLI surfaces (plus the conditional RFC protocol daemon mode of PART 14 and the conditional local client/server IPC mode of PART 2)
- ships exclusively Rust source code (small Docker shell helpers excepted)
- produces one statically linked binary per target with all assets embedded
- runs end-to-end from the binary alone on an air-gapped machine
- supports both X11 and Wayland on Linux/BSD when GUI is in scope
- builds, tests, and runs only inside the project's Docker image — never on the host
- auto-selects GUI, then TUI, then CLI using environment-aware detection
- uses optional explicit privilege escalation only when requested
- uses Rust/Cargo tooling (via `make` targets and Docker) consistently across development, testing, and CI/CD

---

# PART 13: IDEA.md REFERENCE

**NEVER modify this PART. Read-only reference for IDEA.md structure.**

The canonical layout, variable rules, immutability constraints, and migration procedure are defined at the top of this file under "IDEA.md Required Layout" (the section preceding PART 0). PART 13 here is the worked-example reference — every example below uses the same three-section structure.

---

## IDEA.md Structure

Every IDEA.md has exactly three top-level sections, in this order:

1. `## Project description` — free-form prose: what the project is, who uses it, what problem it solves
2. `## Project variables` — `key: value` lines that provide the canonical values AI.md resolves for `project_name`, `project_org`, `internal_name`, `internal_org`, etc.
3. `## Business logic` — features, data models, user flows, trust boundaries, abuse cases, platform constraints, security assumptions (WHAT, not HOW)

See "IDEA.md Required Layout" at the top of this file for the authoritative rules: variable-key naming, the stable `internal_name` / `internal_org` rule, the missing-value setup flow, and the migration procedure for legacy `CLAUDE.md` files.

---

## IDEA.md Template

```markdown
## Project description

{Brief description of what this project does, its primary users, and what problem it
solves. Free-form prose, 1–3 paragraphs.}

## Project variables

project_name:     {project_name}
project_org:      {project_org}
# STABLE — equals project_name on first install; change only on an explicit rename or fork reset
internal_name:    {project_name}
# STABLE — equals project_org on first install; change only on an explicit rename or fork reset
internal_org:     {project_org}
app_name:         {App Display Name}
crate_name:       {project_name}
official_site:    {full URL with scheme as the user gave it — or empty (the default; never invented)}
maintainer_name:  {Maintainer Name — defaults to {project_org} if unset}
maintainer_email: {maintainer@example.com — or empty; used only if set}

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

**Trust boundaries & abuse cases:**
- {Untrusted input source → where it is validated / what trusts what}
- {Abuse case: what a hostile input or user could attempt → expected mitigation posture}

**Security exceptions:** {intentionally allowed security-sensitive choice + documented reason — or `none`}

**Outbound network use (if any — see PART 9 → "Security-First Design"):**
- {Remote service consumed, purpose, auth model}

**Stored data location (per-user — see PART 4 → "Path Rule"):**
- {What is stored at the per-user paths}

**License exceptions (always present — write `none` when no denylisted or vendored-C dep is used; see PART 11 → "License Compliance"):**
- {crate, upstream license, rationale, `distribution_license` consequence if applicable}
```

**Rules for the example contents above:**

- No implementation details — describe behavior, not algorithms or libraries. AI.md PARTs 0–11 define HOW; PART 12 verifies compliance.
- This specification targets GUI / TUI / CLI applications (PART 0 → "One Coherent Product").
- Cross-reference AI.md PARTs by number for any pattern that already exists there (Docker → PART 5, security → PART 9, license exceptions → PART 11, etc.).
- `internal_name` and `internal_org` are stable after first set and reset only on an explicit rename or at a fork's first setup (see "IDEA.md Required Layout" → Project variables rules).

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

**Trust boundaries & abuse cases:**
- All data is local and owned by the user; the only untrusted input is imported markdown, which is parsed for rendering and never executed
- Abuse case: a crafted markdown file targeting the parser → memory-safe parsing, no HTML/script execution, size limit on import

**Security exceptions:** none

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

**Trust boundaries & abuse cases:**
- Remote feed content (XML/HTML) is fully untrusted → hardened parsing, HTML sanitized to text, scripts and external resource loads never executed
- Feed URLs are user-supplied → fetched over HTTPS only; non-HTTP schemes refused; no credentials attached
- Abuse case: a hostile feed serving oversized or decompression-bomb responses → response size and time limits on sync

**Security exceptions:** none

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
- POSIX-style filesystem operations; Windows support uses real NTFS symlinks via `std::os::windows::fs::symlink_file` / `symlink_dir` (not junctions — a separate reparse-point mechanism), which require Developer Mode or elevated privileges to create

**Trust boundaries & abuse cases:**
- Repo contents are the user's own but treated as untrusted path input: every target path is validated to resolve inside `$HOME`
- Abuse case: a repo entry attempting path traversal (`../../etc/...`) or a symlink escape outside `$HOME` → refused; nothing outside `$HOME` is ever written

**Security exceptions:** none

**Outbound network use:** none

**Stored data location (per-user — PART 4 → "Path Rule"):**
- Config: `~/.config/casjay/dotctl/config.toml`
- State (last known link map): `~/.local/share/casjay/dotctl/state.json`

**License exceptions:** none
```

---

# PART 14: RFC PROTOCOL SERVER / DAEMON MODE (CONDITIONAL)

**This PART applies only when IDEA.md `## Business logic` declares the project an RFC protocol server (daemon). If it does not, this PART is inert and adds no requirements.**

**Local client/server (IPC) applications — tmux/nvim-style Unix-socket splits — are NOT protocol daemons and never use this PART**; they are governed by PART 2 → "Local Client/Server (IPC) Mode".

## What This Mode Is

A protocol daemon is an infrastructure server in the tradition of nginx/httpd/caddy (HTTP), proftpd/vsftpd (FTP), postfix/exim (SMTP), dovecot (IMAP/POP3), squid (HTTP proxy), or bind/unbound (DNS). It is still a single-binary application under this specification — it is NOT an API server and NOT a web frontend server:

| Type | Speaks | Governing spec |
|------|--------|----------------|
| API server | JSON/REST over HTTP for its own API | the API server specification |
| Full-stack web server | its own HTML frontend + optional REST | the web server specification |
| **Protocol daemon (this PART)** | **a standard wire protocol, exactly as its RFC defines** | this specification + this PART |

**Every listening socket speaks a standard, RFC-governed wire protocol. The daemon never exposes an invented protocol, and never mixes non-protocol traffic (dashboards, JSON APIs, health endpoints) into protocol sockets.**

## IDEA.md Declaration

IDEA.md `## Business logic` MUST declare:
- that the project is a protocol server (daemon)
- every protocol implemented, with its governing RFC numbers
- which optional protocol extensions are in scope

Example:

```markdown
### Protocols

| Protocol | RFCs | Extensions |
|----------|------|------------|
| SMTP (MTA) | RFC 5321, RFC 5322 | STARTTLS (RFC 3207), AUTH (RFC 4954), 8BITMIME (RFC 6152) |
| Submission | RFC 6409 | — |
```

## RFC Conformance Rules

| Rule | Detail |
|------|--------|
| **The RFC is the contract** | Wire behavior implements the RFCs IDEA.md declares. RFC 2119/8174 key words apply: RFC MUST requirements are hard gates; any SHOULD-level deviation requires an IDEA.md entry with reasoning |
| **Never invent wire protocols** | No proprietary commands, replies, or framing on protocol sockets |
| **Extensions via the protocol's own mechanism** | ESMTP EHLO keywords, IMAP CAPABILITY, FTP FEAT, HTTP headers/upgrade — advertised through the protocol's negotiation, declared in IDEA.md |
| **Protocol constants from the RFC** | Reply codes, status codes, and grammar come from the RFC — never invented values |
| **Interop is the ground truth** | Conformance is verified against real-world clients, not only unit tests |

## Common Protocol → RFC Map (Reference Only — IDEA.md's Declared List Governs)

| Protocol | Core RFCs | Example daemons |
|----------|-----------|-----------------|
| HTTP/1.1 | 9110, 9111, 9112 | nginx, httpd, caddy |
| HTTP/2 · HTTP/3 | 9113 · 9114 | nginx, caddy |
| HTTP proxy (CONNECT) | 9110 | squid |
| FTP | 959, 2228, 3659 | proftpd, vsftpd |
| SMTP / submission | 5321, 5322, 6409 | postfix, exim |
| IMAP4rev2 / POP3 | 9051 (3501), 1939 | dovecot |
| DNS | 1034, 1035 | bind, unbound |
| TLS / STARTTLS | 8446, 3207 | — |
| WebSocket | 6455 | — |

## Runtime Model

| Rule | Detail |
|------|--------|
| **Serve entrypoint** | `{project_name} serve` starts the daemon; IDEA.md may define aliases |
| **Foreground only** | The process runs in the foreground and never self-daemonizes (no fork/detach) — systemd or the container runtime supervises it |
| **UI mode** | `serve` forces CLI mode; the GUI/TUI auto-detect of PART 3 never applies to a service process. Optional GUI/TUI surfaces are management consoles over the same shared core |
| **Console output** | Startup banner, then console silence in normal operation — protocol traffic and periodic work log to files only; the console shows WARN/ERROR |

## Signals & Lifecycle

| Signal | Action |
|--------|--------|
| `SIGTERM` / `SIGINT` | Graceful shutdown: stop accepting, drain active sessions up to `daemon.shutdown_timeout` (default 30 seconds), then exit 0 |
| `SIGHUP` | Reload config and TLS certificates without dropping established sessions; if the new config is invalid, keep running on the old config and log ERROR |
| `SIGUSR1` | Reopen log files (for external rotation tooling; built-in rotation per PART 7 still applies) |

Signal handling uses `tokio::signal` (async runtimes) or `signal-hook` (sync daemons).

**`--config-test` (`-t`) is REQUIRED:** parse and validate the full config, print errors to stderr, exit 0 (valid) or 78 (invalid, `EX_CONFIG`) — mirrors `nginx -t`. It never touches sockets or running state.

## Sockets, Ports & Privileges

- IPv4 + IPv6 listeners (dual-stack by default; per-listener config)
- IANA standard port defaults for the protocol; every listener configurable
- **Privileged ports (<1024): bind first, then drop privileges** — target credentials are resolved before the drop, and the daemon never continues running as root after binding (`nix` crate setuid/setgid pattern)
- TLS: cert/key paths in config, reloaded on SIGHUP; implicit TLS vs STARTTLS per the protocol's RFC

## Health & Metrics Sidecar (Optional — OFF by Default)

```yaml
daemon:
  # Seconds to drain active sessions on SIGTERM
  shutdown_timeout: 30
  sidecar:
    # OFF by default — protocol sockets stay pure protocol
    enabled: false
    # Loopback only unless explicitly configured otherwise
    address: 127.0.0.1
    # REQUIRED when enabled — no implicit default port
    port: 0
```

| Rule | Detail |
|------|--------|
| **Disabled = no HTTP listener** | When `sidecar.enabled: false` (default), the daemon opens no HTTP socket at all |
| **Same contract as the server specs** | When enabled, the sidecar serves `/server/healthz` and `/server/metrics[/{service}]` with the exact same JSON shape, status semantics, and per-service bearer-token auth defined by the API/SERVER specifications |
| **Loopback bind** | `127.0.0.1` by default; binding a non-loopback address requires explicit config |
| **Never on protocol sockets** | Health/metrics never ride the protocol listeners — even when the daemon's protocol is HTTP itself, the sidecar is a separate listener |

## Protocol Logging

- Transaction logs use the protocol community's de-facto standard format where one exists (HTTP → Apache combined by default; FTP → xferlog; SMTP → per-message queue/delivery lines); otherwise structured text
- Same rotation/retention schema as PART 7 logging
- The wire is a machine surface: protocol bytes are exactly what the RFC prescribes — the human-readable formatting rules of PART 7 apply to CLI/status/TUI/GUI output only, never to wire output

## Testing

- Every implemented RFC MUST requirement has a conformance test; SHOULD-level behavior is tested when in scope
- Interop smoke tests run in `docker/docker-compose.test.yml` against real clients (`curl` for HTTP, `lftp` for FTP, `swaks` for SMTP, `openssl s_client` for TLS, `dig` for DNS)
- Malformed-input tests: negative/fuzz cases return the RFC-mandated error replies and never crash the daemon
- A failed RFC MUST requirement is a bug — never a documented quirk

## Daemon Checklist

- [ ] IDEA.md declares protocols, RFC numbers, and in-scope extensions
- [ ] All wire behavior traces to the declared RFCs; no invented commands, replies, or framing
- [ ] `serve` runs foreground in CLI mode; no self-daemonization
- [ ] SIGTERM drains gracefully within `shutdown_timeout`; SIGHUP reloads config/certs without dropping sessions; an invalid reload keeps the old config running
- [ ] `--config-test` validates the config and exits 0/1
- [ ] Privileged ports use bind-then-drop; the daemon never runs steady-state as root
- [ ] IPv4+IPv6 listeners with IANA defaults, all configurable
- [ ] Sidecar is OFF by default; when enabled it binds loopback and matches the server specs' healthz/metrics contract
- [ ] Transaction logs use the protocol's standard format; console is silent in normal operation
- [ ] Interop smoke tests pass against real clients in Docker
