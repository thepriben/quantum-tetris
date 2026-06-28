//! Empirical confirmation of teleport → piece frequencies (mapping layer C4).

use quantum_tetris_quantum::{
    build_backend, rustqip_probabilities, teleport_piece_label,
    teleport_uniform_piece_probabilities, BackendKind, QuantumBackend, QuantumCircuit,
};

const SAMPLES: usize = 80_000;
const TOL: f64 = 0.015;

#[test]
fn teleporter_piece_frequencies_match_uniform_mapping_classic() {
    let expected = piece_probs_from_uniform_bits();
    measure_against_expected(BackendKind::Classic, &expected);
}

#[test]
fn teleporter_piece_frequencies_match_rustqip_golden() {
    let expected = piece_probs_from_rustqip_golden();
    measure_against_expected(BackendKind::Quantum, &expected);
}

fn piece_probs_from_uniform_bits() -> Vec<(&'static str, f64)> {
    teleport_uniform_piece_probabilities().to_vec()
}

fn piece_probs_from_rustqip_golden() -> Vec<(&'static str, f64)> {
    let circuit = QuantumCircuit::teleporter();
    let probs = rustqip_probabilities(&circuit).expect("rustqip");
    let mut by_piece: std::collections::HashMap<&str, f64> = std::collections::HashMap::new();
    for (bits, p) in probs {
        *by_piece.entry(teleport_piece_label(&bits)).or_insert(0.0) += p as f64;
    }
    let mut out: Vec<_> = by_piece.into_iter().collect();
    out.sort_by_key(|(label, _)| *label);
    out
}

fn measure_against_expected(kind: BackendKind, expected: &[(&'static str, f64)]) {
    let mut backend = build_backend(kind).expect("backend");
    let circuit = QuantumCircuit::teleporter();
    let mut counts = std::collections::HashMap::new();

    for _ in 0..SAMPLES {
        let m = backend.run(&circuit).expect("run");
        *counts
            .entry(teleport_piece_label(&m.bits))
            .or_insert(0usize) += 1;
    }

    for (label, prob) in expected {
        let observed = *counts.get(label).unwrap_or(&0) as f64 / SAMPLES as f64;
        assert!(
            (observed - prob).abs() < TOL,
            "backend={kind:?} piece={label} expected={prob:.4} observed={observed:.4}"
        );
    }
}

#[test]
fn uniform_mapping_gives_t_twice_the_mass_of_other_pieces() {
    let probs = teleport_uniform_piece_probabilities();
    let t = probs.iter().find(|(l, _)| *l == "T").unwrap().1;
    let i = probs.iter().find(|(l, _)| *l == "I").unwrap().1;
    assert!((t - 0.25).abs() < 1e-9);
    assert!((i - 0.125).abs() < 1e-9);
}
