//! The XML client format: a window definition lowered to the lines a window can draw.
//!
//! The second client branch presents the same adopted package through an XML window definition
//! instead of the package's own label nodes, so the two formats are compared on one package rather
//! than on two. The reader is deliberately small and strict: a closed element set, a closed attribute
//! set, and a refusal by name for anything else. It lowers text, it does not carry the XML forward —
//! an element the schema does not declare is refused, never passed through as a payload.
//!
//! Declared elements:
//! - `<window title="…" width="…" height="…">` — the root, exactly once;
//! - `<heading>text</heading>` — a section line at the current depth;
//! - `<label>text</label>` — a line at the current depth;
//! - `<item>text</item>` — a list entry, indented one step under its `<list>`;
//! - `<list>` — a nesting step; holds `item` children;
//! - `<separator/>` — a rule line.
//!
//! Declared attributes: `title`, `width`, `height` on `window`, and nothing elsewhere.

use crate::render::Scene;

/// What the reader refused, by name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    Element { name: String },
    Attribute { element: String, name: String },
    Structure { detail: String },
    Empty,
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::Element { .. } => "xml-element",
            Refusal::Attribute { .. } => "xml-attribute",
            Refusal::Structure { .. } => "xml-structure",
            Refusal::Empty => "xml-empty",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::Element { name } => format!("element <{name}> is not declared by the window schema"),
            Refusal::Attribute { element, name } => {
                format!("attribute {name} is not declared on <{element}>")
            }
            Refusal::Structure { detail } => detail.clone(),
            Refusal::Empty => "the window document declares no elements".to_string(),
        }
    }
}

/// A parsed window definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub lines: Vec<String>,
}

impl Window {
    /// The scene this definition draws: its title and its lowered lines.
    pub fn scene(&self) -> Scene {
        Scene::new(self.title.clone(), self.lines.clone())
    }
}

fn attributes(element: &str, text: &str) -> Result<Vec<(String, String)>, Refusal> {
    let mut found = Vec::new();
    let mut rest = text.trim();
    while !rest.is_empty() {
        let Some(eq) = rest.find('=') else {
            return Err(Refusal::Structure {
                detail: format!("<{element}> carries an unquoted attribute: {rest}"),
            });
        };
        let name = rest[..eq].trim().to_string();
        let after = rest[eq + 1..].trim_start();
        let Some(quote) = after.chars().next().filter(|ch| *ch == '"' || *ch == '\'') else {
            return Err(Refusal::Structure {
                detail: format!("<{element}> attribute {name} is not quoted"),
            });
        };
        let Some(end) = after[1..].find(quote) else {
            return Err(Refusal::Structure {
                detail: format!("<{element}> attribute {name} is not closed"),
            });
        };
        found.push((name, after[1..1 + end].to_string()));
        rest = after[1 + end + 1..].trim_start();
    }
    Ok(found)
}

/// Parse a window definition strictly.
pub fn parse(text: &str) -> Result<Window, Refusal> {
    let mut title: Option<String> = None;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut lines: Vec<String> = Vec::new();
    let mut depth: usize = 0;
    let mut in_window = false;
    let mut pending: Option<String> = None;
    let mut seen_element = false;

    let mut rest = text;
    while let Some(open) = rest.find('<') {
        let before = &rest[..open];
        if let Some(tag) = pending.take() {
            let text = before.trim();
            if !text.is_empty() {
                lines.push(lowered(&tag, text, depth));
            }
        } else if !before.trim().is_empty() {
            return Err(Refusal::Structure {
                detail: format!("text outside an element: {:?}", before.trim()),
            });
        }
        let after = &rest[open + 1..];
        let Some(close) = after.find('>') else {
            return Err(Refusal::Structure {
                detail: "an element is not closed with '>'".to_string(),
            });
        };
        let mut tag = after[..close].trim().to_string();
        rest = &after[close + 1..];
        if tag.starts_with('!') || tag.starts_with('?') {
            continue;
        }
        let closing = tag.starts_with('/');
        if closing {
            tag = tag[1..].trim().to_string();
        }
        let self_closing = tag.ends_with('/');
        if self_closing {
            tag.pop();
            tag = tag.trim_end().to_string();
        }
        let name: String = tag
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
            .collect();
        let attrs = attributes(&name, &tag[name.len()..])?;

        if closing {
            if pending.is_some() {
                return Err(Refusal::Structure {
                    detail: format!("</{name}> closes an element still holding text"),
                });
            }
            match name.as_str() {
                "window" => {
                    in_window = false;
                    depth = 0;
                }
                "list" => {
                    depth = depth.saturating_sub(1);
                }
                "heading" | "label" | "item" | "separator" => {}
                _ => return Err(Refusal::Element { name }),
            }
            continue;
        }

        seen_element = true;
        match name.as_str() {
            "window" => {
                if in_window {
                    return Err(Refusal::Structure {
                        detail: "the document declares <window> twice".to_string(),
                    });
                }
                in_window = true;
                for (key, value) in &attrs {
                    match key.as_str() {
                        "title" => title = Some(value.clone()),
                        "width" => {
                            width = value.parse().map_err(|_| Refusal::Structure {
                                detail: format!("width {value:?} is not a number"),
                            })?
                        }
                        "height" => {
                            height = value.parse().map_err(|_| Refusal::Structure {
                                detail: format!("height {value:?} is not a number"),
                            })?
                        }
                        other => {
                            return Err(Refusal::Attribute {
                                element: "window".to_string(),
                                name: other.to_string(),
                            })
                        }
                    }
                }
                if !attrs.iter().any(|(key, _)| key == "title") {
                    return Err(Refusal::Structure {
                        detail: "<window> declares no title".to_string(),
                    });
                }
            }
            "list" => {
                if !attrs.is_empty() {
                    return Err(Refusal::Attribute {
                        element: "list".to_string(),
                        name: attrs[0].0.clone(),
                    });
                }
                depth += 1;
            }
            "separator" => {
                lines.push("─".repeat(48));
            }
            "heading" | "label" | "item" => {
                if !attrs.is_empty() {
                    return Err(Refusal::Attribute {
                        element: name,
                        name: attrs[0].0.clone(),
                    });
                }
                if !self_closing {
                    pending = Some(name);
                }
            }
            _ => return Err(Refusal::Element { name }),
        }
    }

    if pending.is_some() {
        return Err(Refusal::Structure {
            detail: "an element is not closed".to_string(),
        });
    }
    if !seen_element {
        return Err(Refusal::Empty);
    }
    let Some(title) = title else {
        return Err(Refusal::Structure {
            detail: "the document has no <window>".to_string(),
        });
    };
    Ok(Window {
        title,
        width,
        height,
        lines,
    })
}

fn lowered(tag: &str, text: &str, depth: usize) -> String {
    let pad = "  ".repeat(depth);
    match tag {
        "heading" => format!("{text}"),
        "item" => format!("{pad}• {text}"),
        _ => format!("{pad}{text}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"<window title="Taskbar" width="1920" height="48">
  <heading>Pinned</heading>
  <list>
    <item>Explorer</item>
    <item>Terminal</item>
  </list>
  <separator/>
  <label>8 corners</label>
</window>
"#;

    #[test]
    fn a_window_lowers_to_its_lines() {
        let window = parse(DOC).expect("parsed");
        assert_eq!(window.title, "Taskbar");
        assert_eq!(window.width, 1920);
        assert_eq!(window.height, 48);
        assert_eq!(
            window.lines,
            vec![
                "Pinned".to_string(),
                "  • Explorer".to_string(),
                "  • Terminal".to_string(),
                "─".repeat(48),
                "8 corners".to_string(),
            ]
        );
    }

    #[test]
    fn the_scene_carries_the_title_and_the_lines() {
        let scene = parse(DOC).expect("parsed").scene();
        assert_eq!(scene.title, "Taskbar");
        assert!(scene.body().contains("• Explorer"));
    }

    #[test]
    fn an_undeclared_element_is_refused_by_name() {
        let doc = r#"<window title="x"><script>alert(1)</script></window>"#;
        assert_eq!(
            parse(doc),
            Err(Refusal::Element {
                name: "script".to_string()
            })
        );
    }

    #[test]
    fn an_undeclared_attribute_is_refused_by_name() {
        let doc = r#"<window title="x" theme="dark"></window>"#;
        assert_eq!(
            parse(doc),
            Err(Refusal::Attribute {
                element: "window".to_string(),
                name: "theme".to_string()
            })
        );
    }

    #[test]
    fn text_outside_an_element_is_refused() {
        let doc = r#"stray<window title="x"></window>"#;
        match parse(doc) {
            Err(Refusal::Structure { .. }) => {}
            other => panic!("expected a structure refusal, found {other:?}"),
        }
    }

    #[test]
    fn an_empty_document_is_refused() {
        assert_eq!(parse("   "), Err(Refusal::Empty));
    }
}
