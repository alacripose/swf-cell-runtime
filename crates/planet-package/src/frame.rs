//! The `V4PKG001` frame.
//!
//! Layout: `magic (8) | schema (LE u16) | kind (LE u16) | len (LE u64) | payload digest (32) | payload`.
//! The payload digest is `domain || schema (LE u16) || len (LE u64) || payload`, so the length is
//! inside the preimage and a truncated payload cannot collide with a longer one. `kind` sits in the
//! header and in the whole-frame digest, not in the payload preimage; that limit is named in
//! `tickets/TICKET-0029-frame-mount.md` rather than hidden.

use planet_shell_ledger::sha256;

pub const MAGIC: &[u8; 8] = b"V4PKG001";
pub const SCHEMA: u16 = 1;
pub const DOMAIN: &[u8] = b"planet-package";
pub const PAYLOAD_LIMIT: usize = 4 * 1024 * 1024;
pub const HEADER: usize = 8 + 2 + 2 + 8 + 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Decl = 1,
    Code = 2,
}

impl Kind {
    pub fn value(self) -> u16 {
        self as u16
    }

    pub fn name(self) -> &'static str {
        match self {
            Kind::Decl => "decl",
            Kind::Code => "code",
        }
    }

    pub fn parse(value: u16) -> Option<Kind> {
        match value {
            1 => Some(Kind::Decl),
            2 => Some(Kind::Code),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    FrameMagic {
        found: String,
    },
    FrameSchema {
        found: u16,
        supported: u16,
    },
    FrameKind {
        found: u16,
    },
    FrameTruncated {
        needed: usize,
        found: usize,
    },
    FrameLength {
        declared: u64,
        found: usize,
    },
    FrameDigest {
        declared: String,
        found: String,
    },
    PackagePayloadLimit {
        limit: usize,
        found: usize,
    },
    FrameKindReserved {
        kind: &'static str,
    },
    MountSequence {
        previous: u64,
        found: u64,
    },
    SurfaceRow {
        row: String,
    },
    SurfaceField {
        row: String,
        key: String,
    },
    SurfaceDuplicate {
        key: String,
    },
    SurfaceOrder {
        row: String,
    },
    SurfaceRequires {
        found: String,
    },
    SurfaceNode {
        window: String,
        node: String,
    },
    SurfaceAction {
        window: String,
        verb: String,
    },
    SurfaceSchema {
        found: String,
    },
    UnknownValue {
        vocabulary: &'static str,
        code: &'static str,
        found: String,
    },
    SurfaceWindows {
        found: usize,
    },
    FormatHashMismatch {
        declared: String,
        found: String,
    },
    FormatIdLegacyEmitted {
        found: String,
        current: String,
    },
    CellframeSchema {
        found: String,
        supported: &'static str,
    },
    CellframeRow {
        line: usize,
        row: String,
    },
    CellframeKey {
        line: usize,
        key: String,
    },
    CellframeDuplicate {
        line: usize,
        key: String,
    },
    CellframeMissing {
        key: String,
    },
    CellframeGrid {
        row: String,
    },
    CellframeEmpty {
        id: String,
    },
    CellframeBinding {
        line: usize,
        field: String,
    },
    CellframeBound {
        bound: &'static str,
        limit: u64,
        found: u64,
    },
    CellframeVocabulary {
        vocabulary: &'static str,
        found: String,
    },
    CellframeExtent {
        line: usize,
        frame: String,
        window: String,
        found: [u32; 2],
        allowed: [u32; 2],
    },
    CellframeReference {
        line: usize,
        window: String,
        frame: String,
    },
    CellframeUnreferenced {
        frame: String,
    },
    CellframeSurface {
        line: usize,
        window: String,
        frame: String,
    },
    CellframeRole {
        line: usize,
        window: String,
        frame: String,
        role: String,
    },
    CellframeGlyph {
        line: usize,
        window: String,
        frame: String,
        glyph: char,
    },
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::FrameMagic { .. } => "frame-magic",
            Refusal::FrameSchema { .. } => "frame-schema",
            Refusal::FrameKind { .. } => "frame-kind",
            Refusal::FrameTruncated { .. } => "frame-truncated",
            Refusal::FrameLength { .. } => "frame-length",
            Refusal::FrameDigest { .. } => "frame-digest",
            Refusal::PackagePayloadLimit { .. } => "package-payload-limit",
            Refusal::FrameKindReserved { .. } => "frame-kind-reserved",
            Refusal::MountSequence { .. } => "mount-sequence",
            Refusal::SurfaceRow { .. } => "surface-row",
            Refusal::SurfaceField { .. } => "surface-field",
            Refusal::SurfaceDuplicate { .. } => "surface-duplicate",
            Refusal::SurfaceOrder { .. } => "surface-order",
            Refusal::SurfaceRequires { .. } => "surface-requires",
            Refusal::SurfaceNode { .. } => "surface-node",
            Refusal::SurfaceAction { .. } => "surface-action",
            Refusal::SurfaceSchema { .. } => "surface-schema",
            Refusal::UnknownValue { code, .. } => code,
            Refusal::SurfaceWindows { .. } => "surface-windows",
            Refusal::FormatHashMismatch { .. } => "format-hash-mismatch",
            Refusal::FormatIdLegacyEmitted { .. } => "format-id-legacy-emitted",
            Refusal::CellframeSchema { .. } => "cellframe-schema",
            Refusal::CellframeRow { .. } => "cellframe-row",
            Refusal::CellframeKey { .. } => "cellframe-key",
            Refusal::CellframeDuplicate { .. } => "cellframe-duplicate",
            Refusal::CellframeMissing { .. } => "cellframe-missing",
            Refusal::CellframeGrid { .. } => "cellframe-grid",
            Refusal::CellframeEmpty { .. } => "cellframe-empty",
            Refusal::CellframeBinding { .. } => "cellframe-binding",
            Refusal::CellframeBound { .. } => "cellframe-bound",
            Refusal::CellframeVocabulary { .. } => "cellframe-vocabulary",
            Refusal::CellframeExtent { .. } => "cellframe-extent",
            Refusal::CellframeReference { .. } => "cellframe-reference",
            Refusal::CellframeUnreferenced { .. } => "cellframe-unreferenced",
            Refusal::CellframeSurface { .. } => "cellframe-surface",
            Refusal::CellframeRole { .. } => "cellframe-role",
            Refusal::CellframeGlyph { .. } => "cellframe-glyph",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::FrameMagic { found } => {
                format!("frame magic {found:?} is not V4PKG001")
            }
            Refusal::FrameSchema { found, supported } => {
                format!("frame schema {found} is not supported ({supported}); an older frame is refused, not converted")
            }
            Refusal::FrameKind { found } => {
                format!("frame kind {found} is not in the closed set decl(1)/code(2)")
            }
            Refusal::FrameTruncated { needed, found } => {
                format!("frame needs {needed} bytes and has {found}")
            }
            Refusal::FrameLength { declared, found } => {
                format!("frame declares {declared} payload bytes and carries {found}")
            }
            Refusal::FrameDigest { declared, found } => {
                format!("frame digest {declared} is not the payload's {found}")
            }
            Refusal::PackagePayloadLimit { limit, found } => {
                format!("payload {found} exceeds the {limit}-byte cap")
            }
            Refusal::FrameKindReserved { kind } => {
                format!("kind {kind:?} is reserved; it packs and is refused at mount until a reader exists")
            }
            Refusal::MountSequence { previous, found } => {
                format!("mount sequence {found} does not follow {previous}")
            }
            Refusal::SurfaceRow { row } => {
                format!("surface row {row:?} is not defined by dl-enclave.surface/1")
            }
            Refusal::SurfaceField { row, key } => {
                format!("{row} carries no {key} the grammar defines, or a malformed one")
            }
            Refusal::SurfaceDuplicate { key } => {
                format!("surface declares {key:?} twice; a key appears once")
            }
            Refusal::SurfaceOrder { row } => {
                format!("surface row {row:?} is in the wrong place for the declared grouping")
            }
            Refusal::SurfaceRequires { found } => {
                format!("requires {found:?} is not one of observe, act or admin")
            }
            Refusal::SurfaceNode { window, node } => {
                format!("node {node} is not a declared node of window {window} before it is used")
            }
            Refusal::SurfaceAction { window, verb } => {
                format!("action {verb} is not grouped under the declared window {window}")
            }
            Refusal::SurfaceSchema { found } => {
                format!(
                    "payload is not a {} document: {found}",
                    crate::format_id::id()
                )
            }
            Refusal::UnknownValue {
                vocabulary,
                found,
                ..
            } => {
                format!("{vocabulary} {found:?} is not declared by the surface vocabulary")
            }
            Refusal::SurfaceWindows { found } => {
                format!("surface declares {found} windows; a mount with none declares nothing")
            }
            Refusal::FormatHashMismatch { declared, found } => {
                format!("declared format id {declared} is not the id this reader computes, {found}")
            }
            Refusal::FormatIdLegacyEmitted { found, current } => {
                format!("format id {found} is legacy; a new document carries {current}")
            }
            Refusal::CellframeSchema { found, supported } => {
                format!(
                    "declaration header {found:?} is not {supported}; a different grammar is a \
                     different file, and an older one is refused rather than read"
                )
            }
            Refusal::CellframeRow { line, row } => format!("cellframe line {line}: {row}"),
            Refusal::CellframeKey { line, key } => {
                format!("cellframe line {line}: {key:?} is not a key this grammar defines")
            }
            Refusal::CellframeDuplicate { line, key } => {
                format!("cellframe line {line}: {key:?} is declared twice; a key appears once")
            }
            Refusal::CellframeMissing { key } => {
                format!("the frame declares no {key:?}; every listed key is required")
            }
            Refusal::CellframeGrid { row } => row.clone(),
            Refusal::CellframeEmpty { id } => {
                format!("frame {id:?} draws nothing; an erased window has to be stated as one")
            }
            Refusal::CellframeBinding { line, field } => format!("cellframe line {line}: {field}"),
            Refusal::CellframeBound {
                bound,
                limit,
                found,
            } => format!("{bound} {found} exceeds the format's ceiling of {limit}"),
            Refusal::CellframeVocabulary { vocabulary, found } => {
                format!("{vocabulary} {found:?} is not declared by this package's vocabulary section")
            }
            Refusal::CellframeExtent {
                line,
                frame,
                window,
                found,
                allowed,
            } => format!(
                "cellframe line {line}: frame {frame} is {} by {} and window {window} has {} by {} \
                 of content space; a frame agrees with its window or the binding is refused, never \
                 drawn through the chrome and never leaving declared cells undrawn",
                found[0], found[1], allowed[0], allowed[1]
            ),
            Refusal::CellframeReference {
                line,
                window,
                frame,
            } => format!(
                "cellframe line {line}: window {window} binds frame {frame:?}, which this package \
                 does not declare"
            ),
            Refusal::CellframeUnreferenced { frame } => format!(
                "frame {frame:?} is declared by the package and bound by no declaration; an \
                 unreachable frame is stated deliberately or it is not shipped"
            ),
            Refusal::CellframeSurface {
                line,
                window,
                frame,
            } => format!(
                "cellframe line {line}: window {window} binds frame {frame:?}, and the surface being \
                 mounted declares no such window; a mount carries frames addressed at windows that \
                 exist"
            ),
            Refusal::CellframeRole {
                line,
                window,
                frame,
                role,
            } => format!(
                "cellframe line {line}: window {window} binds frame {frame:?}, whose paper or ink \
                 role {role:?} the drawing palette does not carry; a role is resolved or the frame \
                 is not drawn"
            ),
            Refusal::CellframeGlyph {
                line,
                window,
                frame,
                glyph,
            } => format!(
                "cellframe line {line}: window {window} binds frame {frame:?}, whose run carries \
                 {glyph:?} and the drawing font has no bitmap for it; a glyph is drawn or the frame \
                 is not drawn"
            ),
        }
    }
}

/// The payload digest: `domain || schema (LE u16) || len (LE u64) || payload`.
pub fn payload_digest(schema: u16, payload: &[u8]) -> [u8; 32] {
    let mut message = Vec::with_capacity(DOMAIN.len() + 10 + payload.len());
    message.extend_from_slice(DOMAIN);
    message.extend_from_slice(&schema.to_le_bytes());
    message.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    message.extend_from_slice(payload);
    sha256::sha256(&message)
}

/// The whole-frame digest, which is what a receipt names.
pub fn frame_digest(bytes: &[u8]) -> [u8; 32] {
    sha256::sha256(bytes)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub schema: u16,
    pub kind: Kind,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn new(kind: Kind, payload: &[u8]) -> Result<Self, Refusal> {
        if payload.len() > PAYLOAD_LIMIT {
            return Err(Refusal::PackagePayloadLimit {
                limit: PAYLOAD_LIMIT,
                found: payload.len(),
            });
        }
        Ok(Frame {
            schema: SCHEMA,
            kind,
            payload: payload.to_vec(),
        })
    }

    pub fn payload_digest_hex(&self) -> String {
        sha256::hex(&payload_digest(self.schema, &self.payload))
    }

    pub fn frame_digest_hex(&self) -> String {
        sha256::hex(&frame_digest(&self.encode()))
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(HEADER + self.payload.len());
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&self.schema.to_le_bytes());
        bytes.extend_from_slice(&self.kind.value().to_le_bytes());
        bytes.extend_from_slice(&(self.payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&payload_digest(self.schema, &self.payload));
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    /// Read the schema before anything else, so an older frame is refused by name without its
    /// payload being interpreted.
    pub fn read_schema(bytes: &[u8]) -> Result<u16, Refusal> {
        if bytes.len() < 10 {
            return Err(Refusal::FrameTruncated {
                needed: HEADER,
                found: bytes.len(),
            });
        }
        if &bytes[0..8] != MAGIC {
            return Err(Refusal::FrameMagic {
                found: String::from_utf8_lossy(&bytes[0..8]).into_owned(),
            });
        }
        let schema = u16::from_le_bytes([bytes[8], bytes[9]]);
        if schema != SCHEMA {
            return Err(Refusal::FrameSchema {
                found: schema,
                supported: SCHEMA,
            });
        }
        Ok(schema)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, Refusal> {
        let schema = Frame::read_schema(bytes)?;
        if bytes.len() < HEADER {
            return Err(Refusal::FrameTruncated {
                needed: HEADER,
                found: bytes.len(),
            });
        }
        let kind_value = u16::from_le_bytes([bytes[10], bytes[11]]);
        let kind = Kind::parse(kind_value).ok_or(Refusal::FrameKind { found: kind_value })?;
        let declared = u64::from_le_bytes(bytes[12..20].try_into().expect("eight bytes"));
        let carried = &bytes[HEADER..];
        if declared as usize != carried.len() {
            return Err(Refusal::FrameLength {
                declared,
                found: carried.len(),
            });
        }
        let declared_digest = &bytes[20..52];
        let computed = payload_digest(schema, carried);
        if declared_digest != computed {
            return Err(Refusal::FrameDigest {
                declared: sha256::hex(declared_digest),
                found: sha256::hex(&computed),
            });
        }
        Ok(Frame {
            schema,
            kind,
            payload: carried.to_vec(),
        })
    }
}
