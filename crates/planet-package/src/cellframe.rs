//! `shploingus.cellframe/1`, parsed strictly — the frame grammar this port reads.
//!
//! # Where this grammar comes from
//!
//! Not from a sample file. `shploingus-clean/crates/voxel-package/src/cellframe.rs` is the owning
//! reader and this module is its closure rules restated in this crate's refusal style: the header,
//! the `frame` row, the five required keys, the optional `feature` key, the two draw rows, the seven
//! sampling rows, the format's own bounds, and the one-pass refusal that names every problem at once.
//! `docs/CELL-SCHEMA.md` records the read; CGA-21 of the v2 addendum is why it was done before this
//! module existed, because a grammar inferred from one instance is the refusal it names.
//!
//! # What a frame is
//!
//! A grid of cells addressed to one declared window's content space. It carries two kinds of draw
//! row — `fill`, a run of paper, and `run`, glyphs in one ink — and a frame names a **role**, never a
//! colour, exactly as the owning reader says (`AGENTS.md`: roles, not colours).
//!
//! # The streamed vocabulary section
//!
//! Q-3 was ruled *an open vocabulary streamed in a labelled section at the top of the declaration*.
//! The section is `[vocabulary]`, terminated by one blank line, and it declares the role words this
//! package may use:
//!
//! ```text
//! [vocabulary]
//! paper<TAB>surface,chrome,background,divider,transparent,hazard
//! ink<TAB>text,dim,disabled,accent,divider,red,green,orange
//!
//! shploingus.cellframe/1
//! frame<TAB>MAP-0001
//! ```
//!
//! A package that carries no section gets the format's declared set, which is the owning reader's
//! set unchanged, so the section is additive and never required. When it is present it **replaces**
//! the set, and a draw row using a word the section did not declare is refused `cellframe-vocabulary`
//! rather than drawn as the nearest known role. The section is stripped before the frame grammar
//! sees the document, so the owning grammar is not extended, reordered or made optional by it.
//!
//! # Bounds are the format's
//!
//! An **interim** 8 MiB declaration, a 512 by 512 grid and 65 536 cells, plus the format's own cell
//! size. Each is a refusal, not a truncation. The numbers are a floor: the format owner ruled the
//! 512 KiB / 16 384 pair is not the ceiling and that the exact numbers are still owed
//! (`secure-shell-audit/cell-schema-ruling.md`, Q-4).

use crate::frame::Refusal;
use crate::surface::Surface;

/// The pin. A different grammar is a different file, and an older one is refused rather than read.
pub const HEADER: &str = "shploingus.cellframe/1";
/// One frame's ceiling: a declaration, not a truncation. **Interim**, and a floor rather than a
/// ruling: the owner's Q-4 says the surface is stateful and real-time, the addressing 64-bit, the
/// geometry subvoxel, and the exact numbers still owed — so this admits that class and no refusal
/// may quote it as the format's final answer.
pub const MAX_FRAME_BYTES: usize = 8 * 1024 * 1024;
/// The widest a frame may declare.
pub const MAX_COLS: u32 = 512;
/// The tallest a frame may declare.
pub const MAX_ROWS: u32 = 512;
/// The most cells one frame may hold. **Interim**, with `MAX_FRAME_BYTES`: a 640 by 480 stage at 8 by
/// 16 is 2 400 cells a frame, and a twelve-frame timeline's busiest frames lower past the old 16 384
/// constant. The ruled numbers are still owed.
pub const MAX_CELLS: u32 = 65_536;
/// One cell's width in device-independent pixels. **The format's number.**
pub const CELL_W_DP: u32 = 8;
/// One cell's height in device-independent pixels.
pub const CELL_H_DP: u32 = 16;
/// Every required key a frame carries, in grammar order.
pub const KEYS: [&str; 5] = ["window", "cols", "rows", "order", "requires"];
/// The key a frame may carry and may omit.
pub const OPTIONAL_KEYS: [&str; 1] = ["feature"];
/// The fields this build can sample. An unknown name is a refusal, not a default.
pub const FIELDS: [&str; 2] = ["elevation", "biome"];
/// The only cell law the renderer carries.
pub const SHAPER: &str = "profile32";
/// That law's divisions per axis.
pub const SHAPER_CELLS: u32 = 32;
/// The only camera the renderer carries.
pub const PROJECTION: &str = "orthographic";
/// The paragraph that opens the streamed vocabulary section.
pub const VOCABULARY_SECTION: &str = "[vocabulary]";
/// The paragraph that opens the streamed artifact table. This is family B's own shape, read from
/// `shploingus-clean/crates/voxel-package/src/recipe.rs`: a package carries its files as an artifact
/// table keyed by package-relative path, not as bytes spliced into a drawing row.
pub const ARTIFACT_SECTION: &str = "[artifact]";
/// Longest artifact path: family B's `ARTIFACT_PATH_MAX`, restated here rather than widened.
pub const ARTIFACT_PATH_MAX: usize = 256;
/// Longest one artifact: family B's `ARTIFACT_BYTES_MAX` (4 MiB).
pub const ARTIFACT_BYTES_MAX: usize = 4 * 1024 * 1024;
/// How many artifacts one package may carry: family B's `ARTIFACT_COUNT_MAX`.
pub const ARTIFACT_COUNT_MAX: usize = 1024;
/// The paragraph that opens the streamed audio section. Like the vocabulary section it is additive:
/// a frame that carries no section carries no audio, and the owning grammar is unchanged by it. Its
/// rows **name an artifact**, they do not carry one: family B's frames are sibling artifacts the
/// package names by id (`cellframe.rs`), and `voxel-client`'s mount holds them in `frame_artifacts`.
pub const AUDIO_SECTION: &str = "[audio]";
/// The paragraph that opens the streamed absence section: what the package states it does **not**
/// carry, so a client never has to invent a shape for it.
///
/// The rule is family B's, read from `shploingus-clean/crates/voxel-package/src/cellframe.rs`: a
/// frame with no `fill` and no `run` is refused by name, because "an empty frame is a window's
/// content erased, and an absence has to be stated as one". Its refusal kind is `MalformedRecord`,
/// so that name is what an absence row cites as `cellframe::malformed-record`. The same reader
/// refuses a heading with nothing under it ("a heading with nothing under it is a heading that was
/// not written"), so this section, when it appears, carries at least one row.
pub const ABSENCE_SECTION: &str = "[absence]";
/// The kinds an absence row may name. A closed set: a new kind is a row here and a line in the
/// format document, because registering one is a recorded act.
pub const ABSENCE_KINDS: [&str; 3] = ["sprite", "shape", "cell"];
/// The paragraph that opens the streamed timeline: the frames that follow the bound one, each a
/// carried artifact with the order the client advances them in.
///
/// This is family B's shape, read from `shploingus-clean/crates/voxel-package/src/cellframe.rs` and
/// `voxel-client/src/shell/mount.rs`: "a frame is a sibling artifact the package names by id", and the
/// mount holds them in `frame_artifacts`. A one-frame package is a movie that never advances, so the
/// picture it shows is the frame it happens to carry.
pub const TIMELINE_SECTION: &str = "[timeline]";
/// The paragraph that opens the sound-event list: which carried sound each timeline frame starts.
///
/// An event is not a payload. The character is carried once in `[audio]` and the event names it, so a
/// client can schedule the sound the movie starts without carrying its bytes twelve times — and a
/// package that carried payloads with no events would hold audio nothing can place in time.
pub const EVENT_SECTION: &str = "[event]";

/// One sound the frame carries: its identifier, how it is encoded, and its **payload bytes**.
///
/// The bytes are the sound character's own payload, carried verbatim so a player has audio to
/// consume rather than a count of audio. `format` and `rate` are the source's declared encoding, not
/// a guess: a consumer that cannot decode a format refuses it by name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SoundClip {
    /// `sound` for a definition character, `stream` for a stream block; `id` is the character id,
    /// or `-` where the source carries none.
    pub kind: String,
    pub id: String,
    /// The package-relative artifact the payload came from: the row names it, the table holds it.
    pub artifact: String,
    /// The source's encoding token (`mp3`, `adpcm`, ...).
    pub format: String,
    /// Samples per second.
    pub rate: u32,
    /// The sample count the source declared.
    pub samples: u32,
    /// The payload, byte for byte.
    pub bytes: Vec<u8>,
}

/// One frame of the package's timeline: the order it is shown in, and the artifact that carries it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineFrame {
    /// The order the client advances this frame in, ascending from 1.
    pub order: u32,
    /// The package-relative artifact whose payload is the frame document.
    pub artifact: String,
}

/// One scheduled sound: the timeline frame that starts it, and the audio character it names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SoundEvent {
    /// The timeline frame's order (1 for the bound frame, then each `[timeline]` row).
    pub frame: u32,
    /// The audio character's id, as an `[audio]` row carries it.
    pub sound: String,
}

/// One visual payload the package **does not carry**, stated instead of drawn.
///
/// An absence is not a failed measurement and not a hole: it is the package saying that a character
/// has no art in the file, with the owning rule that makes the statement the honest answer. A
/// client that reads it names the character and the reason; a client that reads nothing draws
/// nothing, which is why the row exists rather than a box.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Absence {
    /// What has no art: one of [`ABSENCE_KINDS`] (`sprite`, `shape`, `cell`).
    pub kind: String,
    /// The character or cell identifier the package names it by.
    pub id: String,
    /// Why there is no art, as one token: `no-display-list`, `no-drawn-record`,
    /// `undecoded-display-record`.
    pub reason: String,
    /// The owning-tree rule that makes the absence a statement rather than a failure, as a law
    /// token such as `cellframe::malformed-record`.
    pub rule: String,
}

/// A stable digest of a payload, so "the bytes that arrived are the bytes that left" is asserted
/// rather than eyeballed. FNV-1a 64, printed as sixteen hex digits.
pub fn payload_digest(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// One file a package carries, keyed by package-relative path.
///
/// The shape is family B's: `recipe.rs` writes a count, then every `(path, bytes)` in path order,
/// and validates the path on compile **and** on load. Here the table is a section of the declaration
/// instead of a trailer after it, because this port's package payload is the declaration; the rules
/// are the same rules, and the digest travels with the row so byte identity is checkable on its own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Artifact {
    pub path: String,
    /// The payload's digest, stated by the row and checked against the bytes on read.
    pub digest: String,
    pub bytes: Vec<u8>,
}

/// Whether a name is a package-relative path and nothing else — `recipe.rs::artifact_path_is_well_formed`
/// restated: one or more non-empty `/`-separated parts, no `..`, no leading `/`, no backslash, and
/// inside the path ceiling.
pub fn artifact_path_is_well_formed(path: &str) -> bool {
    !path.is_empty()
        && path.len() <= ARTIFACT_PATH_MAX
        && !path.starts_with('/')
        && !path.contains('\\')
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

/// An audio id or format token. Unlike a role word it may be a bare number, because a sound's
/// identifier is a character id.
fn audio_token(text: &str) -> bool {
    !text.is_empty()
        && !text.chars().any(char::is_whitespace)
        && text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

fn decode_hex(text: &str) -> Option<Vec<u8>> {
    if text.is_empty() || text.len() % 2 != 0 {
        return None;
    }
    let raw = text.as_bytes();
    let mut out = Vec::with_capacity(raw.len() / 2);
    let mut index = 0;
    while index < raw.len() {
        let high = (raw[index] as char).to_digit(16)?;
        let low = (raw[index + 1] as char).to_digit(16)?;
        out.push((high * 16 + low) as u8);
        index += 2;
    }
    Some(out)
}

/// The paper roles the format declares when a package streams no section of its own.
pub const PAPERS: [&str; 5] = ["surface", "chrome", "background", "divider", "transparent"];
/// The ink roles the format declares when a package streams no section of its own.
pub const INKS: [&str; 8] = [
    "text", "dim", "disabled", "accent", "divider", "red", "green", "orange",
];

/// Every problem one parse found, in the order the rows were read.
///
/// The owning reader refuses with a set rather than a single error ("with every problem named in one
/// pass rather than one per round-trip"), and a caller fixing a declaration should not need seven
/// runs to learn seven missing keys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusals {
    problems: Vec<Refusal>,
}

impl Refusals {
    pub fn new() -> Self {
        Refusals {
            problems: Vec::new(),
        }
    }

    pub fn refuse(&mut self, refusal: Refusal) {
        self.problems.push(refusal);
    }

    pub fn is_empty(&self) -> bool {
        self.problems.is_empty()
    }

    pub fn len(&self) -> usize {
        self.problems.len()
    }

    /// Every refusal, in read order.
    pub fn problems(&self) -> &[Refusal] {
        &self.problems
    }

    /// The first refusal's code: the one a caller branches on when it asks for one.
    pub fn code(&self) -> &'static str {
        self.problems
            .first()
            .map(Refusal::code)
            .unwrap_or("cellframe-schema")
    }

    /// Every message, joined, so one run reports every problem.
    pub fn message(&self) -> String {
        self.problems
            .iter()
            .map(Refusal::message)
            .collect::<Vec<String>>()
            .join("; ")
    }
}

impl Default for Refusals {
    fn default() -> Self {
        Refusals::new()
    }
}

impl std::fmt::Display for Refusals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for Refusals {}

/// The role words one package may use. The declared set is the owning reader's; a streamed section
/// replaces it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vocabulary {
    papers: Vec<String>,
    inks: Vec<String>,
}

impl Vocabulary {
    /// The format's own set, used when the declaration streams no section.
    pub fn declared() -> Self {
        Vocabulary {
            papers: PAPERS.iter().map(|word| word.to_string()).collect(),
            inks: INKS.iter().map(|word| word.to_string()).collect(),
        }
    }

    pub fn papers(&self) -> &[String] {
        &self.papers
    }

    pub fn inks(&self) -> &[String] {
        &self.inks
    }

    pub fn accepts_paper(&self, word: &str) -> bool {
        self.papers.iter().any(|entry| entry == word)
    }

    pub fn accepts_ink(&self, word: &str) -> bool {
        self.inks.iter().any(|entry| entry == word)
    }
}

impl Default for Vocabulary {
    fn default() -> Self {
        Vocabulary::declared()
    }
}

/// One run of paper: `cols` cells from `(col, row)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fill {
    pub col: u32,
    pub row: u32,
    pub cols: u32,
    pub paper: String,
}

/// One run of glyphs: from `(col, row)`, one glyph per cell, in one ink.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    pub col: u32,
    pub row: u32,
    pub ink: String,
    pub text: String,
}

/// One field source a map frame declares: a name the renderer knows and the law that answers it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldRef {
    pub name: String,
    pub source: String,
}

/// A map frame's declaration: the fields, the cell law and the camera it is drawn with.
#[derive(Clone, Debug, PartialEq)]
pub struct MapView {
    pub fields: Vec<FieldRef>,
    pub shaper: String,
    pub shaper_cells: u32,
    pub projection: String,
    pub centre_xz: [f64; 2],
    pub cell_m: f64,
    pub extent: [u32; 2],
    pub pitch_deg: f64,
}

impl MapView {
    /// The source declared for `name`, if this frame declares it.
    pub fn source_of(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|field| field.name == name)
            .map(|field| field.source.as_str())
    }
}

/// One frame, as declared. [`CellFrame::parse`] is the only constructor and it checks every field.
#[derive(Clone, Debug, PartialEq)]
pub struct CellFrame {
    /// The frame's id — and the row id the package names it by.
    pub id: String,
    /// The window whose content space this frame addresses.
    pub window: String,
    pub cols: u32,
    pub rows: u32,
    pub order: u32,
    pub requires: Vec<String>,
    /// The world feature this frame shows, if it declares one.
    pub feature: Option<String>,
    /// The sampling declaration, when the frame draws through the map renderer.
    pub map: Option<MapView>,
    pub fills: Vec<Fill>,
    pub runs: Vec<Run>,
    /// The role words this frame was read against.
    pub vocabulary: Vocabulary,
    /// The colour roles this frame's vocabulary section declared, with their RGB.
    ///
    /// A package states its own palette here: `colour fill-0 0 153 0` both names a role the draw
    /// rows may use and gives it the colour the source carried. The grammar stays roles-not-colours
    /// — a document still names a role, never an RGB, in a draw row — and the declared set of
    /// papers widens by exactly the roles this section declared.
    pub colours: Vec<(String, (u8, u8, u8))>,
    /// The sounds this frame carries, payload bytes included, in the order the source declared them.
    /// Empty when the package carries no audio — a frame is not required to have sound.
    pub audio: Vec<SoundClip>,
    /// The visual payloads the package states it does **not** carry, in the order the source named
    /// them. Empty when every character has art. A client reports these; it never fills them in.
    pub absences: Vec<Absence>,
    /// Every file the frame's artifact table carries, kept so the timeline's frames are reachable by
    /// the payload the package ships rather than by a filename the client guessed.
    pub artifacts: Vec<Artifact>,
    /// The frames after this one, in the order a client advances them. Empty for a one-frame package.
    pub timeline: Vec<TimelineFrame>,
    /// The sounds the timeline starts, and on which frame. Empty when the package carries no audio
    /// event — a frame is not required to have sound.
    pub events: Vec<SoundEvent>,
}

fn number_of(text: &str) -> Option<u32> {
    text.parse::<u32>().ok()
}

fn number_f64(text: &str) -> Option<f64> {
    text.parse::<f64>().ok()
}

/// An id or a token: no whitespace, no prose, and not a bare number.
fn identifier(text: &str) -> bool {
    !text.is_empty()
        && !text.chars().any(char::is_whitespace)
        && text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && !text.chars().all(|c| c.is_ascii_digit())
}

/// A law token: `terrain::elevation_m`, never prose and never an expression.
fn law_token(text: &str) -> bool {
    let mut parts = text.split("::");
    match (parts.next(), parts.next(), parts.next()) {
        (Some(module), Some(name), None) => identifier(module) && identifier(name),
        _ => false,
    }
}

impl CellFrame {
    /// **The only constructor that checks.** A wrong header, an unknown or duplicate key, a missing
    /// key, a row outside the grid, a role the streamed vocabulary does not declare, or a frame that
    /// draws nothing refuses here, with every problem named in one pass.
    pub fn parse(source: &str) -> Result<CellFrame, Refusals> {
        let mut problems = Refusals::new();
        if source.len() > MAX_FRAME_BYTES {
            problems.refuse(Refusal::CellframeBound {
                bound: "declaration bytes",
                limit: MAX_FRAME_BYTES as u64,
                found: source.len() as u64,
            });
            return Err(problems);
        }

        let all: Vec<&str> = source.lines().collect();
        let mut vocabulary = Vocabulary::declared();
        let mut colours: Vec<(String, (u8, u8, u8))> = Vec::new();
        let mut artifacts: Vec<Artifact> = Vec::new();
        let mut audio: Vec<SoundClip> = Vec::new();
        let mut absences: Vec<Absence> = Vec::new();
        let mut timeline: Vec<TimelineFrame> = Vec::new();
        let mut events: Vec<SoundEvent> = Vec::new();
        // The artifact table opens the document, the audio references follow it, then what the
        // package states it does not carry, then the vocabulary section: the package's files, then
        // what the frame names, then what it says is absent, then the words it draws in.
        let mut cursor = 0usize;
        if all
            .first()
            .map(|line| line.trim_end_matches('\r'))
            == Some(ARTIFACT_SECTION)
        {
            cursor = match read_artifacts(&all, &mut artifacts, &mut problems) {
                Some(next) => next,
                None => return Err(problems),
            };
        }
        if all
            .get(cursor)
            .map(|line| line.trim_end_matches('\r'))
            == Some(AUDIO_SECTION)
        {
            cursor = match read_audio(&all[cursor..], cursor, &artifacts, &mut audio, &mut problems) {
                Some(next) => next,
                None => return Err(problems),
            };
        }
        if all
            .get(cursor)
            .map(|line| line.trim_end_matches('\r'))
            == Some(ABSENCE_SECTION)
        {
            cursor = match read_absences(&all[cursor..], cursor, &mut absences, &mut problems) {
                Some(next) => next,
                None => return Err(problems),
            };
        }
        if all
            .get(cursor)
            .map(|line| line.trim_end_matches('\r'))
            == Some(TIMELINE_SECTION)
        {
            cursor = match read_timeline(&all[cursor..], cursor, &artifacts, &mut timeline, &mut problems)
            {
                Some(next) => next,
                None => return Err(problems),
            };
        }
        if all
            .get(cursor)
            .map(|line| line.trim_end_matches('\r'))
            == Some(EVENT_SECTION)
        {
            cursor = match read_events(&all[cursor..], cursor, &audio, &mut events, &mut problems) {
                Some(next) => next,
                None => return Err(problems),
            };
        }
        let start = match read_vocabulary(
            &all[cursor..],
            &mut vocabulary,
            &mut colours,
            &mut problems,
        ) {
            Some(start) => start + cursor,
            None => return Err(problems),
        };
        let lines = &all[start..];
        let at = |index: usize| start + index + 1;

        let header = lines.first().copied().unwrap_or_default();
        if header != HEADER {
            problems.refuse(Refusal::CellframeSchema {
                found: header.to_string(),
                supported: HEADER,
            });
            return Err(problems);
        }

        let Some(declaration) = lines.get(1).copied() else {
            problems.refuse(Refusal::CellframeRow {
                line: at(1),
                row: "the declaration ends at the header; a frame needs a `frame<TAB>id` row"
                    .to_string(),
            });
            return Err(problems);
        };
        let fields: Vec<&str> = declaration.trim_end_matches('\r').split('\t').collect();
        let id = match fields.as_slice() {
            ["frame", id] if identifier(id) => id.to_string(),
            _ => {
                problems.refuse(Refusal::CellframeRow {
                    line: at(1),
                    row: format!("expected `frame<TAB>id`, found {declaration:?}"),
                });
                String::new()
            }
        };

        let mut window: Option<String> = None;
        let mut cols: Option<u32> = None;
        let mut rows: Option<u32> = None;
        let mut order: Option<u32> = None;
        let mut requires: Option<Vec<String>> = None;
        let mut feature: Option<String> = None;
        let mut map_declared = false;
        let mut sampled: Vec<FieldRef> = Vec::new();
        let mut shaper: Option<(String, u32)> = None;
        let mut projection: Option<String> = None;
        let mut centre: Option<[f64; 2]> = None;
        let mut cell_m: Option<f64> = None;
        let mut extent: Option<[u32; 2]> = None;
        let mut pitch: Option<f64> = None;
        let mut fills: Vec<Fill> = Vec::new();
        let mut runs: Vec<Run> = Vec::new();

        for (index, line) in lines.iter().enumerate().skip(2) {
            let number = at(index);
            let trimmed = line.trim_end_matches('\r');
            if trimmed.trim().is_empty() || trimmed.trim_start().starts_with('#') {
                continue;
            }
            // A key row is indented, so the line's leading tab is not a field. A cell row is never
            // indented, so trimming the start cannot eat a run's text.
            let cells: Vec<&str> = trimmed.trim_start().split('\t').collect();
            match cells.as_slice() {
                ["fill", col, row, wide, paper] => {
                    match (
                        number_of(col),
                        number_of(row),
                        number_of(wide),
                        vocabulary.accepts_paper(paper),
                    ) {
                        (Some(col), Some(row), Some(wide), true) if wide > 0 => fills.push(Fill {
                            col,
                            row,
                            cols: wide,
                            paper: (*paper).to_string(),
                        }),
                        (Some(_), Some(_), Some(_), false) => {
                            problems.refuse(Refusal::CellframeVocabulary {
                                vocabulary: "paper",
                                found: (*paper).to_string(),
                            })
                        }
                        _ => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!(
                                "`fill` is `fill<TAB>col<TAB>row<TAB>cols<TAB>paper` with a positive \
                                 count and a declared paper role, found {trimmed:?}"
                            ),
                        }),
                    }
                }
                ["run", col, row, ink, text] => {
                    match (
                        number_of(col),
                        number_of(row),
                        vocabulary.accepts_ink(ink),
                    ) {
                        (Some(col), Some(row), true) if !text.is_empty() => runs.push(Run {
                            col,
                            row,
                            ink: (*ink).to_string(),
                            text: (*text).to_string(),
                        }),
                        (Some(_), Some(_), false) => problems.refuse(Refusal::CellframeVocabulary {
                            vocabulary: "ink",
                            found: (*ink).to_string(),
                        }),
                        _ => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!(
                                "`run` is `run<TAB>col<TAB>row<TAB>ink<TAB>text` with a declared ink \
                                 role and text that is not empty, found {trimmed:?}"
                            ),
                        }),
                    }
                }
                ["field", name, law] => {
                    map_declared = true;
                    if !FIELDS.contains(name) {
                        problems.refuse(Refusal::CellframeBinding {
                            line: number,
                            field: format!("field {name} is not one this renderer samples"),
                        })
                    } else if sampled.iter().any(|field| field.name == *name) {
                        problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: format!("field {name}"),
                        })
                    } else if !law_token(law) {
                        problems.refuse(Refusal::CellframeBinding {
                            line: number,
                            field: format!("field {name} takes a law token such as \
                                            `terrain::elevation_m`, found {law:?}"),
                        })
                    } else {
                        sampled.push(FieldRef {
                            name: (*name).to_string(),
                            source: (*law).to_string(),
                        });
                    }
                }
                ["shaper", name, divisions] => {
                    map_declared = true;
                    let divisions = number_of(divisions);
                    if *name != SHAPER {
                        problems.refuse(Refusal::CellframeBinding {
                            line: number,
                            field: format!("shaper {name} is not a cell law this renderer carries"),
                        })
                    } else if shaper.is_some() {
                        problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "shaper".to_string(),
                        })
                    } else if divisions != Some(SHAPER_CELLS) {
                        problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!(
                                "`shaper {SHAPER}` takes {SHAPER_CELLS} divisions per axis, found \
                                 {divisions:?}"
                            ),
                        })
                    } else {
                        shaper = Some(((*name).to_string(), SHAPER_CELLS));
                    }
                }
                ["projection", name] => {
                    map_declared = true;
                    if *name != PROJECTION {
                        problems.refuse(Refusal::CellframeBinding {
                            line: number,
                            field: format!("projection {name} is not a camera this renderer carries"),
                        })
                    } else if projection.is_some() {
                        problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "projection".to_string(),
                        })
                    } else {
                        projection = Some((*name).to_string());
                    }
                }
                ["centre", x, z] => {
                    map_declared = true;
                    match (number_f64(x), number_f64(z)) {
                        (Some(x), Some(z)) if centre.is_none() => centre = Some([x, z]),
                        (Some(_), Some(_)) => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "centre".to_string(),
                        }),
                        _ => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!("`centre` is `centre<TAB>x<TAB>z` in metres, found {trimmed:?}"),
                        }),
                    }
                }
                ["cell", metres] => {
                    map_declared = true;
                    match number_f64(metres) {
                        Some(m) if m > 0.0 && cell_m.is_none() => cell_m = Some(m),
                        Some(_) => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "cell".to_string(),
                        }),
                        None => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!("`cell` takes a positive number of metres, found {metres:?}"),
                        }),
                    }
                }
                ["extent", across, down] => {
                    map_declared = true;
                    match (number_of(across), number_of(down)) {
                        (Some(across), Some(down))
                            if across > 0 && down > 0 && extent.is_none() =>
                        {
                            extent = Some([across, down])
                        }
                        (Some(_), Some(_)) => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "extent".to_string(),
                        }),
                        _ => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!(
                                "`extent` is `extent<TAB>cols<TAB>rows` with whole numbers, found \
                                 {trimmed:?}"
                            ),
                        }),
                    }
                }
                ["pitch", degrees] => {
                    map_declared = true;
                    match number_f64(degrees) {
                        Some(deg) if (-80.0..=-1.0).contains(&deg) && pitch.is_none() => {
                            pitch = Some(deg)
                        }
                        Some(_) => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "pitch".to_string(),
                        }),
                        None => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!("`pitch` takes a downward tilt in degrees, found {degrees:?}"),
                        }),
                    }
                }
                [key, value, ..] => match *key {
                    "window" => match *value {
                        "" => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: "`window` takes a window id".to_string(),
                        }),
                        _ if !identifier(value) => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!("`window` takes a window id, found {value:?}"),
                        }),
                        _ if window.is_some() => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "window".to_string(),
                        }),
                        _ => window = Some((*value).to_string()),
                    },
                    "cols" => match number_of(value) {
                        Some(found) if cols.is_none() => cols = Some(found),
                        Some(_) => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "cols".to_string(),
                        }),
                        None => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!("`cols` takes a whole number, found {value:?}"),
                        }),
                    },
                    "rows" => match number_of(value) {
                        Some(found) if rows.is_none() => rows = Some(found),
                        Some(_) => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "rows".to_string(),
                        }),
                        None => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!("`rows` takes a whole number, found {value:?}"),
                        }),
                    },
                    "order" => match number_of(value) {
                        Some(found) if order.is_none() => order = Some(found),
                        Some(_) => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "order".to_string(),
                        }),
                        None => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!("`order` takes a whole number, found {value:?}"),
                        }),
                    },
                    "requires" => {
                        if requires.is_some() {
                            problems.refuse(Refusal::CellframeDuplicate {
                                line: number,
                                key: "requires".to_string(),
                            });
                        } else if *value == "none" {
                            requires = Some(Vec::new());
                        } else if value.is_empty()
                            || value.split(',').any(|entry| !identifier(entry))
                        {
                            problems.refuse(Refusal::CellframeRow {
                                line: number,
                                row: format!(
                                    "`requires` takes `none` or frame ids, found {value:?}"
                                ),
                            });
                        } else {
                            let list: Vec<String> =
                                value.split(',').map(|entry| entry.to_string()).collect();
                            if list.iter().any(|entry| entry == &id) {
                                problems.refuse(Refusal::CellframeRow {
                                    line: number,
                                    row: "`requires` names the frame itself".to_string(),
                                });
                            } else {
                                requires = Some(list);
                            }
                        }
                    }
                    "feature" => match feature {
                        Some(_) => problems.refuse(Refusal::CellframeDuplicate {
                            line: number,
                            key: "feature".to_string(),
                        }),
                        None if !identifier(value) => problems.refuse(Refusal::CellframeRow {
                            line: number,
                            row: format!("`feature` takes a feature token, found {value:?}"),
                        }),
                        None => feature = Some((*value).to_string()),
                    },
                    other => problems.refuse(Refusal::CellframeKey {
                        line: number,
                        key: other.to_string(),
                    }),
                },
                [key] => problems.refuse(Refusal::CellframeRow {
                    line: number,
                    row: format!("`{key}` carries no value"),
                }),
                [] => {}
            }
        }

        // Every required key is named if it is absent, in one pass.
        if window.is_none() {
            problems.refuse(Refusal::CellframeMissing {
                key: "window".to_string(),
            });
        }
        for (key, present) in [
            ("cols", cols.is_some()),
            ("rows", rows.is_some()),
            ("order", order.is_some()),
            ("requires", requires.is_some()),
        ] {
            if !present {
                problems.refuse(Refusal::CellframeMissing {
                    key: key.to_string(),
                });
            }
        }

        if let Some(cols) = cols {
            if cols == 0 || cols > MAX_COLS {
                problems.refuse(Refusal::CellframeBound {
                    bound: "cols",
                    limit: MAX_COLS as u64,
                    found: cols as u64,
                });
            }
        }
        if let Some(rows) = rows {
            if rows == 0 || rows > MAX_ROWS {
                problems.refuse(Refusal::CellframeBound {
                    bound: "rows",
                    limit: MAX_ROWS as u64,
                    found: rows as u64,
                });
            }
        }
        if let (Some(cols), Some(rows)) = (cols, rows) {
            let cells = u64::from(cols) * u64::from(rows);
            if cells > u64::from(MAX_CELLS) {
                problems.refuse(Refusal::CellframeBound {
                    bound: "cells",
                    limit: u64::from(MAX_CELLS),
                    found: cells,
                });
            }
            for fill in &fills {
                if fill.col >= cols
                    || fill.row >= rows
                    || u64::from(fill.col) + u64::from(fill.cols) > u64::from(cols)
                {
                    problems.refuse(Refusal::CellframeGrid {
                        row: format!(
                            "fill at ({}, {}) for {} cells does not fit a {} by {} grid",
                            fill.col, fill.row, fill.cols, cols, rows
                        ),
                    });
                }
            }
            for run in &runs {
                let width = run.text.chars().count() as u32;
                if run.col >= cols
                    || run.row >= rows
                    || u64::from(run.col) + u64::from(width) > u64::from(cols)
                {
                    problems.refuse(Refusal::CellframeGrid {
                        row: format!(
                            "run at ({}, {}) for {} cells does not fit a {} by {} grid",
                            run.col,
                            run.row,
                            width,
                            cols,
                            rows
                        ),
                    });
                }
            }
        }

        // **A map is one declaration**: any sampling row present means all of them must be, or the
        // frame would be drawn as glyphs while claiming to be a map.
        let map = if map_declared {
            for (key, present) in [
                ("shaper", shaper.is_some()),
                ("projection", projection.is_some()),
                ("centre", centre.is_some()),
                ("cell", cell_m.is_some()),
                ("extent", extent.is_some()),
                ("pitch", pitch.is_some()),
            ] {
                if !present {
                    problems.refuse(Refusal::CellframeMissing {
                        key: key.to_string(),
                    });
                }
            }
            if sampled.is_empty() {
                problems.refuse(Refusal::CellframeMissing {
                    key: "field".to_string(),
                });
            }
            match (shaper, projection, centre, cell_m, extent, pitch) {
                (Some((name, cells)), Some(projection), Some(centre), Some(cell_m), Some(extent), Some(pitch)) => {
                    Some(MapView {
                        fields: sampled,
                        shaper: name,
                        shaper_cells: cells,
                        projection,
                        centre_xz: centre,
                        cell_m,
                        extent,
                        pitch_deg: pitch,
                    })
                }
                _ => None,
            }
        } else {
            None
        };

        // **A declaration that draws nothing is a window erased**, so it is refused by name.
        if fills.is_empty() && runs.is_empty() {
            problems.refuse(Refusal::CellframeEmpty {
                id: id.clone(),
            });
        }

        if !problems.is_empty() {
            return Err(problems);
        }

        Ok(CellFrame {
            id,
            window: window.unwrap_or_default(),
            cols: cols.unwrap_or(0),
            rows: rows.unwrap_or(0),
            order: order.unwrap_or(0),
            requires: requires.unwrap_or_default(),
            feature,
            map,
            fills,
            runs,
            vocabulary,
            colours,
            audio,
            absences,
            artifacts,
            timeline,
            events,
        })
    }
}

/// Read the streamed audio section: one row per sound, payload bytes as hex.
///
/// ```text
/// [audio]
/// sound<TAB>1<TAB>mp3<TAB>11025<TAB>1784<TAB>7d06ffe3...
/// stream<TAB>-<TAB>mp3<TAB>11025<TAB>0<TAB>7d06ffe3...
/// ```
///
/// An empty payload, an odd-length hex run, a duplicate id or a row that is not one of those five
/// fields is refused; a sound the package cannot carry whole is not carried at all.
/// Read the artifact table: `path<TAB>digest<TAB>hex`, family B's ceilings and path rules.
fn read_artifacts(
    lines: &[&str],
    artifacts: &mut Vec<Artifact>,
    problems: &mut Refusals,
) -> Option<usize> {
    let mut index = 1;
    while index < lines.len() {
        let trimmed = lines[index].trim_end_matches('\r');
        if trimmed.trim().is_empty() {
            break;
        }
        let fields: Vec<&str> = trimmed.split('\t').collect();
        match fields.as_slice() {
            [path, digest, hex] => match (artifact_path_is_well_formed(path), decode_hex(hex)) {
                (true, Some(bytes)) if !bytes.is_empty() => {
                    if artifacts.len() >= ARTIFACT_COUNT_MAX {
                        problems.refuse(Refusal::CellframeBound {
                            bound: "artifacts",
                            limit: ARTIFACT_COUNT_MAX as u64,
                            found: artifacts.len() as u64 + 1,
                        });
                    } else if bytes.len() > ARTIFACT_BYTES_MAX {
                        problems.refuse(Refusal::CellframeBound {
                            bound: "artifact bytes",
                            limit: ARTIFACT_BYTES_MAX as u64,
                            found: bytes.len() as u64,
                        });
                    } else if audio_token(digest) && payload_digest(&bytes) != *digest {
                        problems.refuse(Refusal::CellframeRow {
                            line: index + 1,
                            row: format!(
                                "artifact-digest: {path} states digest {digest}, but its bytes hash to \
                                 {}",
                                payload_digest(&bytes)
                            ),
                        });
                    } else if artifacts.iter().any(|artifact| artifact.path == *path) {
                        problems.refuse(Refusal::CellframeDuplicate {
                            line: index + 1,
                            key: format!("artifact {path}"),
                        });
                    } else {
                        artifacts.push(Artifact {
                            path: (*path).to_string(),
                            digest: (*digest).to_string(),
                            bytes,
                        });
                    }
                }
                _ => problems.refuse(Refusal::CellframeRow {
                    line: index + 1,
                    row: format!(
                        "an artifact row is `path<TAB>digest<TAB>hex` with a package-relative path, \
                         a non-empty even-length payload and its digest, found {trimmed:?}"
                    ),
                }),
            },
            _ => problems.refuse(Refusal::CellframeRow {
                line: index + 1,
                row: format!(
                    "an artifact row is `path<TAB>digest<TAB>hex`, found {trimmed:?}"
                ),
            }),
        }
        index += 1;
    }
    if index == lines.len() {
        problems.refuse(Refusal::CellframeRow {
            line: index + 1,
            row: "the artifact section is not terminated by a blank line".to_string(),
        });
        return None;
    }
    Some(index + 1)
}

fn read_audio(
    lines: &[&str],
    offset: usize,
    artifacts: &[Artifact],
    audio: &mut Vec<SoundClip>,
    problems: &mut Refusals,
) -> Option<usize> {
    let mut index = 1;
    while index < lines.len() {
        let trimmed = lines[index].trim_end_matches('\r');
        if trimmed.trim().is_empty() {
            break;
        }
        let fields: Vec<&str> = trimmed.split('\t').collect();
        match fields.as_slice() {
            [kind @ ("sound" | "stream"), id, path, format, rate, samples] => {
                match (
                    audio_token(id),
                    artifact_path_is_well_formed(path),
                    audio_token(format),
                    rate.parse::<u32>(),
                    samples.parse::<u32>(),
                    artifacts.iter().find(|artifact| artifact.path == *path),
                ) {
                    (true, true, true, Ok(rate), Ok(samples), Some(artifact)) => {
                        if audio.iter().any(|clip| clip.kind == *kind && clip.id == *id) {
                            problems.refuse(Refusal::CellframeDuplicate {
                                line: offset + index + 1,
                                key: format!("audio {kind} {id}"),
                            });
                        } else {
                            audio.push(SoundClip {
                                kind: (*kind).to_string(),
                                id: (*id).to_string(),
                                artifact: (*path).to_string(),
                                format: (*format).to_string(),
                                rate,
                                samples,
                                bytes: artifact.bytes.clone(),
                            });
                        }
                    }
                    (_, true, _, _, _, None) => problems.refuse(Refusal::CellframeRow {
                        line: offset + index + 1,
                        row: format!(
                            "artifact-reference: {kind} {id} names {path:?}, which the artifact \
                             table does not carry"
                        ),
                    }),
                    _ => problems.refuse(Refusal::CellframeRow {
                        line: offset + index + 1,
                        row: format!(
                            "an audio row is `sound|stream<TAB>id<TAB>artifact-path<TAB>format<TAB>rate<TAB>samples` \
                             naming a carried artifact, found {trimmed:?}"
                        ),
                    }),
                }
            }
            _ => problems.refuse(Refusal::CellframeRow {
                line: offset + index + 1,
                row: format!(
                    "an audio row is `sound|stream<TAB>id<TAB>artifact-path<TAB>format<TAB>rate<TAB>samples`, \
                     found {trimmed:?}"
                ),
            }),
        }
        index += 1;
    }
    if index == lines.len() {
        problems.refuse(Refusal::CellframeRow {
            line: offset + index + 1,
            row: "the audio section is not terminated by a blank line".to_string(),
        });
        return None;
    }
    Some(offset + index + 1)
}

/// Read the streamed absence section: one row per visual payload the package does not carry.
///
/// ```text
/// [absence]
/// sprite<TAB>371<TAB>no-display-list<TAB>cellframe::malformed-record
/// ```
///
/// The row says what is missing (`kind` and `id`), why (`reason`, one token so a reader can group by
/// cause) and which owning rule makes the statement the right answer (`rule`, a law token). A kind
/// outside [`ABSENCE_KINDS`], an empty reason, a rule that is not a law token, or a second row for
/// the same character is refused: an absence is a declaration, and a declaration gets the same
/// checking as a `fill`.
fn read_absences(
    lines: &[&str],
    offset: usize,
    absences: &mut Vec<Absence>,
    problems: &mut Refusals,
) -> Option<usize> {
    let mut index = 1;
    while index < lines.len() {
        let trimmed = lines[index].trim_end_matches('\r');
        if trimmed.trim().is_empty() {
            break;
        }
        let fields: Vec<&str> = trimmed.split('\t').collect();
        match fields.as_slice() {
            [kind, id, reason, rule]
                if ABSENCE_KINDS.contains(kind)
                    && audio_token(id)
                    && audio_token(reason)
                    && law_token(rule) =>
            {
                if absences
                    .iter()
                    .any(|row| row.kind == *kind && row.id == *id)
                {
                    problems.refuse(Refusal::CellframeDuplicate {
                        line: offset + index + 1,
                        key: format!("absence {kind} {id}"),
                    });
                } else {
                    absences.push(Absence {
                        kind: (*kind).to_string(),
                        id: (*id).to_string(),
                        reason: (*reason).to_string(),
                        rule: (*rule).to_string(),
                    });
                }
            }
            [kind, _, _, _] if !ABSENCE_KINDS.contains(kind) => {
                problems.refuse(Refusal::CellframeRow {
                    line: offset + index + 1,
                    row: format!(
                        "an absence names a kind in {ABSENCE_KINDS:?}, found {kind:?}"
                    ),
                })
            }
            _ => problems.refuse(Refusal::CellframeRow {
                line: offset + index + 1,
                row: format!(
                    "an absence row is `sprite|shape|cell<TAB>id<TAB>reason<TAB>module::rule` with \
                     a one-token reason and a law token for a rule, found {trimmed:?}"
                ),
            }),
        }
        index += 1;
    }
    if index == lines.len() {
        problems.refuse(Refusal::CellframeRow {
            line: offset + index + 1,
            row: "the absence section is not terminated by a blank line".to_string(),
        });
        return None;
    }
    // Family B refuses a heading with nothing under it: an absence section with no row states
    // nothing, and a statement that states nothing is not a statement.
    if absences.is_empty() {
        problems.refuse(Refusal::CellframeRow {
            line: offset + 1,
            row: "the absence section carries no row; a section that states nothing is not stated"
                .to_string(),
        });
        return None;
    }
    Some(offset + index + 1)
}

/// Read the streamed timeline: one row per frame after the bound one.
///
/// ```text
/// [timeline]
/// frame<TAB>2<TAB>frames/STAGE-0002.frame
/// ```
///
/// The row names a carried artifact, because a frame is a sibling artifact the package names by id —
/// `voxel-client`'s mount holds them in `frame_artifacts` for exactly this. A path the artifact table
/// does not carry, a duplicate order, or a section with no row is refused: a timeline that names a
/// frame it does not carry would advance into nothing.
fn read_timeline(
    lines: &[&str],
    offset: usize,
    artifacts: &[Artifact],
    timeline: &mut Vec<TimelineFrame>,
    problems: &mut Refusals,
) -> Option<usize> {
    let mut index = 1;
    while index < lines.len() {
        let trimmed = lines[index].trim_end_matches('\r');
        if trimmed.trim().is_empty() {
            break;
        }
        let fields: Vec<&str> = trimmed.split('\t').collect();
        match fields.as_slice() {
            ["frame", order, path] => match (
                order.parse::<u32>(),
                artifact_path_is_well_formed(path),
                artifacts.iter().find(|artifact| artifact.path == *path),
            ) {
                (Ok(order), true, Some(_)) => {
                    if timeline.iter().any(|frame| frame.order == order) {
                        problems.refuse(Refusal::CellframeDuplicate {
                            line: offset + index + 1,
                            key: format!("timeline order {order}"),
                        });
                    } else {
                        timeline.push(TimelineFrame {
                            order,
                            artifact: (*path).to_string(),
                        });
                    }
                }
                (_, true, None) => problems.refuse(Refusal::CellframeRow {
                    line: offset + index + 1,
                    row: format!(
                        "artifact-reference: timeline frame {order} names {path:?}, which the \
                         artifact table does not carry"
                    ),
                }),
                _ => problems.refuse(Refusal::CellframeRow {
                    line: offset + index + 1,
                    row: format!(
                        "a timeline row is `frame<TAB>order<TAB>artifact-path` naming a carried \
                         frame document, found {trimmed:?}"
                    ),
                }),
            },
            _ => problems.refuse(Refusal::CellframeRow {
                line: offset + index + 1,
                row: format!(
                    "a timeline row is `frame<TAB>order<TAB>artifact-path`, found {trimmed:?}"
                ),
            }),
        }
        index += 1;
    }
    if index == lines.len() {
        problems.refuse(Refusal::CellframeRow {
            line: offset + index + 1,
            row: "the timeline section is not terminated by a blank line".to_string(),
        });
        return None;
    }
    if timeline.is_empty() {
        problems.refuse(Refusal::CellframeRow {
            line: offset + 1,
            row: "the timeline section carries no frame; a section that states nothing is not stated"
                .to_string(),
        });
        return None;
    }
    Some(offset + index + 1)
}

/// Read the sound-event list: `sound<TAB>frame<TAB>audio-id`, one row per start.
///
/// A row naming a sound the audio section does not carry is refused, because an event that points at
/// nothing is a sound the client can never find; a duplicate row is refused for the same reason one
/// audio row per character is.
fn read_events(
    lines: &[&str],
    offset: usize,
    audio: &[SoundClip],
    events: &mut Vec<SoundEvent>,
    problems: &mut Refusals,
) -> Option<usize> {
    let mut index = 1;
    while index < lines.len() {
        let trimmed = lines[index].trim_end_matches('\r');
        if trimmed.trim().is_empty() {
            break;
        }
        let fields: Vec<&str> = trimmed.split('\t').collect();
        match fields.as_slice() {
            ["sound", frame, sound]
                if frame.parse::<u32>().map(|value| value > 0).unwrap_or(false)
                    && audio_token(sound)
                    && audio.iter().any(|clip| clip.id == *sound) =>
            {
                let frame = frame.parse::<u32>().expect("checked above");
                if events
                    .iter()
                    .any(|event| event.frame == frame && event.sound == *sound)
                {
                    problems.refuse(Refusal::CellframeDuplicate {
                        line: offset + index + 1,
                        key: format!("event {frame} {sound}"),
                    });
                } else {
                    events.push(SoundEvent {
                        frame,
                        sound: (*sound).to_string(),
                    });
                }
            }
            ["sound", frame, sound] => problems.refuse(Refusal::CellframeRow {
                line: offset + index + 1,
                row: format!(
                    "an event names a carried sound on a frame of the timeline: \
                     `sound<TAB>frame<TAB>audio-id`, found frame {frame:?} sound {sound:?}"
                ),
            }),
            _ => problems.refuse(Refusal::CellframeRow {
                line: offset + index + 1,
                row: format!("an event row is `sound<TAB>frame<TAB>audio-id`, found {trimmed:?}"),
            }),
        }
        index += 1;
    }
    if index == lines.len() {
        problems.refuse(Refusal::CellframeRow {
            line: offset + index + 1,
            row: "the event section is not terminated by a blank line".to_string(),
        });
        return None;
    }
    if events.is_empty() {
        problems.refuse(Refusal::CellframeRow {
            line: offset + 1,
            row: "the event section carries no event; a section that states nothing is not stated"
                .to_string(),
        });
        return None;
    }
    Some(offset + index + 1)
}

/// Read the streamed vocabulary section when the declaration opens with one.
///
/// `Ok(Some(start))` is the line the frame grammar begins on; `None` means the section itself was
/// refused. A declaration that does not open with the section begins at line 0 and gets the format's
/// declared roles.
fn read_vocabulary(
    lines: &[&str],
    vocabulary: &mut Vocabulary,
    colours: &mut Vec<(String, (u8, u8, u8))>,
    problems: &mut Refusals,
) -> Option<usize> {
    let first = lines.first().copied().unwrap_or_default().trim_end_matches('\r');
    if first != VOCABULARY_SECTION {
        return Some(0);
    }
    let mut papers: Option<Vec<String>> = None;
    let mut inks: Option<Vec<String>> = None;
    let mut index = 1;
    while index < lines.len() {
        let trimmed = lines[index].trim_end_matches('\r');
        if trimmed.trim().is_empty() {
            break;
        }
        let fields: Vec<&str> = trimmed.trim().split('\t').collect();
        match fields.as_slice() {
            // A package's own colour role: the name the draw rows use, and the colour it is.
            ["colour", role, red, green, blue] => {
                match (
                    identifier(role),
                    red.parse::<u8>(),
                    green.parse::<u8>(),
                    blue.parse::<u8>(),
                ) {
                    (true, Ok(red), Ok(green), Ok(blue)) => {
                        if colours.iter().any(|(name, _)| name == role) {
                            problems.refuse(Refusal::CellframeDuplicate {
                                line: index + 1,
                                key: format!("colour {role}"),
                            });
                        } else {
                            colours.push((role.to_string(), (red, green, blue)));
                        }
                    }
                    _ => problems.refuse(Refusal::CellframeRow {
                        line: index + 1,
                        row: format!(
                            "a colour row is `colour<TAB>role<TAB>r<TAB>g<TAB>b` with channels \
                             0-255, found {trimmed:?}"
                        ),
                    }),
                }
            }
            [kind, list] => {
                let words: Vec<String> = list.split(',').map(|word| word.to_string()).collect();
                if words.iter().any(|word| !identifier(word)) {
                    problems.refuse(Refusal::CellframeRow {
                        line: index + 1,
                        row: format!("`{kind}` declares a role word that is not a token: {list:?}"),
                    });
                } else {
                    match *kind {
                        "paper" if papers.is_some() => problems.refuse(Refusal::CellframeDuplicate {
                            line: index + 1,
                            key: "vocabulary paper".to_string(),
                        }),
                        "paper" => papers = Some(words),
                        "ink" if inks.is_some() => problems.refuse(Refusal::CellframeDuplicate {
                            line: index + 1,
                            key: "vocabulary ink".to_string(),
                        }),
                        "ink" => inks = Some(words),
                        other => problems.refuse(Refusal::CellframeKey {
                            line: index + 1,
                            key: format!("vocabulary {other}"),
                        }),
                    }
                }
            }
            _ => problems.refuse(Refusal::CellframeRow {
                line: index + 1,
                row: format!("a vocabulary row is `paper<TAB>words` or `ink<TAB>words`, found {trimmed:?}"),
            }),
        }
        index += 1;
    }
    if index == lines.len() {
        problems.refuse(Refusal::CellframeRow {
            line: index + 1,
            row: "the vocabulary section is not terminated by a blank line".to_string(),
        });
        return None;
    }
    // A declared colour is a paper role: the draw rows that name it are `fill` rows, and the set a
    // section replaces therefore has to carry it.
    if !colours.is_empty() {
        let mut list = papers.unwrap_or_else(|| Vocabulary::declared().papers.clone());
        for (role, _) in colours.iter() {
            if !list.iter().any(|name| name == role) {
                list.push(role.clone());
            }
        }
        papers = Some(list);
    }

    match (papers, inks) {
        (None, None) => problems.refuse(Refusal::CellframeMissing {
            key: "vocabulary paper or ink".to_string(),
        }),
        (papers, inks) => {
            let declared = Vocabulary::declared();
            *vocabulary = Vocabulary {
                papers: papers.unwrap_or_else(|| declared.papers.clone()),
                inks: inks.unwrap_or_else(|| declared.inks.clone()),
            };
        }
    }
    if problems.is_empty() {
        Some(index + 1)
    } else {
        None
    }
}


/// A frame resolved to the window whose declaration binds it.
#[derive(Clone, Debug, PartialEq)]
pub struct Bound {
    /// The window the declaration binds this frame to.
    pub window: String,
    /// The one-based line of the `frame` row that bound it.
    pub line: usize,
    /// The frame, read through this module's own grammar.
    pub frame: CellFrame,
}

/// A package that loaded: its mount receipt and the frame each declaration resolves to.
#[derive(Clone, Debug, PartialEq)]
pub struct Loaded {
    pub receipt: crate::mount::Receipt,
    /// The surface the declaration parsed into: the one the mount is for.
    pub surface: Surface,
    pub frames: Vec<Bound>,
}

/// **Resolve every frame a package declares against the declarations that name them.**
///
/// `frames` is the package's frame artifacts, in the order the package lists them. Each is read
/// through [`CellFrame::parse`] first, so a malformed frame is refused by its own grammar rather than
/// by its reference. Then:
///
/// * a declaration naming an id the package does not declare is refused `cellframe-reference` with
///   the line of the `frame` row — a reference that resolves to nothing is a window with no cells;
/// * a frame no declaration names is refused `cellframe-unreferenced` — an unreachable frame looks
///   mounted and is not;
/// * a frame whose extent disagrees with its window's content space is refused
///   `cellframe-extent` with the binding line, in the same pass as the reference, so a mismatched
///   binding never loads;
/// * the frame a window loads with is the frame its id names and nothing else, which is what each
///   returned [`Bound`] records.
pub fn resolve(surface: &Surface, frames: &[&str]) -> Result<Vec<Bound>, Refusals> {
    let mut problems = Refusals::new();
    let mut parsed: Vec<CellFrame> = Vec::new();
    for source in frames {
        match CellFrame::parse(source) {
            Ok(frame) => parsed.push(frame),
            Err(refusals) => {
                for refusal in refusals.problems() {
                    problems.refuse(refusal.clone());
                }
            }
        }
    }

    let mut bound: Vec<Bound> = Vec::new();
    let mut referenced: Vec<String> = Vec::new();
    for window in &surface.windows {
        let Some(reference) = &window.frame else {
            continue;
        };
        match parsed.iter().find(|frame| frame.id == reference.id) {
            Some(frame) => {
                referenced.push(frame.id.clone());
                let content = [
                    u32::try_from(window.w).unwrap_or(u32::MAX),
                    u32::try_from(window.h).unwrap_or(u32::MAX),
                ];
                match check_extent(frame, &window.id, content, reference.line) {
                    Ok(()) => {
                        bound.push(Bound {
                            window: window.id.clone(),
                            line: reference.line,
                            frame: frame.clone(),
                        });
                        // **The timeline the bound frame carries.** Each `[timeline]` row names an
                        // artifact the table holds, and each artifact is a frame document this
                        // grammar reads — so the later frames are resolved here, beside the bound
                        // one, instead of being an unreferenced frame the mount refuses. They bind
                        // to the same window, in the order the package states.
                        for entry in &frame.timeline {
                            let Some(artifact) = frame
                                .artifacts
                                .iter()
                                .find(|artifact| artifact.path == entry.artifact)
                            else {
                                continue;
                            };
                            let Ok(text) = std::str::from_utf8(&artifact.bytes) else {
                                problems.refuse(Refusal::CellframeRow {
                                    line: reference.line,
                                    row: format!(
                                        "timeline artifact {} is not a text frame document",
                                        entry.artifact
                                    ),
                                });
                                continue;
                            };
                            match CellFrame::parse(text) {
                                Ok(nested) => {
                                    referenced.push(nested.id.clone());
                                    match check_extent(&nested, &window.id, content, reference.line)
                                    {
                                        Ok(()) => bound.push(Bound {
                                            window: window.id.clone(),
                                            line: reference.line,
                                            frame: nested,
                                        }),
                                        Err(refusal) => problems.refuse(refusal),
                                    }
                                }
                                Err(refusals) => {
                                    for refusal in refusals.problems() {
                                        problems.refuse(refusal.clone());
                                    }
                                }
                            }
                        }
                    }
                    Err(refusal) => problems.refuse(refusal),
                }
            }
            None => problems.refuse(Refusal::CellframeReference {
                line: reference.line,
                window: window.id.clone(),
                frame: reference.id.clone(),
            }),
        }
    }

    for frame in &parsed {
        if !referenced.iter().any(|id| id == &frame.id) {
            problems.refuse(Refusal::CellframeUnreferenced {
                frame: frame.id.clone(),
            });
        }
    }

    if problems.is_empty() {
        Ok(bound)
    } else {
        Err(problems)
    }
}

/// **Load a package: mount the declaration, then resolve its frames.**
///
/// The reference is resolved here rather than deferred to a renderer, so a package whose declaration
/// names a frame it does not declare is refused at load. The mount receipt is the surface's own, and
/// the frames are the ones each declaration named.
#[allow(clippy::too_many_arguments)]
pub fn load(
    frame: &crate::frame::Frame,
    surface: &str,
    documents: &[&str],
    sequence: u64,
    previous_sequence: u64,
    at: u64,
) -> Result<Loaded, Refusals> {
    let loaded = load_declared(frame, documents, sequence, previous_sequence, at)?;
    if loaded.surface.id != surface {
        let mut problems = Refusals::new();
        problems.refuse(Refusal::SurfaceSchema {
            found: format!("surface {:?}", loaded.surface.id),
        });
        return Err(problems);
    }
    Ok(loaded)
}

/// **Load a declaration that names its own surface.**
///
/// The same load as [`load`], for a caller that holds the package and not the surface's id: the id is
/// written in the declaration, which is the only place it belongs. A client uses this so it never
/// parses a declaration once to learn an id and again to mount it.
pub fn load_declared(
    frame: &crate::frame::Frame,
    documents: &[&str],
    sequence: u64,
    previous_sequence: u64,
    at: u64,
) -> Result<Loaded, Refusals> {
    let mut problems = Refusals::new();
    let body = match Surface::parse(&frame.payload) {
        Ok(body) => body,
        Err(refusal) => {
            problems.refuse(refusal);
            return Err(problems);
        }
    };
    let receipt = match crate::mount::mount(frame, &body.id, sequence, previous_sequence, at) {
        Ok(receipt) => receipt,
        Err(refusal) => {
            problems.refuse(refusal);
            return Err(problems);
        }
    };
    let frames = resolve(&body, documents)?;
    Ok(Loaded {
        receipt,
        surface: body,
        frames,
    })
}

impl Loaded {
    /// Mount the loaded package into the surface it declares, consuming these bindings.
    pub fn mount(&self) -> Result<SurfaceMount, Refusals> {
        mount_loaded(self, &self.surface)
    }
}

/// One window of a mounted surface: the frame its declaration bound, already validated.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowMount {
    /// The window the declaration binds this frame to.
    pub window: String,
    /// The one-based line of the `frame` row that bound it.
    pub line: usize,
    /// The frame resolved and extent-checked at load.
    pub frame: CellFrame,
}

/// **A package mounted into one surface**: the surface's receipt and the frame each window carries.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceMount {
    /// The mount receipt the surface produced, unchanged by the frame work.
    pub receipt: crate::mount::Receipt,
    /// The bound windows, in declaration order.
    pub windows: Vec<WindowMount>,
}

impl SurfaceMount {
    /// The frame a window carries, or `None` when its declaration bound none.
    ///
    /// The bound frame is bound first, so this is the frame the declaration names — the timeline's
    /// later frames follow it in `order`.
    pub fn frame_of(&self, window: &str) -> Option<&CellFrame> {
        self.windows
            .iter()
            .find(|entry| entry.window == window)
            .map(|entry| &entry.frame)
    }

    /// **Every frame the window carries, in the order the package advances them.**
    ///
    /// The bound frame, then each `[timeline]` artifact `resolve` read for it, sorted by the frame's
    /// own `order`. A window with no frame carries none, and a one-frame package answers one.
    pub fn frames_of(&self, window: &str) -> Vec<&CellFrame> {
        let mut frames: Vec<&CellFrame> = self
            .windows
            .iter()
            .filter(|entry| entry.window == window)
            .map(|entry| &entry.frame)
            .collect();
        frames.sort_by_key(|frame| frame.order);
        frames
    }
}

/// **Mount a loaded package into the surface it was loaded for.**
///
/// The bindings arrive resolved and extent-checked, so this consumes them as they are: nothing is
/// re-resolved, nothing is re-measured, and nothing is drawn. The one question the mount asks of its
/// own pairing is whether every bound window is a window this surface declares — otherwise the mount
/// would carry a frame addressed at a window that does not exist. A refusal names the window, the
/// frame and the line of the binding row that made the claim.
pub fn mount_loaded(loaded: &Loaded, surface: &Surface) -> Result<SurfaceMount, Refusals> {
    let mut problems = Refusals::new();
    let mut windows: Vec<WindowMount> = Vec::new();
    for entry in &loaded.frames {
        if !surface
            .windows
            .iter()
            .any(|window| window.id == entry.window)
        {
            problems.refuse(Refusal::CellframeSurface {
                line: entry.line,
                window: entry.window.clone(),
                frame: entry.frame.id.clone(),
            });
            continue;
        }
        windows.push(WindowMount {
            window: entry.window.clone(),
            line: entry.line,
            frame: entry.frame.clone(),
        });
    }
    if problems.is_empty() {
        Ok(SurfaceMount {
            receipt: loaded.receipt.clone(),
            windows,
        })
    } else {
        Err(problems)
    }
}


/// **A frame is addressed to one window's content space, and the two extents agree.**
///
/// The owning reader refuses a frame that does not fit its window by cell count rather than drawing
/// through the window's chrome. This port enforces the same property as an agreement, because a frame
/// smaller than its content space leaves declared cells undrawn and that erased region must not be
/// implicit. `line` is the binding row, so the refusal names the row that made the claim.
pub fn check_extent(
    frame: &CellFrame,
    window: &str,
    content: [u32; 2],
    line: usize,
) -> Result<(), Refusal> {
    if frame.window != window || frame.cols != content[0] || frame.rows != content[1] {
        return Err(Refusal::CellframeExtent {
            line,
            frame: frame.id.clone(),
            window: window.to_string(),
            found: [frame.cols, frame.rows],
            allowed: content,
        });
    }
    Ok(())
}
