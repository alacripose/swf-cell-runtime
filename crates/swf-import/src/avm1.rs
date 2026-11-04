//! **AVM1 action records, lowered.** A loader states its behaviour in action records (tags 12 and
//! 59). This module turns a record's bytes into a canonical textual program the package carries and
//! a program lane executes, and it **refuses by name** anything outside its table: an unknown
//! opcode, a truncated field, or a push value whose framing does not add up. A refusal names the byte
//! offset and the opcode number, so a gap is measured rather than silently skipped.
//!
//! What it is not: an interpreter. It decodes and canonicalises; execution is the program lane's.

/// The opcodes this reader decodes, with the plan's names. A code outside this table is refused
/// `avm1-opcode-unimplemented` with its number, never no-opped.
pub const DECODED: &[(u8, &str)] = &[
    (0x00, "end"),
    (0x04, "next-frame"),
    (0x05, "prev-frame"),
    (0x06, "play"),
    (0x07, "stop"),
    (0x0a, "add"),
    (0x0b, "subtract"),
    (0x0c, "multiply"),
    (0x0d, "divide"),
    (0x0e, "equals"),
    (0x0f, "less"),
    (0x10, "and"),
    (0x11, "or"),
    (0x12, "not"),
    (0x17, "pop"),
    (0x18, "to-integer"),
    (0x1c, "get-variable"),
    (0x1d, "set-variable"),
    (0x20, "set-target-2"),
    (0x21, "string-add"),
    (0x22, "get-property"),
    (0x23, "set-property"),
    (0x26, "trace"),
    (0x3a, "delete"),
    (0x3c, "define-local"),
    (0x3d, "call-function"),
    (0x3e, "return"),
    (0x3f, "modulo"),
    (0x40, "new-object"),
    (0x44, "typeof"),
    (0x45, "target-path"),
    (0x47, "add-2"),
    (0x48, "less-2"),
    (0x49, "equals-2"),
    (0x4a, "to-number"),
    (0x4b, "to-string"),
    (0x4c, "push-duplicate"),
    (0x4d, "stack-swap"),
    (0x4e, "get-member"),
    (0x4f, "set-member"),
    (0x50, "increment"),
    (0x51, "decrement"),
    (0x52, "call-method"),
    (0x53, "new-method"),
    (0x60, "bit-and"),
    (0x61, "bit-or"),
    (0x62, "bit-xor"),
    (0x63, "bit-lshift"),
    (0x64, "bit-rshift"),
    (0x65, "bit-urshift"),
    (0x66, "strict-equals"),
    (0x67, "greater"),
    (0x81, "goto-frame"),
    (0x83, "get-url"),
    (0x84, "store-register"),
    (0x85, "constant-pool"),
    (0x87, "set-target"),
    (0x88, "goto-label"),
    (0x8a, "wait-for-frame"),
    (0x8b, "set-tab-index"),
    (0x96, "push"),
    (0x99, "jump"),
    (0x9b, "define-function"),
    (0x9d, "if"),
    (0x9e, "call"),
    (0x9f, "goto-frame-2"),
];

pub fn name_of(code: u8) -> Option<&'static str> {
    DECODED
        .iter()
        .find(|(id, _)| *id == code)
        .map(|(_, name)| *name)
}

/// One decoded instruction: its opcode, its name, and its payload exactly as the file states it. The
/// payload is kept, not interpreted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub code: u8,
    pub name: &'static str,
    pub payload: Vec<u8>,
}

/// A refusal that names what it could not read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub code: &'static str,
    pub message: String,
}

/// **Lower one action record.** Every byte is accounted for: a record that ends mid-field is refused,
/// an opcode outside the table is refused with its number, and a push value whose framing does not
/// add up is refused rather than guessed at.
pub fn lower(tag: u16, record: &[u8]) -> Result<Vec<Instruction>, Refusal> {
    let mut out = Vec::new();
    // **`DoInitAction` states a character before its actions.** The record is `[id: u16][actions]`,
    // so the action stream starts at byte 2; reading it from byte 0 turns an id into an opcode.
    let mut at = if tag == 59 {
        if record.len() < 2 {
            return Err(Refusal {
                code: "avm1-record-truncated",
                message: "a DoInitAction record ends before its character id".to_string(),
            });
        }
        2
    } else {
        0
    };
    while at < record.len() {
        let code = record[at];
        let name = match name_of(code) {
            Some(name) => name,
            None => {
                return Err(Refusal {
                    code: "avm1-opcode-unimplemented",
                    message: format!(
                        "action 0x{code:02x} at byte {at} is not in the decoder's table"
                    ),
                })
            }
        };
        let (payload_start, payload_len) = if code >= 0x80 {
            if record.len() < at + 3 {
                return Err(Refusal {
                    code: "avm1-record-truncated",
                    message: format!("action 0x{code:02x} at byte {at} has no length field"),
                });
            }
            (
                at + 3,
                u16::from_le_bytes([record[at + 1], record[at + 2]]) as usize,
            )
        } else {
            (at + 1, 0)
        };
        let end = payload_start + payload_len;
        if end > record.len() {
            return Err(Refusal {
                code: "avm1-record-truncated",
                message: format!(
                    "action 0x{code:02x} at byte {at} declares {payload_len} bytes, {} remain",
                    record.len().saturating_sub(payload_start)
                ),
            });
        }
        let payload = record[payload_start..end].to_vec();
        if code == 0x96 {
            pushes(&payload, at)?;
        }
        out.push(Instruction { code, name, payload });
        at = end;
        if code == 0x00 {
            break;
        }
    }
    Ok(out)
}

/// Walk a push payload's values: type 0 is a null-terminated string, the rest have fixed widths. An
/// unknown type is refused by number.
fn pushes(payload: &[u8], at: usize) -> Result<(), Refusal> {
    let mut i = 0usize;
    while i < payload.len() {
        let kind = payload[i];
        i += 1;
        let width = match kind {
            0 => match payload[i..].iter().position(|byte| *byte == 0) {
                Some(end) => {
                    i += end + 1;
                    continue;
                }
                None => {
                    return Err(Refusal {
                        code: "avm1-push-unterminated",
                        message: format!("push at byte {at} has an unterminated string value"),
                    })
                }
            },
            1 => 4,
            2 | 3 => 0,
            4 | 5 | 8 => 1,
            6 => 8,
            7 => 4,
            9 => 2,
            other => {
                return Err(Refusal {
                    code: "avm1-push-type-unimplemented",
                    message: format!("push value type {other} at byte {at} is not in the table"),
                })
            }
        };
        if i + width > payload.len() {
            return Err(Refusal {
                code: "avm1-push-truncated",
                message: format!(
                    "push value at byte {at} declares {width} bytes inside a short payload"
                ),
            });
        }
        i += width;
    }
    Ok(())
}

/// The canonical text the package carries: one line per instruction, with its opcode, name and payload
/// as hex. A program lane reads this; a person can diff it.
pub fn canonical(instructions: &[Instruction]) -> String {
    let mut text = String::new();
    for instruction in instructions {
        let mut hex = String::new();
        for byte in &instruction.payload {
            hex.push_str(&format!("{byte:02x}"));
        }
        text.push_str(&format!(
            "op\t{:02x}\t{}\t{}\t{}\n",
            instruction.code,
            instruction.name,
            instruction.payload.len(),
            hex
        ));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_length_field_is_refused_with_its_opcode() {
        let refusal = lower(12, &[0x96, 0x01]).expect_err("a push with no length");
        assert_eq!(refusal.code, "avm1-record-truncated");
        assert!(refusal.message.contains("0x96"));
    }

    #[test]
    fn a_do_init_action_record_starts_after_its_character_id() {
        // id 0x1234, then stop, then end: read from byte 0 the id would be two opcodes.
        let record = [0x34, 0x12, 0x07, 0x00];
        let instructions = lower(59, &record).expect("a decodable record");
        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0].name, "stop");
    }

    #[test]
    fn an_opcode_outside_the_table_is_refused_by_number() {
        let refusal = lower(12, &[0x77]).expect_err("an unknown action");
        assert_eq!(refusal.code, "avm1-opcode-unimplemented");
        assert!(refusal.message.contains("0x77"));
    }

    #[test]
    fn a_push_payload_is_walked_and_its_values_kept() {
        // push "a" (00 61 00), integer 7 (07 07 00 00 00), then end
        let record = [
            0x96, 0x08, 0x00, 0x00, 0x61, 0x00, 0x07, 0x07, 0x00, 0x00, 0x00, 0x00,
        ];
        let instructions = lower(12, &record).expect("a decodable record");
        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0].name, "push");
        assert_eq!(instructions[1].code, 0x00);
    }

    #[test]
    fn an_unterminated_push_string_is_refused() {
        let refusal = lower(12, &[0x96, 0x02, 0x00, 0x00, 0x41]).expect_err("unterminated string");
        assert_eq!(refusal.code, "avm1-push-unterminated");
    }

    #[test]
    fn the_canonical_form_is_one_line_per_instruction() {
        let instructions = lower(12, &[0x07, 0x00]).expect("stop then end");
        assert_eq!(canonical(&instructions).lines().count(), 2);
    }
}
