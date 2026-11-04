//! **The score: a carried frame's sounds start on the tick that frame is shown.**
//!
//! A package carries its audio characters once and names, per timeline frame, which character that
//! frame starts (`planet_package::cellframe::SoundEvent`, resolved by the mount into each
//! [`CellFrame`]'s own `audio` list). Until this module existed the client decoded those clips in the
//! headless inspection and never attached them to the clock: a frame's sound was readable and never
//! *started*, so a mounted movie was silent no matter how many payloads it carried.
//!
//! [`Score`] is that attachment and nothing else. It decodes every carried frame's clips **once**, and
//! on the tick the loop shows a frame it queues that frame's clips into the client's own
//! [`Channel`]. It starts them when the **shown frame changes**, not on every redraw: a movie that
//! holds one frame for sixty opportunities starts that frame's sound once, and a movie that wraps back
//! to a frame starts it again, because that is a new visit and not a redraw.
//!
//! Three honest limits are stated here rather than papered over:
//!
//! - **No device.** Nothing in this repository opens an audio output device. [`Score::render`] pulls
//!   samples out of the channel at the caller's block size, which is what proves a start produced
//!   audio; handing those samples to hardware is the shell's or the platform's business and is not
//!   claimed here.
//! - **No program.** A compiled cell program's `sound` operation is announced by
//!   [`crate::script::Driver`]. These are the frame's carried events, which exist in every mounted
//!   package; the program lane still needs admission (`AGENTS.md` §12).
//! - **No invented sound.** A frame with no clips starts nothing and says so with an empty list; a
//!   payload that cannot be decoded is kept in [`FrameAudio::unreadable`] with the decoder's reason,
//!   never replaced by silence or a generated tone.

use planet_package::cellframe::CellFrame;

use crate::audio::{Channel, Decoded, Played, Sink, Unreadable};

/// One carried frame's audio, decoded once.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrameAudio {
    /// The frame document's id, as the package named it.
    pub frame: String,
    /// The clips that decoded, in the frame's own declaration order.
    pub clips: Vec<Decoded>,
    /// The clips that did not decode, with the decoder's reason.
    pub unreadable: Vec<Unreadable>,
}

/// One frame's sounds started on one tick.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Started {
    /// The movie tick the frame was shown on.
    pub tick: u64,
    /// The frame whose sounds were started.
    pub frame: String,
    /// The sound characters queued, in order. Empty when the frame carries none.
    pub sounds: Vec<String>,
    /// The samples those characters put into the channel.
    pub samples: usize,
}

/// What one [`Score::advance`] did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Advance {
    /// The start, when this tick entered a frame different from the previous one.
    pub started: Option<Started>,
    /// Everything the channel has rendered so far, so a run can state it without a device.
    pub played: Played,
}

/// The frames' decoded audio and the channel it is queued into.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Score {
    frames: Vec<FrameAudio>,
    channel: Channel,
    started: Vec<Started>,
    /// The frame the clock last showed. A redraw of the same frame starts nothing.
    visited: Option<usize>,
}

impl Score {
    /// **Decode every carried frame's clips once.** The decoding is the audio module's; this only
    /// keeps each frame's result beside the frame's id so the start is a lookup, not a second decode.
    pub fn prepare<'a>(frames: impl IntoIterator<Item = &'a CellFrame>) -> Score {
        let mut decoded = Vec::new();
        for frame in frames {
            let mut sink = Sink::new();
            sink.push_all(frame.audio.iter());
            decoded.push(FrameAudio {
                frame: frame.id.clone(),
                clips: sink.decoded().to_vec(),
                unreadable: sink.unreadable().to_vec(),
            });
        }
        Score::from_frames(decoded)
    }

    /// A score over already-decoded frames. Nothing is decoded here, which is what makes the start
    /// rule testable without an audio payload.
    pub fn from_frames(frames: Vec<FrameAudio>) -> Score {
        Score {
            frames,
            channel: Channel::new(),
            started: Vec::new(),
            visited: None,
        }
    }

    /// **One tick.** `index` is the carried frame the tick shows; a frame different from the previous
    /// one starts its sounds, the same frame starts nothing.
    pub fn advance(&mut self, tick: u64, index: usize) -> Advance {
        let started = if self.visited == Some(index) {
            None
        } else {
            self.visited = Some(index);
            let started = match self.frames.get(index) {
                Some(frame) => {
                    let mut sounds = Vec::new();
                    let mut samples = 0usize;
                    for clip in &frame.clips {
                        samples += clip.samples.len();
                        sounds.push(clip.id.clone());
                        self.channel.queue(clip.clone());
                    }
                    Started {
                        tick,
                        frame: frame.frame.clone(),
                        sounds,
                        samples,
                    }
                }
                // A tick that lands on a frame the score does not carry starts nothing. The caller
                // asked for an index outside the carried frames; inventing a silent start would be
                // reporting a sound the package does not hold.
                None => Started {
                    tick,
                    frame: String::new(),
                    sounds: Vec::new(),
                    samples: 0,
                },
            };
            self.started.push(started.clone());
            Some(started)
        };
        Advance {
            started,
            played: self.channel.played(),
        }
    }

    /// Render up to `frames` sample frames out of the channel. There is no device behind this: the
    /// samples come out of the decoder and are counted, and a caller with hardware hands them on.
    pub fn render(&mut self, frames: usize) -> Played {
        let _ = self.channel.render(frames);
        self.channel.played()
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    /// Every start this score has reported, in tick order.
    pub fn started(&self) -> &[Started] {
        &self.started
    }

    pub fn frames(&self) -> &[FrameAudio] {
        &self.frames
    }

    /// Clips that could not be decoded, across every carried frame.
    pub fn unreadable_total(&self) -> usize {
        self.frames
            .iter()
            .map(|frame| frame.unreadable.len())
            .sum()
    }

    /// Clips the score holds ready to start.
    pub fn clip_total(&self) -> usize {
        self.frames.iter().map(|frame| frame.clips.len()).sum()
    }

    /// **The stated limit.** No audio device is opened by this client; the channel's rendered samples
    /// are counted, and handing them to hardware is not this module's claim.
    pub const DEVICE: &'static str = "none: samples are rendered and counted, no device is opened";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decoded(id: &str, samples: usize) -> Decoded {
        Decoded {
            kind: "sound".to_string(),
            id: id.to_string(),
            format: "mp3".to_string(),
            declared_rate: 44_100,
            rate: 44_100,
            channels: 1,
            frames: samples,
            samples: vec![0.5; samples],
            bytes: samples,
            digest: format!("digest-{id}"),
            peak: 0.5,
        }
    }

    fn frame(id: &str, clips: Vec<Decoded>) -> FrameAudio {
        FrameAudio {
            frame: id.to_string(),
            clips,
            unreadable: Vec::new(),
        }
    }

    fn score() -> Score {
        Score::from_frames(vec![
            frame("frame-a", vec![decoded("7", 4)]),
            frame("frame-b", vec![decoded("9", 6), decoded("11", 2)]),
            frame("frame-c", Vec::new()),
        ])
    }

    #[test]
    fn entering_a_frame_starts_its_sounds_and_queues_their_samples() {
        let mut score = score();
        let advance = score.advance(0, 1);
        let started = advance.started.expect("a start");
        assert_eq!(started.tick, 0);
        assert_eq!(started.frame, "frame-b");
        assert_eq!(started.sounds, vec!["9".to_string(), "11".to_string()]);
        assert_eq!(started.samples, 8, "six samples plus two");
        assert_eq!(score.channel().queued(), 2);
    }

    #[test]
    fn redrawing_the_same_frame_starts_nothing_a_second_time() {
        let mut score = score();
        assert!(score.advance(4, 0).started.is_some());
        assert!(
            score.advance(4, 0).started.is_none(),
            "a redraw of the same frame is not a new sound"
        );
        assert_eq!(score.started().len(), 1);
        assert_eq!(score.channel().queued(), 1);
    }

    #[test]
    fn wrapping_back_to_a_frame_starts_it_again() {
        let mut score = score();
        score.advance(0, 0);
        score.advance(1, 1);
        let again = score.advance(2, 0).started.expect("a second visit");
        assert_eq!(again.tick, 2);
        assert_eq!(again.sounds, vec!["7".to_string()]);
        assert_eq!(score.started().len(), 3);
    }

    #[test]
    fn a_frame_with_no_sounds_starts_and_says_so_with_an_empty_list() {
        let mut score = score();
        let started = score.advance(7, 2).started.expect("a start");
        assert!(started.sounds.is_empty());
        assert_eq!(started.samples, 0);
        assert_eq!(score.channel().queued(), 0);
    }

    #[test]
    fn an_index_the_score_does_not_carry_starts_nothing_it_does_not_hold() {
        let mut score = score();
        let started = score.advance(3, 9).started.expect("a stated start");
        assert!(started.frame.is_empty());
        assert!(started.sounds.is_empty());
        assert_eq!(score.channel().queued(), 0);
    }

    #[test]
    fn rendering_a_started_frame_yields_the_decoded_samples_and_a_peak() {
        let mut score = score();
        let started = score.advance(0, 0).started.expect("a start");
        let played = score.render(started.samples);
        assert_eq!(played.samples, 4, "the four samples the clip carries");
        assert!(played.peak > 0.0, "a decoded clip is not silence");
    }

    #[test]
    fn the_same_visits_produce_the_same_starts() {
        let mut first = score();
        let mut second = score();
        let mut a = Vec::new();
        let mut b = Vec::new();
        for (tick, index) in [(0u64, 0usize), (1, 1), (2, 2), (3, 0)] {
            a.push(first.advance(tick, index).started);
            b.push(second.advance(tick, index).started);
        }
        assert_eq!(a, b);
        assert_eq!(first.started(), second.started());
    }
}
