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
//! A 512 KiB declaration, a 512 by 512 grid and 16 384 cells, plus the format's own cell size. Each
//! is a refusal, not a truncation.

use crate::frame::Refusal;

/// The pin. A different grammar is a different file, and an older one is refused rather than read.
pub const HEADER: &str = "shploingus.cellframe/1";
/// One frame's ceiling: a declaration, not a truncation.
pub const MAX_FRAME_BYTES: usize = 512 * 1024;
/// The widest a frame may declare.
pub const MAX_COLS: u32 = 512;
/// The tallest a frame may declare.
pub const MAX_ROWS: u32 = 512;
/// The most cells one frame may hold.
pub const MAX_CELLS: u32 = 16_384;
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
        let start = match read_vocabulary(&all, &mut vocabulary, &mut problems) {
            Some(start) => start,
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
        })
    }
}

/// Read the streamed vocabulary section when the declaration opens with one.
///
/// `Ok(Some(start))` is the line the frame grammar begins on; `None` means the section itself was
/// refused. A declaration that does not open with the section begins at line 0 and gets the format's
/// declared roles.
fn read_vocabulary(
    lines: &[&str],
    vocabulary: &mut Vocabulary,
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

/// **A frame is addressed to one window's content space**, so it must fit it.
///
/// The owning reader asks the window for its extent and refuses a frame that does not fit by cell
/// count rather than drawing through the window's chrome. This port has the window's extent in the
/// declaration (`surface::Window::w` and `h`), so the caller passes it and the answer is the format's.
pub fn check_extent(frame: &CellFrame, window: &str, content: [u32; 2]) -> Result<(), Refusal> {
    if frame.window != window {
        return Err(Refusal::CellframeExtent {
            frame: frame.id.clone(),
            window: window.to_string(),
            found: [frame.cols, frame.rows],
            allowed: content,
        });
    }
    if frame.cols > content[0] || frame.rows > content[1] {
        return Err(Refusal::CellframeExtent {
            frame: frame.id.clone(),
            window: window.to_string(),
            found: [frame.cols, frame.rows],
            allowed: content,
        });
    }
    Ok(())
}
