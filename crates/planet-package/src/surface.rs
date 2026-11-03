//! `dl-enclave.surface/1`, parsed strictly.
//!
//! The grammar is the one `dl-enclave/docs/PACKAGE-FORMAT.md` §2.5 owns: TSV, LF, one final newline,
//! `#` comments, and no positional column beyond the key. This module parses the document into typed
//! windows, nodes and actions and refuses, by name, anything the grammar does not define.
//!
//! The vocabularies are not inline literals: `lane`, `rule`, `node kind`, `theme role`, `type role`,
//! `chrome`, `moves` and `capability` come from [`crate::vocabulary`], which is the shell family's
//! declared set (`secure-shell/tools/port/surface.mjs`). An undeclared value is refused with the
//! family's own code — `unknown-lane`, `unknown-rule` — or the same shape for the rest, and the
//! refusal names the value it refused.

use crate::format_id;
use crate::frame::Refusal;
use crate::vocabulary::{self, RoleClass, Vocabulary};

pub const SURFACE_SCHEMA: &str = "dl-enclave.surface/1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Requirement {
    Observe,
    Act,
    Admin,
}

impl Requirement {
    pub fn name(self) -> &'static str {
        match self {
            Requirement::Observe => "observe",
            Requirement::Act => "act",
            Requirement::Admin => "admin",
        }
    }

    /// The capability names are the declared table's, so `requires` cannot drift from it; the
    /// refusal keeps its existing code `surface-requires`.
    pub fn parse(text: &str) -> Option<Requirement> {
        if !Vocabulary::Capability.accepts(text) {
            return None;
        }
        match text {
            "observe" => Some(Requirement::Observe),
            "act" => Some(Requirement::Act),
            "admin" => Some(Requirement::Admin),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub parent: Option<String>,
    pub kind: String,
    pub role: String,
    pub role_class: RoleClass,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub verb: String,
    pub tool: String,
    pub target: String,
}

/// A frame a declaration binds to a window, and the line it bound it on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclaredFrame {
    /// The frame id the package declares and this window loads with.
    pub id: String,
    /// The one-based line of the `frame` row that bound it.
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Window {
    pub id: String,
    pub title: String,
    pub lane: String,
    pub rule: String,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
    pub min_w: i64,
    pub min_h: i64,
    pub pinnable: bool,
    pub pinned: bool,
    pub chrome: String,
    pub moves: String,
    pub theme_role: String,
    pub content: String,
    /// The frame this window loads with, when the declaration binds one.
    pub frame: Option<DeclaredFrame>,
    pub nodes: Vec<Node>,
    pub actions: Vec<Action>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Surface {
    pub id: String,
    pub requires: Option<Requirement>,
    pub windows: Vec<Window>,
}

fn integer(row: &str, key: &str, text: &str) -> Result<i64, Refusal> {
    text.parse::<i64>().map_err(|_| Refusal::SurfaceField {
        row: row.to_string(),
        key: key.to_string(),
    })
}

fn boolean(row: &str, key: &str, text: &str) -> Result<bool, Refusal> {
    match text {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(Refusal::SurfaceField {
            row: row.to_string(),
            key: key.to_string(),
        }),
    }
}

fn non_empty(row: &str, key: &str, text: &str) -> Result<String, Refusal> {
    if text.is_empty() {
        return Err(Refusal::SurfaceField {
            row: row.to_string(),
            key: key.to_string(),
        });
    }
    Ok(text.to_string())
}

fn columns(row: &str, fields: &[&str], expected: usize) -> Result<(), Refusal> {
    if fields.len() != expected {
        return Err(Refusal::SurfaceField {
            row: row.to_string(),
            key: "columns".to_string(),
        });
    }
    Ok(())
}

fn declared(vocabulary: Vocabulary, text: &str) -> Result<String, Refusal> {
    if vocabulary.accepts(text) {
        Ok(text.to_string())
    } else {
        Err(Refusal::UnknownValue {
            vocabulary: vocabulary.name(),
            code: vocabulary.code(),
            found: text.to_string(),
        })
    }
}

fn declared_list(vocabulary: Vocabulary, text: &str) -> Result<String, Refusal> {
    if vocabulary::list_accepts(vocabulary, text) {
        Ok(text.to_string())
    } else {
        Err(Refusal::UnknownValue {
            vocabulary: vocabulary.name(),
            code: vocabulary.code(),
            found: text.to_string(),
        })
    }
}

fn declared_role(text: &str) -> Result<(String, RoleClass), Refusal> {
    match vocabulary::classify_role(text) {
        Some(class) => Ok((text.to_string(), class)),
        None => Err(Refusal::UnknownValue {
            vocabulary: "node role",
            code: "unknown-node-role",
            found: text.to_string(),
        }),
    }
}

impl Surface {
    /// Parse a surface document strictly. Every refusal names the row, key or value it is about.
    pub fn parse(payload: &[u8]) -> Result<Surface, Refusal> {
        if !payload.ends_with(b"\n") {
            return Err(Refusal::SurfaceSchema {
                found: "no final newline".to_string(),
            });
        }
        let text = std::str::from_utf8(payload).map_err(|_| Refusal::SurfaceSchema {
            found: "not UTF-8".to_string(),
        })?;
        let mut lines = text.lines();
        let header = lines.next().unwrap_or("");
        // The computed id reads as current; a legacy numeric id reads as the current format and is
        // re-emitted under the hashed id; a hash this reader did not compute is refused by name.
        format_id::read(header)?;

        let mut id: Option<String> = None;
        let mut requires: Option<Requirement> = None;
        let mut windows: Vec<Window> = Vec::new();

        for (index, line) in lines.enumerate() {
            let number = index + 2;
            if line.starts_with('#') {
                continue;
            }
            if line.is_empty() {
                return Err(Refusal::SurfaceRow {
                    row: "<blank>".to_string(),
                });
            }
            let fields: Vec<&str> = line.split('\t').collect();
            let key = fields[0];
            match key {
                "surface" => {
                    columns(key, &fields, 2)?;
                    if id.is_some() {
                        return Err(Refusal::SurfaceDuplicate {
                            key: "surface".to_string(),
                        });
                    }
                    id = Some(non_empty(key, "id", fields[1])?);
                }
                "requires" => {
                    columns(key, &fields, 2)?;
                    if requires.is_some() {
                        return Err(Refusal::SurfaceDuplicate {
                            key: "requires".to_string(),
                        });
                    }
                    if !windows.is_empty() {
                        return Err(Refusal::SurfaceOrder {
                            row: "requires".to_string(),
                        });
                    }
                    requires = Some(Requirement::parse(fields[1]).ok_or_else(|| {
                        Refusal::SurfaceRequires {
                            found: fields[1].to_string(),
                        }
                    })?);
                }
                "window" => {
                    columns(key, &fields, 17)?;
                    let window_id = non_empty(key, "id", fields[1])?;
                    if id.is_none() {
                        return Err(Refusal::SurfaceOrder {
                            row: "window".to_string(),
                        });
                    }
                    if windows.iter().any(|window| window.id == window_id) {
                        return Err(Refusal::SurfaceDuplicate { key: window_id });
                    }
                    let row = format!("window {window_id}");
                    let w = integer(&row, "w", fields[7])?;
                    let h = integer(&row, "h", fields[8])?;
                    if w < 1 || h < 1 {
                        return Err(Refusal::SurfaceField {
                            row,
                            key: "extent".to_string(),
                        });
                    }
                    windows.push(Window {
                        id: window_id,
                        title: non_empty(&row, "title", fields[2])?,
                        lane: declared(Vocabulary::Lane, fields[3])?,
                        rule: declared(Vocabulary::Rule, fields[4])?,
                        x: integer(&row, "x", fields[5])?,
                        y: integer(&row, "y", fields[6])?,
                        w,
                        h,
                        min_w: integer(&row, "min-w", fields[9])?,
                        min_h: integer(&row, "min-h", fields[10])?,
                        pinnable: boolean(&row, "pinnable", fields[11])?,
                        pinned: boolean(&row, "pinned", fields[12])?,
                        chrome: declared_list(Vocabulary::Chrome, fields[13])?,
                        moves: declared_list(Vocabulary::Moves, fields[14])?,
                        theme_role: declared(Vocabulary::ThemeRole, fields[15])?,
                        content: non_empty(&row, "content", fields[16])?,
                        frame: None,
                        nodes: Vec::new(),
                        actions: Vec::new(),
                    });
                }
                "node" => {
                    columns(key, &fields, 7)?;
                    let window_id = fields[1].to_string();
                    let current = windows.last_mut().ok_or_else(|| Refusal::SurfaceOrder {
                        row: "node".to_string(),
                    })?;
                    if current.id != window_id {
                        return Err(Refusal::SurfaceNode {
                            window: window_id,
                            node: fields[2].to_string(),
                        });
                    }
                    let row = format!("node {}", fields[2]);
                    let node_id = non_empty(key, "id", fields[2])?;
                    if current.nodes.iter().any(|node| node.id == node_id) {
                        return Err(Refusal::SurfaceDuplicate { key: node_id });
                    }
                    let parent = if fields[3] == "-" {
                        None
                    } else {
                        if !current.nodes.iter().any(|node| node.id == fields[3]) {
                            return Err(Refusal::SurfaceNode {
                                window: current.id.clone(),
                                node: fields[2].to_string(),
                            });
                        }
                        Some(fields[3].to_string())
                    };
                    let (role, role_class) = declared_role(fields[5])?;
                    current.nodes.push(Node {
                        id: node_id,
                        parent,
                        kind: declared(Vocabulary::NodeKind, fields[4])?,
                        role,
                        role_class,
                        value: non_empty(&row, "value", fields[6])?,
                    });
                }
                "action" => {
                    columns(key, &fields, 5)?;
                    let window_id = fields[1].to_string();
                    let current = windows.last_mut().ok_or_else(|| Refusal::SurfaceOrder {
                        row: "action".to_string(),
                    })?;
                    if current.id != window_id {
                        return Err(Refusal::SurfaceAction {
                            window: window_id,
                            verb: fields[2].to_string(),
                        });
                    }
                    let row = format!("action {}", fields[2]);
                    current.actions.push(Action {
                        verb: non_empty(&row, "verb", fields[2])?,
                        tool: non_empty(&row, "tool", fields[3])?,
                        target: non_empty(&row, "target", fields[4])?,
                    });
                }
                "frame" => {
                    columns(key, &fields, 3)?;
                    let window_id = fields[1].to_string();
                    let index = windows
                        .len()
                        .checked_sub(1)
                        .ok_or_else(|| Refusal::SurfaceOrder {
                            row: "frame".to_string(),
                        })?;
                    if windows[index].id != window_id {
                        return Err(Refusal::SurfaceOrder {
                            row: format!("frame under window {window_id}"),
                        });
                    }
                    let frame_id = non_empty(key, "frame", fields[2])?;
                    if windows[index].frame.is_some() {
                        return Err(Refusal::CellframeDuplicate {
                            line: number,
                            key: format!("frame for window {window_id}"),
                        });
                    }
                    if windows.iter().any(|window| {
                        window
                            .frame
                            .as_ref()
                            .map_or(false, |bound| bound.id == frame_id)
                    }) {
                        return Err(Refusal::CellframeDuplicate {
                            line: number,
                            key: frame_id,
                        });
                    }
                    windows[index].frame = Some(DeclaredFrame { id: frame_id, line: number });
                }
                other => {
                    return Err(Refusal::SurfaceRow {
                        row: other.to_string(),
                    })
                }
            }
        }

        match id {
            Some(id) => Ok(Surface {
                id,
                requires,
                windows,
            }),
            None => Err(Refusal::SurfaceSchema {
                found: "no surface row".to_string(),
            }),
        }
    }
}
