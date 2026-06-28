//! Quantum Tetris — gravity, input, circuit-driven spawns.

use crate::audit_io;
use crate::board::{ActivePiece, Board, RunPhase, SPAWN_Y};
use crate::config::QuantumSession;
use crate::game_state::GameRun;
use crate::i18n::{self, GameplayMoment, Locale};
use crate::input;
use crate::measurement_fx::{
    drop_interval_from_bits, line_circuit, line_clear_bonus, observe_circuit, observe_from_bits,
    piece_circuit, piece_from_teleport, record_measurement, rotation_circuit, rotation_from_bits,
    spawn_x_from_bits, speed_circuit,
};
use crate::pieces::PieceKind;
use bevy::prelude::*;

pub fn init_first_piece(
    session: Res<QuantumSession>,
    locale: Res<Locale>,
    mut board: ResMut<Board>,
    mut run: ResMut<GameRun>,
) {
    if board.active.is_some() {
        return;
    }
    spawn_next(&session, &mut board, &mut run, *locale);
}

pub fn tick_gravity(
    time: Res<Time>,
    mut acc: Local<f32>,
    session: Res<QuantumSession>,
    locale: Res<Locale>,
    mut board: ResMut<Board>,
    mut run: ResMut<GameRun>,
) {
    if board.phase != RunPhase::Playing {
        return;
    }
    let Some(active) = board.active else {
        return;
    };

    *acc += time.delta_secs();
    if *acc < run.drop_interval {
        return;
    }
    *acc = 0.0;

    let mut moved = active;
    moved.y -= 1;
    if board.fits(&moved) {
        board.active = Some(moved);
    } else {
        lock_and_continue(&session, &mut board, &mut run, *locale);
    }
}

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<QuantumSession>,
    locale: Res<Locale>,
    mut board: ResMut<Board>,
    mut run: ResMut<GameRun>,
) {
    if board.phase != RunPhase::Playing {
        if keys.just_pressed(KeyCode::Space) {
            reset_game(&session, &mut board, &mut run, *locale);
        }
        return;
    }

    let Some(mut active) = board.active else {
        return;
    };

    if keys.just_pressed(KeyCode::ArrowLeft) {
        active.x -= 1;
        if board.fits(&active) {
            board.active = Some(active);
        }
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        active.x += 1;
        if board.fits(&active) {
            board.active = Some(active);
        }
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        let rotated = ActivePiece {
            rotation: active.rotation.wrapping_add(1),
            ..active
        };
        if board.fits(&rotated) {
            board.active = Some(rotated);
        }
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        active.y -= 1;
        if board.fits(&active) {
            board.active = Some(active);
            run.score += 1;
        } else {
            active.y += 1;
            board.active = Some(active);
            lock_and_continue(&session, &mut board, &mut run, *locale);
        }
    }
    if input::space_just_pressed(&keys) {
        observe_hard_drop(&session, &mut board, &mut run, *locale);
    }
}

fn observe_hard_drop(
    session: &QuantumSession,
    board: &mut Board,
    run: &mut GameRun,
    locale: Locale,
) {
    let Some(mut active) = board.active.take() else {
        return;
    };

    while {
        active.y -= 1;
        board.fits(&active)
    } {}
    active.y += 1;

    let circuit = observe_circuit();
    let (measurement, backend_used) = session.run_draw(&circuit);
    let fx = observe_from_bits(&measurement.bits);
    session.audit_draw(
        &circuit,
        &measurement,
        "observe",
        Some(fx.label),
        backend_used,
    );
    record_measurement(run, &measurement);
    run.score += fx.score_bonus;
    board.lock_piece(&active);

    if fx.line_bonus > 0 {
        let extra = board.clear_lines();
        run.lines += extra.max(fx.line_bonus);
        run.score += fx.line_bonus * 150;
    }

    run.last_moment = GameplayMoment::Observe;
    run.last_event = i18n::observe_event(
        locale,
        &measurement.bits,
        i18n::observe_fx_label(locale, &measurement.bits),
    );
    after_lock(session, board, run, locale);
}

fn lock_and_continue(
    session: &QuantumSession,
    board: &mut Board,
    run: &mut GameRun,
    locale: Locale,
) {
    let Some(active) = board.active.take() else {
        return;
    };
    board.lock_piece(&active);
    after_lock(session, board, run, locale);
}

fn after_lock(session: &QuantumSession, board: &mut Board, run: &mut GameRun, locale: Locale) {
    let cleared = board.clear_lines();
    if cleared > 0 {
        let circuit = line_circuit();
        let (m, backend_used) = session.run_draw(&circuit);
        let bonus = line_clear_bonus(&m.bits, cleared);
        session.audit_draw(
            &circuit,
            &m,
            "line_clear",
            Some(&format!("mult={} lines={cleared} bonus={bonus}", m.bits)),
            backend_used,
        );
        record_measurement(run, &m);
        run.lines += cleared;
        run.score += bonus;
        run.level_from_lines();
        run.last_moment = GameplayMoment::LineClear;
        run.last_event = i18n::line_clear_event(locale, cleared, &m.bits, bonus);
    }
    spawn_next(session, board, run, locale);
}

fn spawn_next(session: &QuantumSession, board: &mut Board, run: &mut GameRun, locale: Locale) {
    let piece = piece_circuit();
    let (tele_now, bu_piece) = session.run_draw(&piece);
    let (kind, now_readout) = piece_from_teleport(&tele_now);
    session.audit_draw(
        &piece,
        &tele_now,
        "spawn_piece",
        Some(&format!("piece={}", kind_label(kind))),
        bu_piece,
    );
    record_measurement(run, &tele_now);

    let rotation = rotation_circuit();
    let (rot_m, bu_rot) = session.run_draw(&rotation);
    let rot = rotation_from_bits(&rot_m.bits);
    session.audit_draw(
        &rotation,
        &rot_m,
        "spawn_rotation",
        Some(&format!("rot={rot} x={}", spawn_x_from_bits(&rot_m.bits))),
        bu_rot,
    );

    let speed = speed_circuit();
    let (speed_m, bu_speed) = session.run_draw(&speed);
    let interval = drop_interval_from_bits(&speed_m.bits, run.level);
    session.audit_draw(
        &speed,
        &speed_m,
        "spawn_speed",
        Some(&format!("drop={interval:.3}")),
        bu_speed,
    );

    let rotation = rot;
    let x = spawn_x_from_bits(&rot_m.bits);
    run.drop_interval = interval;

    let candidate = ActivePiece {
        kind,
        rotation,
        x,
        y: SPAWN_Y,
    };

    if !board.fits(&candidate) {
        board.phase = RunPhase::GameOver;
        board.active = None;
        run.last_moment = GameplayMoment::GameOver;
        run.last_event = i18n::game_over_event(locale, run.score);
        run.hint = i18n::retry_hint(locale).into();
        export_audit_on_game_over(session);
        return;
    }

    board.active = Some(candidate);
    run.last_moment = GameplayMoment::Spawn;
    run.last_event = i18n::spawn_event(locale, &now_readout.bell, kind_label(kind));
}

fn export_audit_on_game_over(session: &QuantumSession) {
    let journal = session.finalize_audit();
    if journal.verify().is_err() {
        eprintln!("[audit] journal verification failed on game over");
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    match audit_io::export_journal(&journal) {
        Ok(path) => eprintln!("[audit] journal exported to {}", path.display()),
        Err(error) => eprintln!("[audit] export failed: {error}"),
    }
    #[cfg(target_arch = "wasm32")]
    if let Ok(json) = journal.to_json_pretty() {
        eprintln!(
            "[audit] session {} ({} entries)",
            journal.session_id,
            journal.entry_count()
        );
        eprintln!("{json}");
    }
}

fn reset_game(session: &QuantumSession, board: &mut Board, run: &mut GameRun, locale: Locale) {
    if board.phase != RunPhase::GameOver {
        let _ = session.finalize_audit();
    }
    *board = Board::default();
    *run = GameRun::new(session.kind);
    run.last_event.clear();
    run.last_moment = GameplayMoment::None;
    spawn_next(session, board, run, locale);
}

/// New board after mode change or game-over restart.
pub fn restart_game(
    session: &QuantumSession,
    board: &mut Board,
    run: &mut GameRun,
    locale: Locale,
) {
    reset_game(session, board, run, locale);
}

fn kind_label(k: PieceKind) -> &'static str {
    match k {
        PieceKind::I => "I",
        PieceKind::O => "O",
        PieceKind::T => "T",
        PieceKind::S => "S",
        PieceKind::Z => "Z",
        PieceKind::J => "J",
        PieceKind::L => "L",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::config::QuantumSession;
    use crate::game_state::GameRun;
    use crate::i18n::Locale;
    use quantum_tetris_quantum::BackendKind;

    #[test]
    fn spawn_records_three_audit_entries() {
        let session = QuantumSession::with_fallback(BackendKind::Classic);
        let mut board = Board::default();
        let mut run = GameRun::new(BackendKind::Classic);
        spawn_next(&session, &mut board, &mut run, Locale::En);
        assert_eq!(session.audit_entry_count(), 3);
    }
}
