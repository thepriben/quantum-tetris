//! Runtime configuration — classic or RustQIP quantum (local + WASM).

use bevy::prelude::*;
use quantum_tetris_quantum::{
    build_backend, AuditJournal, BackendKind, ClassicBackend, Measurement, QuantumBackend,
    QuantumCircuit, QuantumError,
};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

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
    /// Native: `QUANTUM_MODE=classic|quantum` (default `quantum` / RustQIP).
    pub fn desktop() -> Self {
        Self {
            platform: GamePlatform::Desktop,
            window_title: "Quantum Tetris",
            backend_kind: BackendKind::from_env(),
        }
    }

    /// Browser: RustQIP quantum by default.
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

fn new_session_id() -> String {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("qt-{ms}")
}

/// Shared quantum backend + append-only audit journal (C5/C6).
#[derive(Resource)]
pub struct QuantumSession {
    pub kind: BackendKind,
    backend: Mutex<Box<dyn QuantumBackend>>,
    audit: Mutex<AuditJournal>,
}

impl QuantumSession {
    pub fn new(kind: BackendKind) -> Result<Self, QuantumError> {
        let backend = build_backend(kind)?;
        Ok(Self {
            kind,
            backend: Mutex::new(backend),
            audit: Mutex::new(AuditJournal::new(new_session_id(), kind.label())),
        })
    }

    pub fn with_fallback(kind: BackendKind) -> Self {
        Self::new(kind).unwrap_or_else(|error| {
            eprintln!("[quantum] {kind:?} unavailable ({error}), falling back to classic");
            Self::new(BackendKind::Classic).expect("classic backend")
        })
    }

    pub fn seed_commitment(&self) -> String {
        self.audit.lock().expect("audit").seed_commitment.clone()
    }

    pub fn audit_entry_count(&self) -> usize {
        self.audit.lock().expect("audit").entry_count()
    }

    /// Execute a circuit on the backend (no journal write yet).
    pub fn run_draw(&self, circuit: &QuantumCircuit) -> (Measurement, &'static str) {
        self.run_circuit_inner(circuit)
    }

    /// Append one draw to the session journal (C5 receipt + C6 entry).
    pub fn audit_draw(
        &self,
        circuit: &QuantumCircuit,
        measurement: &Measurement,
        moment: &str,
        effect: Option<&str>,
        backend_used: &str,
    ) {
        self.audit.lock().expect("audit").record_draw(
            circuit,
            measurement,
            moment,
            effect,
            backend_used,
        );
    }

    /// Run a circuit and append a hash-chained receipt to the session journal.
    pub fn run_circuit_audited(
        &self,
        circuit: &QuantumCircuit,
        moment: &str,
        effect: Option<&str>,
    ) -> Measurement {
        let (measurement, backend_used) = self.run_circuit_inner(circuit);
        self.audit_draw(circuit, &measurement, moment, effect, backend_used);
        measurement
    }

    /// Backward-compatible alias without audit metadata (still records a generic entry).
    pub fn run_circuit(&self, circuit: &QuantumCircuit) -> Measurement {
        self.run_circuit_audited(circuit, "draw", None)
    }

    fn run_circuit_inner(&self, circuit: &QuantumCircuit) -> (Measurement, &'static str) {
        let mut backend = self.backend.lock().expect("backend");
        match backend.run(circuit) {
            Ok(measurement) => (measurement, self.kind.label()),
            Err(error) => {
                eprintln!("[quantum] backend run failed ({error}), using classic fallback");
                let mut classic = ClassicBackend;
                let measurement = classic
                    .run(circuit)
                    .expect("classic backend always succeeds");
                (measurement, "classic (fallback)")
            }
        }
    }

    /// Finalize the current journal (seed reveal) and begin a new session.
    pub fn finalize_audit(&self) -> AuditJournal {
        let mut audit = self.audit.lock().expect("audit");
        let mut finished = AuditJournal::new(new_session_id(), self.kind.label());
        std::mem::swap(&mut *audit, &mut finished);
        finished.reveal_seed();
        finished
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
                *self.audit.lock().expect("audit") =
                    AuditJournal::new(new_session_id(), self.kind.label());
                true
            }
            Err(error) => {
                eprintln!("[quantum] cannot switch to {kind:?}: {error}");
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quantum_tetris_quantum::QuantumCircuit;

    #[test]
    fn wasm_defaults_to_rustqip_quantum() {
        let config = GameConfig::wasm();
        assert_eq!(config.platform, GamePlatform::Wasm);
        assert_eq!(config.backend_kind, BackendKind::Quantum);
    }

    #[test]
    fn fallback_session_keeps_quantum_when_available() {
        let session = QuantumSession::with_fallback(BackendKind::Quantum);
        assert_eq!(session.kind, BackendKind::Quantum);
    }

    #[test]
    fn audited_draws_append_to_journal() {
        let session = QuantumSession::with_fallback(BackendKind::Classic);
        session.run_circuit_audited(&QuantumCircuit::teleporter(), "spawn", Some("piece=T"));
        assert_eq!(session.audit_entry_count(), 1);
        let journal = session.finalize_audit();
        assert!(journal.verify().is_ok());
        assert!(journal.seed_revealed.is_some());
    }
}
