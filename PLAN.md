# TabSSH Desktop — Plan

> What desktop needs to do. Tactical work-list lives in `TODO.AI.md`.
> Implementation specifics are deliberately omitted — the only thing
> that has to be explained in detail is **sync**, because the wire
> format is a contract with mobile and any drift breaks both sides.
>
> **Last updated:** 2026-05-09. Mobile is at v0.0.9, DB v29, with OCI
> support shipped.

---

## 1. Mission

TabSSH is one product with two clients:

- **Mobile** (`../android`) — primary. Drives the protocol and storage
  formats.
- **Desktop** (this repo) — second client, currently boilerplate.

Desktop must:

1. Round-trip every piece of user-visible data byte-exactly with mobile
   through the shared sync format. Section 5 is the contract.
2. Surface the same connections, identities, snippets, themes, host
   keys, workspaces, groups, hypervisors so a user moving between
   phone and laptop never feels like they're using a different app.
3. Diverge only where the form factor demands it. Section 4 lists the
   desktop-only ergonomics; section 3 lists mobile-only patterns we
   deliberately don't carry over.

When in doubt, mobile leads.

---

## 2. Feature parity (what desktop has to be)

Status legend: ✅ done · 🔧 partial · 🔴 not started · 🚫 deliberately
out of scope (see §3).

| Capability | Mobile | Desktop |
|---|---|---|
| **SSH core** | | |
| Password / public-key / keyboard-interactive auth | ✅ | 🔧 |
| OpenSSH user certificates (`*-cert.pub`) | ✅ | 🔴 |
| SSH agent forwarding | ✅ | 🔴 |
| Always-on keepalive | ✅ | 🔴 |
| Proxy / Jump host | ✅ | 🔴 |
| `~/.ssh/config` import (live read on desktop) | ✅ | 🔧 |
| **Terminal** | | |
| VT100 / ANSI / 256-color / 24-bit | ✅ | 🔧 |
| Find / search in scrollback | ✅ | 🔴 |
| URL detection + click to open | ✅ | 🔴 |
| Drag-to-select range copy | ✅ | 🔴 |
| Cursor styles, font sizing | ✅ | 🔴 |
| **SFTP** | | |
| Dual-pane browser | ✅ | 🔧 |
| Multi-file upload/download with progress | ✅ | 🔴 |
| In-place remote file edit | ✅ | 🔴 |
| chmod editor | ✅ | 🔴 |
| SCP fallback | ✅ | 🔴 |
| **Tabs / sessions** | | |
| Multi-tab independent shells (one host, many shells) | ✅ | 🔴 |
| Reconnect on disconnected tab | ✅ | 🔴 |
| Reattach prompt when an existing tab is alive | ✅ | 🔴 |
| Tmux / Screen / Zellij auto-launch | ✅ | 🔴 |
| Session persistence across restarts | ✅ | 🔴 |
| **Connections / organisation** | | |
| Connection groups / folders | ✅ | 🔴 |
| Group sort + persistence | ✅ | 🔴 |
| Per-host color tag | ✅ | 🔴 |
| Identity abstraction (reusable credentials) | ✅ | 🔴 |
| Workspaces (named tab groups) | ✅ | 🔴 |
| Search / sort / multi-select bulk edit | ✅ | 🔴 |
| **Snippets / macros** | | |
| Snippets library with `{?var}` placeholders | ✅ | 🔴 |
| Recordable macros (raw byte replay) | ✅ | 🔴 |
| **Themes** | | |
| 23 built-in themes | ✅ | 🔧 (1 of 23) |
| Custom theme editor (GUI) | ✅ | 🔴 |
| WCAG contrast validation | ✅ | 🔴 |
| Per-host theme override | ✅ | 🔴 |
| **Hypervisor** | | |
| Proxmox VE | ✅ | 🔴 |
| XCP-ng | ✅ | 🔴 |
| Xen Orchestra (REST + WebSocket live updates) | ✅ | 🔴 |
| VMware ESXi / vCenter | ✅ | 🔴 |
| **Oracle Cloud Infrastructure (OCI Compute)** | ✅ | 🔴 |
| VM serial console via hypervisor API | ✅ | 🔴 |
| Snapshot / backup management | ✅ | 🔴 |
| **Sync** | | |
| `TABSSH_SYNC_V2` round-trip with mobile | ✅ | 🔴 |
| 3-way merge (base / local / remote, conflicts surfaced) | ✅ | 🔴 |
| QR pairing — receive | ✅ | 🚫 mobile-only direction |
| QR pairing — send | 🚫 | 🔴 desktop's role |
| **Storage** | | |
| SQLite schema at parity with mobile DB v29 | ✅ | 🔧 |
| OS keyring for every secret (no plaintext in DB) | ✅ | 🔴 |
| **Hardening** | | |
| Centralised error dialog with **Copy** button | ✅ | 🔴 |
| Crash reporter on next launch | ✅ | 🔴 |
| Cold-start commit-id marker in logs | ✅ | 🔴 |

Tactical tasks (which crate, which file, which method) are in
`TODO.AI.md`. This page only says **what** desktop has to be.

---

## 3. Mobile-only patterns we deliberately don't carry over

| Mobile feature | Why not on desktop |
|---|---|
| Custom on-screen keyboard (1–5 rows of CTL/ALT/arrows/symbols) | Real keyboard is always available |
| Volume keys adjust font size | Use Ctrl+`+`/Ctrl+`-`/Ctrl+Scroll |
| Swipe between tabs | Use Ctrl+Tab + mouse |
| SAF document URI permissions | Direct filesystem access |
| Foreground service + sticky notification | System tray |
| Android home screen widget | `.desktop` shortcut / Start menu pin |
| Foldable / sw720dp / book-mode layouts | Window is resizable |
| `FLAG_SECURE` screenshot blocker | Desktop OSes don't all expose an equivalent |
| Multi-touch / pinch-zoom | Mouse |
| Tasker integration | Shell scripts + the Phase-3 IPC |
| Voice typing into terminal | OS-level dictation |
| Shake-to-Tab, edge-swipe gestures | N/A |
| Bluetooth keyboard pairing UI | OS handles |

---

## 4. Desktop-only ergonomics (no mobile equivalent)

These are the desktop-side wins. They're explicitly **not** parity gaps —
they're divergences the form factor invites.

- System tray with show / hide window
- Auto-launch on user login
- CLI / headless mode
- Native installers per OS (`.deb`, `.rpm`, AppImage, `.dmg`, `.msi`)
- IPC for shell-script integration
- Multi-window per workspace
- Native drag-and-drop file upload from the OS file manager
- Live `~/.ssh/config` read (don't import a copy — reflect the user's
  existing setup)

---

## 5. Sync — the contract with mobile

This is the only thing that needs to be specified in detail in this
document, because it's a wire-format contract between two binaries.
Any drift breaks both sides.

### 5.1 Wire format (`TABSSH_SYNC_V2`)

Identical to mobile's. Mobile's sources of truth:
`../android/AI.md` §10 and `app/src/main/java/io/github/tabssh/sync/`.

| Offset | Bytes | Content |
|---|---|---|
| 0 | 32 | Header — ASCII `TABSSH_SYNC_V2` + null padding to 32 bytes |
| 32 | 32 | PBKDF2 salt |
| 64 | 12 | AES-GCM IV |
| 76 | … | Ciphertext |

Ciphertext = `AES-256-GCM(key, IV, GZIP(JSON(SyncDataPackage)))`. The
GCM auth tag (128 bits) is appended by the cipher.

### 5.2 Crypto parameters

These numbers are not negotiable. Drift = blob unreadable on the other
side.

| Parameter | Value |
|---|---|
| KDF | PBKDF2-HMAC-SHA256 |
| KDF iterations | 100 000 |
| KDF salt size | 32 bytes |
| Derived key size | 256 bits |
| Cipher | AES-256-GCM |
| IV size | 12 bytes |
| Auth tag size | 128 bits |
| Compression | GZIP, default level |
| Inner format | UTF-8 JSON |

### 5.3 `SyncDataPackage` (inner JSON)

```json
{
  "metadata": {
    "device_id": "...",
    "device_name": "...",
    "sync_version": 7,
    "app_version": "0.1.0",
    "item_counts": { "connections": 12, "stored_keys": 3, ... }
  },
  "connections": [],
  "stored_keys": [],
  "themes": [],
  "host_keys": [],
  "workspaces": [],
  "snippets": [],
  "identities": [],
  "connection_groups": [],
  "trusted_certificates": [],
  "hypervisor_profiles": [],
  "preferences": {
    "general": {},
    "security": {},
    "terminal": {},
    "ui": {},
    "connection": {},
    "sync": {}
  }
}
```

**Field shape per entity matches the SQLite columns one-to-one** —
no rename, no extra, no reordering of nested objects (the JSON parsers
on both sides read by name, but minifiers/obfuscators must not rewrite
the field names — see §5.6).

When mobile adds a column to a synced entity, desktop's struct grows
the same field with `serde`'s default-value attribute so older blobs
still parse. Same in reverse.

### 5.4 Sync coverage matrix

Both clients must agree on which entities are synced and how
conflicts are resolved.

| Entity | Synced | Strategy |
|---|---|---|
| `connections` | ✅ | 3-way merge |
| `stored_keys` | ✅ | 3-way merge |
| `themes` | ✅ | 3-way merge |
| `host_keys` | ✅ | 3-way merge |
| `preferences` (per category) | ✅ | last-write-wins |
| `workspaces` | ✅ | last-write-wins |
| `snippets` | ✅ | last-write-wins |
| `identities` | ✅ | last-write-wins |
| `connection_groups` | ✅ | last-write-wins |
| `trusted_certificates` | ✅ | last-write-wins |
| `hypervisor_profiles` | ✅ | last-write-wins |
| `cloud_accounts` | ❌ | Hardware-bound token in keyring; row would land but the secret wouldn't |
| `tab_sessions` | ❌ | Per-device runtime state |
| `audit_log` | ❌ | Per-device security trail |

OCI hypervisor rows: same rules as the other hypervisor types — the
DB columns sync (tenancy/user/region/fingerprint/compartment OCIDs +
the `auth_type` discriminator), but the PEM private key and its
passphrase live in the platform keyring under
`oci_private_key_${id}` / `oci_passphrase_${id}` and never enter the
sync blob. After receiving an OCI row, the destination prompts the
user to re-import the PEM via Path A on first use.

### 5.5 3-way merge

For `connections`, `stored_keys`, `themes`, `host_keys` — the four
"3-way merge" entities — for each entity id present on either side
the merge engine produces a `MergeResult` containing `merged`,
`conflicts`, `deleted`, `added`, `updated` lists.

Cases (must match mobile):

- **Both unchanged** → unchanged
- **Local changed, remote unchanged** → local wins
- **Remote changed, local unchanged** → remote wins
- **Both changed identically** → unchanged
- **Both changed differently** → conflict, surface to user
- **Local deleted, remote unchanged** → delete
- **Remote deleted, local unchanged** → delete
- **Local deleted, remote modified** → conflict
- **Added on both with same id, identical content** → unchanged
- **Added on both with same id, different content** → conflict

Conflicts are **surfaced**, not silently dropped. Mobile's
`SyncConflictsActivity` is the UX template.

### 5.6 Stable field names — the minifier rule

Mobile and desktop both have build pipelines that can rename fields.
For sync to round-trip, neither side may rename JSON field names.

- Mobile: `kotlinx.serialization` keeps the source field names; the
  R8 ProGuard rules at `android/app/proguard-rules.pro` `-keep` every
  `@Serializable` class and its `Companion.serializer()`. **Do not
  remove those rules.**
- Desktop: `serde` keeps the source field names by default. **Do not
  add `#[serde(rename = "…")]` to sync structs unless mobile changes
  too.** No mangling, no JSON-with-comments, no field reordering for
  binary stability (JSON is order-independent at parse time but human
  diffs prefer consistency — match mobile's emit order).

### 5.7 Round-trip test

CI on both sides produces a fixture blob (mobile generates with a
seeded password + salt + IV, encrypts a known `SyncDataPackage`,
commits the bytes to a test fixture). The opposite client decrypts
the fixture and asserts the deserialised `SyncDataPackage` is
byte-equal to the same known struct on its own side.

Until that test is wired and green, sync is not "done" — it's
"works on my machine". This test catches every form of drift
(field rename, KDF tweak, IV size change, compression level shift)
before users see it.

---

## 6. QR pairing — the other contract with mobile

**Direction:** desktop sends, mobile receives. Desktop has filesystem
write access (e.g. `~/.ssh/authorized_keys`); phones don't.

**Wire format and 6-digit code derivation** are documented in
`../android/AI.md` §18. Same drift rules apply — match the mobile spec
exactly or pairing breaks.

**Non-goals (v1):**
- Bidirectional pairing (mobile → desktop).
- Continuous sync. Use §5 for that.
- Private key transfer. Public-key fingerprint + comment only.
- Multi-frame animated QR.

---

## 7. Recent mobile changes desktop must absorb

Mobile shipped these between the last desktop spec sync (2026-05-01)
and today. Desktop should mirror behaviour where the form factor
allows.

| Change | Mobile | Desktop |
|---|---|---|
| OCI hypervisor support (4th hypervisor type) | ✅ shipped Phases 1–7 | 🔴 add to feature list above |
| Reattach prompt for existing tab session | ✅ | 🔴 |
| Drag-to-select range copy in terminal | ✅ | 🔴 |
| Group sort + persistence | ✅ | 🔴 (relevant once groups land) |
| Custom keyboard always visible | ✅ | 🚫 N/A — desktop has no on-screen keyboard |
| Fullscreen Terminal pref reapplied on resume | ✅ | 🔴 (re-apply window decoration / cursor / font prefs on focus event) |
| `Ctrl+Space` / `Ctrl+@` / `Ctrl+[` etc chord routing | ✅ | 🔴 (desktop's key handler must mirror the same chord table) |

---

## 8. Acceptance criteria

Desktop is **"in line with mobile"** when:

1. Build is green on Linux / macOS / Windows in CI.
2. Sync round-trips — fixture blobs from mobile decrypt + apply on
   desktop and vice versa, with byte-equal `SyncDataPackage`. CI test
   wired and green.
3. Every row in §2 is ✅, OR has a tracked TODO in `TODO.AI.md`, OR
   is in §3.
4. Database schema matches mobile's current version (v29) with
   identical column names and types.
5. OS keyring stores every secret (passwords, OCI PEMs, OCI
   passphrases, cloud tokens). The DB has zero secrets after lazy
   migration.
6. The §7 list is mirrored or marked N/A.
7. Telemetry-free — no analytics, no crash-reporter-as-a-service.

---

## 9. Cross-references

- `AI.md` — architectural ground truth for desktop.
- `TODO.AI.md` — tactical task list, dependency-ordered.
- `../android/AI.md` — mobile's spec. §10 sync, §11 hypervisors (incl.
  §11.5 OCI), §18 QR pairing.
- `../android/TODO.AI.md` — mobile's roadmap.
- `../android/CLAUDE.md` — mobile's project tracker. The
  "What landed since" sections show recent mobile changes that need a
  desktop equivalent.
