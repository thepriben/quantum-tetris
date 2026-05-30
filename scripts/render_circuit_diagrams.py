#!/usr/bin/env python3
"""Render gameplay quantum circuits to docs/circuits/ (Qiskit diagram export).

Mirrors gate definitions in crates/quantum/src/circuit.rs.

Usage:
    pip install -r scripts/requirements.txt
    python scripts/render_circuit_diagrams.py
"""

from __future__ import annotations

import math
from pathlib import Path
from typing import Any

from qiskit import QuantumCircuit

ROOT = Path(__file__).resolve().parent.parent
OUT_DIR = ROOT / "docs" / "circuits"

# (filename stem, qubits, gates) — same IR as the Rust game crate.
GAMEPLAY_CIRCUITS: list[tuple[str, int, list[dict[str, Any]]]] = [
    (
        "quantum-teleportation-gate-v1",
        3,
        [
            {"Ry": {"qubit": 0, "theta_deg": 64.0}},
            {"H": 1},
            {"Cx": {"control": 1, "target": 2}},
            {"Cx": {"control": 0, "target": 1}},
            {"H": 0},
        ],
    ),
    (
        "imp-brain-v1",
        2,
        [
            {"H": 0},
            {"H": 1},
            {"Cx": {"control": 0, "target": 1}},
        ],
    ),
    (
        "enemy-profile-hunter-v1",
        2,
        [
            {"Ry": {"qubit": 0, "theta_deg": 54.0}},
            {"H": 1},
            {"Cx": {"control": 0, "target": 1}},
        ],
    ),
    (
        "observation-pulse-v1",
        2,
        [
            {"H": 0},
            {"Ry": {"qubit": 1, "theta_deg": 68.0}},
            {"Cx": {"control": 0, "target": 1}},
        ],
    ),
    (
        "q-shard-stabilizer-v1",
        2,
        [
            {"H": 0},
            {"Cx": {"control": 0, "target": 1}},
            {"Ry": {"qubit": 1, "theta_deg": 38.0}},
        ],
    ),
]


def _apply_gate(qc: QuantumCircuit, gate: dict[str, Any]) -> None:
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


def build_circuit(qubits: int, gates: list[dict[str, Any]]) -> QuantumCircuit:
    qc = QuantumCircuit(qubits, qubits)
    for gate in gates:
        _apply_gate(qc, gate)
    qc.measure_all()
    return qc


def render_circuit(qc: QuantumCircuit, stem: str) -> None:
    png_path = OUT_DIR / f"{stem}.png"
    style = {"backgroundcolor": "#eef3fb", "textcolor": "#1a2840", "linecolor": "#2a5080"}

    import matplotlib.pyplot as plt

    qc.draw(
        output="mpl",
        filename=str(png_path),
        scale=0.85,
        fold=-1,
        style=style,
        plot_barriers=False,
    )
    plt.close("all")
    print(f"  OK {stem} → {png_path.name}")


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    print(f"→ Rendering {len(GAMEPLAY_CIRCUITS)} circuits to {OUT_DIR}/")
    for stem, qubits, gates in GAMEPLAY_CIRCUITS:
        qc = build_circuit(qubits, gates)
        render_circuit(qc, stem)
    print("Done.")


if __name__ == "__main__":
    main()
