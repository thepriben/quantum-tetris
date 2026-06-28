//! Public 3-bit teleporter → tetromino mapping (layer C4).

/// Tetromino label from a `quantum-teleportation-gate-v1` readout.
///
/// Eight bitstrings map onto seven pieces; `111` folds onto `T` (documented bias).
pub fn teleport_piece_label(bits: &str) -> &'static str {
    match bits {
        "000" => "I",
        "001" => "O",
        "010" => "T",
        "011" => "S",
        "100" => "Z",
        "101" => "J",
        "110" => "L",
        "111" => "T",
        _ => "T",
    }
}

/// Analytical piece probabilities under uniform 3-bit draws.
pub fn teleport_uniform_piece_probabilities() -> [(&'static str, f64); 7] {
    [
        ("I", 1.0 / 8.0),
        ("O", 1.0 / 8.0),
        ("T", 2.0 / 8.0),
        ("S", 1.0 / 8.0),
        ("Z", 1.0 / 8.0),
        ("J", 1.0 / 8.0),
        ("L", 1.0 / 8.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_piece_is_doubled_under_uniform_bits() {
        let probs = teleport_uniform_piece_probabilities();
        let t = probs.iter().find(|(label, _)| *label == "T").unwrap().1;
        let i = probs.iter().find(|(label, _)| *label == "I").unwrap().1;
        assert!((t - 0.25).abs() < 1e-9);
        assert!((i - 0.125).abs() < 1e-9);
        assert!((t - 2.0 * i).abs() < 1e-9);
    }
}
