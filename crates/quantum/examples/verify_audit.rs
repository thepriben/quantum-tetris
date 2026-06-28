//! Verify an exported audit journal JSON file.
//!
//! Usage: `cargo run -p quantum-tetris-quantum --example verify_audit -- audit/qt-123.json`

use quantum_tetris_quantum::AuditJournal;
use std::env;
use std::fs;
use std::process;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: verify_audit <journal.json>");
        process::exit(2);
    });

    let text = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("Cannot read {path}: {error}");
        process::exit(1);
    });

    let journal = AuditJournal::from_json(&text).unwrap_or_else(|error| {
        eprintln!("Invalid JSON: {error}");
        process::exit(1);
    });

    journal.verify().unwrap_or_else(|error| {
        eprintln!("Verification failed: {error}");
        process::exit(1);
    });

    println!("OK — session {}", journal.session_id);
    println!("  entries: {}", journal.entry_count());
    println!("  mapping: {}", journal.mapping_policy);
    println!("  distribution: {}", journal.distribution_policy);
    println!("  seed commitment: {}", journal.seed_commitment);
    if let Some(seed) = &journal.seed_revealed {
        println!("  seed revealed: {seed}");
    }
}
