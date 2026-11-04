//! **The code lane**: a deterministic, bounded cell program, and the reader that validates it.
//!
//! The movie's ActionScript is behaviour, not decoration
//! (`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.6): a package that carries pictures and no
//! behaviour has carried the loading screen and stopped there. The plan also fixes who may read the
//! source: "The shell determines whether the source uses AVM1, AVM2, or both. It alone reads action
//! records, script bytes, source identifiers, names, class tables, string tables, imports/exports,
//! and runtime references. It compiles the required source semantics into the canonical cell-program
//! representation."
//!
//! That split is the whole reason this module is generic. It owns **the canonical cell-program
//! representation**: the grammar, the reader, refusal by name, and a bounded executor. It owns no
//! source name, offset, string table or decoded media, and it is proved on synthetic programs
//! (`docs/PORT-BASELINE.md`: "the shell privately builds source-derived AVM1/AVM2 coverage and test
//! vectors; agent work uses synthetic fixtures and generic schemas only").
//!
//! **Read, not admitted.** `AGENTS.md` §12 keeps the frame kind `code` refused at mount
//! (`frame-kind-reserved`) until the shell rules on admission, so this reader is exercised by the
//! runtime and its tests while a mounted package still refuses a code frame by name.
//! `tickets/TICKET-0080-cell-program-lane.md` carries that boundary.
//!
//! Everything here is pure and deterministic: the same program and the same starting tick produce
//! the same [`Execution`], on any machine, in any process, at any priority. The executor performs no
//! I/O, allocates no device and reads no package: a program says what the timeline does, and the
//! caller does it.

/// The document's first line. A code payload that does not begin with this is refused by name, so a
/// declaration frame, a picture or a truncated artifact is never mistaken for a program.
pub const HEADER: &str = "shploingus.cellprogram";

/// The only schema this reader speaks. A different number is refused with `found` and `supported`,
/// never converted — the same rule the frame reader applies to its own schema.
pub const VERSION: u16 = 1;

/// The most operations one program may declare. A ceiling, not a target: a program longer than this
/// is refused by name rather than truncated, because a truncated program is a different program.
pub const MAX_OPS: usize = 4_096;

/// The most operations one execution may run, so a malformed loop cannot spin. Execution is linear
/// here (there are no jumps between operations), and the budget is still enforced: a caller that
/// later adds a branch inherits the bound.
pub const MAX_STEPS: u32 = 4_096;

/// One operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    /// Move the script tick to an absolute movie tick.
    Goto(u64),
    /// Advance one movie frame.
    Next,
    /// Stop the timeline at its current tick: later ticks do not advance the movie.
    Stop,
    /// Resume the timeline from its current tick.
    Play,
    /// Hold the current tick for that many movie ticks.
    Wait(u64),
    /// Announce the sound character the current frame starts. The payload is the package's audio
    /// table's business; this names it.
    Sound(String),
    /// The end of the declared behaviour.
    End,
}

impl Op {
    pub fn name(&self) -> &'static str {
        match self {
            Op::Goto(_) => "goto",
            Op::Next => "next",
            Op::Stop => "stop",
            Op::Play => "play",
            Op::Wait(_) => "wait",
            Op::Sound(_) => "sound",
            Op::End => "end",
        }
    }
}

/// One refusal, by name, with the line it was read from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The document does not open with [`HEADER`].
    Header { found: String },
    /// A schema this reader does not speak.
    Version { found: u16, supported: u16 },
    /// No `program` row, or an empty identifier.
    Identity,
    /// A row whose first field is not a row this grammar declares.
    Row { line: usize, found: String },
    /// An operation this reader does not execute.
    Opcode { line: usize, found: String },
    /// An operand the operation does not state, states twice, or states unreadably.
    Operand {
        line: usize,
        found: String,
        why: &'static str,
    },
    /// More operations than [`MAX_OPS`].
    TooLong { found: usize },
    /// The execution ran past [`MAX_STEPS`].
    Budget { steps: u32 },
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::Header { .. } => "program-header",
            Refusal::Version { .. } => "program-version",
            Refusal::Identity => "program-identity",
            Refusal::Row { .. } => "program-row",
            Refusal::Opcode { .. } => "program-opcode",
            Refusal::Operand { .. } => "program-operand",
            Refusal::TooLong { .. } => "program-length",
            Refusal::Budget { .. } => "program-budget",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::Header { found } => format!(
                "the payload opens with {found:?}, not {HEADER:?}; a program is not a picture"
            ),
            Refusal::Version { found, supported } => format!(
                "program schema {found} is not the supported schema {supported}"
            ),
            Refusal::Identity => "no `program` row states the program's identifier".to_string(),
            Refusal::Row { line, found } => {
                format!("line {line}: row {found:?} is not one this grammar declares")
            }
            Refusal::Opcode { line, found } => {
                format!("line {line}: operation {found:?} is not one this reader executes")
            }
            Refusal::Operand { line, found, why } => {
                format!("line {line}: operand {found:?} {why}")
            }
            Refusal::TooLong { found } => {
                format!("{found} operations exceed the declared ceiling of {MAX_OPS}")
            }
            Refusal::Budget { steps } => {
                format!("the program ran {steps} operations, past its budget of {MAX_STEPS}")
            }
        }
    }
}

/// One program: its identifier and its operations, in the order the shell compiled them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub id: String,
    pub ops: Vec<Op>,
}

/// What one execution did. Every field is a fact the caller can act on or print; nothing here is a
/// guess about the intent behind the program.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Execution {
    /// The movie tick the timeline is on when the program finishes.
    pub tick: u64,
    /// Whether the timeline is playing (`false` after a `stop`, `true` after a `play`).
    pub playing: bool,
    /// The sound characters the program announced, in order.
    pub sounds: Vec<String>,
    /// How many operations ran.
    pub steps: u32,
    /// Whether an `end` was reached. A program without one ran out of operations, which is stated
    /// rather than assumed to be the same thing.
    pub ended: bool,
}

impl Program {
    /// **Read one program.** The document is lines of tab-separated fields: the header, a `version`
    /// row, a `program` row, then one `op` row per operation. Every row is validated; the first
    /// refusal stops the read and is returned by name with its line.
    pub fn parse(document: &str) -> Result<Program, Refusal> {
        let mut lines = document.lines().enumerate().filter(|(_, line)| !line.trim().is_empty());
        let Some((_, first)) = lines.next() else {
            return Err(Refusal::Header {
                found: String::new(),
            });
        };
        if first.trim_end() != HEADER {
            return Err(Refusal::Header {
                found: first.trim_end().to_string(),
            });
        }
        let mut version: Option<u16> = None;
        let mut id: Option<String> = None;
        let mut ops: Vec<Op> = Vec::new();
        for (index, line) in lines {
            let line = line.trim_end();
            let mut fields = line.split('\t');
            let kind = fields.next().unwrap_or_default();
            let rest: Vec<&str> = fields.collect();
            match kind {
                "version" => {
                    let found: u16 = rest
                        .first()
                        .copied()
                        .unwrap_or_default()
                        .parse()
                        .map_err(|_| Refusal::Operand {
                            line: index + 1,
                            found: rest.first().copied().unwrap_or_default().to_string(),
                            why: "is not a schema number",
                        })?;
                    if found != VERSION {
                        return Err(Refusal::Version {
                            found,
                            supported: VERSION,
                        });
                    }
                    version = Some(found);
                }
                "program" => {
                    let Some(name) = rest.first().copied() else {
                        return Err(Refusal::Identity);
                    };
                    let name = name.trim();
                    if name.is_empty() || name.split_whitespace().count() != 1 {
                        return Err(Refusal::Identity);
                    }
                    id = Some(name.to_string());
                }
                "op" => {
                    let Some(opcode) = rest.first().copied() else {
                        return Err(Refusal::Opcode {
                            line: index + 1,
                            found: String::new(),
                        });
                    };
                    let operands = &rest[1..];
                    ops.push(parse_op(index + 1, opcode, operands)?);
                    if ops.len() > MAX_OPS {
                        return Err(Refusal::TooLong { found: ops.len() });
                    }
                }
                other => {
                    return Err(Refusal::Row {
                        line: index + 1,
                        found: other.to_string(),
                    })
                }
            }
        }
        if version.is_none() {
            return Err(Refusal::Version {
                found: 0,
                supported: VERSION,
            });
        }
        let Some(id) = id else {
            return Err(Refusal::Identity);
        };
        Ok(Program { id, ops })
    }

    /// **Run one program from a starting tick**, bounded by [`MAX_STEPS`].
    ///
    /// The execution is linear and deterministic: no clock, no I/O, no allocation of a device. A
    /// caller that offers the same program and tick gets the same [`Execution`] every time, which is
    /// what lets a replay check that a scripted tick was scripted and not guessed.
    pub fn run(&self, start: u64) -> Result<Execution, Refusal> {
        let mut execution = Execution {
            tick: start,
            playing: true,
            ..Execution::default()
        };
        for op in &self.ops {
            if execution.steps >= MAX_STEPS {
                return Err(Refusal::Budget {
                    steps: execution.steps,
                });
            }
            execution.steps += 1;
            match op {
                Op::Goto(tick) => execution.tick = *tick,
                Op::Next => execution.tick = execution.tick.saturating_add(1),
                Op::Stop => execution.playing = false,
                Op::Play => execution.playing = true,
                Op::Wait(ticks) => execution.tick = execution.tick.saturating_add(*ticks),
                Op::Sound(id) => execution.sounds.push(id.clone()),
                Op::End => {
                    execution.ended = true;
                    break;
                }
            }
        }
        Ok(execution)
    }
}

fn number(line: usize, field: &str) -> Result<u64, Refusal> {
    field.parse::<u64>().map_err(|_| Refusal::Operand {
        line,
        found: field.to_string(),
        why: "is not a whole number",
    })
}

fn parse_op(line: usize, opcode: &str, operands: &[&str]) -> Result<Op, Refusal> {
    let expected = match opcode {
        "goto" | "wait" | "sound" => 1,
        "next" | "stop" | "play" | "end" => 0,
        other => {
            return Err(Refusal::Opcode {
                line,
                found: other.to_string(),
            })
        }
    };
    if operands.len() != expected {
        return Err(Refusal::Operand {
            line,
            found: operands.join(","),
            why: if expected == 0 {
                "is present where the operation states none"
            } else {
                "is missing or extra where the operation states one"
            },
        });
    }
    match opcode {
        "goto" => Ok(Op::Goto(number(line, operands[0])?)),
        "wait" => Ok(Op::Wait(number(line, operands[0])?)),
        "sound" => {
            let id = operands[0].trim();
            if id.is_empty() || id.split_whitespace().count() != 1 {
                return Err(Refusal::Operand {
                    line,
                    found: id.to_string(),
                    why: "does not name one sound character",
                });
            }
            Ok(Op::Sound(id.to_string()))
        }
        "next" => Ok(Op::Next),
        "stop" => Ok(Op::Stop),
        "play" => Ok(Op::Play),
        _ => Ok(Op::End),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program(body: &str) -> Program {
        Program::parse(&format!("{HEADER}\nversion\t{VERSION}\nprogram\ttest\n{body}"))
            .expect("a readable program")
    }

    #[test]
    fn a_program_is_read_in_order() {
        let parsed = program("op\tgoto\t40\nop\tstop\nop\tsound\t7\nop\tend\n");
        assert_eq!(parsed.id, "test");
        assert_eq!(
            parsed.ops,
            vec![Op::Goto(40), Op::Stop, Op::Sound("7".to_string()), Op::End]
        );
    }

    #[test]
    fn a_picture_is_not_a_program() {
        let refusal = Program::parse("[vocabulary]\npaper\tsurface\n").expect_err("refused");
        assert_eq!(refusal.code(), "program-header");
    }

    #[test]
    fn another_schema_is_refused_by_name_not_converted() {
        let document = format!("{HEADER}\nversion\t2\nprogram\ttest\n");
        let refusal = Program::parse(&document).expect_err("refused");
        assert_eq!(refusal.code(), "program-version");
        assert!(refusal.message().contains("2"), "the found schema is named");
        assert!(refusal.message().contains("1"), "the supported schema is named");
    }

    #[test]
    fn an_unknown_operation_is_refused_with_its_line() {
        let refusal = Program::parse(&format!(
            "{HEADER}\nversion\t{VERSION}\nprogram\ttest\nop\tteleport\t3\n"
        ))
        .expect_err("refused");
        assert_eq!(refusal.code(), "program-opcode");
        assert!(refusal.message().contains("line 4"));
    }

    #[test]
    fn an_operand_that_is_missing_or_extra_is_refused() {
        let missing = Program::parse(&format!(
            "{HEADER}\nversion\t{VERSION}\nprogram\ttest\nop\tgoto\n"
        ))
        .expect_err("refused");
        assert_eq!(missing.code(), "program-operand");
        let extra = Program::parse(&format!(
            "{HEADER}\nversion\t{VERSION}\nprogram\ttest\nop\tstop\t1\n"
        ))
        .expect_err("refused");
        assert_eq!(extra.code(), "program-operand");
        let unreadable = Program::parse(&format!(
            "{HEADER}\nversion\t{VERSION}\nprogram\ttest\nop\tgoto\tnear\n"
        ))
        .expect_err("refused");
        assert_eq!(unreadable.code(), "program-operand");
    }

    #[test]
    fn a_program_without_an_identity_is_refused() {
        let refusal = Program::parse(&format!("{HEADER}\nversion\t{VERSION}\nop\tnext\n"))
            .expect_err("refused");
        assert_eq!(refusal.code(), "program-identity");
    }

    #[test]
    fn a_row_the_grammar_does_not_declare_is_refused() {
        let refusal = Program::parse(&format!(
            "{HEADER}\nversion\t{VERSION}\nprogram\ttest\nfill\t0\t0\t1\tsurface\n"
        ))
        .expect_err("refused");
        assert_eq!(refusal.code(), "program-row");
    }

    #[test]
    fn a_run_moves_the_tick_and_reports_the_sounds() {
        let parsed = program("op\tgoto\t12\nop\tnext\nop\twait\t4\nop\tsound\t3\nop\tstop\nop\tend\n");
        let run = parsed.run(0).expect("executed");
        assert_eq!(run.tick, 17);
        assert_eq!(run.sounds, vec!["3".to_string()]);
        assert!(!run.playing, "the program stopped the timeline");
        assert!(run.ended, "the program reached its end");
        assert_eq!(run.steps, 6);
    }

    #[test]
    fn the_same_program_and_tick_always_produce_the_same_execution() {
        // **Determinism is the claim.** No clock, no I/O, no allocation of a device: a replay can
        // check that a scripted tick was scripted rather than guessed.
        let parsed = program("op\tgoto\t7\nop\tsound\t9\nop\tplay\nop\tend\n");
        let first = parsed.run(3).expect("executed");
        let second = parsed.run(3).expect("executed");
        assert_eq!(first, second);
    }

    #[test]
    fn a_program_that_never_ends_is_stated_as_unfinished() {
        let parsed = program("op\tnext\nop\tnext\n");
        let run = parsed.run(0).expect("executed");
        assert_eq!(run.tick, 2);
        assert!(!run.ended, "running out of operations is not an `end`");
    }

    #[test]
    fn too_many_operations_are_refused_rather_than_truncated() {
        let mut body = String::new();
        for _ in 0..=MAX_OPS {
            body.push_str("op\tnext\n");
        }
        let refusal =
            Program::parse(&format!("{HEADER}\nversion\t{VERSION}\nprogram\ttest\n{body}"))
                .expect_err("refused");
        assert_eq!(refusal.code(), "program-length");
    }

    #[test]
    fn a_wait_holds_the_tick_without_asking_the_clock() {
        let parsed = program("op\twait\t40\nop\tend\n");
        let run = parsed.run(100).expect("executed");
        assert_eq!(run.tick, 140);
    }
}
