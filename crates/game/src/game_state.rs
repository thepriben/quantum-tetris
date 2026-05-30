//! Score, quantum readout, HUD strings.

use bevy::prelude::*;
use quantum_town_quantum::BackendKind;

#[derive(Resource, Debug)]
pub struct GameRun {
    pub backend_label: String,
    pub is_quantum: bool,
    pub last_bits: String,
    pub last_confidence: f32,
    pub last_event: String,
    pub hint: String,
    pub drop_interval: f32,
    pub score: u32,
    pub lines: u32,
    pub level: u32,
}

impl GameRun {
    pub fn new(backend: BackendKind) -> Self {
        Self {
            backend_label: backend.label().into(),
            is_quantum: backend == BackendKind::Qip,
            last_bits: String::new(),
            last_confidence: 0.0,
            last_event: "New game".into(),
            hint: "← → move · ↑ rotate · ↓ drop · Space observe".into(),
            drop_interval: 0.85,
            score: 0,
            lines: 0,
            level: 1,
        }
    }

    pub fn level_from_lines(&mut self) {
        self.level = 1 + self.lines / 5;
        self.drop_interval = (0.85 - self.level as f32 * 0.04).max(0.15);
    }
}
