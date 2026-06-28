//! Persist audit journals to disk (desktop).

use quantum_tetris_quantum::AuditJournal;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Write a finalized journal under `audit/{session_id}.json`.
pub fn export_journal(journal: &AuditJournal) -> io::Result<PathBuf> {
    let dir = Path::new("audit");
    fs::create_dir_all(dir)?;
    let path = dir.join(format!("{}.json", journal.session_id));
    let json = journal
        .to_json_pretty()
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write(&path, json)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quantum_tetris_quantum::{BackendKind, QuantumCircuit};

    #[test]
    fn export_writes_json_file() {
        let mut journal = AuditJournal::with_seed("export-test", BackendKind::Classic.label(), "1");
        journal.record_draw(
            &QuantumCircuit::teleporter(),
            &quantum_tetris_quantum::Measurement {
                bits: "010".into(),
                probabilities: vec![],
            },
            "spawn",
            Some("piece=T"),
            "classic",
        );
        journal.reveal_seed();
        let path = export_journal(&journal).expect("export");
        assert!(path.exists());
        let text = fs::read_to_string(&path).expect("read");
        assert!(text.contains("seed_revealed"));
        let _ = fs::remove_file(path);
    }
}
