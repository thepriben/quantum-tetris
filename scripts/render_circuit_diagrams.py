#!/usr/bin/env python3
"""Render gameplay quantum circuits to docs/circuits/.

Mirrors gate definitions in crates/quantum/src/circuit.rs.

Usage:
    pip install -r scripts/requirements.txt
    python scripts/render_circuit_diagrams.py
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
OUT_DIR = ROOT / "docs" / "circuits"

WIRE = "#82aee0"
CLASSICAL = "#8492a6"
GATE = "#f84b5c"
ROT = "#a9165a"
MEASURE = "#a6a6a6"
LABEL = "#dbe5f2"
CX = "#5b8dff"

# (filename stem, qubits, gates) - same IR as the Rust game crate.
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


def _gate_label(gate: dict[str, Any]) -> tuple[str, int]:
    if "H" in gate:
        return "H", int(gate["H"])
    if "X" in gate:
        return "X", int(gate["X"])
    if "Z" in gate:
        return "Z", int(gate["Z"])
    if "Ry" in gate:
        body = gate["Ry"]
        return f"Ry\n{float(body['theta_deg']):.0f} deg", int(body["qubit"])
    raise ValueError(f"not a single-qubit gate: {gate}")


def _cx(gate: dict[str, Any]) -> tuple[int, int]:
    body = gate["Cx"]
    return int(body["control"]), int(body["target"])


def render_circuit(stem: str, qubits: int, gates: list[dict[str, Any]]) -> None:
    import matplotlib.pyplot as plt
    from matplotlib.patches import Arc, FancyArrowPatch, Rectangle

    columns = max(len(gates), 1) + 2
    width = columns * 1.25 + 1.7
    height = qubits * 0.9 + 1.6
    fig, ax = plt.subplots(figsize=(width, height), dpi=180)
    fig.patch.set_alpha(0)
    ax.set_facecolor("none")
    ax.axis("off")

    y_positions = {q: qubits - q - 1 for q in range(qubits)}
    x_start = 0.8
    x_end = columns * 1.18

    for q, y in y_positions.items():
        ax.plot([x_start, x_end], [y, y], color=WIRE, lw=2.4)
        ax.text(0.35, y, f"q{q}", ha="right", va="center", fontsize=17, color=LABEL)

    c_y = -0.75
    ax.plot([x_start, x_end], [c_y, c_y], color=CLASSICAL, lw=2.2)
    ax.plot([x_start, x_end], [c_y - 0.06, c_y - 0.06], color=CLASSICAL, lw=2.2)
    ax.text(0.35, c_y, "meas", ha="right", va="center", fontsize=16, color=LABEL)

    for index, gate in enumerate(gates, start=1):
        x = x_start + index * 1.05
        if "Cx" in gate:
            control, target = _cx(gate)
            y1 = y_positions[control]
            y2 = y_positions[target]
            ax.plot([x, x], [y1, y2], color=CX, lw=2.3)
            ax.scatter([x], [y1], s=105, color=CX, zorder=5)
            ax.scatter([x], [y2], s=520, color=CX, zorder=5)
            ax.text(x, y2, "+", ha="center", va="center", fontsize=23, color="white", zorder=6)
            continue

        label, qubit = _gate_label(gate)
        y = y_positions[qubit]
        color = ROT if label.startswith("Ry") else GATE
        rect = Rectangle((x - 0.32, y - 0.32), 0.64, 0.64, facecolor=color, edgecolor=color)
        ax.add_patch(rect)
        ax.text(x, y, label, ha="center", va="center", fontsize=13, color="white")

    measure_x = x_start + (len(gates) + 1) * 1.05
    for q, y in y_positions.items():
        rect = Rectangle(
            (measure_x - 0.34, y - 0.34),
            0.68,
            0.68,
            facecolor=MEASURE,
            edgecolor=MEASURE,
        )
        ax.add_patch(rect)
        ax.add_patch(Arc((measure_x, y - 0.05), 0.42, 0.38, theta1=15, theta2=175, lw=2.2))
        ax.plot([measure_x + 0.03, measure_x + 0.22], [y - 0.03, y + 0.16], color="#1b1f26", lw=2.0)
        ax.add_patch(
            FancyArrowPatch(
                (measure_x, y - 0.34),
                (measure_x, c_y + 0.02),
                arrowstyle="-|>",
                mutation_scale=13,
                lw=1.5,
                color=CLASSICAL,
            )
        )

    ax.set_xlim(0, x_end + 0.2)
    ax.set_ylim(c_y - 0.45, qubits - 0.05)
    png_path = OUT_DIR / f"{stem}.png"
    fig.savefig(png_path, transparent=True, edgecolor="none", bbox_inches="tight", pad_inches=0.15)
    plt.close(fig)
    print(f"  OK {stem} -> {png_path.name}")


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    print(f"Rendering {len(GAMEPLAY_CIRCUITS)} circuits to {OUT_DIR}/")
    for stem, qubits, gates in GAMEPLAY_CIRCUITS:
        render_circuit(stem, qubits, gates)
    print("Done.")


if __name__ == "__main__":
    main()
