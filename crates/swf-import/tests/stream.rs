//! The import stream, proved on synthetic rows only: sealing, cadence, flush, and the refusal that makes
//! a killed import visible instead of silent.

use std::path::PathBuf;

use planet_aead::Key;
use swf_import::stream::{self, Kind, Refusal, Stream, HEADER};

fn key() -> Key {
    Key::from_bytes([11; planet_aead::KEY_LEN])
}

fn path(name: &str) -> PathBuf {
    PathBuf::from("target").join("test-stream").join(format!("{name}.jsonl"))
}

fn clean(name: &str) {
    let _ = std::fs::remove_file(path(name));
}

#[test]
fn rows_are_sealed_and_open_in_order() {
    let name = "ordered";
    clean(name);
    let mut writer = Stream::create(&path(name), key()).expect("a stream");
    let first = writer.row(Kind::Stage, "tags 12").expect("a stage row");
    let second = writer.row(Kind::Record, "tags 0-999").expect("a record row");
    let done = writer.finish().expect("a done row");
    assert_eq!((first.seq, second.seq, done.seq), (1, 2, 3));

    let read = stream::read_complete(&path(name), &key()).expect("a complete stream");
    assert!(read.complete);
    assert_eq!(read.rows.len(), 3);
    assert_eq!(read.rows[0].kind, Kind::Stage);
    assert_eq!(read.rows[0].detail, "tags 12");
    assert_eq!(read.rows[2].kind, Kind::Done);
    clean(name);
}

#[test]
fn a_row_is_written_and_flushed_before_the_stream_finishes() {
    let name = "flushed";
    clean(name);
    let mut writer = Stream::create(&path(name), key()).expect("a stream");
    writer.row(Kind::Record, "tags 0-999").expect("a row");

    // No finish, no drop: the row must already be on disk for a reader to find.
    let read = stream::read(&path(name), &key()).expect("readable mid-stream");
    assert_eq!(read.rows.len(), 1, "the row was flushed as written");
    assert!(!read.complete, "an unfinished stream is not complete");
    clean(name);
}

#[test]
fn a_stream_cut_before_its_done_row_is_refused_as_truncated() {
    let name = "truncated";
    clean(name);
    let mut writer = Stream::create(&path(name), key()).expect("a stream");
    writer.row(Kind::Stage, "tags 4").expect("a row");
    drop(writer);

    let refusal = stream::read_complete(&path(name), &key()).expect_err("no done row");
    assert_eq!(refusal.code(), "import-truncated");
    assert!(matches!(refusal, Refusal::Truncated { rows: 1 }));
    assert!(refusal.message().contains("did not finish"));

    // The evidence survives the refusal: the rows that were sealed are still readable.
    let partial = stream::read(&path(name), &key()).expect("the rows are still there");
    assert_eq!(partial.rows.len(), 1);
    assert!(!partial.complete);
    clean(name);
}

#[test]
fn an_aborted_stream_names_the_code_in_its_last_row() {
    let name = "aborted";
    clean(name);
    let mut writer = Stream::create(&path(name), key()).expect("a stream");
    writer.row(Kind::Stage, "tags 4").expect("a row");
    writer.abort("swf-truncated").expect("an abort row");
    assert_eq!(
        writer.row(Kind::Stage, "after the abort").expect_err("closed").code(),
        "stream-order"
    );

    let read = stream::read(&path(name), &key()).expect("readable");
    assert!(!read.complete, "an abort is not a finished import");
    let last = read.rows.last().expect("a last row");
    assert_eq!(last.kind, Kind::Truncated);
    assert_eq!(last.detail, "swf-truncated");
    clean(name);
}

#[test]
fn a_row_sealed_by_another_key_is_refused_by_name() {
    let name = "other-key";
    clean(name);
    let mut writer = Stream::create(&path(name), key()).expect("a stream");
    writer.row(Kind::Stage, "tags 1").expect("a row");
    writer.finish().expect("done");

    let other = Key::from_bytes([12; planet_aead::KEY_LEN]);
    let refusal = stream::read(&path(name), &other).expect_err("another key");
    assert_eq!(refusal.code(), "stream-order");
    clean(name);
}

#[test]
fn a_tampered_row_does_not_open() {
    let name = "tampered";
    clean(name);
    let mut writer = Stream::create(&path(name), key()).expect("a stream");
    writer.row(Kind::Stage, "tags 1").expect("a row");
    writer.finish().expect("done");

    let text = std::fs::read_to_string(path(&name)).expect("the stream");
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let last = lines[0].len() - 3;
    let mut bytes = lines[0].clone().into_bytes();
    bytes[last] = if bytes[last] == b'a' { b'b' } else { b'a' };
    lines[0] = String::from_utf8(bytes).expect("still text");
    std::fs::write(path(&name), lines.join("\n") + "\n").expect("rewritten");

    let refusal = stream::read(&path(&name), &key()).expect_err("edited bytes");
    assert_eq!(refusal.code(), "aead-tag");
    clean(name);
}

#[test]
fn a_line_that_is_not_a_row_is_refused_by_name() {
    let name = "not-a-row";
    clean(name);
    std::fs::create_dir_all("target/test-stream").expect("a directory");
    std::fs::write(path(&name), "{\"row\":\"stage\"}\n").expect("a bad line");
    let refusal = stream::read(&path(&name), &key()).expect_err("not a row");
    assert_eq!(refusal.code(), "stream-row");
    clean(name);
}

#[test]
fn the_header_carries_no_format_id_and_the_schema_is_sealed() {
    assert_eq!(HEADER, "planet.import-stream", "no /N id for the gate to register");
    assert_eq!(stream::SCHEMA, 1);
}
