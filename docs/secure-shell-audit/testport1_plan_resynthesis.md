# testport1 plan — resynthesized from the 2026-11-02 drive audit

**Status:** proposal for the mission planner, with the owner's twenty answers recorded in §8. Not a
repo document: it lives outside every checkout because `testport1` forbids direct-to-main plan
changes (AGENTS.md §4) and this supersedes a document that took five errata to stay honest.

**Governing plan being resynthesized:** `testport1/docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md`
plus `docs/PORT-BASELINE.md`, `docs/PACKAGE-OWNERSHIP.md`, `docs/SCOPE-ACCOUNTING.md`.

**Checkpoint, quoted from the plan being resynthesized:** *"The base game and its mods must become
peers in one cell-native package system."* (v3, §1). Nothing below widens that sentence.

---

## 1. What the audit changed

Source: `secure_shell_synthesis.md` (607 lines, streamed per root, 119,928 files, 187s, two roots
truncated honestly). Measurements that matter here:

| Measurement | Value | Consequence |
|---|---|---|
| Distinct numeric format ids | 129 raw → **83 qualified, 908 occurrences** | The id rule applies to all of them; none is a version. |
| Unqualified ids (`recipe/3`, `axis-grant/1`, `surface/1`, `cell/4`) | 46 distinct, 153 occurrences | An id with no family has no resolvable owner. |
| Zero-based ids (`shploingus.cells/0`, `dl-enclave.surface/0`, `secure-shell.served/0`, `shploingus.receipt/0`, `shploingus.cellframe/0`) | 5 distinct | `0` is not "prototype"; it is a live id with no hash. |
| Package magics in the trees | **5**: `V4PKG001` ×10, `SHPKG002` ×8, `SHPKG003` ×9, `V4PKG002` ×1, `SHPKG009` ×1 | "One format" is five today; `V4PKG002` is inside this repository. |
| XML under `Documents/` | 2,215 files (1,452 game, 726 shploingus, 36 openlock, 1 secure-shell) | A real streaming surface, now ruled to convert to cells (§8 Q9). |
| Qt/QML sites | **0** | No client exists anywhere; nothing here may claim a surface. |
| CI workflows | 2 (`testport1` gate, `openlock` pages) | Seven trees ungated, including `secure-shell` and `dl-enclave`. |

Three v3 claims are superseded rather than annotated: "one package format" is aspirational;
`surface.mount/2` has no object (E-2, E-5); `shploingus.cells/1` belongs to family B (E-1).

---

## 2. What `testport1` is now

`testport1` is the **ingest module of secure-shell v2**, not the runtime owner (Q16, Q17).

| Layer | Owner after this resynthesis | `testport1`'s role |
|---|---|---|
| Package frame (`V4PKG001`) | `planet-package` (this repo) | keeps it, as the only magic (Q1) |
| Row grammar, `.decl` | `planet-package` | keeps it |
| Hashed format ids | secure-shell v2 format owner | **implements here** (Q1–Q3) |
| Streamed transport (TLS WebSocket + AEAD) | secure-shell v2 | gives up ownership; implements the AEAD half it writes |
| Store (Mongo, systemd, `/etc/secure-shell/store.env`) | v2 host | writes through a driver (Q8) |
| Client surface (Qt 6: Core, Gui, Quick/QML, Network, Xml) | v2 client | gives up `winit`/`wgpu` as a product surface |
| SWF ingestion | this repo, §1.1 authorization | keeps, and is the next mission |

`planet-client` (`package_owner.rs`, `cell_mount.rs`) remains as evidence that the frame is adoptable;
it is no longer the client. Runtime rows in `docs/SCOPE-ACCOUNTING.md` move to a status this repo does
not yet have — `moved to v2` — with the owner named.

---

## 3. The id rule, and what "legacy" means here

The rule: a format id is `family.noun/<hex-hash>`; the hash covers canonical format text, schema
numbers, reader version and the full refusal set (Q2), truncated to 32 hex characters (Q3).

`V4PKG001` stays the only magic; the hash replaces the numeric schema identity (Q1). No second magic,
no `V4PKG002`, no conversion in place.

**Numeric ids are not refused by the gate** (Q20). They are recorded programmatically as **legacy
formats awaiting update from the upstream secure-shell v2 store**. That changes the artifact and the
check:

- `config/legacy-formats.json` — one row per distinct id: the id, its occurrence count, the owning
  tree, and the v2 store key it must be updated from. The audit's 83 qualified / 46 unqualified / 5
  zero-based sets seed it.
- Gate check `legacy_formats` passes only when **every** numeric id found in tracked code and config
  has a registry row. The id itself is advisory; an unregistered legacy id is the failure
  (`legacy-unregistered`), and so is a registry row whose upstream key is missing
  (`legacy-unprovenanced`).
- A legacy id is never presented as current: an emitted package, window or stream row carrying one is
  refused `format-id-legacy-emitted`, because the proof's job is to produce only hashed ids.

So the gate distinguishes three things it previously conflated: a hash-versioned id (`held`), a
legacy id on the register (`noted`, awaiting the v2 store), and a legacy id used to produce a new
artifact (`refused`).

---

## 4. The import pipeline, restated

The audit's own failure mode was holding output until exit; the importer now obeys the same rule. An
import that produces a package at the end and nothing on the way is refused `import-unstreamed`.

```
bytes → header → tag stream → survey → lower (xml→cells, code) → emit windows → seal → store
  |         |          |            |              |                  |         |        |
  +---------+----------+------------+--------------+------------------+---------+--------+
        every stage appends sealed rows, flushed per stage and every 1,000 records (Q7)
```

Required properties:

1. **Per-stage streaming**, per stage and every 1,000 records inside a stage, flushed (Q7).
2. **AEAD on every row.** XChaCha20-Poly1305 (Q4); one master in `/etc/secure-shell/store.env` with
   three derived domain keys — stream rows, ledger rows, store records (Q5). Both ledgers are sealed:
   `testport1/ledger/ledger.jsonl` and the shell's v2 rows (Q6). A row without a tag is refused
   `aead-missing`; a row sealed in the wrong domain fails to open.
3. **Hashed identity (Q1–Q3).** Emitted packages name `planet.<noun>/<32-hex>`; the acceptor
   recomputes before decode; a mismatch is refused `format-hash-mismatch`.
4. **Named refusals, not silence (Q14).** An unknown tag, an overrunning body or an unlowerable
   construct aborts the import with a named code and leaves nothing half-written.
5. **Windows are packages (Q10).** Taskbar and dashboard ship as two separate window packages, each
   with its own digest, receipt and replacement semantics.
6. **XML converts to cells (Q9).** The windows' XML is ingested and lowered into declared cells
   inside the package — not carried as a payload, not stored, not wired. This is the one answer that
   depends on a decision this repository does not own: `docs/CELL-SCHEMA.md` Q-1…Q-6 is still
   awaiting the format owner's ruling, so the lowering has a named precondition (§6 P6).
7. **Code frames execute (Q13).** The importer may emit code frames only together with a minimal,
   bounded executor that validates and runs them under D-3's instruction set, budgets and
   capabilities. Until that executor exists, `planet-shell/src/mount.rs:111` keeps refusing the kind.

---

## 5. Where the importer stands

| Fact | Evidence |
|---|---|
| Authorized for one file, structure only | `AGENTS.md` §1.1; `crates/swf-import/src/lib.rs:1-6` |
| Signature from bytes, `CWS` inflated, `ZWS` refused | `src/header.rs:96-131` |
| Tag walk, short and long lengths, End tag | `src/tags.rs:23-88` |
| Shape bounds, sprite nesting, media classes | `src/content.rs:158-226` |
| Emits a **declaration** document only, validated before framing | `src/emit.rs:104-116` |
| Records through `mount_and_record` | `src/bin/swf-import.rs:71-83` |
| Counted, not lowered: shapes, sprites, action tags | `src/lib.rs:4-6`, `content.rs:1-6` |
| 12 synthetic tests, no supplied bytes | `tests/import.rs`, `tests/content.rs` |
| No code executor; mount refuses code frames | `planet-package/src/frame.rs:11`, `planet-shell/src/mount.rs:111` |

It measures a movie correctly and emits a label stack. It proves the packaging path and says so; it
does not yet prove the new system.

---

## 6. The next mission — the importer as the proof of concept

Goal, one sentence: *the SWF importer streams a source-derived package whose format id is hashed,
whose every row is sealed, whose windows are cell packages converted from XML, whose code frames run
under a bounded executor, and whose path ends in the shell's Qt client.*

Units, one artifact each, each with an exit test and a ledger row (`LEDGER-0043…` here; v2 starts its
own ledger). Every unit stays under the 40-file / 4,000-insertion budget (AGENTS.md §11).

| Unit | Artifact | Exit test | Blocks on |
|---|---|---|---|
| **P0** | `secure-shell-v2` scaffold: `AGENTS.md`, `VOWS.md`, `CHECKPOINT.md`, format doc skeleton, gate, first ledger row | gate passes on the empty tree; first commit cites it | remote URL (Q18) |
| **P1** | `config/legacy-formats.json` + gate check `legacy_formats` | every numeric id from the audit is registered with an upstream v2 key; an unregistered id fails `legacy-unregistered` | — |
| **P2** | `planet-package`: hashed schema identity, reader recompute | round-trip; tampered hash refused `format-hash-mismatch`; emitted legacy id refused `format-id-legacy-emitted` | P1 |
| **P3** | `crates/planet-aead`: three domain keys, seal/open | cross-domain open fails; `aead-missing` fixture; both ledgers sealed | — |
| **P4** | importer streaming (`stream.jsonl`, flush per stage + 1,000) | kill mid-import leaves N sealed rows and `import-truncated`, never a silent partial | P3 |
| **P5** | importer refusal completion | unknown tag, overrun, unlowerable each named with a fixture; nothing written on abort | P4 |
| **P6** | XML → declared cells for taskbar and dashboard | the two windows' XML lowers into cells the package declares; an undeclared construct is refused `cells-schema-undeclared` | `CELL-SCHEMA.md` Q-1…Q-6 ruling |
| **P7** | two window packages (taskbar, dashboard) | two hashed ids, two digests, two receipts; replacement of one leaves the other mounted | P2, P6 |
| **P8** | LZMA (`ZWS`) decoder | a `ZWS` movie imports; a corrupt LZMA stream still refused `swf-compression` | — |
| **P9** | code frames + minimal executor | a code frame validates, runs under budgets/capabilities, and mount stops refusing it — only with the test that proves the executor | D-3 op set (H5) |
| **P10** | Mongo writer (driver) | packages and stream rows land in the v2 store, read back and verified; absent store refused `stream-store-unavailable` | v2 host, `store.env` |
| **P11** | `docs/IMPORT-PROOF.md` + ledger rows | the Q19 chain, each step's command, exit status, check count and digest | P1–P10 |

**Done when (Q19):** the source is streamed into sealed rows, framed as a hashed package, opened under
AEAD, recorded in the v2 store, and mounted as a window in the Qt client — with the user watching
(Q15).

**Not claimed by the proof:** that the game plays. Geometry, full timeline and AVM1/2 lowering remain
blocked on `CELL-SCHEMA.md` Q-1…Q-6, media M-1…M-5 and D-3 (`D-7a`). The `client_witness` row stays
`NOT-ENFORCED` until the Qt mount is witnessed.

---

## 7. Carried refusals

- A browser, Node or headless result reported as a client witness.
- A second magic, renderer, input router, scheduler or store.
- A legacy id used to emit a new package, window or stream row.
- An XML payload stored or carried instead of lowered (Q9).
- A scan or import that holds its output until exit.
- A row (ledger, stream, store) without its AEAD tag.
- A code frame emitted or mounted without an executor that validates and budgets it.
- Lowering a shape, full timeline or script before the format declares the representation.
- Counting drives, windows or features in a plan whose package does not carry them.

---

## 8. The owner's twenty answers (2026-11-02)

| # | Question | Answer |
|---|---|---|
| Q1 | Frame vs hashed id | Keep `V4PKG001` as the only magic; the hash replaces the numeric schema identity. |
| Q2 | Hash preimage | Canonical format text + schema numbers + reader version + full refusal set. |
| Q3 | Hash width | 32 hex characters. |
| Q4 | AEAD | XChaCha20-Poly1305. |
| Q5 | Key topology | One master in `/etc/secure-shell/store.env`, three derived domain keys. |
| Q6 | Ledger sealing | Both `testport1/ledger/ledger.jsonl` and the shell's v2 rows. |
| Q7 | Stream cadence | Per stage, plus every 1,000 records inside a stage. |
| Q8 | Store writer | The importer writes to Mongo through a driver. |
| Q9 | XML | **Convert into declared cells inside the package** (off the recommendation). |
| Q10 | Window shipping | Two separate window packages. |
| Q11 | Drive enumeration | Later, inside the settings window package. |
| Q12 | ZWS/LZMA | Add the decoder; drop the `ZWS` refusal. |
| Q13 | Code frames | **Yes, with a minimal op set executed by a new executor** (off the recommendation). |
| Q14 | Unknown tags | Named refusal, import aborts, nothing half-written. |
| Q15 | Witness | The user, watching the Qt client. |
| Q16 | Repair locus | In place in `testport1`, as its ingest module. |
| Q17 | v2 repository | Create it now; `testport1` becomes its ingest module. |
| Q18 | Review path | Configure a remote; ticket branches and PRs. |
| Q19 | Done test | Source → streamed rows → hashed package → AEAD open → store record → Qt window, all witnessed. |
| Q20 | Numeric ids | **Not refused; registered as legacy formats requiring update from the upstream v2 store** (off the recommendation). |

### What the three deviations force

1. **Q9 (XML → cells)** is the strongest answer and the most exposed: it makes the window path depend
   on a schema ruling that is still open. P6 cannot start lowering until Q-1…Q-6 is answered; until
   then the importer refuses `cells-schema-undeclared` by name rather than carrying the XML as a
   payload "temporarily". The ruling request is the mission's first dependency.
2. **Q13 (code frames + executor)** converts H5 from a blocked decision into a build target. D-3
   already rules the instruction set bounded and deterministic; P9 must fix the op set, budgets,
   capabilities and cancellation, and lift the mount refusal only in the commit that proves the
   executor. Emitting code frames before that commit would repeat the "reader without executor"
   gap the errata caught.
3. **Q20 (legacy, not refused)** replaces a hard gate with a registry. It is honest — the ids are
   real and the upstream store is where their replacements come from — but it means the gate can no
   longer be the thing that stops a legacy id reaching a package. That job moves to
   `format-id-legacy-emitted` at emission and mount, which P2 must implement before P7 emits windows.

### Open items carried into the mission

- **Q18 has no remote URL.** Branches and PRs need one; until it is supplied, P0 lands locally with
  the gate reporting review as `NOT-ENFORCED`, as it does today.
- **Q6 implies the shell's v2 ledger exists.** It does not yet; P0 creates it, P3 seals it.
- **Q8 needs the Debian host reachable** and `store.env` present; neither is verified.

---

## 9. Store model — corrected from the remotes (2026-11-02)

Read through the GitHub integration (`alacripose/secure-shell`, `docs/ARCHITECTURE.md` at `main`,
blob `b3d5257`) and from the local `dl-enclave` checkout. This supersedes every `store.env` and
Mongo entry earlier in this file.

| Question | What the sources actually say |
|---|---|
| Store of record | **append-only file + the filesystem's durability contract**; the ledger is the durable outbox. Quoted: *"No MongoDB. MongoDB was the `dl-enclave` derail; this backend uses an append-only file and the filesystem's durability contract."* |
| Ledger | `secure-shell.ledger/1`: typed entry, `expected_seq`, idempotency key, chain `prev`/`hash`, Ed25519-signed checkpoint; stale write refused `ledger-stale`; malformed tail `ledger-corrupt`, never repaired. |
| Package stream | `dl-enclave.surface/1` with `surface.mount/2` receipts. |
| Vault | AES-256-GCM for prompt text; the entry carries only a digest and a reference. |
| Host | Debian stable + systemd units (`dl-enclave/docs/BOUNDARIES.md:15,78`). |
| Secrets at rest | **LUKS2 + TPM2 sealed key; `systemd-creds` for service secrets** — no plaintext key on disk (`dl-enclave/docs/PACKAGE-FORMAT.md:746`). |
| Boot binding / update | UKI + `systemd-measure` signed PCR policy; `systemd-sysupdate` A/B, experimental (`PACKAGE-FORMAT.md:747-749`, `PLAN.md:156`). |
| Ledger location | outside the checkout by default: `$SECURE_SHELL_LEDGER_DIR` or `~/.secure-shell/ledger`, owner-only. |

Ruled by the owner the same day, superseding the implications above:

1. **Store of record: MongoDB**, systemd-managed on the Debian host. P10 keeps its Mongo driver; the
   earlier Q8 answer stands.
2. **Credentials come from `systemd-creds`**, never a plaintext `/etc/secure-shell/store.env`. Q5's
   "one master, three derived domain keys" survives; only its location changes.
3. **No branch is authoritative over the secure shell's requirements** (the owner's words). The
   `ARCHITECTURE.md` sentence calling Mongo "the dl-enclave derail" is that branch's position, not a v2
   requirement; it is recorded as a resolved conflict, not as a correction to be obeyed.
4. **AEAD unchanged**: XChaCha20-Poly1305, three derived domains. `ARCHITECTURE.md`'s vault cipher
   (AES-256-GCM) and its Ed25519 checkpoint are different objects and must be named separately, so the
   vault's cipher is never silently reused for frames, ledger rows or store records.
5. **The ledger stays the port's checkpoint record** even with Mongo as the store: `ledger/ledger.jsonl`
   rows keep the command, exit status, check count and digest, and `secure-shell.ledger/1` keeps its
   chain and Ed25519 checkpoint. Mongo holds the registry and stream rows; it does not replace either.

---

## 10. Second round — twenty plain-language answers (2026-11-02)

| # | Question | Answer |
|---|---|---|
| 1 | Where the shaper list lives | A labelled section at the top of the declaration. |
| 2 | Largest package | 1 GiB now, **4 GiB** as the ceiling that covers the SWFs. |
| 3 | v2 repository | New repo under `alacripose`; exact name still to be given. |
| 4 | Debian host | Up a few days ago; not verified reachable today. |
| 5 | MongoDB | Has worked before; agents have broken it repeatedly. |
| 6 | Key topology | **One key per package, no master.** |
| 7 | Frame sealing | **Sealed frame containing sealed rows.** |
| 8 | Missing key | Stop, toast the refusal, ask whether rendering may be exposed to agents unsecurely, and still prevent reading game content while displaying it. |
| 9 | First code frame | Count, compare, branch — nothing visible. |
| 10 | Program budget | 100,000 instructions or 16 ms. |
| 11 | Taskbar XML source | Convert an existing openlock window definition. |
| 12 | Taskbar behaviour | Full taskbar, 8-corner pinning, small-button mode shrinking window buttons to circles. |
| 13 | Dashboard | A window layout; used as the applications start menu with a Windows 7 start-menu layout. |
| 14 | Dashboard and the store | The dashboard must see the Debian store's own packages; the store runs the same reader and its own visual shell. |
| 15 | Legacy registry keys | Derived automatically from the ids and trees. |
| 16 | Ticket numbering | Restart at 0001 in v2. |
| 17 | Proof document | `docs/IMPORT-PROOF.md` in the repo. |
| 18 | Qt in this mission | **No — stop at the store; mount in Qt next mission.** |
| 19 | Watching it | A script printing each step and its digest. |
| 20 | Unit size | A whole day's work, then review. |

### What these answers change, including where they conflict

1. **Ceiling is 4 GiB, not 256 MiB** (Q2). With 64-bit offsets that is consistent; the refusal is
   `package-ceiling` and the value is declared, not implied. 1 GiB is the working size, 4 GiB the
   limit.
2. **Per-package keys, no master** (Q6) supersedes the earlier master-plus-three-domains answer and
   the `systemd-creds` sentence in §9. Consequence to resolve: ledger rows and store records span
   packages, so they cannot be sealed by a package key. Either they get a store-level key (a master
   under another name) or they are sealed per package and the registry row carries the package key
   id. **This is an open question the plan cannot answer by itself.**
3. **Sealed frame containing sealed rows** (Q7) changes P2 and P3: the mount must decrypt before it
   can adopt a declaration, and the digest is over ciphertext. A frame whose key is unavailable has
   no readable declaration, which is exactly the state Q8 describes.
4. **Q8 creates a real state, not a fallback:** `unsealed-display-only` — refused by default, entered
   only with explicit per-run authorization, announced by toast, and bounded by guards that keep the
   content renderable and unreadable (no extraction path, no text export, no capture to a file the
   agent can open). It must be a named state with its own ledger row, never a silent downgrade.
5. **The taskbar is a feature, not a stub** (Q12): 8-corner pinning and circular small buttons need
   input routing and state, which the proof otherwise does not have. P7 grows from "two windows" to
   "taskbar with pinning and button modes", and it can no longer be the first window unit.
6. **The dashboard reads the store** (Q14), contradicting the "no reads in the proof" recommendation:
   it must list the Debian store's own packages through the same reader. That makes P10 a write *and*
   read path, and the dashboard the first real consumer of the store.
7. **Qt is out of this mission** (Q18), so the done test from Q19 of the first round no longer
   applies: the mission ends at the store, `client_witness` stays `NOT-ENFORCED`, and the script's
   digest log (Q19 now) is progress evidence, never a mount witness.
8. **Tickets restart at 0001 in v2** (Q16), so `LEDGER-0043…` numbering above is void; v2 starts its
   own `TICKET-0001` / `LEDGER-0001`. This repository's 42 entries stay as they are.
9. **Day-sized review** (Q20) does not suspend AGENTS.md §11: a day may hold several commits, but no
   commit exceeds 40 files or 4,000 insertions, and each still cites its ticket and ledger row.
10. **Legacy keys derived automatically** (Q15) makes the registry a derivation, not a provenance
    claim: each row records the tree path and the rule that produced it, marked `derived`, and the
    gate refuses a derived row presented as an upstream key.
11. **Mongo is fragile in this family's hands** (Q5): P10 needs a dedicated role with no DDL, a dump
    before every write unit, and a gate check refusing `drop`/`dropDatabase` anywhere in code.
12. **The host is unverified** (Q4) and **the repo name is missing** (Q3): P10 and P0 are blocked on
    those two facts, and neither is something to assume.
