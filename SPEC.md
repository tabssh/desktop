# TabSSH Desktop — Project-Specific Rule Overrides

This file overrides rules from AI.md and the global CLAUDE.md.
**SPEC.md wins over everything.** Only add entries here when a rule must actively differ from the template or global default.

## Active overrides

None. All AI.md and global CLAUDE.md rules apply as written.

## Notes

- `project_name` resolves to `desktop` (the repository name), not the binary name.
  The binary is named `tabssh` — resolved from `crate_name` in IDEA.md `## Project variables`.
  Use `crate_name` anywhere the binary/crate name is needed; use `project_name` for the repo.
- `internal_name: tabssh` is frozen — on-disk paths (config, data, cache dirs) use `tabssh`, never `desktop`.
