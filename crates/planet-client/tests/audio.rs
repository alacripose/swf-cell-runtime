//! The client's audio path, proved on synthetic input. Synthetic bytes only: no byte of any supplied
//! movie appears here, and no test asserts anything about decoded media.

use planet_client::audio::{Channel, Decoded, Sink};
use planet_package::cellframe::SoundClip;

fn clip(id: &str, bytes: Vec<u8>) -> SoundClip {
    SoundClip {
        kind: "sound".to_string(),
        id: id.to_string(),
        artifact: format!("audio/{id}.mp3"),
        format: "mp3".to_string(),
        rate: 11_025,
        samples: 0,
        bytes,
    }
}

fn decoded(id: &str, rate: u32, channels: u16, frames: usize, level: f32) -> Decoded {
    Decoded {
        kind: "sound".to_string(),
        id: id.to_string(),
        format: "mp3".to_string(),
        declared_rate: rate,
        rate,
        channels,
        frames,
        samples: vec![level; frames * channels as usize],
        bytes: frames * channels as usize,
        digest: format!("digest-{id}"),
        peak: level,
    }
}

#[test]
fn an_empty_payload_is_refused_and_never_queued() {
    let mut sink = Sink::new();
    let refusal = sink.push(&clip("1", Vec::new())).expect_err("no payload");
    assert_eq!(refusal.code, "audio-empty");
    assert!(sink.decoded().is_empty());
    assert_eq!(sink.channel().queued(), 0, "nothing was queued for playback");
}

#[test]
fn a_payload_with_no_frame_is_unreadable_with_its_reason() {
    // Four bytes cannot hold an MPEG frame. The sink must record that as unreadable and must not
    // substitute silence, a tone or an empty buffer for it.
    let mut sink = Sink::new();
    sink.push_all(&[clip("9", vec![0x7d, 0x06, 0xff, 0xe3])]);
    assert!(sink.decoded().is_empty(), "nothing was decoded");
    assert_eq!(sink.unreadable().len(), 1);
    let entry = &sink.unreadable()[0];
    assert_eq!(entry.code, "audio-unreadable");
    assert!(!entry.reason.is_empty(), "the reason is named");
    assert_eq!(entry.bytes, 4);
    assert_eq!(sink.channel().queued(), 0, "no clip reached playback");
    let played = sink.play();
    assert_eq!(played.frames, 0, "silence was not played in its place");
    assert_eq!(played.samples, 0);
}

#[test]
fn a_decoded_clip_plays_real_samples_through_the_channel() {
    let mut channel = Channel::new();
    channel.queue(decoded("1", 11_025, 1, 8, 0.5));
    assert_eq!(channel.format(), Some((11_025, 1)));
    let played = channel.play_to_end();
    assert_eq!(played.frames, 8, "eight sample frames were rendered");
    assert_eq!(played.samples, 8, "mono: one sample per frame");
    assert_eq!(played.peak, 0.5, "the peak is the decoded peak");
    assert_eq!(channel.queued(), 0, "the queue drained");
}

#[test]
fn a_small_block_renders_only_what_was_asked_for() {
    let mut channel = Channel::new();
    channel.queue(decoded("1", 11_025, 2, 4, 0.25));
    let first = channel.render(2);
    assert_eq!(first.len(), 4, "two frames of two channels");
    assert_eq!(channel.played().frames, 2);
    let rest = channel.play_to_end();
    assert_eq!(rest.frames, 4, "the whole clip was played by the end");
    assert_eq!(rest.samples, 8);
}

#[test]
fn clips_play_in_the_order_they_were_queued() {
    let mut channel = Channel::new();
    channel.queue(decoded("1", 11_025, 1, 2, 0.25));
    channel.queue(decoded("2", 11_025, 1, 2, 0.75));
    let rendered = channel.render(4);
    assert_eq!(rendered.len(), 4);
    assert_eq!(&rendered[..2], &[0.25, 0.25], "the first clip plays first");
    assert_eq!(&rendered[2..], &[0.75, 0.75], "then the second");
    assert_eq!(channel.played().peak, 0.75);
}
