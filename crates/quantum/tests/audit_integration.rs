//! End-to-end audit journal: record, export JSON, verify offline.

use quantum_tetris_quantum::{
    build_backend, AuditJournal, BackendKind, QuantumBackend, QuantumCircuit,
};

#[test]
fn gameplay_session_journal_verifies_after_export() {
    let mut backend = build_backend(BackendKind::Classic).expect("classic");
    let mut journal = AuditJournal::with_seed("integration", BackendKind::Classic.label(), "777");

    let piece = backend.run(&QuantumCircuit::teleporter()).expect("piece");
    journal.record_draw(
        &QuantumCircuit::teleporter(),
        &piece,
        "spawn_piece",
        Some("piece=T"),
        "classic",
    );

    let rot = backend.run(&QuantumCircuit::imp_brain()).expect("rot");
    journal.record_draw(
        &QuantumCircuit::imp_brain(),
        &rot,
        "spawn_rotation",
        Some("rot=1"),
        "classic",
    );

    journal.reveal_seed();
    let json = journal.to_json_pretty().expect("json");
    let loaded = AuditJournal::from_json(&json).expect("parse");
    loaded.verify().expect("verify");
    assert_eq!(loaded.entry_count(), 2);
    assert!(loaded.seed_revealed.is_some());
}
