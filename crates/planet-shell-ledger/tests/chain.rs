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
