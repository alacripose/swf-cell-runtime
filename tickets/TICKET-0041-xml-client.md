# TICKET-0041 — The XML client draws the same scene as the wgpu client

- Owner: implementation agent
- Block: H9 client surface
- Branch: `ticket/TICKET-0041-xml-client`
- Ledger: LEDGER-0047
- Depends on: TICKET-0039 (the wgpu window), the owner's ruling that the windows' XML is converted
  into declared cells inside the package (Q9) and that taskbar and dashboard are separate packages (Q10)

## What was missing

There was one client format. The owner asked for two branches carrying formats for streaming media: a
wgpu window over the mounted package, and an XML version. Without the second, "the client shows it"
meant one renderer rather than two formats compared on one package.

## What this builds

- `crates/planet-client/src/render.rs` — the window loop both binaries use: title plus lines, glyphon
  over a wgpu surface, `Scene` as the shared shape. Extracted from TICKET-0039's binary so the formats
  cannot drift in the part they are compared on.
- `crates/planet-client/src/xml.rs` — the second format: a small, strict window reader. Declared
  elements are `window`, `heading`, `label`, `item`, `list`, `separator`; declared attributes are
  `title`, `width`, `height` on `window`. Anything else is refused by name — `xml-element`,
  `xml-attribute`, `xml-structure`, `xml-empty`. The XML is lowered to lines and is never carried
  forward as a payload.
- `crates/planet-client/src/bin/xml_window.rs` — takes a path or the newest `windows/*.xml`, lowers
  it, and draws the scene; a refusal is shown by code rather than passed through.
- `windows/taskbar.xml` — a taskbar window definition: pinned list, separator, and the 8-corner and
  small-button rules as declared labels.

`window.rs` now builds its `Scene` from the adopted package and calls the same `render::run`.

## What this does not do

It does not lower XML into declared cells inside a package: that is P6 and it is blocked on
`docs/CELL-SCHEMA.md` Q-1…Q-6, so this unit lowers to the renderer's lines and says so. It does not
implement taskbar input, pinning behaviour or button modes — the definition names them, the client
does not act on them. It does not make the dashboard list the Debian store's packages; that needs the
store. Neither binary claims the game plays.

## Acceptance test

- `cargo test -p planet-client` exits 0: 6 XML tests (lowering, scene, undeclared element, undeclared
  attribute, stray text, empty document) and the 8 adopt tests.
- `cargo build -p planet-client --bins` exits 0, producing `window` and `xml_window`.
- Each binary runs for 12 seconds with an empty log while its window process stays alive, then is
  stopped by its recorded pid.

## Closing

- Commit: this unit's commit on `ticket/TICKET-0041-xml-client` (unmerged; no remote is configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0047
