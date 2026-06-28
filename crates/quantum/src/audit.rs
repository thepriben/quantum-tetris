//! Auditable randomness — commit-reveal session seed, hash-chained draw receipts,
//! and an append-only journal (layers C5 + C6 of the randomness infrastructure).

use crate::measurement::Measurement;
use crate::QuantumCircuit;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

const JOURNAL_VERSION: u32 = 1;
const MAPPING_POLICY: &str = "teleport-v1";
const DISTRIBUTION_POLICY: &str = "raw-3bit";

/// Hex-encoded SHA-256 digest.
pub type DigestHex = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEntry {
    pub seq: u64,
    pub moment: String,
    pub circuit: String,
    pub bits: String,
    pub effect: Option<String>,
    pub backend_used: String,
    pub entry_hash: DigestHex,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditJournal {
    pub version: u32,
    pub session_id: String,
    pub backend: String,
    pub mapping_policy: String,
    pub distribution_policy: String,
    /// `SHA-256(session_seed)` — published before any draw (commitment).
    pub seed_commitment: DigestHex,
    /// Revealed after the session ends so anyone can re-derive draws.
    pub seed_revealed: Option<String>,
    pub entries: Vec<AuditEntry>,
    pub chain_head: DigestHex,
    #[serde(skip)]
    session_seed: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditError {
    BrokenChain { seq: u64 },
    BadSeedReveal,
    EmptyJournal,
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BrokenChain { seq } => write!(f, "hash chain broken at entry {seq}"),
            Self::BadSeedReveal => write!(f, "revealed seed does not match commitment"),
            Self::EmptyJournal => write!(f, "journal has no entries"),
        }
    }
}

impl std::error::Error for AuditError {}

pub fn sha256_hex(data: &[u8]) -> DigestHex {
    let digest = Sha256::digest(data);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn commitment_from_seed(seed: &str) -> DigestHex {
    sha256_hex(seed.as_bytes())
}

fn genesis_hash(session_id: &str, seed_commitment: &str, backend: &str) -> DigestHex {
    sha256_hex(
        format!(
            "genesis|{session_id}|{seed_commitment}|{backend}|{MAPPING_POLICY}|{DISTRIBUTION_POLICY}"
        )
        .as_bytes(),
    )
}

fn entry_hash(
    chain_head: &str,
    seq: u64,
    moment: &str,
    circuit: &str,
    bits: &str,
    effect: Option<&str>,
    backend_used: &str,
) -> DigestHex {
    sha256_hex(
        format!(
            "entry|{chain_head}|{seq}|{moment}|{circuit}|{bits}|{}|{backend_used}",
            effect.unwrap_or("-")
        )
        .as_bytes(),
    )
}

impl AuditJournal {
    /// Start a new audited session. The seed commitment is fixed before the first draw.
    pub fn new(session_id: impl Into<String>, backend: impl Into<String>) -> Self {
        let seed: u64 = rand::rng().random();
        let seed_str = seed.to_string();
        let seed_commitment = commitment_from_seed(&seed_str);
        let backend = backend.into();
        let session_id = session_id.into();
        let chain_head = genesis_hash(&session_id, &seed_commitment, &backend);
        Self {
            version: JOURNAL_VERSION,
            session_id,
            backend,
            mapping_policy: MAPPING_POLICY.into(),
            distribution_policy: DISTRIBUTION_POLICY.into(),
            seed_commitment,
            seed_revealed: None,
            entries: Vec::new(),
            chain_head,
            session_seed: Some(seed_str),
        }
    }

    /// Deterministic session for tests.
    pub fn with_seed(
        session_id: impl Into<String>,
        backend: impl Into<String>,
        seed: &str,
    ) -> Self {
        let seed_commitment = commitment_from_seed(seed);
        let backend = backend.into();
        let session_id = session_id.into();
        let chain_head = genesis_hash(&session_id, &seed_commitment, &backend);
        Self {
            version: JOURNAL_VERSION,
            session_id,
            backend,
            mapping_policy: MAPPING_POLICY.into(),
            distribution_policy: DISTRIBUTION_POLICY.into(),
            seed_commitment,
            seed_revealed: None,
            entries: Vec::new(),
            chain_head,
            session_seed: Some(seed.into()),
        }
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Record one circuit draw and extend the hash chain (C5 receipt + C6 journal append).
    pub fn record_draw(
        &mut self,
        circuit: &QuantumCircuit,
        measurement: &Measurement,
        moment: &str,
        effect: Option<&str>,
        backend_used: &str,
    ) {
        let seq = self.entries.len() as u64 + 1;
        let hash = entry_hash(
            &self.chain_head,
            seq,
            moment,
            &circuit.label,
            &measurement.bits,
            effect,
            backend_used,
        );
        self.entries.push(AuditEntry {
            seq,
            moment: moment.into(),
            circuit: circuit.label.clone(),
            bits: measurement.bits.clone(),
            effect: effect.map(str::to_string),
            backend_used: backend_used.into(),
            entry_hash: hash.clone(),
        });
        self.chain_head = hash;
    }

    /// Reveal the session seed (commit-reveal second phase).
    pub fn reveal_seed(&mut self) {
        if let Some(seed) = self.session_seed.take() {
            self.seed_revealed = Some(seed);
        }
    }

    /// Verify hash chain integrity and, if revealed, seed commitment.
    pub fn verify(&self) -> Result<(), AuditError> {
        if self.entries.is_empty() {
            return Err(AuditError::EmptyJournal);
        }
        if let Some(seed) = &self.seed_revealed {
            if commitment_from_seed(seed) != self.seed_commitment {
                return Err(AuditError::BadSeedReveal);
            }
        }

        let mut chain = genesis_hash(&self.session_id, &self.seed_commitment, &self.backend);
        for entry in &self.entries {
            let expected = entry_hash(
                &chain,
                entry.seq,
                &entry.moment,
                &entry.circuit,
                &entry.bits,
                entry.effect.as_deref(),
                &entry.backend_used,
            );
            if expected != entry.entry_hash {
                return Err(AuditError::BrokenChain { seq: entry.seq });
            }
            chain = entry.entry_hash.clone();
        }
        if chain != self.chain_head {
            return Err(AuditError::BrokenChain {
                seq: self.entries.len() as u64,
            });
        }
        Ok(())
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load a journal from JSON (for offline verification).
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BackendKind;
    use crate::QuantumCircuit;

    fn sample_measurement(bits: &str) -> Measurement {
        Measurement {
            bits: bits.into(),
            probabilities: vec![],
        }
    }

    #[test]
    fn commitment_and_reveal_match() {
        let mut journal = AuditJournal::with_seed("test", "classic", "424242");
        assert_eq!(journal.seed_commitment, commitment_from_seed("424242"));
        journal.record_draw(
            &QuantumCircuit::teleporter(),
            &sample_measurement("010"),
            "spawn",
            Some("piece=T"),
            "classic",
        );
        journal.reveal_seed();
        assert!(journal.verify().is_ok());
    }

    #[test]
    fn tampered_entry_breaks_chain() {
        let mut journal = AuditJournal::with_seed("test", "classic", "1");
        journal.record_draw(
            &QuantumCircuit::teleporter(),
            &sample_measurement("000"),
            "spawn",
            None,
            "classic",
        );
        journal.entries[0].bits = "111".into();
        assert!(matches!(
            journal.verify(),
            Err(AuditError::BrokenChain { seq: 1 })
        ));
    }

    #[test]
    fn wrong_seed_reveal_fails() {
        let mut journal = AuditJournal::with_seed("test", "classic", "secret");
        journal.record_draw(
            &QuantumCircuit::teleporter(),
            &sample_measurement("101"),
            "spawn",
            None,
            "classic",
        );
        journal.seed_revealed = Some("wrong".into());
        assert!(matches!(journal.verify(), Err(AuditError::BadSeedReveal)));
    }

    #[test]
    fn journal_serializes_without_internal_seed() {
        let mut journal = AuditJournal::with_seed("s1", BackendKind::Classic.label(), "99");
        journal.record_draw(
            &QuantumCircuit::imp_brain(),
            &sample_measurement("01"),
            "spawn",
            Some("rot=1"),
            "classic",
        );
        let json = journal.to_json_pretty().unwrap();
        assert!(json.contains("seed_commitment"));
        assert!(json.contains("entry_hash"));
        assert!(!json.contains("session_seed"));
    }
}
