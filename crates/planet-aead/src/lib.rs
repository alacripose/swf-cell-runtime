//! Domain-separated sealing for the things this family writes: package frames, ledger rows and
//! store records. One key per package; rows that span packages carry a store key.
//!
//! The cipher is XChaCha20-Poly1305. The nonce is derived from the domain and the plaintext, so
//! sealing the same bytes twice answers the same ciphertext and re-sealing cannot reuse a keystream
//! under a different message. The domain is also the AAD, so a row sealed for one purpose does not
//! open as another.

use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use sha2::{Digest, Sha256};

pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 24;

/// What a sealed blob is for. The label is both the AAD and the nonce's first ingredient.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    Frame,
    LedgerRow,
    StoreRecord,
}

impl Domain {
    pub fn label(self) -> &'static str {
        match self {
            Domain::Frame => "planet-frame-aead-v1",
            Domain::LedgerRow => "secure-shell-ledger-row-aead-v1",
            Domain::StoreRecord => "secure-shell-store-record-aead-v1",
        }
    }
}

/// A 32-byte key. Held by the host; never written into a package or a ledger row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Key([u8; KEY_LEN]);

impl Key {
    pub fn from_bytes(bytes: [u8; KEY_LEN]) -> Key {
        Key(bytes)
    }

    /// Parse 64 hex characters. A short, long or non-hex value is refused by name.
    pub fn from_hex(text: &str) -> Result<Key, Refusal> {
        let text = text.trim();
        if text.len() != KEY_LEN * 2 {
            return Err(Refusal::BadKey {
                found: text.len(),
                wanted: KEY_LEN * 2,
            });
        }
        let mut bytes = [0u8; KEY_LEN];
        for (index, slot) in bytes.iter_mut().enumerate() {
            let pair = &text[index * 2..index * 2 + 2];
            *slot = u8::from_str_radix(pair, 16).map_err(|_| Refusal::BadKey {
                found: text.len(),
                wanted: KEY_LEN * 2,
            })?;
        }
        Ok(Key(bytes))
    }

    pub fn to_hex(&self) -> String {
        self.0.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    /// A short public identifier for a row, so a ledger says which key sealed it without carrying it.
    pub fn id(&self) -> String {
        let digest = Sha256::digest(self.0);
        digest[..8].iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    MissingKey,
    BadKey { found: usize, wanted: usize },
    Short { found: usize },
    Tag,
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::MissingKey => "aead-missing",
            Refusal::BadKey { .. } => "aead-key",
            Refusal::Short { .. } => "aead-short",
            Refusal::Tag => "aead-tag",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::MissingKey => "no key was supplied for this domain".to_string(),
            Refusal::BadKey { found, wanted } => {
                format!("key is {found} hex characters; {wanted} are required")
            }
            Refusal::Short { found } => {
                format!("sealed blob is {found} bytes; a nonce alone needs {NONCE_LEN}")
            }
            Refusal::Tag => "the tag did not verify: wrong key, wrong domain, or tampered bytes".to_string(),
        }
    }
}

pub fn nonce_for(domain: Domain, plaintext: &[u8]) -> [u8; NONCE_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(domain.label().as_bytes());
    hasher.update((plaintext.len() as u64).to_le_bytes());
    hasher.update(plaintext);
    let digest = hasher.finalize();
    let mut nonce = [0u8; NONCE_LEN];
    nonce.copy_from_slice(&digest[..NONCE_LEN]);
    nonce
}

/// Seal `plaintext` for `domain`: returns `nonce || ciphertext || tag`.
pub fn seal(key: &Key, domain: Domain, plaintext: &[u8]) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new((&key.0).into());
    let nonce = nonce_for(domain, plaintext);
    let mut out = Vec::with_capacity(NONCE_LEN + plaintext.len() + 16);
    out.extend_from_slice(&nonce);
    let sealed = cipher
        .encrypt(
            XNonce::from_slice(&nonce),
            Payload {
                msg: plaintext,
                aad: domain.label().as_bytes(),
            },
        )
        .expect("encrypting with a fixed-size key cannot fail");
    out.extend_from_slice(&sealed);
    out
}

/// Open a blob sealed by [`seal`]. A wrong key, a wrong domain and a tampered byte all answer
/// `aead-tag`; nothing is returned on failure.
pub fn open(key: &Key, domain: Domain, sealed: &[u8]) -> Result<Vec<u8>, Refusal> {
    if sealed.len() < NONCE_LEN + 16 {
        return Err(Refusal::Short { found: sealed.len() });
    }
    let (nonce, body) = sealed.split_at(NONCE_LEN);
    let cipher = XChaCha20Poly1305::new((&key.0).into());
    cipher
        .decrypt(
            XNonce::from_slice(nonce),
            Payload {
                msg: body,
                aad: domain.label().as_bytes(),
            },
        )
        .map_err(|_| Refusal::Tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(byte: u8) -> Key {
        Key::from_bytes([byte; KEY_LEN])
    }

    #[test]
    fn round_trips_each_domain() {
        for domain in [Domain::Frame, Domain::LedgerRow, Domain::StoreRecord] {
            let sealed = seal(&key(7), domain, b"the mounted declaration");
            assert!(sealed.len() > b"the mounted declaration".len());
            assert_eq!(open(&key(7), domain, &sealed).unwrap(), b"the mounted declaration");
        }
    }

    #[test]
    fn a_row_does_not_open_in_another_domain() {
        let sealed = seal(&key(7), Domain::LedgerRow, b"one row");
        assert_eq!(open(&key(7), Domain::StoreRecord, &sealed), Err(Refusal::Tag));
        assert_eq!(Refusal::Tag.code(), "aead-tag");
    }

    #[test]
    fn a_tampered_byte_is_refused() {
        let mut sealed = seal(&key(7), Domain::Frame, b"payload");
        let last = sealed.len() - 1;
        sealed[last] ^= 0x01;
        assert_eq!(open(&key(7), Domain::Frame, &sealed), Err(Refusal::Tag));
    }

    #[test]
    fn a_wrong_key_is_refused() {
        let sealed = seal(&key(7), Domain::Frame, b"payload");
        assert_eq!(open(&key(8), Domain::Frame, &sealed), Err(Refusal::Tag));
    }

    #[test]
    fn sealing_twice_is_stable() {
        let first = seal(&key(7), Domain::Frame, b"payload");
        let second = seal(&key(7), Domain::Frame, b"payload");
        assert_eq!(first, second, "a deterministic nonce must not move");
    }

    #[test]
    fn keys_parse_and_refuse_by_name() {
        let hex = key(7).to_hex();
        assert_eq!(Key::from_hex(&hex).unwrap(), key(7));
        assert_eq!(Key::from_hex("00").unwrap_err().code(), "aead-key");
        let short = open(&key(7), Domain::Frame, &[0u8; 4]).unwrap_err();
        assert_eq!(short.code(), "aead-short");
        assert_eq!(Refusal::MissingKey.code(), "aead-missing");
        assert_eq!(key(7).id().len(), 16);
    }
}
