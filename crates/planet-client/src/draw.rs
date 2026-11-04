//! The drawn picture: a mounted frame turned into cells, each in a resolved colour.
//!
//! The input is the [`Adopted`] a package already produced — resolved, extent-checked, mounted — so
//! nothing here parses a frame document, a binding or a surface, and nothing re-measures an extent.
//! The output is a raster of typed cells: a position, a paper colour, and optionally a glyph with its
//! ink colour. No `String` role survives into it, which is what "resolved through the palette" means.
//!
//! A role the palette does not carry is a refusal (`cellframe-role`), naming the window, the frame and
//! the line of the binding row, with no raster produced.

use planet_package::{Refusal, Refusals};

use crate::cell_mount::Adopted;
use crate::palette::{Palette, Rgba};

/// One cell of a drawn frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    /// The paper under the cell, already resolved.
    pub paper: Rgba,
    /// The ink of the glyph, when the cell carries one.
    pub ink: Option<Rgba>,
    /// One glyph per cell.
    pub glyph: Option<char>,
}

/// One frame, drawn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raster {
    pub cols: u32,
    pub rows: u32,
    /// Row-major, `cols * rows` cells.
    pub cells: Vec<Cell>,
}

impl Raster {
    /// The cell at `(x, y)`, if the position is inside the grid.
    pub fn cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if x >= self.cols || y >= self.rows {
            return None;
        }
        self.cells.get((y * self.cols + x) as usize)
    }

    /// The glyphs of one row, left to right, with the unpainted positions skipped.
    pub fn row_text(&self, y: u32) -> String {
        (0..self.cols)
            .filter_map(|x| self.cell(x, y).and_then(|cell| cell.glyph))
            .collect()
    }
}

/// A drawn frame and the declaration that bound it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowRaster {
    pub window: String,
    pub frame: String,
    /// The one-based line of the `frame` row that bound it.
    pub line: usize,
    pub raster: Raster,
}

/// **Draw every bound frame of a mounted package.** The rows are emitted in the frame's own order:
/// its paper runs first, then its glyph runs, each cell in the colour the palette answers for its role.
pub fn rasters(adopted: &Adopted, palette: &Palette) -> Result<Vec<WindowRaster>, Refusals> {
    let mut problems = Refusals::new();
    let mut drawn: Vec<WindowRaster> = Vec::new();
    for window in &adopted.surface.windows {
        let Some(frame) = adopted.frame_of(&window.id) else {
            continue;
        };
        let line = adopted
            .mount
            .windows
            .iter()
            .find(|entry| entry.window == window.id)
            .map(|entry| entry.line)
            .unwrap_or(1);
        match raster_of(&window.id, frame, line, palette) {
            Ok(raster) => drawn.push(raster),
            Err(refusals) => {
                for refusal in refusals.problems() {
                    problems.refuse(refusal.clone());
                }
            }
        }
    }
    if problems.is_empty() {
        Ok(drawn)
    } else {
        Err(problems)
    }
}

/// **Draw one frame of a window.** The timeline asks this per tick: the frame is a document the mount
/// already resolved, so nothing here re-resolves, re-checks or re-measures it — the cells are the
/// file's own rows, in the colour the package declared for each role.
pub fn raster_of(
    window: &str,
    frame: &planet_package::cellframe::CellFrame,
    line: usize,
    palette: &Palette,
) -> Result<WindowRaster, Refusals> {
    let mut problems = Refusals::new();
    let window = window.to_string();
    let transparent = palette
        .resolve("transparent")
        .unwrap_or(Rgba::TRANSPARENT);
    {
        let mut cells: Vec<Cell> = Vec::with_capacity((frame.cols * frame.rows) as usize);
        for y in 0..frame.rows {
            for x in 0..frame.cols {
                cells.push(Cell {
                    x,
                    y,
                    paper: transparent,
                    ink: None,
                    glyph: None,
                });
            }
        }

        let mut refused = false;
        for fill in &frame.fills {
            // A role the package declared with its own colour is drawn in **that** colour; the
            // client palette answers only the format's declared roles. This is the one place a
            // package's palette reaches the raster, and it stays roles-not-colours: the row named a
            // role, and the package said what that role is.
            let declared = frame
                .colours
                .iter()
                .find(|(name, _)| *name == fill.paper)
                .map(|(_, (r, g, b))| Rgba::rgba(*r, *g, *b, 255));
            let Some(paper) = declared.or_else(|| palette.resolve(&fill.paper)) else {
                problems.refuse(Refusal::CellframeRole {
                    line,
                    window: window.clone(),
                    frame: frame.id.clone(),
                    role: fill.paper.clone(),
                });
                refused = true;
                continue;
            };
            for offset in 0..fill.cols {
                let x = fill.col + offset;
                if let Some(cell) =
                    cells.get_mut((fill.row * frame.cols + x) as usize)
                {
                    cell.paper = paper;
                }
            }
        }
        for run in &frame.runs {
            let Some(ink) = palette.resolve(&run.ink) else {
                problems.refuse(Refusal::CellframeRole {
                    line,
                    window: window.clone(),
                    frame: frame.id.clone(),
                    role: run.ink.clone(),
                });
                refused = true;
                continue;
            };
            for (offset, glyph) in run.text.chars().enumerate() {
                let x = run.col + offset as u32;
                if let Some(cell) = cells.get_mut((run.row * frame.cols + x) as usize) {
                    cell.ink = Some(ink);
                    cell.glyph = Some(glyph);
                }
            }
        }
        if refused {
            return Err(problems);
        }
        Ok(WindowRaster {
            window,
            frame: frame.id.clone(),
            line,
            raster: Raster {
                cols: frame.cols,
                rows: frame.rows,
                cells,
            },
        })
    }
}
