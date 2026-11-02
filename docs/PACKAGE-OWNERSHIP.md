# PACKAGE-OWNERSHIP — H2: ownership named from the authoritative source

**Status:** `present`. **Block:** H2. **Recorded:** 2026-11-02. **Supersedes** the `blocked`
record of TICKET-0002 / LEDGER-0002.

**Sources read (read-only, no execution, no build):**

- `planet-shploingus` — `crates/planet-package/{src/frame.rs, src/rows.rs, src/shell.rs, src/error.rs, src/lib.rs, src/bin/package.rs, tests/tool.rs}`, `crates/planet-core/src/{hash.rs, canon.rs}`, `crates/planet-shell/{src/mount.rs, src/main.rs, src/chrome.rs, Cargo.toml}`, workspace `Cargo.toml`.
- `secure-shell` — `docs/FORMAT-SOURCES-AUDIT.md` (§3), `server/test/ingest.test.mjs`.
- `openlock` — `src/Openlock/Shell/ShellPackage.cs`, `tools/gen_shell_data.py`, `clientaddon/panorama/scripts/openlock_shell_data.js`.

Nothing was mounted, built, or run. This document is a decision record, not an implementation:
it fixes **who owns what** so that one reader exists. No parser, magic constant, or grammar is
written in this repository until a ticket says so.

## 1. Ownership

| Layer | Owner | Authority in the source |
|---|---|---|
| Package frame (magic, schema, kind, length, digest, payload) | `planet-package` crate | `frame.rs`: `FRAME_MAGIC`, `FRAME_SCHEMA`, `FrameKind`, `Package::{encode, decode, read, write}`, `frame_digest`, `file_name`, `id_and_digest` |
| Payload / row grammar | `planet-package` crate | `rows.rs`: `ROW_SCHEMA`, `Kind`, `Family`, `Row::{new, encode, decode}`, `KINDS_ARE_TOTAL` |
| Cell/shell declaration grammar | `planet-package` crate | `shell.rs`: `ShellDeclaration::{parse, encode, row}`, `PANELS`, `DeclarationError` |
| Refusal vocabulary | `planet-package` crate | `error.rs`: `PackageError`; `shell.rs`: `DeclarationError` |
| Content identity (hash function, domains) | `planet-core` crate | `hash.rs`: `DOMAIN_PACKAGE = b"planet-package-v1"`, `digest(domain, schema, bytes)`; `canon.rs`: `put_u16/put_u32/put_u64/put_bytes` (little-endian) |
| Builder (writer) | `planet-package` binary | `src/bin/package.rs`: `package build <id> --into <dir> [--code] [--row …] [--bands …] [--materials …] [--shell …]`, `package show <path>` |
| Acceptor / mount | `planet-shell` crate | `src/mount.rs`: `Mount::watch`, `Reload`; `src/main.rs`: the watch and `present_package` witness |
| Admission witness | `planet-mcp` (package door) | `crates/planet-mcp/src/surface.rs` (`present_package` → `load(&bytes)`) |
| Store on disk | the shell's `packages/` directory | `<id>-<64 hex sha256>.pkg`; the name must match the bytes |

**One grammar, two callers, by construction.** The builder parses a `.decl` with
`ShellDeclaration::parse` and immediately calls `declaration.row("shell")`; the shell mount takes
the `Kind::Shell` row's field and calls **the same** `ShellDeclaration::parse`. Neither compiles a
second copy. The crate's own module header states the rule: *"the tool that builds a shell package
and the shell that mounts one read a declaration through the same function."*

**Decision: depend, never vendor.** This port consumes `planet-package` and `planet-core` as path
dependencies. Copying either into this repository, or writing a local framing parser "to avoid the
dependency", is a second reader — the exact drift PATCH-0005 exists to refuse. The `package_owner`
module named in the plan is a thin adapter that calls the crate; it owns no bytes.

## 2. The frame, as the code defines it

```
magic      8 bytes   "V4PKG001"                    (FRAME_MAGIC; mismatch = BadMagic)
schema     2 bytes   u16 LE = 1                    (FRAME_SCHEMA; other = BadSchema)
kind       2 bytes   u16 LE  1 = Data, 2 = Code    (else UnknownFrameKind)
length     8 bytes   u64 LE, payload byte count    (mismatch = BadLength)
digest    32 bytes   payload digest                (mismatch = BadDigest)
payload    length    u16 ROW_SCHEMA, u32 row count, rows
                                             (head = 52 bytes; trailing bytes = TrailingBytes)
```

Two digests, over two different preimages — do not conflate them:

1. **Header digest (payload):** `digest(DOMAIN_PACKAGE, FRAME_SCHEMA, payload)` — the schema is
   inside the preimage, so a schema bump changes the digest rather than only the header.
2. **File-name digest (whole frame):** `frame_digest(bytes) = digest(DOMAIN_PACKAGE, FRAME_SCHEMA, bytes)`,
   and `Package::read` refuses a mismatch with `NameMismatch`.

`digest(domain, schema, bytes)` = SHA-256 over `domain ‖ schema.to_le_bytes() ‖ (len as u64).to_le_bytes() ‖ bytes`.

Identity rules: `<id>-<digest>.pkg`; an id that could be a path is `BadId`; a name with no digest
is `NamelessDigest`. A frame is read whole or refused — `Truncated`, `BadLength`, `TrailingBytes`
are all named refusals.

## 3. The payload

`ROW_SCHEMA: u16 = 1`, then `u32` row count, then rows. A row is
`put_bytes(kind name) ‖ put_bytes(subject) ‖ put_u32(field count) ‖ put_bytes(field)*`.

| Kind | Family | Fields | Meaning |
|---|---|---|---|
| `row` | declaration | 1 | one recipe row: the work a ticket's result implements |
| `bands` | value | 1 | light band table, as canonical bytes |
| `materials` | value | 1 | material table, as canonical bytes |
| `shell` | declaration | 1 | the shell declaration, as canonical text |

`Kind::COUNT = 4` and `KINDS_ARE_TOTAL: [(); Kind::COUNT] = [(); Kind::ALL.len()]` — a kind
without a line in the table fails to compile. **Adding a media type is a new kind: a row in
`rows.rs` plus a line in the format document, decided by the format owner.** There is no separate
asset archive, and this port must not create one.

## 4. The declaration grammar (`.decl`)

Tab-separated text. `#` comments and blank lines ignored; a trailing `\r` stripped.

- `window <title>` — exactly 2 fields, exactly one row per declaration.
- `destination <panel> <mark> <title>` — exactly 4 fields; `panel` must be one of
  `PANELS = [world, light, axis, changes, about]`; `mark` is exactly one character; at least one
  destination; a panel may be declared once.

`encode()` is canonical (window row, then destinations in rail order), so a comment-only or
spacing-only edit re-encodes to identical bytes and therefore to an identical package digest —
"a reload that changes nothing is one reload."

Ten named refusals: `NotText`, `UnknownVerb`, `FieldCount`, `Empty`, `UnknownPanel`,
`DuplicatePanel`, `MarkLength`, `NoWindow`, `TwoWindows`, `NoDestinations`.

## 5. The mount, as it actually is

`Mount::watch(dir)` — one pass, not a stream protocol:

1. Pick the newest `*.pkg` by `(mtime, name)`; the name breaks ties.
2. Digest it; a digest already answered for returns nothing **once** — a refusal also counts as
   answered, so a bad package is named once, not once per refresh.
3. A file mid-write (unreadable bytes) is skipped and retried next pass, not refused.
4. `Package::read` — refusal returns a `Reload { adopted: false, detail }` and the declaration in
   force is untouched.
5. `kind == Code` → refused (`mount.rs:111`).
6. The first `Kind::Shell` row is parsed; a package with no `shell` row, no field, or a refused
   declaration is refused whole.
7. Success replaces the declaration in force and reports window title, destination count, and the
   artifact `packages/<name>`.

`planet-shell/src/main.rs` then presents the adopted package at the host's package door
(`present_package`), so the reload leaves an admission on the ledger. **The receipt is the door
admission plus the ledger record** — there is no separate receipt object to invent.

Invariant to preserve: a refused reload leaves the previous declaration in force. There is no
half-adopted state.

## 6. The reserved code kind — confirmed, with one correction

- `FrameKind::Code = 2` is declared, encoded by `package build --code`, and decoded by the frame
  reader. The seam is real, as the plan says.
- **`planet-shell` refuses a code frame on the mount path** — the reload is refused by name and the
  declaration in force stands.
- **`planet-mcp` will accept presenting a code frame** (`surface.rs:450` tests exactly that kind), so
  presentation and mounting have deliberately different verdicts.
- **No executor exists.** Nothing in the tree runs a code frame's contents.

Consequence for the port: the *frame reader* for code exists; the *code-program semantics* do not.
The plan's rule stands — claim no script parity until an executor exists — but the missing piece is
the executor, not the reader.

## 7. Corrections to the plan's assumptions

The plan (§3.1, §12) was written from an audit that had not opened this tree. Three of its premises
are wrong, and adopting them would have created the drift the plan forbids.

1. **`shploingus.cells/1` is not the package's declaration format.** No Rust grammar for it exists
   in `planet-shploingus`, `secure-shell`, or `openlock`. It appears only in openlock's C# reader
   (`ShellPackage.cs`), its Python generator (`gen_shell_data.py`), and its shipped JS
   (`openlock_shell_data.js:11812`). The plan's §3.1 line that it is an "existing cell
   declaration … within the package system" is contradicted by the tree. The declaration surface of
   the authoritative system is the `.decl` grammar in `planet-package/src/shell.rs`.
2. **`shploingus.cellframe/1` has no reader anywhere**, in any of the three repositories, in any
   language: only the header check inside openlock's C#/Python paths, plus prose in secure-shell's
   audit. It cannot be adopted, and it must not be invented here.
3. **`surface.mount/2` is not the mount route in the authoritative tree.** There is no
   `surface.mount`, `mount/2`, or mount-v2 code in `planet-shploingus`; the string belongs to
   openlock/dl-enclave declarations. The mount discipline that exists is `Mount::watch` above, with
   admission through the package door.

Two further facts the plan did not have:

4. **The frame's identity is two digests** (§2): payload digest in the header, whole-frame digest in
   the file name. A port that keeps one and drops the other silently loses a refusal.
5. **Media rides in row fields, not in an asset archive** (§3). A new media type is a new `Kind`.

## 8. Consequences for the port

- `package_owner` (plan §5.1): thin adapter over `planet-package` + `planet-core`. No framing code.
- `cell_mount`: maps to `Mount::watch` semantics — newest-by-`(mtime, name)`, digest dedup, adopt or
  refuse whole, never a partial mount.
- `cell_stream`: the transport is the file the shell watches plus the door admission. If this port
  ever needs a streaming transport, that is a **new, explicitly decided layer** — not a claim that
  mount-v2 already exists.
- The declaration surface for the ported game is the `.decl` grammar; the game's placement inside
  `PANELS = [world, light, axis, changes, now about]` is a real constraint to design against, not a
  detail to widen silently.
- A code-kind executor is a separate, blocked decision (H5), and it must not arrive by widening a
  data row.

## 9. Exit condition — met

Named from source: the frame and row owners, the declaration owner, the refusal vocabulary, the
builder, the acceptor, the admission witness, and the shared-grammar evidence. Rejected
alternatives are recorded in §1 (vendoring) and §7 (adopting openlock's cell/cellframe grammar or a
non-existent mount-v2).

**Still blocked, and named as such:** `shploingus.cells/1` and `shploingus.cellframe/1` (no owner
anywhere — a format-owner decision if they are wanted), the code-kind executor (H5), a new media
kind (format owner), and any streaming transport beyond the watched file.
