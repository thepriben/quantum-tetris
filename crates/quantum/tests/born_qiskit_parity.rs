//! Born probabilities must match Qiskit Aer for all gameplay circuits.

use quantum_tetris_quantum::{born_probabilities, QuantumCircuit};

fn python_ready() -> bool {
    if std::env::var("QUANTUM_SKIP_PYTHON_TESTS").is_ok() {
        return false;
    }
    let python = std::env::var("QUANTUM_PYTHON").unwrap_or_else(|_| "python3".into());
    std::process::Command::new(&python)
        .args(["-c", "import qiskit, qiskit_aer"])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn qiskit_probabilities(circuit: &QuantumCircuit) -> Option<Vec<(String, f32)>> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let python = std::env::var("QUANTUM_PYTHON").unwrap_or_else(|_| "python3".into());
    let shim =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts/quantum_shim.py");
    let payload = serde_json::json!({
        "backend": "qiskit",
        "mode": "exact",
        "circuit": circuit,
    });

    let mut child = Command::new(&python)
        .arg(shim)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    child
        .stdin
        .as_mut()?
        .write_all(payload.to_string().as_bytes())
        .ok()?;

    let output = child.wait_with_output().ok()?;
    if !output.status.success() {
        return None;
    }

    let value: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let array = value["probabilities"].as_array()?;
    let mut probabilities = Vec::with_capacity(array.len());
    for entry in array {
        let bits = entry[0].as_str()?.to_string();
        let probability = entry[1].as_f64()? as f32;
        probabilities.push((bits, probability));
    }
    Some(probabilities)
}

fn assert_close(born: &[(String, f32)], qiskit: &[(String, f32)]) {
    assert_eq!(born.len(), qiskit.len());
    for ((born_bits, born_p), (qiskit_bits, qiskit_p)) in born.iter().zip(qiskit.iter()) {
        assert_eq!(born_bits, qiskit_bits);
        assert!(
            (born_p - qiskit_p).abs() < 1e-4,
            "{born_bits}: born={born_p} qiskit={qiskit_p}"
        );
    }
}

#[test]
fn born_matches_qiskit_on_gameplay_circuits() {
    if !python_ready() {
        eprintln!("skip: install scripts/requirements.txt");
        return;
    }

    std::env::set_var("QUANTUM_SHOTS", "65536");

    for circuit in [
        QuantumCircuit::imp_brain(),
        QuantumCircuit::hunter_profile(),
        QuantumCircuit::patrol_profile(),
        QuantumCircuit::observation_pulse(),
        QuantumCircuit::shard_stabilizer(),
        QuantumCircuit::teleporter(),
    ] {
        let born = born_probabilities(&circuit).expect("born");
        let qiskit = qiskit_probabilities(&circuit).expect("qiskit shim");
        assert_close(&born, &qiskit);
    }
}
