//! The save transaction machinery, exercised only by synthetic records built here: nothing reads a
//! real save, nothing touches the shell's storage, and every store in these tests is in memory.

use planet_save::{
    journal, recover, JournalEntry, JournalState, MemoryStore, Receipt, Record, Recovery, SaveRefusal,
    Store, Transaction, JOURNAL_SLOT, SUPPORTED_SCHEMA,
};

fn record(slot: &str, revision: u64, bytes: &[u8]) -> Record {
    Record::new(slot, SUPPORTED_SCHEMA, revision, bytes.to_vec())
}

fn committed(store: &mut MemoryStore, slot: &str, revision: u64, bytes: &[u8]) -> Record {
    let held = record(slot, revision, bytes);
    store.save(held.clone()).expect("the fixture store accepts");
    held
}

#[test]
fn a_commit_writes_every_staged_record_and_reports_its_digests() {
    let mut store = MemoryStore::new();
    let before = committed(&mut store, "slot/inventory", 4, b"arrows=3");
    let inventory = record("slot/inventory", 5, b"arrows=7");
    let map = record("slot/map", 1, b"visited=2");

    let mut transaction = Transaction::begin();
    transaction.stage(&store, inventory.clone()).expect("newer revision");
    transaction.stage(&store, map.clone()).expect("a new slot");
    let receipt: Receipt = transaction.commit(&mut store).expect("it commits");

    assert_eq!(receipt.slots, vec!["slot/inventory", "slot/map"]);
    assert_eq!(receipt.before[0], before.digest_hex());
    assert_eq!(receipt.before[1], "-", "a slot that did not exist has no pre-image");
    assert_eq!(receipt.after[0], inventory.digest_hex());
    assert_eq!(receipt.after[1], map.digest_hex());
    assert_eq!(store.bytes("slot/inventory"), Some(b"arrows=7".to_vec()));
    assert_eq!(store.bytes("slot/map"), Some(b"visited=2".to_vec()));
    assert!(store.get(JOURNAL_SLOT).is_none(), "the journal is erased on success");
}

#[test]
fn an_unsupported_schema_is_refused_before_anything_is_written() {
    let store = MemoryStore::new();
    let strange = Record::new("slot/inventory", SUPPORTED_SCHEMA + 1, 1, b"arrows=3".to_vec());
    let mut transaction = Transaction::begin();
    let refusal = transaction.stage(&store, strange).expect_err("schema 2 is not read here");
    assert_eq!(refusal.code(), "save-schema");
    assert_eq!(refusal.slot(), "slot/inventory");
    assert!(refusal.message().contains("schema 2"), "{}", refusal.message());
    assert!(store.slots().is_empty(), "staging wrote nothing");
}

#[test]
fn a_record_whose_digest_does_not_match_its_bytes_is_refused() {
    let store = MemoryStore::new();
    let honest = record("slot/inventory", 1, b"arrows=3");
    let tampered = Record::declared(
        "slot/inventory",
        SUPPORTED_SCHEMA,
        1,
        b"arrows=999".to_vec(),
        honest.digest(),
    );
    assert!(!tampered.verifies());
    let mut transaction = Transaction::begin();
    let refusal = transaction.stage(&store, tampered).expect_err("the bytes are not the digest");
    assert_eq!(refusal.code(), "save-digest");
    assert_eq!(refusal.slot(), "slot/inventory");
}

#[test]
fn a_revision_that_is_not_newer_is_refused_and_the_mounted_record_stays() {
    let mut store = MemoryStore::new();
    committed(&mut store, "slot/inventory", 5, b"arrows=7");
    let mut transaction = Transaction::begin();
    let same = record("slot/inventory", 5, b"arrows=9");
    let refusal = transaction.stage(&store, same).expect_err("revision 5 is already mounted");
    assert_eq!(refusal.code(), "save-stale");
    assert!(refusal.message().contains("not newer"), "{}", refusal.message());
    let older = record("slot/inventory", 4, b"arrows=1");
    assert_eq!(
        transaction.stage(&store, older).expect_err("older still").code(),
        "save-stale"
    );
    assert_eq!(store.bytes("slot/inventory"), Some(b"arrows=7".to_vec()));
}

#[test]
fn one_slot_staged_twice_is_refused() {
    let store = MemoryStore::new();
    let mut transaction = Transaction::begin();
    transaction
        .stage(&store, record("slot/map", 1, b"visited=1"))
        .expect("the first staging is fine");
    let refusal = transaction
        .stage(&store, record("slot/map", 2, b"visited=2"))
        .expect_err("one slot, one record per transaction");
    assert_eq!(refusal.code(), "save-duplicate");
}

#[test]
fn the_journal_slot_cannot_be_staged() {
    let store = MemoryStore::new();
    let mut transaction = Transaction::begin();
    let refusal = transaction
        .stage(&store, record(JOURNAL_SLOT, 1, b"state\tstaged"))
        .expect_err("the journal is the library's own slot");
    assert_eq!(refusal.code(), "save-slot");
}

#[test]
fn a_store_fault_mid_commit_restores_what_it_had_replaced() {
    let mut store = MemoryStore::new();
    committed(&mut store, "slot/inventory", 4, b"arrows=3");
    // Allow the journal write and the first record, then fail the second record.
    store.fail_after(2);

    let mut transaction = Transaction::begin();
    transaction
        .stage(&store, record("slot/inventory", 5, b"arrows=7"))
        .expect("it stages");
    transaction
        .stage(&store, record("slot/map", 1, b"visited=2"))
        .expect("it stages");
    let refusal = transaction.commit(&mut store).expect_err("the store refuses");
    assert_eq!(refusal.code(), "save-store");
    assert_eq!(refusal.slot(), "slot/map", "the refusal names the write that failed");
    assert!(
        refusal.detail().contains("recovery"),
        "a store that cannot restore leaves the journal behind: {}",
        refusal.message()
    );
    assert!(
        store.get(JOURNAL_SLOT).is_some(),
        "the evidence of the interrupted commit stays until it is reconciled"
    );

    // The store comes back: recovery reconciles the interruption instead of leaving it mixed.
    store.clear_fault();
    assert_eq!(
        recover(&mut store).expect("recovery runs"),
        Recovery::RolledBack(vec!["slot/inventory".to_string(), "slot/map".to_string()])
    );
    assert_eq!(
        store.bytes("slot/inventory"),
        Some(b"arrows=3".to_vec()),
        "the replaced slot is back to its pre-operation bytes"
    );
    assert!(store.get("slot/map").is_none(), "the unwritten slot never appears");
    assert!(store.get(JOURNAL_SLOT).is_none(), "the journal is reconciled away");
}

#[test]
fn an_interrupted_commit_is_reconciled_to_rolled_back_on_restart() {
    let mut store = MemoryStore::new();
    committed(&mut store, "slot/inventory", 4, b"arrows=3");
    let before = store.get("slot/inventory").cloned().expect("the pre-image");
    let after = record("slot/inventory", 5, b"arrows=7");
    // Exactly what a crash between the record write and the commit marker leaves behind.
    let entries = vec![JournalEntry {
        slot: "slot/inventory".to_string(),
        before: Some(before),
        after: after.digest_hex(),
    }];
    store.save(journal(JournalState::Staged, &entries)).expect("journal");
    store.save(after).expect("the write that landed before the crash");

    let recovery = recover(&mut store).expect("recovery runs");
    assert_eq!(
        recovery,
        Recovery::RolledBack(vec!["slot/inventory".to_string()]),
        "an unmarked journal means the transaction did not finish"
    );
    assert_eq!(store.bytes("slot/inventory"), Some(b"arrows=3".to_vec()));
    assert!(store.get(JOURNAL_SLOT).is_none(), "the journal is reconciled away");
}

#[test]
fn a_committed_journal_reconciles_to_committed_and_is_erased() {
    let mut store = MemoryStore::new();
    let after = record("slot/map", 1, b"visited=2");
    let entries = vec![JournalEntry {
        slot: "slot/map".to_string(),
        before: None,
        after: after.digest_hex(),
    }];
    store.save(journal(JournalState::Committed, &entries)).expect("journal");
    store.save(after).expect("the committed record");

    let recovery = recover(&mut store).expect("recovery runs");
    assert_eq!(recovery, Recovery::Committed(vec!["slot/map".to_string()]));
    assert_eq!(store.bytes("slot/map"), Some(b"visited=2".to_vec()), "a commit stands");
    assert!(store.get(JOURNAL_SLOT).is_none());
    assert_eq!(recover(&mut store).expect("a second recovery"), Recovery::Clean);
}

#[test]
fn a_slot_the_transaction_never_touched_keeps_its_bytes() {
    let mut store = MemoryStore::new();
    // An unrelated namespace, with bytes no schema here reads: preserved verbatim.
    let unrelated = b"\x00opaque\xffbytes\x01from another namespace".to_vec();
    committed(&mut store, "namespace/other", 3, &unrelated);
    let untouched = store.get("namespace/other").cloned().expect("it is there");

    let mut transaction = Transaction::begin();
    transaction
        .stage(&store, record("slot/inventory", 1, b"arrows=1"))
        .expect("stages");
    transaction.commit(&mut store).expect("commits");

    let still = store.get("namespace/other").expect("still mounted");
    assert_eq!(still.bytes, unrelated);
    assert_eq!(still.digest_hex(), untouched.digest_hex());
    assert_eq!(still.revision, 3, "an untouched slot keeps its revision too");
}

#[test]
fn a_rollback_returns_every_staged_slot_to_its_snapshot() {
    let mut store = MemoryStore::new();
    committed(&mut store, "slot/inventory", 4, b"arrows=3");
    let mut transaction = Transaction::begin();
    transaction
        .stage(&store, record("slot/inventory", 5, b"arrows=7"))
        .expect("stages");
    transaction
        .stage(&store, record("slot/map", 1, b"visited=2"))
        .expect("stages");

    let slots: Vec<String> = transaction.rollback(&mut store).expect("it rolls back");
    assert_eq!(slots, vec!["slot/inventory", "slot/map"]);
    assert_eq!(store.bytes("slot/inventory"), Some(b"arrows=3".to_vec()));
    assert!(store.get("slot/map").is_none());
    assert!(store.get(JOURNAL_SLOT).is_none());
}

#[test]
fn a_refusal_is_a_result_that_names_the_slot_and_the_rule() {
    let store = MemoryStore::new();
    let mut transaction = Transaction::begin();
    let refusal: SaveRefusal = transaction
        .stage(&store, Record::new("slot/inventory", 7, 1, b"x".to_vec()))
        .expect_err("schema 7 is unknown here");
    assert!(refusal.code().starts_with("save-"));
    assert_eq!(refusal.slot(), "slot/inventory");
    assert!(refusal.message().contains("slot/inventory"));
    let _: fn(&MemoryStore, &str) -> Option<Record> = |store, slot| store.load(slot);
}
