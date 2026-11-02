use std::path::PathBuf;

use planet_shell_ledger::landing;
use planet_shell_ledger::{Appended, Evidence, Kind, Ledger, Refusal, Request};

fn evidence() -> Evidence {
    Evidence::new(
        "cargo test -p planet-shell-ledger",
        0,
        6,
        "ledger",
        &"a".repeat(64),
    )
}

fn request(operation: &str, subject: &str) -> Request {
    Request::new(
        operation,
        1_700_000_000,
        "implementation agent",
        Kind::CodeChange,
        "TICKET-0026",
        subject,
        &"b".repeat(64),
        evidence(),
    )
}

#[test]
fn genesis_then_two_entries_link() {
    let mut ledger = Ledger::new();
    assert_eq!(ledger.head_seq(), 0);
    assert_eq!(ledger.head_hash(), "genesis");
    assert_eq!(
        ledger.append(0, request("op-1", "crates/a")).unwrap(),
        Appended::New
    );
    assert_eq!(
        ledger.append(1, request("op-2", "crates/b")).unwrap(),
        Appended::New
    );
    assert_eq!(ledger.head_seq(), 2);
    assert_eq!(ledger.entries()[0].prev, "genesis");
    assert_eq!(ledger.entries()[1].prev, ledger.entries()[0].hash);
    ledger.verify().unwrap();
}

#[test]
fn stale_writer_is_refused_not_rebased() {
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    assert_eq!(
        ledger.append(0, request("op-2", "crates/b")),
        Err(Refusal::StaleSeq {
            expected: 0,
            head: 1
        })
    );
    assert_eq!(ledger.head_seq(), 1);
}

#[test]
fn retrying_a_landed_operation_appends_nothing() {
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    assert_eq!(
        ledger.append(1, request("op-1", "crates/a")).unwrap(),
        Appended::Existing(1)
    );
    assert_eq!(ledger.head_seq(), 1);
}

#[test]
fn an_entry_without_authority_is_refused() {
    let mut ledger = Ledger::new();
    let mut bad = request("op-1", "crates/a");
    bad.authority_ref = String::new();
    assert_eq!(
        ledger.append(0, bad),
        Err(Refusal::MissingField("authority_ref"))
    );
    assert_eq!(ledger.head_seq(), 0);
}

#[test]
fn an_entry_without_evidence_is_refused() {
    let mut ledger = Ledger::new();
    let mut bad = request("op-1", "crates/a");
    bad.evidence.command = "  ".to_string();
    assert_eq!(
        ledger.append(0, bad),
        Err(Refusal::MissingField("evidence.command"))
    );
}

#[test]
fn a_digest_that_is_not_sha256_is_refused() {
    let mut ledger = Ledger::new();
    let mut bad = request("op-1", "crates/a");
    bad.digest = "not-a-digest".to_string();
    assert_eq!(
        ledger.append(0, bad),
        Err(Refusal::DigestNotSha256 { field: "digest" })
    );
}

#[test]
fn an_uppercase_digest_is_refused() {
    let mut ledger = Ledger::new();
    let mut bad = request("op-1", "crates/a");
    bad.digest = "A".repeat(64);
    assert_eq!(
        ledger.append(0, bad),
        Err(Refusal::DigestNotSha256 { field: "digest" })
    );
}

#[test]
fn editing_a_landed_entry_breaks_the_chain() {
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    ledger.append(1, request("op-2", "crates/b")).unwrap();
    let rendered = ledger.render().unwrap();
    let tampered = rendered.replacen("crates/a", "crates/z", 1);
    assert_eq!(
        Ledger::read(&tampered).err(),
        Some(Refusal::BrokenChain { seq: 1 })
    );
}

#[test]
fn the_durable_form_round_trips_and_verifies() {
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    ledger.append(1, request("op-2", "crates/b")).unwrap();
    let rendered = ledger.render().unwrap();
    let reread = Ledger::read(&rendered).unwrap();
    assert_eq!(reread.entries(), ledger.entries());
    reread.verify().unwrap();
}

#[test]
fn a_dropped_first_entry_breaks_the_chain() {
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    ledger.append(1, request("op-2", "crates/b")).unwrap();
    let rendered = ledger.render().unwrap();
    let mut lines: Vec<&str> = rendered.lines().collect();
    lines.remove(0);
    assert_eq!(
        Ledger::read(&(lines.join("\n") + "\n")).err(),
        Some(Refusal::NoGenesis)
    );
}

#[test]
fn a_reordered_pair_cannot_open_a_chain() {
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    ledger.append(1, request("op-2", "crates/b")).unwrap();
    let rendered = ledger.render().unwrap();
    let lines: Vec<&str> = rendered.lines().collect();
    let swapped = format!("{}\n{}\n", lines[1], lines[0]);
    // The moved entry carries a hash in `prev`, so the chain no longer opens at genesis.
    assert_eq!(Ledger::read(&swapped).err(), Some(Refusal::NoGenesis));
}

fn scratch(name: &str) -> PathBuf {
    // Inside the repository tree, beside the destination: never an OS temp directory.
    let dir = PathBuf::from("target").join("ledger-land-tests").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_landed_chain_reloads_and_verifies() {
    let dir = scratch("landed");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    ledger.append(1, request("op-2", "crates/b")).unwrap();
    landing::land(&ledger, &path).unwrap();
    assert!(path.exists());
    assert!(
        !landing::staging_path(&path).exists(),
        "staging must not survive a landed write"
    );
    let reloaded = landing::load(&path).unwrap().unwrap();
    assert_eq!(reloaded.entries(), ledger.entries());
    reloaded.verify().unwrap();
}

#[test]
fn a_missing_log_reads_as_an_empty_chain() {
    let dir = scratch("missing");
    let ledger = landing::load(&dir.join("absent.tsv")).unwrap().unwrap();
    assert_eq!(ledger.head_seq(), 0);
    assert!(ledger.entries().is_empty());
}

#[test]
fn an_edited_file_refuses_on_load() {
    let dir = scratch("edited");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    landing::land(&ledger, &path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    std::fs::write(&path, text.replacen("landed", "moved", 1)).unwrap();
    assert_eq!(
        landing::load(&path).unwrap().err(),
        Some(Refusal::BrokenChain { seq: 1 })
    );
}

#[test]
fn an_unrepresentable_entry_lands_nothing() {
    let dir = scratch("refused");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    let mut bad = request("op-2", "crates/b");
    bad.subject = "crates/b	c".to_string();
    ledger.append(1, bad).unwrap();
    assert!(landing::land(&ledger, &path).is_err());
    assert!(!path.exists(), "a refused render must not create the log");
    assert!(!landing::staging_path(&path).exists());
}

#[test]
fn landing_twice_keeps_one_chain() {
    let dir = scratch("twice");
    let path = dir.join("ledger.tsv");
    let mut ledger = Ledger::new();
    ledger.append(0, request("op-1", "crates/a")).unwrap();
    landing::land(&ledger, &path).unwrap();
    let first = std::fs::read_to_string(&path).unwrap();
    landing::land(&ledger, &path).unwrap();
    let second = std::fs::read_to_string(&path).unwrap();
    assert_eq!(first, second, "landing is not additive");
    assert_eq!(landing::load(&path).unwrap().unwrap().head_seq(), 1);
}
