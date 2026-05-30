//! Runtime configuration — desktop (classic / Qiskit) and WASM (classic / Born quantum).

use bevy::prelude::*;
use quantum_tetris_quantum::{
    build_backend, BackendKind, ClassicBackend, Measurement, QuantumBackend, QuantumCircuit,
    QuantumError,
};
use std::sync::Mutex;

/// How the game is launched (native binary vs WASM bundle).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlatform {
    Desktop,
    Wasm,
}

/// Session-wide options for [`crate::app::TetrisPlugin`].
#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub platform: GamePlatform,
    pub window_title: &'static str,
    pub backend_kind: BackendKind,
}

impl GameConfig {
    /// Native: `QUANTUM_MODE=classic|qiskit` (default `classic`).
    pub fn desktop() -> Self {
        Self {
            platform: GamePlatform::Desktop,
            window_title: "Quantum Tetris",
            backend_kind: BackendKind::from_env(),
        }
    }

    /// Browser: Born-rule quantum by default (Qiskit-matched statevector).
    pub fn wasm() -> Self {
        Self {
            platform: GamePlatform::Wasm,
            window_title: "Quantum Tetris",
            backend_kind: BackendKind::Quantum,
        }
    }

    pub fn backend_label(&self) -> &'static str {
        self.backend_kind.label()
    }
}

/// Shared quantum backend (wrapped for Bevy `Resource` + `Sync`).
#[derive(Resource)]
pub struct QuantumSession {
    pub kind: BackendKind,
    pub backend: Mutex<Box<dyn QuantumBackend>>,
}

impl QuantumSession {
    pub fn new(kind: BackendKind) -> Result<Self, QuantumError> {
        let backend = build_backend(kind)?;
        Ok(Self {
            kind,
            backend: Mutex::new(backend),
        })
    }

    pub fn with_fallback(kind: BackendKind) -> Self {
        Self::new(kind).unwrap_or_else(|error| {
            eprintln!("[quantum] {kind:?} unavailable ({error}), falling back to classic");
            Self::new(BackendKind::Classic).expect("classic backend")
        })
    }

    /// Run a circuit; on failure, log and fall back to uniform classic randomness.
    pub fn run_circuit(&self, circuit: &QuantumCircuit) -> Measurement {
        let mut backend = self.backend.lock().expect("backend");
        match backend.run(circuit) {
            Ok(measurement) => measurement,
            Err(error) => {
                eprintln!("[quantum] backend run failed ({error}), using classic fallback");
                let mut classic = ClassicBackend;
                classic
                    .run(circuit)
                    .expect("classic backend always succeeds")
            }
        }
    }

    /// Hot-swap backend (classic ↔ quantum) and return whether it succeeded.
    pub fn switch_to(&mut self, kind: BackendKind) -> bool {
        if self.kind == kind {
            return true;
        }
        match Self::new(kind) {
            Ok(next) => {
                self.kind = next.kind;
                self.backend = next.backend;
                true
            }
            Err(error) => {
                eprintln!("[quantum] cannot switch to {kind:?}: {error}");
                false
            }
        }
    }
}
