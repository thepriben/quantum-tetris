//! Runtime configuration — Quantum Sub (desktop ; WASM entry kept for later).

use bevy::prelude::*;
use quantum_town_quantum::{build_backend, BackendKind, QuantumBackend, QuantumError};
use std::sync::Mutex;

/// How the game is launched (native binary vs future WASM bundle).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePlatform {
    Desktop,
    Wasm,
}

/// Session-wide options for [`crate::app::DrivePlugin`].
#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub platform: GamePlatform,
    pub window_title: &'static str,
    pub backend_kind: BackendKind,
}

impl GameConfig {
    /// Native build: `QUANTUM_MODE=classic|quantum`; default is QIP Rust.
    pub fn desktop() -> Self {
        Self {
            platform: GamePlatform::Desktop,
            window_title: "Quantum Tetris: LA",
            backend_kind: BackendKind::from_env(),
        }
    }

    /// Browser: classic by default; `?mode=quantum` for QIP (see `play.html`).
    pub fn wasm(kind: BackendKind) -> Self {
        Self {
            platform: GamePlatform::Wasm,
            window_title: "Quantum Tetris: LA",
            backend_kind: kind,
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
            eprintln!("[quantum] {kind:?} unavailable ({error}), using classic");
            Self::new(BackendKind::Classic).expect("classic backend")
        })
    }
}
