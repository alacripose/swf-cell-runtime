use std::path::PathBuf;

use planet_package::record::ledger_code;
use planet_package::{mount_and_record, Frame, Kind, MountRefusal, Recorded};
use planet_shell_ledger::{landing, Ledger, Refusal as LedgerRefusal};

fn scratch(name: &str) -> PathBuf {
    // Inside the repository tree, beside the destination: never an OS temp directory.
    let dir = PathBuf::from("target")
        .join("mount-entry-tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn surface_document(content: &str) -> String {
    let mut text = String::from("dl-enclave.surface/1\nsurface\talpha-shell\nrequires\tobserve\n");
    text.push_str("window\tmain\tMain\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\t");
    text.push_str(content);
    text.push('\n');
    text
}

fn declaration(content: &str) -> Frame {
    Frame::new(Kind::Decl, surface_document(content).as_bytes()).expect("a declaration frame")
}

#[test]
fn a_mount_lands_one_entry_that_a_reload_reads_back() {
    let dir = scratch("landed");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    let frame = declaration("content-main");

    let recorded = mount_and_record(&frame, "alpha-shell", 1, 0, 1_700_000_000, &mut ledger, &path, 0)
        .expect("a mounted declaration records");
    match recorded {
        Recorded::Landed { receipt, seq } => {
            assert_eq!(seq, 1);
            assert_eq!(receipt.surface, "alpha-shell");
            assert_eq!(receipt.windows, 1);
        }
        other => panic!("expected a landed entry, got {other:?}"),
    }
    assert_eq!(ledger.head_seq(), 1, "the in-memory head advanced once");

    let reloaded = landing::load(&path).unwrap().unwrap();
    reloaded.verify().unwrap();
    assert_eq!(reloaded.head_seq(), 1);
    let entry = &reloaded.entries()[0];
    assert_eq!(entry.kind, planet_shell_ledger::Kind::Mount);
    assert_eq!(entry.subject, "mount/alpha-shell");
    assert_eq!(entry.digest, frame.frame_digest_hex());
    assert_eq!(entry.operation_id, format!("mount-{}", frame.frame_digest_hex()));
    assert_eq!(entry.evidence.surface, "planet-package");
    assert_eq!(entry.evidence.checks, 1);
}

#[test]
fn mounting_the_same_frame_twice_records_once() {
    let dir = scratch("idempotent");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    let frame = declaration("content-main");

    mount_and_record(&frame, "alpha-shell", 1, 0, 1_700_000_000, &mut ledger, &path, 0).unwrap();
    let again = mount_and_record(&frame, "alpha-shell", 2, 1, 1_700_000_010, &mut ledger, &path, 1)
        .expect("a duplicate mount answers the first entry");
    match again {
        Recorded::AlreadyRecorded { seq, receipt } => {
            assert_eq!(seq, 1);
            assert_eq!(receipt.surface, "alpha-shell");
        }
        other => panic!("expected an idempotent answer, got {other:?}"),
    }
    assert_eq!(ledger.head_seq(), 1);
    assert_eq!(landing::load(&path).unwrap().unwrap().head_seq(), 1);
}

#[test]
fn a_refused_mount_leaves_no_entry() {
    let dir = scratch("refused");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    let mut payload = surface_document("content-main");
    payload.push_str("pane\tmain\tstray\n");
    let frame = Frame::new(Kind::Decl, payload.as_bytes()).unwrap();

    let refused = mount_and_record(&frame, "alpha-shell", 1, 0, 0, &mut ledger, &path, 0)
        .expect_err("an unknown row cannot mount");
    assert_eq!(refused.code(), "surface-row");
    assert!(refused.message().contains("pane"), "{}", refused.message());
    assert_eq!(ledger.head_seq(), 0, "the caller's chain did not move");
    assert!(!path.exists(), "a refused mount created a ledger file");
}

#[test]
fn a_code_frame_leaves_no_entry() {
    let dir = scratch("code");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    let frame = Frame::new(Kind::Code, surface_document("content-main").as_bytes()).unwrap();

    let refused = mount_and_record(&frame, "alpha-shell", 1, 0, 0, &mut ledger, &path, 0)
        .expect_err("a code frame is reserved");
    assert_eq!(refused.code(), "frame-kind-reserved");
    assert!(!path.exists());
    assert_eq!(ledger.head_seq(), 0);
}

#[test]
fn two_distinct_frames_land_two_entries() {
    let dir = scratch("two");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();

    mount_and_record(&declaration("content-a"), "alpha-shell", 1, 0, 1, &mut ledger, &path, 0).unwrap();
    let second =
        mount_and_record(&declaration("content-b"), "alpha-shell", 2, 1, 2, &mut ledger, &path, 1)
            .unwrap();
    assert!(matches!(second, Recorded::Landed { seq: 2, .. }));
    let reloaded = landing::load(&path).unwrap().unwrap();
    assert_eq!(reloaded.head_seq(), 2);
    reloaded.verify().unwrap();
}

#[test]
fn a_stale_writer_is_refused_and_the_file_is_unchanged() {
    let dir = scratch("stale");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    mount_and_record(&declaration("content-a"), "alpha-shell", 1, 0, 1, &mut ledger, &path, 0).unwrap();
    let before = std::fs::read_to_string(&path).unwrap();

    let refused = mount_and_record(&declaration("content-b"), "alpha-shell", 2, 1, 2, &mut ledger, &path, 0)
        .expect_err("a stale head is refused");
    assert_eq!(refused.code(), "ledger-stale");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
    assert_eq!(ledger.head_seq(), 1);
}

#[test]
fn an_unwritable_destination_lands_nothing() {
    let dir = scratch("unwritable");
    let path = dir.join("absent-directory").join("ledger.tsv");
    let mut ledger = Ledger::new();
    let refused = mount_and_record(&declaration("content-main"), "alpha-shell", 1, 0, 1, &mut ledger, &path, 0)
        .expect_err("a destination that cannot be written is refused");
    assert_eq!(refused.code(), "ledger-unavailable");
    assert_eq!(ledger.head_seq(), 0, "the chain does not advance on a failed land");
    assert!(!path.exists());
}

#[test]
fn the_ledger_codes_are_the_family_vocabulary() {
    assert_eq!(ledger_code(&LedgerRefusal::StaleSeq { expected: 0, head: 1 }), "ledger-stale");
    assert_eq!(ledger_code(&LedgerRefusal::NoGenesis), "ledger-corrupt");
    assert_eq!(ledger_code(&LedgerRefusal::BrokenChain { seq: 2 }), "ledger-corrupt");
    assert_eq!(ledger_code(&LedgerRefusal::EmptyOperationId), "ledger-invalid");
    assert_eq!(
        ledger_code(&LedgerRefusal::DigestNotSha256 { field: "digest" }),
        "ledger-invalid"
    );
    let refusal: MountRefusal = MountRefusal::LedgerUnavailable;
    assert_eq!(refusal.code(), "ledger-unavailable");
}
