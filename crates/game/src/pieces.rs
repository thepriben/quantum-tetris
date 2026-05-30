//! Tetromino definitions and rotation tables.

use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKind {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
}

impl PieceKind {
    pub fn from_index(i: usize) -> Self {
        match i % 7 {
            0 => Self::I,
            1 => Self::O,
            2 => Self::T,
            3 => Self::S,
            4 => Self::Z,
            5 => Self::J,
            _ => Self::L,
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::I => Color::srgb(0.35, 0.95, 0.98),
            Self::O => Color::srgb(0.98, 0.85, 0.35),
            Self::T => Color::srgb(0.72, 0.55, 0.98),
            Self::S => Color::srgb(0.45, 0.95, 0.62),
            Self::Z => Color::srgb(0.98, 0.45, 0.52),
            Self::J => Color::srgb(0.45, 0.62, 0.98),
            Self::L => Color::srgb(0.98, 0.62, 0.35),
        }
    }
}

/// Four (x, y) offsets for each rotation state (0..4).
pub fn cells(kind: PieceKind, rotation: u8) -> [(i32, i32); 4] {
    let r = (rotation % 4) as usize;
    match kind {
        PieceKind::I => I_ROT[r],
        PieceKind::O => O_ROT[r],
        PieceKind::T => T_ROT[r],
        PieceKind::S => S_ROT[r],
        PieceKind::Z => Z_ROT[r],
        PieceKind::J => J_ROT[r],
        PieceKind::L => L_ROT[r],
    }
}

const I_ROT: [[(i32, i32); 4]; 4] = [
    [(-1, 0), (0, 0), (1, 0), (2, 0)],
    [(0, -1), (0, 0), (0, 1), (0, 2)],
    [(-1, 1), (0, 1), (1, 1), (2, 1)],
    [(1, -1), (1, 0), (1, 1), (1, 2)],
];

const O_ROT: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
];

const T_ROT: [[(i32, i32); 4]; 4] = [
    [(-1, 0), (0, 0), (1, 0), (0, 1)],
    [(0, -1), (0, 0), (0, 1), (1, 0)],
    [(-1, 0), (0, 0), (1, 0), (0, -1)],
    [(0, -1), (0, 0), (0, 1), (-1, 0)],
];

const S_ROT: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (-1, 1), (0, 1)],
    [(0, -1), (0, 0), (1, 0), (1, 1)],
    [(0, 0), (1, 0), (-1, 1), (0, 1)],
    [(0, -1), (0, 0), (1, 0), (1, 1)],
];

const Z_ROT: [[(i32, i32); 4]; 4] = [
    [(-1, 0), (0, 0), (0, 1), (1, 1)],
    [(1, -1), (0, 0), (1, 0), (0, 1)],
    [(-1, 0), (0, 0), (0, 1), (1, 1)],
    [(1, -1), (0, 0), (1, 0), (0, 1)],
];

const J_ROT: [[(i32, i32); 4]; 4] = [
    [(-1, 0), (0, 0), (1, 0), (-1, 1)],
    [(0, -1), (0, 0), (0, 1), (1, -1)],
    [(-1, 0), (0, 0), (1, 0), (1, -1)],
    [(0, -1), (0, 0), (0, 1), (-1, 1)],
];

const L_ROT: [[(i32, i32); 4]; 4] = [
    [(-1, 0), (0, 0), (1, 0), (1, 1)],
    [(0, -1), (0, 0), (0, 1), (1, 1)],
    [(-1, -1), (-1, 0), (0, 0), (1, 0)],
    [(-1, -1), (0, -1), (0, 0), (0, 1)],
];
