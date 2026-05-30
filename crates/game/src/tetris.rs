//! Quantum Tetris — gravity, input, circuit-driven spawns.

use crate::board::{ActivePiece, Board, RunPhase, SPAWN_Y};
use crate::config::QuantumSession;
use crate::game_state::GameRun;
use crate::i18n::{self, GameplayMoment, Locale};
use crate::input;
use crate::measurement_fx::{
    drop_interval_from_bits, line_circuit, line_clear_bonus, observe_circuit, observe_from_bits,
    piece_circuit, pieces_from_teleport_pair, record_measurement, rotation_circuit,
    rotation_from_bits, spawn_x_from_bits, speed_circuit,
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

    let measurement = session.run_circuit(&observe_circuit());
    record_measurement(run, &measurement);
    let fx = observe_from_bits(&measurement.bits);
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
        let m = session.run_circuit(&line_circuit());
        record_measurement(run, &m);
        let bonus = line_clear_bonus(&m.bits, cleared);
        run.lines += cleared;
        run.score += bonus;
        run.level_from_lines();
        run.last_moment = GameplayMoment::LineClear;
        run.last_event = i18n::line_clear_event(locale, cleared, &m.bits, bonus);
    }
    spawn_next(session, board, run, locale);
}

fn spawn_next(session: &QuantumSession, board: &mut Board, run: &mut GameRun, locale: Locale) {
    let tele_now = session.run_circuit(&piece_circuit());
    let tele_next = session.run_circuit(&piece_circuit());
    let rot_m = session.run_circuit(&rotation_circuit());
    let speed_m = session.run_circuit(&speed_circuit());

    record_measurement(run, &tele_now);

    let (kind, next_kind, now_readout, next_readout) =
        pieces_from_teleport_pair(&tele_now, &tele_next);

    board.next = next_kind;
    board.next_family = next_readout.family;
    run.active_family = now_readout.family;
    run.next_family = next_readout.family;

    let rotation = rotation_from_bits(&rot_m.bits);
    let x = spawn_x_from_bits(&rot_m.bits);
    run.drop_interval = drop_interval_from_bits(&speed_m.bits, run.level);

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
        return;
    }

    board.active = Some(candidate);
    run.last_moment = GameplayMoment::Spawn;
    run.last_event = i18n::spawn_event(locale, &now_readout.bell, kind_label(kind));
}

fn reset_game(
    session: &QuantumSession,
    board: &mut Board,
    run: &mut GameRun,
    locale: Locale,
) {
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
