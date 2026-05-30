//! French-first UI strings and Qiskit circuit explanations.

use bevy::prelude::*;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

#[derive(Resource, Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Locale {
    #[default]
    Fr,
    En,
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
static WEB_LOCALE_DIRTY: AtomicBool = AtomicBool::new(false);
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
static WEB_LOCALE: AtomicU8 = AtomicU8::new(0);

impl Locale {
    pub fn toggle(self) -> Self {
        match self {
            Self::Fr => Self::En,
            Self::En => Self::Fr,
        }
    }

    pub fn toggle_label(self) -> &'static str {
        match self {
            Self::Fr => "(en)",
            Self::En => "(fr)",
        }
    }

    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub fn from_web_storage() -> Self {
        web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .and_then(|s| s.get_item("qt-lang").ok())
            .flatten()
            .filter(|v| v == "en")
            .map(|_| Self::En)
            .unwrap_or(Self::Fr)
    }
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub fn push_web_locale(lang: &str) {
    WEB_LOCALE.store(u8::from(lang == "en"), Ordering::Relaxed);
    WEB_LOCALE_DIRTY.store(true, Ordering::Relaxed);
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub fn sync_web_locale(mut locale: ResMut<Locale>) {
    if WEB_LOCALE_DIRTY.swap(false, Ordering::Relaxed) {
        *locale = match WEB_LOCALE.load(Ordering::Relaxed) {
            1 => Locale::En,
            _ => Locale::Fr,
        };
    }
}

pub fn initial_locale() -> Locale {
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    {
        Locale::from_web_storage()
    }
    #[cfg(not(all(feature = "wasm", target_arch = "wasm32")))]
    {
        Locale::default()
    }
}

/// Gameplay moment that triggers one or more Qiskit circuits.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum GameplayMoment {
    #[default]
    None,
    Spawn,
    Observe,
    LineClear,
    GameOver,
}

pub fn circuit_explain(locale: Locale, moment: GameplayMoment) -> &'static str {
    match (locale, moment) {
        (Locale::Fr, GameplayMoment::Spawn) => {
            "Spawn — 4 mesures Qiskit :\n\
             • quantum-teleportation-gate-v1 ×2 : pièce en cours + pièce suivante\n\
             • imp-brain-v1 : rotation et colonne\n\
             • enemy-profile-hunter-v1 : vitesse de chute"
        }
        (Locale::En, GameplayMoment::Spawn) => {
            "Spawn — 4 Qiskit measurements:\n\
             • quantum-teleportation-gate-v1 ×2: active piece + next preview\n\
             • imp-brain-v1: rotation and column\n\
             • enemy-profile-hunter-v1: drop speed"
        }
        (Locale::Fr, GameplayMoment::Observe) => {
            "Espace — observation-pulse-v1 (2 qubits) :\n\
             mesure volontaire à l'atterrissage forcé → bonus de score (parfois une ligne bonus)"
        }
        (Locale::En, GameplayMoment::Observe) => {
            "Space — observation-pulse-v1 (2 qubits):\n\
             deliberate measure on hard drop → score bonus (sometimes an extra line)"
        }
        (Locale::Fr, GameplayMoment::LineClear) => {
            "Ligne effacée — q-shard-stabilizer-v1 (2 qubits) :\n\
             stabilisation après effacement → multiplicateur de points (×1 à ×4)"
        }
        (Locale::En, GameplayMoment::LineClear) => {
            "Line clear — q-shard-stabilizer-v1 (2 qubits):\n\
             post-clear stabilizer → score multiplier (×1 to ×4)"
        }
        (Locale::Fr, GameplayMoment::GameOver) => {
            "Partie terminée — plus de place pour la nouvelle pièce."
        }
        (Locale::En, GameplayMoment::GameOver) => "Game over — no room for the next piece.",
        (_, GameplayMoment::None) => "—",
    }
}

pub fn mode_classic(locale: Locale) -> &'static str {
    match locale {
        Locale::Fr => "CLASSIQUE",
        Locale::En => "CLASSIC",
    }
}

pub fn mode_quantum(locale: Locale) -> &'static str {
    match locale {
        Locale::Fr => "QUANTIQUE",
        Locale::En => "QUANTUM",
    }
}

pub fn hint_move(locale: Locale) -> &'static str {
    match locale {
        Locale::Fr => "BOUGER",
        Locale::En => "MOVE",
    }
}

pub fn hint_rotate(locale: Locale) -> &'static str {
    match locale {
        Locale::Fr => "TOURNER",
        Locale::En => "ROTATE",
    }
}

pub fn hint_faster(locale: Locale) -> &'static str {
    match locale {
        Locale::Fr => "VITE",
        Locale::En => "FASTER",
    }
}

pub fn hint_drop(locale: Locale) -> &'static str {
    match locale {
        Locale::Fr => "POSER",
        Locale::En => "DROP",
    }
}

pub fn lines_level(locale: Locale, lines: u32, level: u32) -> String {
    match locale {
        Locale::Fr => format!("lignes {lines} · niv {level}"),
        Locale::En => format!("lines {lines} · lv {level}"),
    }
}

pub fn next_piece(locale: Locale, label: &str) -> String {
    match locale {
        Locale::Fr => format!("suiv. {label}"),
        Locale::En => format!("next {label}"),
    }
}

pub fn spawn_event(locale: Locale, bell: &str, piece: &str) -> String {
    match locale {
        Locale::Fr => format!("[{bell}] pièce {piece}"),
        Locale::En => format!("[{bell}] piece {piece}"),
    }
}

pub fn observe_event(locale: Locale, bits: &str, fx_label: &str) -> String {
    match locale {
        Locale::Fr => format!("observation [{bits}] {fx_label}"),
        Locale::En => format!("observed [{bits}] {fx_label}"),
    }
}

pub fn line_clear_event(locale: Locale, cleared: u32, bits: &str, bonus: u32) -> String {
    match locale {
        Locale::Fr => format!("{cleared} ligne(s) [{bits}] +{bonus}"),
        Locale::En => format!("{cleared} line(s) [{bits}] +{bonus}"),
    }
}

pub fn game_over_event(locale: Locale, score: u32) -> String {
    match locale {
        Locale::Fr => format!("Partie terminée — score {score}"),
        Locale::En => format!("Game over — score {score}"),
    }
}

pub fn retry_hint(locale: Locale) -> &'static str {
    match locale {
        Locale::Fr => "Espace — rejouer",
        Locale::En => "Space — retry",
    }
}

pub fn observe_fx_label(locale: Locale, bits: &str) -> &'static str {
    match (locale, bits) {
        (Locale::Fr, "00") => "effondrement doux",
        (Locale::Fr, "01") => "décalage de phase",
        (Locale::Fr, "10") => "écho de ligne",
        (Locale::Fr, "11") => "mesure forte",
        (Locale::En, "00") => "soft collapse",
        (Locale::En, "01") => "phase shift",
        (Locale::En, "10") => "row echo",
        (Locale::En, "11") => "strong measure",
        (_, _) => "measure",
    }
}

pub fn circuit_heading(locale: Locale) -> &'static str {
    match locale {
        Locale::Fr => "Circuit Qiskit",
        Locale::En => "Qiskit circuit",
    }
}
