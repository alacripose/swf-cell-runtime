//! The generic save/recovery transaction machinery the shell calls.
//!
//! Ownership, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.8: *"The shell/runtime
//! alone selects, authorizes, stages, and executes any save migration, package replacement, addon
//! unload, state rewrite, rollback, or recovery reconciliation."* and *"An agent may implement generic
//! transaction machinery against synthetic fixtures through a reviewed code ticket, but it cannot
//! inspect live saves, decide migration semantics, initiate a live migration, or perform a destructive
//! content operation."*
//!
//! So this crate is mechanics and nothing else. It takes records the caller hands it, checks the schema
//! and the digest the caller declared, writes a journal carrying the pre-operation snapshot and the
//! before/after digests, commits through a store the caller supplies, keeps a rollback path, and
//! reconciles an interrupted transaction into a named recovery state. It reads no save of its own,
//! decides no migration, writes nothing outside the store it is given, and has no authority: the shell
//! calls it, and every state the plan assigns stays the shell's.
//!
//! Records are opaque bytes keyed by slot, so a slot a transaction never touches keeps exactly the
//! bytes it had (the plan's "unknown fields and unrelated base-game namespaces must be preserved").

use planet_shell_ledger::sha256;

/// The digest framing's domain for a save record: `domain || schema (LE u16) || len (LE u64) || bytes`.
///
/// It carries no version suffix on purpose: the schema inside the preimage is the version, exactly as
/// the package family's domain does, and a second format name is a second format (AGENTS.md §2).
pub const DOMAIN: &[u8] = b"planet-save";
/// The record schema this library reads and writes.
pub const SUPPORTED_SCHEMA: u16 = 1;
/// The one slot the journal lives in. It may never be staged as a save.
pub const JOURNAL_SLOT: &str = "planet-save/journal";
/// The journal record's own schema.
pub const JOURNAL_SCHEMA: u16 = 1;

pub fn record_digest(schema: u16, bytes: &[u8]) -> [u8; 32] {
    let mut message = Vec::with_capacity(DOMAIN.len() + 10 + bytes.len());
    message.extend_from_slice(DOMAIN);
    message.extend_from_slice(&schema.to_le_bytes());
    message.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    message.extend_from_slice(bytes);
    sha256::sha256(&message)
}

fn to_hex(bytes: &[u8]) -> String {
    sha256::hex(bytes)
}

fn from_hex(text: &str) -> Option<Vec<u8>> {
    if text.len() % 2 != 0 {
        return None;
    }
    (0..text.len() / 2)
        .map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).ok())
        .collect()
}

/// One refusal: a `save-*` code, the slot it names and what was wrong. Never a panic, never a warning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveRefusal {
    code: &'static str,
    slot: String,
    detail: String,
}

impl SaveRefusal {
    fn new(code: &'static str, slot: &str, detail: impl Into<String>) -> SaveRefusal {
        SaveRefusal {
            code,
            slot: slot.to_string(),
            detail: detail.into(),
        }
    }

    /// A store's own fault, for an implementation of [`Store`] to return.
    pub fn store(slot: &str, detail: impl Into<String>) -> SaveRefusal {
        SaveRefusal::new("save-store", slot, detail)
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn slot(&self) -> &str {
        &self.slot
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn message(&self) -> String {
        format!("{} at {}: {}", self.code, self.slot, self.detail)
    }
}

/// One save record: opaque bytes, the schema that reads them, the revision they replace, and the
/// digest the caller declared for them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub slot: String,
    pub schema: u16,
    pub revision: u64,
    pub bytes: Vec<u8>,
    digest: [u8; 32],
}

impl Record {
    /// A record whose digest is computed from its own bytes.
    pub fn new(slot: &str, schema: u16, revision: u64, bytes: Vec<u8>) -> Record {
        let digest = record_digest(schema, &bytes);
        Record {
            slot: slot.to_string(),
            schema,
            revision,
            bytes,
            digest,
        }
    }

    /// A record that declares a digest its bytes do not hash to: a synthetic tamper fixture, and the
    /// shape a corrupt save arrives in.
    pub fn declared(
        slot: &str,
        schema: u16,
        revision: u64,
        bytes: Vec<u8>,
        digest: [u8; 32],
    ) -> Record {
        Record {
            slot: slot.to_string(),
            schema,
            revision,
            bytes,
            digest,
        }
    }

    pub fn digest(&self) -> [u8; 32] {
        self.digest
    }

    pub fn digest_hex(&self) -> String {
        to_hex(&self.digest)
    }

    /// Whether the declared digest is the digest of the bytes.
    pub fn verifies(&self) -> bool {
        record_digest(self.schema, &self.bytes) == self.digest
    }
}

/// What a transaction commits through. The shell supplies the real store; this crate never opens,
/// reads or writes anything itself.
pub trait Store {
    fn load(&self, slot: &str) -> Option<Record>;
    fn save(&mut self, record: Record) -> Result<(), SaveRefusal>;
    fn erase(&mut self, slot: &str) -> Result<(), SaveRefusal>;
}

/// A reference store for synthetic fixtures: records in memory, with an injectable fault so an
/// interrupted commit can be exercised without a filesystem.
#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    records: Vec<Record>,
    remaining_writes: Option<usize>,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        MemoryStore::default()
    }

    pub fn get(&self, slot: &str) -> Option<&Record> {
        self.records.iter().find(|record| record.slot == slot)
    }

    pub fn bytes(&self, slot: &str) -> Option<Vec<u8>> {
        self.get(slot).map(|record| record.bytes.clone())
    }

    pub fn slots(&self) -> Vec<String> {
        self.records.iter().map(|record| record.slot.clone()).collect()
    }

    /// Allow this many writes, then fail the next one: a synthetic interruption.
    pub fn fail_after(&mut self, writes: usize) {
        self.remaining_writes = Some(writes);
    }

    /// Let writes through again, so a test can run recovery after an interruption.
    pub fn clear_fault(&mut self) {
        self.remaining_writes = None;
    }
}

impl Store for MemoryStore {
    fn load(&self, slot: &str) -> Option<Record> {
        self.get(slot).cloned()
    }

    fn save(&mut self, record: Record) -> Result<(), SaveRefusal> {
        if let Some(remaining) = self.remaining_writes {
            if remaining == 0 {
                return Err(SaveRefusal::store(&record.slot, "the store refused the write"));
            }
            self.remaining_writes = Some(remaining - 1);
        }
        match self.records.iter_mut().find(|held| held.slot == record.slot) {
            Some(held) => *held = record,
            None => self.records.push(record),
        }
        Ok(())
    }

    fn erase(&mut self, slot: &str) -> Result<(), SaveRefusal> {
        self.records.retain(|record| record.slot != slot);
        Ok(())
    }
}

/// Where an in-flight journal stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalState {
    Staged,
    Committed,
}

impl JournalState {
    fn token(self) -> &'static str {
        match self {
            JournalState::Staged => "staged",
            JournalState::Committed => "committed",
        }
    }

    fn parse(text: &str) -> Option<JournalState> {
        match text {
            "staged" => Some(JournalState::Staged),
            "committed" => Some(JournalState::Committed),
            _ => None,
        }
    }
}

/// One journal entry: the slot, its immutable pre-operation snapshot, and the digest it was replaced by.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JournalEntry {
    pub slot: String,
    pub before: Option<Record>,
    pub after: String,
}

/// The journal record for a state and its entries, encoded as text rows so it stays inspectable.
pub fn journal(state: JournalState, entries: &[JournalEntry]) -> Record {
    let mut text = String::new();
    text.push_str("state\t");
    text.push_str(state.token());
    text.push('\n');
    for entry in entries {
        let before = match &entry.before {
            None => "-\t-\t-".to_string(),
            Some(record) => format!(
                "{}\t{}\t{}",
                record.schema,
                record.revision,
                to_hex(&record.bytes)
            ),
        };
        text.push_str(&format!(
            "entry\t{}\t{}\t{}\n",
            entry.slot, before, entry.after
        ));
    }
    Record::new(JOURNAL_SLOT, JOURNAL_SCHEMA, 1, text.into_bytes())
}

fn malformed() -> SaveRefusal {
    SaveRefusal::new("save-journal", JOURNAL_SLOT, "the journal is not readable")
}

fn parse_journal(record: &Record) -> Result<(JournalState, Vec<JournalEntry>), SaveRefusal> {
    let text = std::str::from_utf8(&record.bytes).map_err(|_| malformed())?;
    let mut state: Option<JournalState> = None;
    let mut entries: Vec<JournalEntry> = Vec::new();
    for line in text.lines() {
        let mut parts = line.split('\t');
        match parts.next() {
            Some("state") => {
                state = parts.next().and_then(JournalState::parse);
            }
            Some("entry") => {
                let slot = parts.next().ok_or_else(malformed)?;
                let schema = parts.next().ok_or_else(malformed)?;
                let revision = parts.next().ok_or_else(malformed)?;
                let bytes = parts.next().ok_or_else(malformed)?;
                let after = parts.next().ok_or_else(malformed)?;
                let before = if schema == "-" {
                    None
                } else {
                    let parsed_schema = schema.parse().map_err(|_| malformed())?;
                    let parsed_revision = revision.parse().map_err(|_| malformed())?;
                    let parsed_bytes = from_hex(bytes).ok_or_else(malformed)?;
                    Some(Record::new(slot, parsed_schema, parsed_revision, parsed_bytes))
                };
                entries.push(JournalEntry {
                    slot: slot.to_string(),
                    before,
                    after: after.to_string(),
                });
            }
            _ => return Err(malformed()),
        }
    }
    state.map(|state| (state, entries)).ok_or_else(malformed)
}

/// What a commit did, in the shell's terms: the slots, their pre-operation digests and their new ones.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub slots: Vec<String>,
    /// The pre-operation digest per slot, or `-` when the slot did not exist.
    pub before: Vec<String>,
    pub after: Vec<String>,
}

/// The named recovery state a restart reports, per the plan's "the shell/runtime reports a named
/// recovery state and reconciles in-flight operations".
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Recovery {
    /// No journal existed: nothing was in flight.
    Clean,
    /// A journal said committed: the new revision stands and the journal is erased.
    Committed(Vec<String>),
    /// A journal was in flight: the pre-operation snapshots were restored and the journal erased.
    RolledBack(Vec<String>),
    /// A slot matched neither its before nor its after digest: the shell has to look at these.
    Unresolved(Vec<String>),
}

#[derive(Clone, Debug)]
struct Staged {
    record: Record,
    before: Option<Record>,
}

/// One staged change set. Staging checks what the caller declared; committing is the only write path.
#[derive(Clone, Debug, Default)]
pub struct Transaction {
    staged: Vec<Staged>,
}

impl Transaction {
    pub fn begin() -> Transaction {
        Transaction::default()
    }

    /// Stage one record: its schema, digest, revision and slot are all checked before it is accepted.
    pub fn stage<S: Store>(&mut self, store: &S, record: Record) -> Result<(), SaveRefusal> {
        if record.slot == JOURNAL_SLOT {
            return Err(SaveRefusal::new(
                "save-slot",
                &record.slot,
                "the journal slot is reserved",
            ));
        }
        if record.schema != SUPPORTED_SCHEMA {
            return Err(SaveRefusal::new(
                "save-schema",
                &record.slot,
                format!(
                    "schema {} is not the supported {SUPPORTED_SCHEMA}",
                    record.schema
                ),
            ));
        }
        if !record.verifies() {
            return Err(SaveRefusal::new(
                "save-digest",
                &record.slot,
                "the declared digest is not the digest of these bytes",
            ));
        }
        if self
            .staged
            .iter()
            .any(|staged| staged.record.slot == record.slot)
        {
            return Err(SaveRefusal::new(
                "save-duplicate",
                &record.slot,
                "the slot is already staged in this transaction",
            ));
        }
        let before = store.load(&record.slot);
        if let Some(existing) = &before {
            if existing.revision >= record.revision {
                return Err(SaveRefusal::new(
                    "save-stale",
                    &record.slot,
                    format!(
                        "revision {} is not newer than the mounted {}",
                        record.revision, existing.revision
                    ),
                ));
            }
        }
        self.staged.push(Staged { record, before });
        Ok(())
    }

    /// Commit atomically: journal first, then the records, then the journal says committed and goes.
    /// A store fault restores every snapshot this commit had replaced and leaves the journal erased.
    pub fn commit<S: Store>(self, store: &mut S) -> Result<Receipt, SaveRefusal> {
        let entries: Vec<JournalEntry> = self
            .staged
            .iter()
            .map(|staged| JournalEntry {
                slot: staged.record.slot.clone(),
                before: staged.before.clone(),
                after: staged.record.digest_hex(),
            })
            .collect();
        store.save(journal(JournalState::Staged, &entries))?;

        let mut written: Vec<usize> = Vec::new();
        for (index, staged) in self.staged.iter().enumerate() {
            match store.save(staged.record.clone()) {
                Ok(()) => written.push(index),
                Err(fault) => {
                    let mut restored = true;
                    for done in written.iter().rev() {
                        let held = &self.staged[*done];
                        let outcome = match &held.before {
                            Some(before) => store.save(before.clone()),
                            None => store.erase(&held.record.slot),
                        };
                        restored &= outcome.is_ok();
                    }
                    // The journal may only be erased once every snapshot is back. If the store is
                    // still refusing writes, the journal stays so recovery reconciles the
                    // interruption later: a half-applied set is never left unattended.
                    if restored {
                        let _ = store.erase(JOURNAL_SLOT);
                        return Err(SaveRefusal::new(
                            "save-store",
                            &staged.record.slot,
                            fault.detail().to_string(),
                        ));
                    }
                    return Err(SaveRefusal::new(
                        "save-store",
                        &staged.record.slot,
                        format!("{}; the journal is left for recovery", fault.detail()),
                    ));
                }
            }
        }

        store.save(journal(JournalState::Committed, &entries))?;
        store.erase(JOURNAL_SLOT)?;
        Ok(Receipt {
            slots: entries.iter().map(|entry| entry.slot.clone()).collect(),
            before: entries
                .iter()
                .map(|entry| match &entry.before {
                    Some(before) => before.digest_hex(),
                    None => "-".to_string(),
                })
                .collect(),
            after: entries.iter().map(|entry| entry.after.clone()).collect(),
        })
    }

    /// Abandon the staged set: every slot returns to its immutable pre-operation snapshot.
    pub fn rollback<S: Store>(self, store: &mut S) -> Result<Vec<String>, SaveRefusal> {
        let mut slots = Vec::new();
        for staged in self.staged.iter().rev() {
            match &staged.before {
                Some(before) => store.save(before.clone())?,
                None => store.erase(&staged.record.slot)?,
            }
            slots.push(staged.record.slot.clone());
        }
        if store.load(JOURNAL_SLOT).is_some() {
            store.erase(JOURNAL_SLOT)?;
        }
        slots.reverse();
        Ok(slots)
    }
}

/// Reconcile whatever a crash left behind, and report the named state it resolved to.
pub fn recover<S: Store>(store: &mut S) -> Result<Recovery, SaveRefusal> {
    let Some(held) = store.load(JOURNAL_SLOT) else {
        return Ok(Recovery::Clean);
    };
    let (state, entries) = parse_journal(&held)?;
    let slots: Vec<String> = entries.iter().map(|entry| entry.slot.clone()).collect();
    match state {
        JournalState::Committed => {
            store.erase(JOURNAL_SLOT)?;
            Ok(Recovery::Committed(slots))
        }
        JournalState::Staged => {
            let mut unresolved: Vec<String> = Vec::new();
            for entry in &entries {
                let current = store.load(&entry.slot);
                let at_after = current
                    .as_ref()
                    .map(|record| record.digest_hex())
                    .as_ref()
                    == Some(&entry.after);
                let at_before = match (&current, &entry.before) {
                    (None, None) => true,
                    (Some(now), Some(before)) => now.digest_hex() == before.digest_hex(),
                    _ => false,
                };
                if at_after {
                    match &entry.before {
                        Some(before) => {
                            store.save(before.clone())?;
                        }
                        None => {
                            store.erase(&entry.slot)?;
                        }
                    }
                } else if !at_before {
                    unresolved.push(entry.slot.clone());
                }
            }
            if unresolved.is_empty() {
                store.erase(JOURNAL_SLOT)?;
                Ok(Recovery::RolledBack(slots))
            } else {
                Ok(Recovery::Unresolved(unresolved))
            }
        }
    }
}
