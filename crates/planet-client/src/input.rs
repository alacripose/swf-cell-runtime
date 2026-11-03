//! The input router: pointer, touch and key events turned into intents against the declared surface.
//!
//! `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.7 puts this in the client: *"Focus, key repeat,
//! pointer capture, dragging, wheel behavior, touch (when present), and event propagation are
//! implemented in the client."* The same section keeps the running client out of agent hands — *"Agents
//! have no pointer/keyboard MCP, no CUA control, no arbitrary runtime-advance tool, and no ability to
//! probe the game"* — so this module is mechanics fed by synthetic events in tests, never a way to
//! drive or observe a live window.
//!
//! It invents no semantics. The geometry is the window rectangle the surface declares (`x`, `y`, `w`,
//! `h` in cells) and the format's own cell size; a pointer outside every window is outside, and a
//! pointer over no focused window routes to no window rather than to a guess. What a press *means*, and
//! the source game's hit areas and button semantics, are compiled by the shell and stay the shell's.
//!
//! Touch is folded onto the pointer path rather than given a second state machine: a touch down sets
//! focus and captures exactly as a press does, a touch move is hit-tested through the same capture, and
//! a touch up ends it. The routed shape still says `Touch`, so a caller can tell a finger from a mouse
//! without either having to re-derive the geometry.
//!
//! Event propagation into the surface's node tree is deliberately absent: nodes declare an id, a
//! parent, a kind, a role and a value, and no geometry, while the frame's drawn rectangles carry no node
//! identity. Routing a press into a node would mean inventing the rectangle it lands in, so the
//! window-and-cell route is the most the declaration can decide today.

use planet_package::Surface;

use crate::font::{CELL_H, CELL_W};

/// One cell of the surface's own grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

/// A pointer button. The router reports it; it does not decide what a button does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    Primary,
    Secondary,
    Middle,
}

/// A key, named by the router rather than by any game binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Character(char),
    Enter,
    Escape,
    Tab,
    Space,
    Backspace,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
}

/// Which phase a touch reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TouchPhase {
    Down,
    Moved,
    Up,
}

/// One event, as the window layer hands it over. Synthetic in every test here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    PointerMoved {
        x: i64,
        y: i64,
    },
    PointerPressed {
        button: Button,
    },
    PointerReleased {
        button: Button,
    },
    PointerLeft,
    Wheel {
        delta: i64,
    },
    TouchDown {
        id: u64,
        x: i64,
        y: i64,
    },
    TouchMoved {
        id: u64,
        x: i64,
        y: i64,
    },
    TouchUp {
        id: u64,
    },
    KeyPressed(Key),
    KeyReleased(Key),
    Text(char),
}

/// What the router decided. Every variant names the window it belongs to, and none names a game action:
/// the bindings are the shell's, not this module's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Routed {
    /// Nothing under the pointer, or nothing focused.
    Nothing,
    /// The pointer left every window.
    Left,
    /// A pointer position over a window; `cell` is `None` when a capture carries the drag outside it.
    Hover {
        window: String,
        cell: Option<Cell>,
    },
    Press {
        window: String,
        cell: Cell,
    },
    /// `captured` says whether this release ended a pointer capture.
    Release {
        window: String,
        cell: Option<Cell>,
        captured: bool,
    },
    Wheel {
        window: String,
        cell: Option<Cell>,
        delta: i64,
    },
    /// A touch, folded onto the pointer path. `window` is `None` for a touch that began on nothing.
    Touch {
        window: Option<String>,
        cell: Option<Cell>,
        id: u64,
        phase: TouchPhase,
    },
    /// `repeat` is true when the key was already held: the client's own key-repeat bookkeeping.
    Key {
        window: Option<String>,
        key: Key,
        repeat: bool,
    },
    Text {
        window: Option<String>,
        glyph: char,
    },
}

#[derive(Clone, Debug)]
struct Placed {
    id: String,
    left: i64,
    top: i64,
    right: i64,
    bottom: i64,
}

impl Placed {
    fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }

    fn cell(&self, x: i64, y: i64) -> Option<Cell> {
        if !self.contains(x, y) {
            return None;
        }
        Some(Cell {
            x: ((x - self.left) / CELL_W as i64) as u32,
            y: ((y - self.top) / CELL_H as i64) as u32,
        })
    }
}

/// The router's state: the declared windows, the pointer, the focus and the capture.
#[derive(Clone, Debug)]
pub struct Router {
    windows: Vec<Placed>,
    pointer: Option<(i64, i64)>,
    focus: Option<String>,
    captured: Option<String>,
    held: Vec<Key>,
}

impl Router {
    /// Build a router over the surface the package declared. The geometry is read, never re-derived.
    pub fn new(surface: &Surface) -> Router {
        let windows = surface
            .windows
            .iter()
            .map(|window| Placed {
                id: window.id.clone(),
                left: window.x * CELL_W as i64,
                top: window.y * CELL_H as i64,
                right: (window.x + window.w) * CELL_W as i64,
                bottom: (window.y + window.h) * CELL_H as i64,
            })
            .collect();
        Router {
            windows,
            pointer: None,
            focus: None,
            captured: None,
            held: Vec::new(),
        }
    }

    pub fn focus(&self) -> Option<&str> {
        self.focus.as_deref()
    }

    pub fn captured(&self) -> Option<&str> {
        self.captured.as_deref()
    }

    pub fn pointer(&self) -> Option<(i64, i64)> {
        self.pointer
    }

    /// The declared window rectangles the router hit-tests, in cells, in declaration order.
    pub fn window_cells(&self) -> Vec<(String, [i64; 4])> {
        self.windows
            .iter()
            .map(|window| {
                (
                    window.id.clone(),
                    [window.left, window.top, window.right, window.bottom],
                )
            })
            .collect()
    }

    fn window_at(&self, x: i64, y: i64) -> Option<&Placed> {
        self.windows.iter().find(|window| window.contains(x, y))
    }

    fn placed(&self, id: &str) -> Option<&Placed> {
        self.windows.iter().find(|window| window.id == id)
    }

    fn window_under_capture(&self, x: i64, y: i64) -> (Option<String>, Option<Cell>) {
        match self.captured.clone() {
            Some(window) => {
                let cell = self.placed(&window).and_then(|placed| placed.cell(x, y));
                (Some(window), cell)
            }
            None => match self.window_at(x, y) {
                Some(placed) => (Some(placed.id.clone()), placed.cell(x, y)),
                None => (None, None),
            },
        }
    }

    /// One event in, one routing decision out. State changes only where the plan names it: focus from a
    /// press or a touch down, capture from a press and release, key-repeat from the held set.
    pub fn handle(&mut self, event: Event) -> Routed {
        match event {
            Event::PointerMoved { x, y } => {
                self.pointer = Some((x, y));
                match self.window_under_capture(x, y) {
                    (Some(window), cell) => Routed::Hover { window, cell },
                    (None, _) => Routed::Nothing,
                }
            }
            Event::PointerPressed { .. } => {
                let Some((x, y)) = self.pointer else {
                    return Routed::Nothing;
                };
                let Some(placed) = self.window_at(x, y) else {
                    return Routed::Nothing;
                };
                let window = placed.id.clone();
                let cell = match placed.cell(x, y) {
                    Some(cell) => cell,
                    None => return Routed::Nothing,
                };
                self.captured = Some(window.clone());
                self.focus = Some(window.clone());
                Routed::Press { window, cell }
            }
            Event::PointerReleased { .. } => {
                let Some(window) = self.captured.clone() else {
                    return Routed::Nothing;
                };
                let cell = match self.pointer {
                    Some((x, y)) => self.placed(&window).and_then(|placed| placed.cell(x, y)),
                    None => None,
                };
                self.captured = None;
                Routed::Release {
                    window,
                    cell,
                    captured: true,
                }
            }
            Event::PointerLeft => {
                self.pointer = None;
                Routed::Left
            }
            Event::Wheel { delta } => {
                let window = match self.captured.clone() {
                    Some(window) => Some(window),
                    None => self
                        .pointer
                        .and_then(|(x, y)| self.window_at(x, y))
                        .map(|placed| placed.id.clone()),
                };
                let Some(window) = window else {
                    return Routed::Nothing;
                };
                let cell = self
                    .pointer
                    .and_then(|(x, y)| self.placed(&window).and_then(|placed| placed.cell(x, y)));
                Routed::Wheel {
                    window,
                    cell,
                    delta,
                }
            }
            Event::TouchDown { id, x, y } => {
                self.pointer = Some((x, y));
                match self.window_at(x, y) {
                    Some(placed) => {
                        let window = placed.id.clone();
                        let cell = placed.cell(x, y);
                        self.captured = Some(window.clone());
                        self.focus = Some(window.clone());
                        Routed::Touch {
                            window: Some(window),
                            cell,
                            id,
                            phase: TouchPhase::Down,
                        }
                    }
                    None => Routed::Touch {
                        window: None,
                        cell: None,
                        id,
                        phase: TouchPhase::Down,
                    },
                }
            }
            Event::TouchMoved { id, x, y } => {
                self.pointer = Some((x, y));
                let (window, cell) = self.window_under_capture(x, y);
                Routed::Touch {
                    window,
                    cell,
                    id,
                    phase: TouchPhase::Moved,
                }
            }
            Event::TouchUp { id } => {
                let window = self.captured.clone();
                let cell = match (window.as_deref(), self.pointer) {
                    (Some(window), Some((x, y))) => {
                        self.placed(window).and_then(|placed| placed.cell(x, y))
                    }
                    _ => None,
                };
                self.captured = None;
                Routed::Touch {
                    window,
                    cell,
                    id,
                    phase: TouchPhase::Up,
                }
            }
            Event::KeyPressed(key) => {
                let repeat = self.held.contains(&key);
                if !repeat {
                    self.held.push(key);
                }
                Routed::Key {
                    window: self.focus.clone(),
                    key,
                    repeat,
                }
            }
            Event::KeyReleased(key) => {
                self.held.retain(|held| held != &key);
                Routed::Key {
                    window: self.focus.clone(),
                    key,
                    repeat: false,
                }
            }
            Event::Text(glyph) => Routed::Text {
                window: self.focus.clone(),
                glyph,
            },
        }
    }
}
