//! Timer, score, run state — shared HUD + systems.

use bevy::prelude::*;
use quantum_town_quantum::BackendKind;

pub const RUN_DURATION_SECS: f32 = 120.0;
pub const ENERGY_GOAL: u8 = 3;
pub const MAX_COHERENCE: f32 = 100.0;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum RunState {
    #[default]
    Playing,
    Won,
    LostTime,
}

#[derive(Resource)]
pub struct GameRun {
    pub state: RunState,
    pub time_remaining: f32,
    pub energy: u8,
    pub coherence: f32,
    pub hull_stress: f32,
    pub backend_label: String,
    pub is_quantum: bool,
    pub quantum_current: Vec3,
    pub current_strength: f32,
    pub current_label: String,
    pub last_bits: String,
    pub last_confidence: f32,
    pub last_event: String,
    pub hint: String,
    pub warp_cooldown: f32,
    pub observe_cooldown: f32,
}

impl GameRun {
    pub fn new(backend: BackendKind) -> Self {
        Self {
            state: RunState::Playing,
            time_remaining: RUN_DURATION_SECS,
            energy: 0,
            coherence: MAX_COHERENCE,
            hull_stress: 0.0,
            backend_label: backend.label().into(),
            is_quantum: backend == BackendKind::Qip,
            quantum_current: Vec3::ZERO,
            current_strength: 1.0,
            current_label: "stable".into(),
            last_bits: String::new(),
            last_confidence: 0.0,
            last_event: String::new(),
            hint: "Arrows — drive · Space — measure / act".into(),
            warp_cooldown: 0.0,
            observe_cooldown: 0.0,
        }
    }
}
