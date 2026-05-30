//! Direct invocation of `scripts/quantum_shim.py` (no Rust wrapper).

use std::io::Write;
use std::process::{Command, Stdio};

use quantum_town_quantum::QuantumCircuit;

fn python_ready() -> bool {
    if std::env::var("QUANTUM_SKIP_PYTHON_TESTS").is_ok() {
        return false;
    }
    let python = std::env::var("QUANTUM_PYTHON").unwrap_or_else(|_| "python3".into());
    Command::new(&python)
        .args(["-c", "import qiskit, qiskit_aer"])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn shim_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts/quantum_shim.py")
}

#[test]
fn qiskit_shim_returns_valid_json() {
    if !python_ready() {
        eprintln!("skip: install scripts/requirements.txt");
        return;
    }

    let python = std::env::var("QUANTUM_PYTHON").unwrap_or_else(|_| "python3".into());
    let payload = serde_json::json!({
        "backend": "qiskit",
        "circuit": QuantumCircuit::imp_brain(),
    });

    let mut child = Command::new(&python)
        .arg(shim_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn shim");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(payload.to_string().as_bytes())
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait");
    assert!(
        output.status.success(),
        "shim failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json stdout");
    let bits = value["bits"].as_str().expect("bits field");
    assert_eq!(bits.len(), 2);
    let probabilities = value["probabilities"].as_array().expect("probabilities");
    assert_eq!(probabilities.len(), 4);
}

#[test]
fn teleporter_via_qiskit_shim() {
    if !python_ready() {
        return;
    }
    let python = std::env::var("QUANTUM_PYTHON").unwrap_or_else(|_| "python3".into());
    let payload = serde_json::json!({
        "backend": "qiskit",
        "circuit": QuantumCircuit::teleporter(),
    });

    let mut child = Command::new(&python)
        .arg(shim_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.to_string().as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["bits"].as_str().unwrap().len(), 3);
}
