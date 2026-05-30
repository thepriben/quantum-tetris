//! Quantum Tetris — gravity, input, circuit-driven spawns.

use crate::board::{ActivePiece, Board, RunPhase, ROWS};
use crate::config::QuantumSession;
use crate::game_state::GameRun;
use crate::input;
use crate::measurement_fx::{
    line_clear_bonus, line_circuit, observe_circuit, observe_from_bits, piece_circuit,
    piece_index, record_measurement, rotation_circuit, rotation_from_bits, spawn_x_from_bits,
};
use crate::pieces::PieceKind;
use bevy::prelude::*;

pub fn init_first_piece(
    session: Res<QuantumSession>,
    mut board: ResMut<Board>,
    mut run: ResMut<GameRun>,
) {
    if board.active.is_some() {
        return;
    }
    let next_m = session
        .backend
        .lock()
        .expect("backend")
        .run(&piece_circuit())
        .expect("next");
    board.next = PieceKind::from_index(piece_index(&next_m));
    spawn_next(&session, &mut board, &mut run);
}

pub fn tick_gravity(
    time: Res<Time>,
    mut acc: Local<f32>,
    session: Res<QuantumSession>,
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
        lock_and_continue(&session, &mut board, &mut run);
    }
}

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<QuantumSession>,
    mut board: ResMut<Board>,
    mut run: ResMut<GameRun>,
) {
    if board.phase != RunPhase::Playing {
        if keys.just_pressed(KeyCode::Space) {
            reset_game(&session, &mut board, &mut run);
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
            lock_and_continue(&session, &mut board, &mut run);
        }
    }
    if input::space_just_pressed(&keys) {
        observe_hard_drop(&session, &mut board, &mut run);
    }
}

fn observe_hard_drop(session: &QuantumSession, board: &mut Board, run: &mut GameRun) {
    let Some(mut active) = board.active.take() else {
        return;
    };

    while {
        active.y -= 1;
        board.fits(&active)
    } {}
    active.y += 1;

    let measurement = session
        .backend
        .lock()
        .expect("backend")
        .run(&observe_circuit())
        .expect("observe");
    record_measurement(run, &measurement);
    let fx = observe_from_bits(&measurement.bits);
    run.score += fx.score_bonus;
    board.lock_piece(&active);

    if fx.line_bonus > 0 {
        let extra = board.clear_lines();
        run.lines += extra.max(fx.line_bonus);
        run.score += fx.line_bonus * 150;
    }

    run.last_event = format!("observe [{}] {}", measurement.bits, fx.label);
    after_lock(session, board, run);
}

fn lock_and_continue(session: &QuantumSession, board: &mut Board, run: &mut GameRun) {
    let Some(active) = board.active.take() else {
        return;
    };
    board.lock_piece(&active);
    after_lock(session, board, run);
}

fn after_lock(session: &QuantumSession, board: &mut Board, run: &mut GameRun) {
    let cleared = board.clear_lines();
    if cleared > 0 {
        let m = session
            .backend
            .lock()
            .expect("backend")
            .run(&line_circuit())
            .expect("line");
        record_measurement(run, &m);
        let bonus = line_clear_bonus(&m.bits, cleared);
        run.lines += cleared;
        run.score += bonus;
        run.level_from_lines();
        run.last_event = format!("{cleared} line(s) [{}] +{bonus}", m.bits);
    }
    spawn_next(session, board, run);
}

fn spawn_next(session: &QuantumSession, board: &mut Board, run: &mut GameRun) {
    let mut backend = session.backend.lock().expect("backend");

    let kind = board.next;
    let piece_m = backend.run(&piece_circuit()).expect("piece");
    let rot_m = backend.run(&rotation_circuit()).expect("rotation");
    let next_m = backend.run(&piece_circuit()).expect("next");

    record_measurement(run, &piece_m);
    board.next = PieceKind::from_index(piece_index(&next_m));

    let rotation = rotation_from_bits(&rot_m.bits);
    let x = spawn_x_from_bits(&rot_m.bits);

    let candidate = ActivePiece {
        kind,
        rotation,
        x,
        y: ROWS as i32 - 1,
    };

    if !board.fits(&candidate) {
        board.phase = RunPhase::GameOver;
        board.active = None;
        run.last_event = format!("Game over — score {}", run.score);
        run.hint = "Space — new game".into();
        return;
    }

    board.active = Some(candidate);
    run.last_event = format!(
        "piece {} r{rotation}  tele[{tp}] brain[{tr}]",
        kind_label(kind),
        tp = piece_m.bits,
        tr = rot_m.bits
    );
}

fn reset_game(session: &QuantumSession, board: &mut Board, run: &mut GameRun) {
    *board = Board::default();
    *run = GameRun::new(session.kind);
    run.last_event = "New game".into();

    let next_m = session
        .backend
        .lock()
        .expect("backend")
        .run(&piece_circuit())
        .expect("next");
    board.next = PieceKind::from_index(piece_index(&next_m));

    spawn_next(session, board, run);
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
