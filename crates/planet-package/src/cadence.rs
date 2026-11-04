//! **The cadence: the movie's exact frame clock, scheduled in the package layer.**
//!
//! The governing plan is explicit, and it is the specification this module implements
//! (`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.5):
//!
//! > "The SWF frame rate is part of source semantics. **Do not hardcode `ala-cities`' 20 Hz tick
//! > rate.** Implement a deterministic scheduler that represents the movie's frame clock exactly
//! > (prefer rational/integer tick accumulation over floating-point drift), processes
//! > gameplay/timeline updates in source order, and lets `wgpu` render independently at the display's
//! > refresh cadence."
//!
//! So there is no 20 Hz here, and no floating point in the clock.
//!
//! # 1. The tick stream is unbounded by the CPU, and bounded only by the process's share of it
//!
//! A client offers one **opportunity** per event-loop turn — once per displayed frame — and nothing
//! waits for a tick to arrive. If the process is fast and unpinned the stream is fast; if it is
//! deprioritised, descheduled or sharing a core the stream is slow. **Neither changes the movie's
//! rate.** An opportunity is not a tick: it is a chance to convert elapsed time into ticks, and that
//! conversion is [`TickStream::offer`].
//!
//! # 2. The conversion is exact integer accumulation, never float drift
//!
//! A SWF frame rate is fixed-point `units / 256`. Time arrives as whole nanoseconds. The stream
//! carries `nanos × units` in a `u128` and divides by `256 × 10⁹` — so the frame clock is the source's
//! own rational number, and 7 offers of half a second at 20 fps are exactly 70 ticks with no
//! remainder. There is no `f64` in the arithmetic, and therefore no rounding to drift with.
//!
//! One offer is capped at [`MAX_CATCH_UP`] ticks and what it refused is **counted** (`dropped`), not
//! discarded: a stalled process shows the same ticks late, and its `dropped` says how many it could
//! not carry. The tick is what schedules the horizons ([`Cadence::is_due_at`], the family's
//! `voxel-shapers/src/stages.rs` rule), so process speed can change only **lateness**, never **which**
//! ticks a rung runs on.
//!
//! # 3. Drawing is not on the ladder, and is not the movie's clock either
//!
//! [`Horizon::Render`] has no period: `wgpu` draws at the display's refresh cadence, and a draw never
//! advances a tick. A display faster than the movie draws the same frame several times; a display
//! slower than the movie shows it fewer times. Both are the plan's "render independently", and
//! neither is a dropped movie frame — the movie's clock is the stream's, and the drawing only samples
//! it.
//!
//! # The numbers, and where they come from
//!
//! * The ladder's **periods** are the family's (`voxel-shapers/src/stages.rs:97-104`):
//!   `EveryTick = 1`, `Water = 4`, `Sediment = 20`, `Daily = TICKS_PER_DAY`, with its determinism
//!   note at `:106-118`. They are expressed here in **movie ticks**, because the plan forbids a
//!   second base clock; the family's 20 Hz base is not this movie's rate.
//! * `MAX_CATCH_UP = 4`, the fraction carried in ticks, and counting a refusal rather than discarding
//!   it are `shploingus-clean/crates/voxel-client/src/clock.rs`'s.
//! * `TICKS_PER_DAY` is the unscaled real day: the family scales its day by `REAL_METRES_PER_VOXEL`
//!   (`climate.rs:49-50`) and this package layer has no voxel scale to apply. A named difference, not
//!   a silent one.

/// The most ticks one offer may carry, whatever time passed. A stall is refused and **counted**,
/// never simulated: without this an unresponsive process would run every tick it missed at once and
/// the movie's speed would be a function of the renderer's worst frame.
pub const MAX_CATCH_UP: u32 = 4;

/// A SWF frame rate's denominator: the header states frames per second as a fixed-point
/// `u16 / 256`, and the clock keeps it that way rather than converting to a float.
pub const RATE_DENOMINATOR: u128 = 256;

/// Nanoseconds in a second, as the integer unit elapsed time arrives in.
pub const NANOS_PER_SECOND: u128 = 1_000_000_000;

/// The accumulator's unit per tick: nanoseconds × the rate denominator. Dividing by this converts
/// `nanos × units` into whole ticks.
pub const UNITS_PER_TICK: u128 = NANOS_PER_SECOND * RATE_DENOMINATOR;

/// Seconds in a real day, unscaled. See the module docs for the difference from the family's day.
pub const SECONDS_PER_DAY: u64 = 86_400;

/// One in-game day in **movie** ticks — the whole seconds the movie plays in a day, times its frame
/// rate. `Daily` is the rung that runs once per day; the number is kept as a constant so the rung is
/// readable and the family's `TICKS_PER_DAY` day length is the one intended.
pub const TICKS_PER_DAY: u64 = SECONDS_PER_DAY * 40;

/// The movie's frame rate, exactly as the source states it: `units / 256` frames per second.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct FrameRate {
    units: u16,
}

impl FrameRate {
    /// The source's own fixed-point rate. `0` is a movie that states none.
    pub const fn units(units: u16) -> FrameRate {
        FrameRate { units }
    }

    /// A rate from a decimal frame rate, snapped to the source's `1/256` grid. The importer already
    /// reads the header's `u16`; this is for a caller that only has the printed number.
    pub fn approximating(fps: f64) -> FrameRate {
        if !fps.is_finite() || fps <= 0.0 {
            return FrameRate { units: 0 };
        }
        let units = (fps * 256.0).round();
        FrameRate {
            units: units.clamp(0.0, f64::from(u16::MAX)) as u16,
        }
    }

    pub const fn units_value(self) -> u16 {
        self.units
    }

    pub fn frames_per_second(self) -> f64 {
        f64::from(self.units) / 256.0
    }

    /// Whether the movie states a rate at all.
    pub fn is_declared(self) -> bool {
        self.units > 0
    }
}

/// How often a rung of the ladder is asked, in **movie ticks**.
///
/// **Part of the determinism contract, not a performance knob** (family `stages.rs:66-68`): a rung
/// that ran at a different rate is a different world. Nothing here stores anything — a period says
/// *when* work is asked, and the answer is a pure function of the tick.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Cadence {
    /// Every tick: what the player just did, and the meshes that dirtied.
    EveryTick,
    /// ×4: water and settling.
    Water,
    /// ×20: erosion and deposition — the material that moves.
    Sediment,
    /// One in-game day: geology and climate.
    Daily,
}

impl Cadence {
    pub const ALL: [Cadence; 4] = [
        Cadence::EveryTick,
        Cadence::Water,
        Cadence::Sediment,
        Cadence::Daily,
    ];

    pub const COUNT: usize = 4;

    pub const fn period_ticks(self) -> u64 {
        match self {
            Cadence::EveryTick => 1,
            Cadence::Water => 4,
            Cadence::Sediment => 20,
            Cadence::Daily => TICKS_PER_DAY,
        }
    }

    /// Whether a rung with this cadence is **due** at `tick`.
    ///
    /// One line, because there is nothing else to schedule: a period that divides the tick is asked
    /// this tick. There is no accumulator and no catch-up here — the stream already refused what it
    /// could not carry, so a rung never has to ask "how many times did I miss?". Tick 0 is due for
    /// every rung, so a world asked for the first time is complete rather than one period stale.
    pub const fn is_due_at(self, tick: u64) -> bool {
        tick % self.period_ticks() == 0
    }

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn name(self) -> &'static str {
        match self {
            Cadence::EveryTick => "every-tick",
            Cadence::Water => "water",
            Cadence::Sediment => "sediment",
            Cadence::Daily => "daily",
        }
    }
}

/// One thing the tick drives. Rendering is one of them **and is the only one without a period**.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Horizon {
    /// The picture. Drawn at the display's refresh cadence, between ticks, never advancing one.
    Render,
    /// The movie's timeline: one carried frame per tick, in source order.
    Script,
    /// The world's logic rung.
    Logic,
    /// The world's physics rung.
    Physics,
}

impl Horizon {
    pub const ALL: [Horizon; 4] = [
        Horizon::Render,
        Horizon::Script,
        Horizon::Logic,
        Horizon::Physics,
    ];

    pub const COUNT: usize = 4;

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn name(self) -> &'static str {
        match self {
            Horizon::Render => "render",
            Horizon::Script => "script",
            Horizon::Logic => "logic",
            Horizon::Physics => "physics",
        }
    }
}

/// The ladder's assignment to the horizons.
///
/// `Script` is the tick itself — one movie frame per tick, in source order, which is what §4.5 asks
/// for. `Logic` and `Physics` take the family's `Water` and `Sediment` rungs. Those two assignments
/// are this port's declaration; the rungs and the rule that reads them are the family's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Horizons {
    logic: Cadence,
    physics: Cadence,
}

impl Default for Horizons {
    fn default() -> Self {
        Horizons {
            logic: Cadence::Water,
            physics: Cadence::Sediment,
        }
    }
}

impl Horizons {
    pub fn at() -> Horizons {
        Horizons::default()
    }

    pub fn period(&self, horizon: Horizon) -> Option<u64> {
        match horizon {
            Horizon::Render => None,
            Horizon::Script => Some(Cadence::EveryTick.period_ticks()),
            Horizon::Logic => Some(self.logic.period_ticks()),
            Horizon::Physics => Some(self.physics.period_ticks()),
        }
    }

    pub fn cadence(&self, horizon: Horizon) -> Option<Cadence> {
        match horizon {
            Horizon::Render => None,
            Horizon::Script => Some(Cadence::EveryTick),
            Horizon::Logic => Some(self.logic),
            Horizon::Physics => Some(self.physics),
        }
    }

    /// Which horizons are due **at** `tick`. Rendering is never due here: it is not on the ladder.
    pub fn due_at(&self, tick: u64) -> [bool; Horizon::COUNT] {
        let mut due = [false; Horizon::COUNT];
        for horizon in Horizon::ALL {
            if let Some(period) = self.period(horizon) {
                due[horizon.index()] = tick % period == 0;
            }
        }
        due
    }
}

/// What one offer bought: the ticks it moved, what it refused, and the fraction it carried.
#[derive(Clone, Debug, PartialEq)]
pub struct Due {
    /// Opportunities offered so far, including this one. **Not** ticks: a fast process banks far more
    /// of these than it earns ticks, and that surplus is the drawing it gets to do between them.
    pub opportunities: u64,
    /// The tick after the offer. One tick is one movie frame.
    pub tick: u64,
    /// Ticks this offer moved, `0..=MAX_CATCH_UP`.
    pub ticks: u32,
    /// Ticks refused by [`MAX_CATCH_UP`] on this offer.
    pub dropped: u64,
    /// `Some` when the offer was not a duration (no declared rate) and moved nothing.
    pub refused: Option<String>,
    /// The sub-tick remainder carried forward, in `nanos × rate units`.
    pub remainder_units: u128,
    /// Which horizons ran in this offer's tick span. `Render` is always `false`.
    pub due: [bool; Horizon::COUNT],
}

impl Due {
    pub fn runs(&self, horizon: Horizon) -> bool {
        self.due[horizon.index()]
    }

    /// The horizons that ran, in ladder order, excluding rendering (which has no ticks).
    pub fn ran(&self) -> Vec<Horizon> {
        Horizon::ALL
            .iter()
            .copied()
            .filter(|horizon| *horizon != Horizon::Render && self.runs(*horizon))
            .collect()
    }
}

/// **The tick stream: opportunities and elapsed nanoseconds in, exact movie ticks out.**
#[derive(Clone, PartialEq, Debug)]
pub struct TickStream {
    opportunities: u64,
    tick: u64,
    /// Carried `nanos × units`, below [`UNITS_PER_TICK`]. Integer, so nothing drifts.
    units_accum: u128,
    dropped: u64,
    rate: FrameRate,
    horizons: Horizons,
}

impl TickStream {
    /// A stream on the movie's exact rate. A rate of `0` is a movie that declares none: `offer` then
    /// refuses and the tick stays at zero, which is a state, not a default.
    pub fn at(rate: FrameRate) -> TickStream {
        TickStream {
            opportunities: 0,
            tick: 0,
            units_accum: 0,
            dropped: 0,
            rate,
            horizons: Horizons::at(),
        }
    }

    /// The same, from the decimal frame rate a caller printed. See [`FrameRate::approximating`].
    pub fn at_rate(fps: f64) -> TickStream {
        TickStream::at(FrameRate::approximating(fps))
    }

    pub fn rate(&self) -> FrameRate {
        self.rate
    }

    pub fn horizons(&self) -> &Horizons {
        &self.horizons
    }

    pub fn opportunities(&self) -> u64 {
        self.opportunities
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn dropped(&self) -> u64 {
        self.dropped
    }

    /// How far through the current tick the clock is, `[0, 1)`. For a renderer that draws between
    /// ticks; this client draws whole carried frames, so it reads the tick, and the fraction is kept
    /// because nothing else should have to add a second accumulator to interpolate.
    pub fn interpolation(&self) -> f64 {
        (self.units_accum as f64 / UNITS_PER_TICK as f64).clamp(0.0, 1.0)
    }

    /// The arithmetic of one offer, without mutating the stream.
    pub fn due(&self, nanos: u128) -> Due {
        let mut due = Due {
            opportunities: self.opportunities + 1,
            tick: self.tick,
            ticks: 0,
            dropped: 0,
            refused: None,
            remainder_units: self.units_accum,
            due: [false; Horizon::COUNT],
        };
        if !self.rate.is_declared() {
            due.refused = Some("the movie declares no frame rate".to_string());
            return due;
        }
        // **Time becomes ticks once, here**, as an integer: `nanos × units` over `256 × 10⁹`. No
        // float, so a 20 fps movie fed seven half-seconds is exactly 70 ticks and not 69.
        let carried = self
            .units_accum
            .saturating_add(nanos.saturating_mul(u128::from(self.rate.units_value())));
        let whole = carried / UNITS_PER_TICK;
        let owed = u64::try_from(whole).unwrap_or(u64::MAX);
        let moved = owed.min(u64::from(MAX_CATCH_UP));
        due.ticks = moved as u32;
        due.dropped = owed - moved;
        due.tick = self.tick.saturating_add(moved);
        due.remainder_units = carried % UNITS_PER_TICK;
        due.due = horizons_due(&self.horizons, self.tick + 1, due.tick);
        due
    }

    /// Commit a computed [`Due`] to the stream.
    fn apply(&mut self, due: Due) -> Due {
        self.opportunities = due.opportunities;
        self.tick = due.tick;
        self.units_accum = due.remainder_units;
        self.dropped = self.dropped.saturating_add(due.dropped);
        due
    }

    /// **Offer one opportunity**: this displayed frame, `nanos` since the previous one.
    pub fn offer(&mut self, nanos: u128) -> Due {
        self.apply(self.due(nanos))
    }

    /// **Offer `opportunities` displays, each `nanos_each` after the last.** A window loop offers
    /// one per displayed frame; a batched caller (a test proving the stream is unbounded, a replay)
    /// offers the count it actually made, and each of those is a separate opportunity with its own
    /// elapsed time and its own cap — a batch is not one stall.
    pub fn offer_many(&mut self, opportunities: u64, nanos_each: u128) -> Due {
        if opportunities == 0 {
            return Due {
                opportunities: self.opportunities,
                tick: self.tick,
                ticks: 0,
                dropped: 0,
                refused: None,
                remainder_units: self.units_accum,
                due: [false; Horizon::COUNT],
            };
        }
        let mut ticks = 0u32;
        let mut dropped = 0u64;
        let mut due = [false; Horizon::COUNT];
        let mut last = self.due(nanos_each);
        for _ in 0..opportunities {
            last = self.apply(self.due(nanos_each));
            ticks = ticks.saturating_add(last.ticks);
            dropped = dropped.saturating_add(last.dropped);
            for (slot, ran) in due.iter_mut().zip(last.due.iter()) {
                *slot |= *ran;
            }
        }
        Due {
            opportunities: self.opportunities,
            tick: self.tick,
            ticks,
            dropped,
            refused: last.refused,
            remainder_units: self.units_accum,
            due,
        }
    }
}

/// Which horizons a rung is due for over `first..=last`. At most [`MAX_CATCH_UP`] ticks, so this is
/// bounded work even when the offer was refused.
fn horizons_due(horizons: &Horizons, first: u64, last: u64) -> [bool; Horizon::COUNT] {
    let mut due = [false; Horizon::COUNT];
    if first > last {
        return due;
    }
    for tick in first..=last {
        for horizon in Horizon::ALL {
            if let Some(period) = horizons.period(horizon) {
                if tick % period == 0 {
                    due[horizon.index()] = true;
                }
            }
        }
    }
    due
}

#[cfg(test)]
mod tests {
    use super::*;

    const F40: FrameRate = FrameRate::units(10_240); // 40 fps exactly: 40 × 256
    const F20: FrameRate = FrameRate::units(5_120); // 20 fps exactly

    #[test]
    fn the_ladder_is_the_family_ladder() {
        assert_eq!(Cadence::EveryTick.period_ticks(), 1, "player edits and dirty meshes");
        assert_eq!(Cadence::Water.period_ticks(), 4, "water and settling");
        assert_eq!(Cadence::Sediment.period_ticks(), 20, "erosion and deposition");
        assert_eq!(Cadence::Daily.period_ticks(), TICKS_PER_DAY, "geology and climate");
        assert_eq!(Cadence::ALL.len(), 4, "the ladder has four rungs and no others");
        assert_eq!(
            Horizons::at().period(Horizon::Script),
            Some(1),
            "the movie's script horizon is the tick itself"
        );
    }

    #[test]
    fn a_rung_is_due_exactly_when_the_tick_is_a_multiple_of_its_period() {
        for tick in 0..100 {
            assert!(Cadence::EveryTick.is_due_at(tick), "tick {tick}");
            assert_eq!(Cadence::Water.is_due_at(tick), tick % 4 == 0, "tick {tick}");
            assert_eq!(Cadence::Sediment.is_due_at(tick), tick % 20 == 0, "tick {tick}");
        }
        assert!(!Cadence::Sediment.is_due_at(19));
        assert!(Cadence::Sediment.is_due_at(20));
        let horizons = Horizons::at();
        // A slower rung never runs alone: its period is a multiple of every faster one.
        for tick in 0..=200u64 {
            if horizons.due_at(tick)[Horizon::Physics.index()] {
                assert!(horizons.due_at(tick)[Horizon::Logic.index()], "tick {tick}: alone");
                assert!(horizons.due_at(tick)[Horizon::Script.index()], "tick {tick}: alone");
            }
        }
    }

    #[test]
    fn a_second_of_sixty_fps_offers_is_exactly_the_declared_rate() {
        let mut stream = TickStream::at(F40);
        let mut offers = 0;
        while stream.tick() < 40 && offers < 100_000 {
            stream.offer(NANOS_PER_SECOND / 60);
            offers += 1;
        }
        assert_eq!(stream.tick(), 40, "a second of displayed frames was not 40 movie frames");
        assert_eq!(stream.dropped(), 0, "a normal display rate was treated as a stall");
        assert!((59..=61).contains(&offers), "it took {offers} offers to reach a second");
    }

    #[test]
    fn ten_thousand_opportunities_in_a_second_are_still_forty_movie_ticks() {
        // **The claim the window makes.** The loop is unbounded by the CPU: it offered ten thousand
        // times in one second of real time. The package manager converted *time* into ticks, so the
        // movie advanced forty frames and the other 9 960 offers are the drawing between them.
        let mut stream = TickStream::at(F40);
        // Ten thousand displayed frames in one second: 0.1 ms each, so no single offer is a stall.
        let due = stream.offer_many(10_000, NANOS_PER_SECOND / 10_000);
        assert_eq!(due.opportunities, 10_000);
        assert_eq!(due.ticks, 40, "the process rate leaked into the movie rate");
        assert_eq!(stream.dropped(), 0, "a fast process must not drop ticks");
    }

    #[test]
    fn the_process_rate_does_not_change_the_movie_rate() {
        // One second of 60 fps offers and one second of three third-second offers: the same forty
        // ticks, the same frames, the same horizons. This is what "unbound to CPU time" has to mean.
        let mut fast = TickStream::at(F40);
        while fast.tick() < 40 {
            fast.offer(NANOS_PER_SECOND / 60);
        }
        let mut coarse = TickStream::at(F40);
        // Ten offers of 100 ms: four ticks each, exactly at the cap, so nothing is refused.
        for _ in 0..10 {
            coarse.offer(NANOS_PER_SECOND / 10);
        }
        assert_eq!(fast.tick(), 40);
        assert_eq!(coarse.tick(), 40, "the offer rate changed the movie rate");
        assert_eq!(fast.dropped(), 0);
        assert_eq!(coarse.dropped(), 0);
    }

    #[test]
    fn three_and_a_half_seconds_at_forty_fps_are_exactly_one_hundred_and_forty_ticks() {
        // **No float drift.** 3.5 s at 40 fps is 140 frames. Fed as 35 offers of exactly 100 ms
        // (four frames each, at the cap), the result is exactly 140 with no remainder. The family's
        // `clock.rs` records the mould this avoids: seconds held as `f64` and divided by the
        // reciprocal came out a tick short.
        let mut stream = TickStream::at(F40);
        for _ in 0..35 {
            stream.offer(NANOS_PER_SECOND / 10);
        }
        assert_eq!(stream.tick(), 140);
        assert_eq!(stream.dropped(), 0);
        assert!(stream.interpolation() < f64::EPSILON, "a remainder survived");
    }

    #[test]
    fn a_quarter_of_a_frame_four_times_is_one_tick() {
        // 40 fps is 25 ms a frame; a quarter of that, four times, is one tick with nothing lost.
        let quarter = NANOS_PER_SECOND / 160;
        let mut stream = TickStream::at(F40);
        for _ in 0..3 {
            assert_eq!(stream.offer(quarter).ticks, 0, "a sub-frame offer moved the movie");
        }
        assert_eq!(stream.offer(quarter).ticks, 1, "the fourth quarter did not complete a frame");
        assert_eq!(stream.tick(), 1);
    }

    #[test]
    fn a_stalled_process_drops_ticks_and_the_phase_survives() {
        // Ten seconds of being descheduled: four hundred frames owed at 40 fps, four carried, 396
        // refused and counted. The tick that results still schedules by period, so the next water
        // tick is a water tick — starvation delays the movie and cannot rephase it.
        let mut stream = TickStream::at(F40);
        let due = stream.offer(10 * NANOS_PER_SECOND);
        assert_eq!(due.ticks, MAX_CATCH_UP);
        assert_eq!(due.dropped, 396, "the refused ticks were not counted");
        assert_eq!(stream.tick(), 4);
        assert!(stream.horizons().due_at(stream.tick())[Horizon::Logic.index()]);
        assert!(!stream.horizons().due_at(stream.tick() + 1)[Horizon::Logic.index()]);
    }

    #[test]
    fn drawing_has_no_period_and_advances_nothing() {
        let horizons = Horizons::at();
        for tick in 0..64 {
            assert!(!horizons.due_at(tick)[Horizon::Render.index()], "render was put on the ladder");
        }
        assert_eq!(horizons.period(Horizon::Render), None);
        let mut stream = TickStream::at(F40);
        let due = stream.offer(0);
        assert_eq!(due.ticks, 0, "a draw advanced the tick");
        assert!(due.ran().is_empty(), "a draw ran a rung");
    }

    #[test]
    fn a_display_faster_than_the_movie_redraws_the_same_frame_without_dropping_one() {
        // 120 fps display, 40 fps movie: every third draw advances a frame, and no frame is lost.
        let mut stream = TickStream::at(F40);
        let mut frames = Vec::new();
        let mut last = 0u64;
        while stream.tick() < 40 {
            stream.offer(NANOS_PER_SECOND / 120);
            while last < stream.tick() {
                frames.push(last);
                last += 1;
            }
        }
        assert_eq!(frames, (0..40).collect::<Vec<u64>>(), "the movie skipped or repeated a frame");
    }

    #[test]
    fn a_movie_with_no_declared_rate_has_no_cadence_to_invent() {
        let mut stream = TickStream::at(FrameRate::units(0));
        let due = stream.offer(NANOS_PER_SECOND);
        assert_eq!(due.ticks, 0);
        assert!(due.refused.is_some());
        assert_eq!(stream.tick(), 0);
        assert!(!FrameRate::approximating(f64::NAN).is_declared());
    }

    #[test]
    fn the_source_rate_is_fixed_point_and_read_back_exactly() {
        assert_eq!(FrameRate::units(10_240).frames_per_second(), 40.0);
        assert_eq!(FrameRate::approximating(40.0).units_value(), 10_240);
        // A rate that is not on the 1/256 grid keeps the nearest value the source could state.
        assert_eq!(FrameRate::approximating(29.97).units_value(), 7_672);
    }
}
