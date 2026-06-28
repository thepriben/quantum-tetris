//! Micro-benchmark for the auditable randomness layers (C5 receipt + C6 journal).
//!
//! Measures, on the host machine:
//!   * backend draw latency (Classic uniform vs RustQIP statevector) for the
//!     3-qubit teleportation circuit used for piece selection;
//!   * the marginal cost of the audit layer per draw (hash-chained receipt +
//!     append to the journal);
//!   * full-journal offline verification cost;
//!   * serialized journal size per entry.
//!
//! Run with: `cargo run --release --example bench_audit -p quantum-tetris-quantum`

use quantum_tetris_quantum::{
    build_backend, AuditJournal, BackendKind, QuantumBackend, QuantumCircuit,
};
use std::time::Instant;

fn bench_backend(kind: BackendKind, circuit: &QuantumCircuit, n: u32) -> f64 {
    let mut backend = build_backend(kind).expect("backend");
    // warmup
    for _ in 0..1_000 {
        let _ = backend.run(circuit).expect("run");
    }
    let t = Instant::now();
    for _ in 0..n {
        let _ = backend.run(circuit).expect("run");
    }
    t.elapsed().as_nanos() as f64 / n as f64
}

fn main() {
    let tele = QuantumCircuit::teleporter(); // 3 qubits -> piece selection
    let n: u32 = 200_000;

    println!("== Backend draw latency (3-qubit teleportation circuit) ==");
    let classic_ns = bench_backend(BackendKind::Classic, &tele, n);
    let quantum_ns = bench_backend(BackendKind::Quantum, &tele, n);
    println!("  Classic (uniform RNG) : {:8.1} ns/draw", classic_ns);
    println!("  RustQIP (statevector) : {:8.1} ns/draw", quantum_ns);

    println!("\n== Audit layer marginal cost (C5 receipt + C6 append) ==");
    // Use one fixed measurement so we isolate the audit cost from the backend.
    let mut classic = build_backend(BackendKind::Classic).expect("classic");
    let m = classic.run(&tele).expect("measure");
    let mut journal = AuditJournal::with_seed("bench", "classic", "424242");
    // warmup
    for _ in 0..1_000 {
        journal.record_draw(&tele, &m, "spawn_piece", Some("piece=T"), "classic");
    }
    let mut journal = AuditJournal::with_seed("bench", "classic", "424242");
    let t = Instant::now();
    for _ in 0..n {
        journal.record_draw(&tele, &m, "spawn_piece", Some("piece=T"), "classic");
    }
    let record_ns = t.elapsed().as_nanos() as f64 / n as f64;
    println!("  record_draw           : {:8.1} ns/draw", record_ns);
    println!("  entries recorded      : {}", journal.entry_count());

    println!("\n== Offline verification ==");
    journal.reveal_seed();
    let t = Instant::now();
    journal.verify().expect("verify ok");
    let verify_total = t.elapsed();
    println!(
        "  verify {} entries     : {:.2} ms total ({:.1} ns/entry)",
        journal.entry_count(),
        verify_total.as_secs_f64() * 1e3,
        verify_total.as_nanos() as f64 / journal.entry_count() as f64
    );

    println!("\n== Serialized journal size ==");
    let compact = serde_json::to_string(&journal).expect("json");
    let pretty = journal.to_json_pretty().expect("json");
    println!(
        "  compact               : {:.1} bytes/entry",
        compact.len() as f64 / journal.entry_count() as f64
    );
    println!(
        "  pretty                : {:.1} bytes/entry",
        pretty.len() as f64 / journal.entry_count() as f64
    );

    // Realistic session extrapolation: 3 draws per spawn (piece, rotation,
    // gravity) plus occasional observe/line-clear draws.
    let draws_per_min = 3.0 * 60.0; // ~1 spawn/s, conservative
    println!(
        "\n== Extrapolation (a ~10 min match, ~{} draws) ==",
        (draws_per_min * 10.0) as u32
    );
    let draws = draws_per_min * 10.0;
    println!(
        "  audit CPU overhead    : {:.2} ms total",
        record_ns * draws / 1e6
    );
    println!(
        "  journal size (compact): {:.1} KiB",
        compact.len() as f64 / journal.entry_count() as f64 * draws / 1024.0
    );
}
