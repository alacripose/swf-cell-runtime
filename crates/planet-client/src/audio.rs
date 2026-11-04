//! The client's audio channel: decode, hold and play the sounds a frame carries.
//!
//! A frame carries [`SoundClip`]s — the source character's own payload bytes plus the encoding it
//! declares. This module is the whole audio path on the client side:
//!
//! 1. **Decode.** [`Sink::push`] hands the payload to a real MPEG-audio decoder and keeps the samples
//!    it produces, with the sample rate and channel count the bitstream itself states.
//! 2. **Hold.** A payload that decodes is [`Decoded`]; a payload that cannot be decoded is
//!    [`Unreadable`] **with its reason**, never a silent buffer, a beep or a generated waveform. That
//!    distinction is the whole point: a client that plays silence for an undecodable sound is
//!    reporting content it does not have.
//! 3. **Play.** [`Channel`] is the playback path. It holds the decoded clips in order and renders
//!    interleaved `f32` samples a block at a time, so playing a sound consumes decoded samples rather
//!    than a count of bytes. [`Channel::play_to_end`] renders everything and reports the frames, the
//!    samples and the peak amplitude, which is what "something was actually played" means here.
//!
//! Nothing in this module synthesises audio. Every sample it holds came out of a decoder reading the
//! package's payload; a clip with no decodable frame is refused by name.

use std::io::Cursor;

use planet_package::cellframe::{payload_digest, SoundClip};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as DecodeError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// One payload that decoded into samples.
#[derive(Clone, Debug, PartialEq)]
pub struct Decoded {
    pub kind: String,
    pub id: String,
    /// The encoding the package declares for this clip.
    pub format: String,
    /// The sample rate the package's header declares.
    pub declared_rate: u32,
    /// The sample rate the decoded bitstream states.
    pub rate: u32,
    pub channels: u16,
    /// Sample frames per channel: one frame is one sample on every channel.
    pub frames: usize,
    /// Interleaved samples: `frames * channels` values.
    pub samples: Vec<f32>,
    pub bytes: usize,
    /// The payload's digest, so byte identity across the pipeline is checkable.
    pub digest: String,
    /// The peak absolute sample value, so a decoded-but-silent clip is visible as such.
    pub peak: f32,
}

impl Decoded {
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }
}

/// One payload that could not be decoded, and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unreadable {
    pub kind: String,
    pub id: String,
    pub format: String,
    pub bytes: usize,
    pub digest: String,
    pub code: &'static str,
    pub reason: String,
}

/// A refusal raised while consuming a clip, before any decoding is attempted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioRefusal {
    pub code: &'static str,
    pub message: String,
}

/// What a playback pass rendered.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Played {
    pub frames: usize,
    pub samples: usize,
    pub peak: f32,
}

/// The client's audio channel: the queue of decoded clips and the cursor that plays them.
///
/// Rendering is interleaved `f32` at the rate the clip states; a render block that would run past the
/// end of one clip starts the next. When the queue is empty the channel renders nothing at all — it
/// does not pad with silence and call it sound.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Channel {
    queue: Vec<Decoded>,
    /// Interleaved samples of the clip currently playing, and how far into it playback has reached.
    cursor: usize,
    played_frames: usize,
    played_samples: usize,
    peak: f32,
    /// The format of the clip at the head of the queue, for a device that needs it.
    rate: u32,
    channels: u16,
}

impl Channel {
    pub fn new() -> Channel {
        Channel::default()
    }

    /// Queue a decoded clip for playback.
    pub fn queue(&mut self, decoded: Decoded) {
        if self.queue.is_empty() && self.cursor == 0 {
            self.rate = decoded.rate;
            self.channels = decoded.channels;
        }
        self.queue.push(decoded);
    }

    pub fn queued(&self) -> usize {
        self.queue.len()
    }

    /// The sample rate and channel count the next block is rendered at. `None` when nothing is queued.
    pub fn format(&self) -> Option<(u32, u16)> {
        self.queue.first().map(|clip| (clip.rate, clip.channels))
    }

    /// Render up to `frames` sample frames of interleaved audio, in queue order.
    ///
    /// Answers an empty vector when the queue is exhausted: there is no fill, no padding and no
    /// synthetic tail.
    pub fn render(&mut self, frames: usize) -> Vec<f32> {
        let mut out: Vec<f32> = Vec::new();
        let mut want = frames;
        while want > 0 {
            let Some(clip) = self.queue.first() else { break };
            let channels = clip.channels.max(1) as usize;
            let per_frame = channels;
            let start = self.cursor;
            let available = (clip.samples.len() - start) / per_frame;
            if available == 0 {
                self.queue.remove(0);
                self.cursor = 0;
                if let Some(next) = self.queue.first() {
                    self.rate = next.rate;
                    self.channels = next.channels;
                }
                continue;
            }
            let take = available.min(want);
            let end = start + take * per_frame;
            for sample in &clip.samples[start..end] {
                self.peak = self.peak.max(sample.abs());
            }
            out.extend_from_slice(&clip.samples[start..end]);
            self.played_frames += take;
            self.played_samples += take * per_frame;
            self.cursor = end;
            want -= take;
            if self.cursor >= clip.samples.len() {
                self.queue.remove(0);
                self.cursor = 0;
                if let Some(next) = self.queue.first() {
                    self.rate = next.rate;
                    self.channels = next.channels;
                }
            }
        }
        out
    }

    /// Play every queued clip to its end and report what was rendered.
    pub fn play_to_end(&mut self) -> Played {
        while !self.queue.is_empty() {
            let _ = self.render(4096);
        }
        Played {
            frames: self.played_frames,
            samples: self.played_samples,
            peak: self.peak,
        }
    }

    pub fn played(&self) -> Played {
        Played {
            frames: self.played_frames,
            samples: self.played_samples,
            peak: self.peak,
        }
    }
}

/// The sink: decode every clip a frame carries, hold the samples, and play them through the channel.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sink {
    decoded: Vec<Decoded>,
    unreadable: Vec<Unreadable>,
    total_bytes: usize,
    channel: Channel,
}

impl Sink {
    pub fn new() -> Sink {
        Sink::default()
    }

    /// Consume one clip: decode it into samples, queue them for playback, and record the outcome.
    ///
    /// An empty payload is refused outright — a sink that accepted it would be reporting sound where
    /// the file states none. A payload that cannot be decoded is recorded as [`Unreadable`] with the
    /// decoder's own reason; it is never turned into silence.
    pub fn push(&mut self, clip: &SoundClip) -> Result<(), AudioRefusal> {
        if clip.bytes.is_empty() {
            return Err(AudioRefusal {
                code: "audio-empty",
                message: format!("sound {} carries no payload bytes", clip.id),
            });
        }
        let digest = payload_digest(&clip.bytes);
        match decode(clip, digest.clone()) {
            Ok(decoded) => {
                self.total_bytes += decoded.bytes;
                self.channel.queue(decoded.clone());
                self.decoded.push(decoded);
            }
            Err((code, reason)) => {
                self.total_bytes += clip.bytes.len();
                self.unreadable.push(Unreadable {
                    kind: clip.kind.clone(),
                    id: clip.id.clone(),
                    format: clip.format.clone(),
                    bytes: clip.bytes.len(),
                    digest,
                    code,
                    reason,
                });
            }
        }
        Ok(())
    }

    /// Consume every clip a frame carries, in the frame's own order.
    pub fn push_all<'a>(&mut self, clips: impl IntoIterator<Item = &'a SoundClip>) {
        for clip in clips {
            // The only refusal `push` can raise is an empty payload, which is recorded here rather
            // than aborting the rest of the frame: one bad clip must not silence the others.
            if let Err(refusal) = self.push(clip) {
                self.unreadable.push(Unreadable {
                    kind: clip.kind.clone(),
                    id: clip.id.clone(),
                    format: clip.format.clone(),
                    bytes: 0,
                    digest: payload_digest(&clip.bytes),
                    code: refusal.code,
                    reason: refusal.message,
                });
            }
        }
    }

    pub fn decoded(&self) -> &[Decoded] {
        &self.decoded
    }

    pub fn unreadable(&self) -> &[Unreadable] {
        &self.unreadable
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn total_frames(&self) -> usize {
        self.decoded.iter().map(|clip| clip.frames).sum()
    }

    pub fn total_samples(&self) -> usize {
        self.decoded.iter().map(|clip| clip.samples.len()).sum()
    }

    /// Play everything the frame carried and report the samples that reached the channel.
    pub fn play(&mut self) -> Played {
        self.channel.play_to_end()
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }
}

/// Decode one payload into interleaved samples.
///
/// The decoder reads the bitstream, so the rate and channel count come from the audio itself rather
/// than from a header a reader might have misread; `declared_rate` travels beside them so a mismatch
/// is visible instead of hidden. A payload with no complete frame is `audio-unreadable` with the
/// decoder's reason.
fn decode(clip: &SoundClip, digest: String) -> Result<Decoded, (&'static str, String)> {
    let source = MediaSourceStream::new(
        Box::new(Cursor::new(clip.bytes.clone())),
        Default::default(),
    );
    let mut hint = Hint::new();
    hint.with_extension("mp3");
    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            source,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|error| ("audio-unreadable", format!("{error}")))?;
    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|track| track.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .ok_or(("audio-unreadable", "the payload declares no decodable track".to_string()))?;
    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|error| ("audio-unreadable", format!("{error}")))?;

    let mut samples: Vec<f32> = Vec::new();
    let mut rate = 0u32;
    let mut channels = 0u16;
    let mut frames = 0usize;
    let mut skipped = 0usize;
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            // The payload ends where the frames end: reaching the end of the stream is not a failure.
            Err(DecodeError::IoError(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof => {
                break
            }
            Err(DecodeError::ResetRequired) => break,
            Err(error) => return Err(("audio-unreadable", format!("{error}"))),
        };
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                rate = spec.rate;
                channels = spec.channels.count() as u16;
                let block_frames = decoded.frames();
                frames += block_frames;
                let mut buffer = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
                buffer.copy_interleaved_ref(decoded);
                samples.extend_from_slice(buffer.samples());
            }
            // A frame the decoder rejects is skipped, not replaced: the samples that remain are the
            // ones the bitstream actually carries.
            Err(DecodeError::DecodeError(_)) => skipped += 1,
            Err(_) => skipped += 1,
        }
    }
    if frames == 0 || samples.is_empty() {
        return Err((
            "audio-unreadable",
            format!(
                "the payload holds no complete MPEG frame ({} bytes, {} frames skipped)",
                clip.bytes.len(),
                skipped
            ),
        ));
    }
    let peak = samples.iter().fold(0.0f32, |peak, sample| peak.max(sample.abs()));
    Ok(Decoded {
        kind: clip.kind.clone(),
        id: clip.id.clone(),
        format: clip.format.clone(),
        declared_rate: clip.rate,
        rate,
        channels,
        frames,
        samples,
        bytes: clip.bytes.len(),
        digest,
        peak,
    })
}
