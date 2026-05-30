#!/usr/bin/env python3
"""JSON stdin → Qiskit Aer shot → JSON stdout for Quantum Town: LA.

Used locally and on GitHub Actions (see .github/workflows/ci.yml).

Install: pip install -r scripts/requirements.txt
"""

from __future__ import annotations

import json
import math
import os
import random
import sys
from typing import Any


def _fail(message: str, code: int = 1) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(code)


def _apply_gate(qc: Any, gate: dict[str, Any]) -> None:
    if "H" in gate:
        qc.h(int(gate["H"]))
    elif "X" in gate:
        qc.x(int(gate["X"]))
    elif "Z" in gate:
        qc.z(int(gate["Z"]))
    elif "Cx" in gate:
        body = gate["Cx"]
        qc.cx(int(body["control"]), int(body["target"]))
    elif "Ry" in gate:
        body = gate["Ry"]
        qc.ry(math.radians(float(body["theta_deg"])), int(body["qubit"]))
    else:
        raise ValueError(f"unsupported gate: {gate}")


def _build_qiskit_circuit(circuit: dict[str, Any]) -> Any:
    from qiskit import QuantumCircuit

    n = int(circuit["qubits"])
    qc = QuantumCircuit(n, n)
    for gate in circuit["gates"]:
        _apply_gate(qc, gate)
    qc.measure_all()
    return qc


def _counts_to_probabilities(counts: dict[str, int], qubits: int) -> list[tuple[str, float]]:
    total = sum(counts.values()) or 1
    width = qubits
    probs: list[tuple[str, float]] = []
    for value in range(1 << qubits):
        bits = format(value, f"0{width}b")
        probs.append((bits, counts.get(bits, 0) / total))
    return probs


def _sample_bits(probabilities: list[tuple[str, float]]) -> str:
    roll = random.random()
    cumulative = 0.0
    for bits, probability in probabilities:
        cumulative += probability
        if roll <= cumulative:
            return bits
    return probabilities[-1][0]


def _run_qiskit(circuit: dict[str, Any]) -> dict[str, Any]:
    from qiskit_aer import AerSimulator

    qc = _build_qiskit_circuit(circuit)
    shots = int(os.environ.get("QUANTUM_SHOTS", "1024"))
    result = AerSimulator().run(qc, shots=shots).result()
    counts = result.get_counts()
    width = int(circuit["qubits"])
    normalized: dict[str, int] = {}
    for key, count in counts.items():
        bits = key.replace(" ", "")
        if len(bits) < width:
            bits = bits.zfill(width)
        normalized[bits[-width:]] = normalized.get(bits[-width:], 0) + int(count)

    probabilities = _counts_to_probabilities(normalized, width)
    return {"bits": _sample_bits(probabilities), "probabilities": probabilities}


def main() -> None:
    try:
        payload = json.load(sys.stdin)
    except json.JSONDecodeError as error:
        _fail(f"invalid JSON on stdin: {error}")

    backend = payload.get("backend", "qiskit")
    circuit = payload.get("circuit")
    if not isinstance(circuit, dict):
        _fail("payload.circuit must be an object")

    if backend != "qiskit":
        _fail(f"unsupported backend: {backend} (only qiskit is enabled)")

    try:
        result = _run_qiskit(circuit)
    except ImportError as error:
        _fail(
            f"Missing Python package ({error}). "
            "Run: pip install -r scripts/requirements.txt"
        )
    except Exception as error:  # noqa: BLE001
        _fail(str(error))

    json.dump(result, sys.stdout)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
