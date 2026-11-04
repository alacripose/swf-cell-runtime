//! **The script driver: the code lane's execution handed to the timeline, one tick at a time.**
//!
//! `planet_package::program` reads a compiled cell program and runs it purely; this module is where
//! that execution meets the movie clock. Each script tick the driver runs the program from the tick
//! the clock reached and answers the tick the timeline should show, whether the timeline plays, and
//! the sound characters the program announced.
//!
//! **No program means no scripting, and that is stated.** The driver carries an `Option<Program>`;
//! without one it shows the clock's own tick, plays, and announces nothing. It never invents a goto,
//! a stop or a sound to fill the gap: a timeline that advances only because the clock did is a
//! different fact from one a script moved, and a run prints which of the two it is.
//!
//! **A program runs once, when it is presented, and the clock carries the timeline after it.** That
//! is what one program can honestly mean: it states the script's decision at the tick it was entered
//! — where the timeline starts, whether it plays, which sounds it starts — and it is not re-executed
//! on every later tick, because a re-executed `sound` would announce the same character sixty times a
//! second. A package whose script changes per frame is a package that carries a program per frame;
//! that needs admission, which `AGENTS.md` §12 has not ruled on, and this driver does not pretend
//! otherwise.
//!
//! A stop freezes the shown tick but not the clock: the movie clock keeps earning ticks (the display
//! and the process decide that, not the script), and after a stop the shown tick does not advance.
//! That is what "stopped" means for a viewer.

use planet_package::program::{Execution, Program, Refusal};

/// What one script tick decided.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Step {
    /// The movie tick the timeline shows.
    pub tick: u64,
    /// Whether the timeline is playing. `false` after the program stopped it.
    pub playing: bool,
    /// The sound characters the program announced on this tick, in order.
    pub sounds: Vec<String>,
    /// Whether a program ran at all.
    pub scripted: bool,
}

/// The movie clock's tick, filtered through the compiled program when one is present.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Driver {
    program: Option<Program>,
    /// The tick the timeline last showed. A stopped timeline keeps this tick while the clock moves.
    shown: u64,
    playing: bool,
    /// Set when a program was refused part-way: the driver then shows the clock's tick and reports
    /// the refusal rather than guessing what the program meant.
    refused: Option<(String, String)>,
    /// Whether the program has been run yet. A program runs once, on the tick it is presented.
    started: bool,
    /// The clock's tick on the previous script tick, so an unstopped timeline advances by exactly the
    /// ticks the clock earned rather than by a count of calls.
    last_due: u64,
}

impl Driver {
    /// A driver over an optional program. No program is a timeline with no scripting, and the driver
    /// says so in every [`Step`] rather than pretending the clock is a script.
    pub fn new(program: Option<Program>) -> Driver {
        Driver {
            program,
            shown: 0,
            playing: true,
            refused: None,
            started: false,
            last_due: 0,
        }
    }

    /// The program's identifier, when one is present.
    pub fn program(&self) -> Option<&str> {
        self.program.as_ref().map(|program| program.id.as_str())
    }

    /// The refusal a program raised, when one was refused during a tick.
    pub fn refused(&self) -> Option<(&str, &str)> {
        self.refused
            .as_ref()
            .map(|(code, message)| (code.as_str(), message.as_str()))
    }

    /// **One script tick.** `due_tick` is the tick the package's clock earned; the answer is what the
    /// timeline should show after the program (if any) has had its say.
    pub fn run(&mut self, due_tick: u64) -> Step {
        let Some(program) = self.program.as_ref() else {
            // No program: the clock decides, and the step says it was not scripted.
            self.shown = due_tick;
            self.playing = true;
            return Step {
                tick: self.shown,
                playing: true,
                sounds: Vec::new(),
                scripted: false,
            };
        };
        // **Already entered: the clock carries the timeline.** No program is re-executed, so a tick
        // announces its sounds once and a stopped timeline holds its frame while the clock runs on.
        if self.started {
            let delta = due_tick.saturating_sub(self.last_due);
            self.last_due = due_tick;
            if self.playing {
                self.shown = self.shown.saturating_add(delta);
            }
            return Step {
                tick: self.shown,
                playing: self.playing,
                sounds: Vec::new(),
                scripted: true,
            };
        }
        self.started = true;
        self.last_due = due_tick;
        match program.run(due_tick) {
            Ok(Execution {
                tick,
                playing,
                sounds,
                ..
            }) => {
                self.shown = tick;
                self.playing = playing;
                Step {
                    tick,
                    playing,
                    sounds,
                    scripted: true,
                }
            }
            Err(Refusal::Budget { .. }) => {
                self.refused = Some((
                    "program-budget".to_string(),
                    "the program ran past its step budget; the clock's tick is shown".to_string(),
                ));
                self.shown = due_tick;
                self.playing = true;
                Step {
                    tick: due_tick,
                    playing: true,
                    sounds: Vec::new(),
                    scripted: true,
                }
            }
            Err(refusal) => {
                self.refused = Some((refusal.code().to_string(), refusal.message()));
                self.shown = due_tick;
                self.playing = true;
                Step {
                    tick: due_tick,
                    playing: true,
                    sounds: Vec::new(),
                    scripted: true,
                }
            }
        }
    }

    /// The tick the timeline shows, without running anything.
    pub fn shown(&self) -> u64 {
        self.shown
    }

    /// Whether the timeline is playing, without running anything.
    pub fn playing(&self) -> bool {
        self.playing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use planet_package::program::HEADER;

    fn program(body: &str) -> Program {
        Program::parse(&format!(
            "{HEADER}\nversion\t1\nprogram\tdriver-test\n{body}"
        ))
        .expect("a readable program")
    }

    #[test]
    fn without_a_program_the_clock_decides_and_the_step_says_so() {
        let mut driver = Driver::new(None);
        let step = driver.run(40);
        assert_eq!(step.tick, 40);
        assert!(step.playing);
        assert!(!step.scripted, "an unscripted tick is stated as unscripted");
        assert!(step.sounds.is_empty());
    }

    #[test]
    fn a_goto_moves_the_shown_tick_away_from_the_clock() {
        let mut driver = Driver::new(Some(program("op\tgoto\t3\nop\tend\n")));
        let step = driver.run(40);
        assert_eq!(step.tick, 3, "the program's tick wins over the clock's");
        assert!(step.scripted);
    }

    #[test]
    fn a_stop_freezes_the_shown_tick_while_the_clock_runs_on() {
        let mut driver = Driver::new(Some(program("op\tstop\nop\tend\n")));
        let first = driver.run(10);
        assert_eq!(first.tick, 10);
        assert!(!first.playing);
        let later = driver.run(90);
        assert_eq!(later.tick, 10, "a stopped timeline does not advance");
        assert!(!later.playing);
        let much_later = driver.run(900);
        assert_eq!(much_later.tick, 10, "eighty more clock ticks change nothing");
    }

    #[test]
    fn a_playing_timeline_after_entry_advances_by_the_clock_not_by_calls() {
        let mut driver = Driver::new(Some(program("op\tgoto\t10\nop\tnext\nop\tend\n")));
        let entered = driver.run(40);
        assert_eq!(entered.tick, 11, "the program's goto and next decide the entry tick");
        let later = driver.run(90);
        assert_eq!(
            later.tick, 61,
            "fifty clock ticks after entry advance the shown tick by fifty, not to ninety"
        );
        assert!(later.playing);
        assert!(later.sounds.is_empty(), "a program is not re-executed per tick");
    }

    #[test]
    fn a_sound_is_announced_on_the_tick_that_starts_it() {
        let mut driver = Driver::new(Some(program("op\tsound\t7\nop\tend\n")));
        let step = driver.run(5);
        assert_eq!(step.sounds, vec!["7".to_string()]);
        let quiet = driver.run(6);
        assert!(quiet.sounds.is_empty(), "a later tick announces nothing new");
    }

    #[test]
    fn the_same_ticks_produce_the_same_steps() {
        let program = program("op\twait\t2\nop\tsound\t4\nop\tend\n");
        let mut first = Driver::new(Some(program.clone()));
        let mut second = Driver::new(Some(program));
        for tick in [0u64, 1, 40, 41] {
            assert_eq!(first.run(tick), second.run(tick));
        }
    }
}
