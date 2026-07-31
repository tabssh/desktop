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

## Follow-up from bootstrap session

- `make build`/`make test` fail in sandboxed environments because
  `casjaysdev/rust:latest`'s shared entrypoint tries to auto-configure an
  SMTP relay before running the passed command, and that fails without
  outbound network access. Verified `cargo build`/`cargo test` work cleanly
  when bypassing the entrypoint (`--entrypoint /bin/sh -c '...'`). Confirm
  on a real machine/CI whether this also occurs there; if so, add an
  entrypoint override to `DOCKER_RUN` in the Makefile.
