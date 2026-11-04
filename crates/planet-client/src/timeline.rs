//! The timeline: the frames a mounted package carries, and the movie's own clock.
//!
//! A package's bound frame names its later frames in a `[timeline]` section, each as a carried
//! artifact with the order the client advances it in (`planet_package::cellframe::TimelineFrame`).
//! The mount resolves every one of them against the window they address, so this module does no
//! reading, no measuring and no guessing: it asks the mount for the window's frames in order.
//!
//! **The clock is the package layer's and it is the movie's own.** [`planet_package::cadence`]
//! converts elapsed nanoseconds into exact frame ticks at the source's fixed-point frame rate, capped
//! and counted, and the tick selects the frame. The client offers one opportunity per displayed frame
//! and draws whatever the script tick says; the display's refresh rate and the movie's rate are
//! independent, and a faster display redraws a frame rather than inventing one between frames.
//!
//! **A tick selects a carried frame.** It never answers a composited guess between two of them and
//! never invents a frame when the package carries none.

use planet_package::cadence::{Due, FrameRate, TickStream, NANOS_PER_SECOND};
use planet_package::CellFrame;

use crate::cell_mount::Adopted;

/// A window's carried frames and the movie clock that advances them.
#[derive(Clone, Debug, PartialEq)]
pub struct Timeline<'a> {
    window: String,
    frames: Vec<&'a CellFrame>,
    stream: TickStream,
}

impl<'a> Timeline<'a> {
    /// The timeline of `window` in an adopted package, with no clock until a rate is declared.
    pub fn of(adopted: &'a Adopted, window: &str) -> Timeline<'a> {
        Timeline {
            window: window.to_string(),
            frames: adopted.frames_of(window),
            stream: TickStream::at(FrameRate::units(0)),
        }
    }

    /// The timeline on the movie's declared frame rate — the source's own number, read from the
    /// package's surface, never a client default.
    pub fn at_rate(mut self, fps: f32) -> Timeline<'a> {
        self.stream = TickStream::at_rate(f64::from(fps));
        self
    }

    pub fn window(&self) -> &str {
        &self.window
    }

    /// How many frames the package carries for this window.
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// The frames themselves, in the order the package advances them.
    pub fn frames(&self) -> &[&'a CellFrame] {
        &self.frames
    }

    /// The movie's frame rate, as the source states it (`units / 256`).
    pub fn frame_rate(&self) -> FrameRate {
        self.stream.rate()
    }

    /// The movie's frame rate in frames per second. `0` when the package states none.
    pub fn rate(&self) -> f32 {
        self.stream.rate().frames_per_second() as f32
    }

    /// **Offer one displayed frame**: `nanos` since the previous one. The package manager decides
    /// how many movie ticks that time earned; the client does not wait for the movie to draw.
    pub fn offer(&mut self, nanos: u128) -> Due {
        self.stream.offer(nanos)
    }

    /// The script tick, which is the carried frame the movie is on.
    pub fn tick(&self) -> u64 {
        self.stream.tick()
    }

    /// How many displayed frames have been offered.
    pub fn opportunities(&self) -> u64 {
        self.stream.opportunities()
    }

    /// Movie ticks refused because one displayed frame took longer than
    /// [`planet_package::MAX_CATCH_UP`] frames.
    pub fn dropped(&self) -> u64 {
        self.stream.dropped()
    }

    /// How far through the current movie tick the clock is, `[0, 1)`.
    pub fn interpolation(&self) -> f64 {
        self.stream.interpolation()
    }

    /// The frame the script tick shows, or `None` when the window carries none or the movie declares
    /// no rate. The frame wraps: a movie loops.
    pub fn frame_at(&self, script_tick: u64) -> Option<&'a CellFrame> {
        if self.frames.is_empty() || !self.frame_rate().is_declared() {
            return None;
        }
        let index = (script_tick % self.frames.len() as u64) as usize;
        self.frames.get(index).copied()
    }

    /// The frame the script tick currently shows.
    pub fn shown(&self) -> Option<&'a CellFrame> {
        self.frame_at(self.stream.tick())
    }

    /// The seconds `script_tick` into the movie, at the declared rate — `None` when the package
    /// states no rate, because a guessed rate is a cadence the source never declared.
    pub fn seconds_at(&self, script_tick: u64) -> Option<f64> {
        if self.rate() <= 0.0 {
            return None;
        }
        Some(script_tick as f64 / f64::from(self.rate()))
    }

    /// The frame duration in whole nanoseconds, for a client that wants to print its clock.
    pub fn frame_nanos(&self) -> Option<u128> {
        let units = u128::from(self.frame_rate().units_value());
        if units == 0 {
            return None;
        }
        Some(NANOS_PER_SECOND * 256 / units)
    }
}
