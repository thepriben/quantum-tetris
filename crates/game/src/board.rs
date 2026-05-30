//! 10×20 playfield.

use crate::pieces::{cells, PieceFamily, PieceKind};
use bevy::prelude::*;

pub const COLS: usize = 10;
/// Visible playfield height (UI rows 0 = bottom, ROWS - 1 = top).
pub const ROWS: usize = 20;
/// Extra rows above the visible grid where pieces spawn (standard Tetris buffer).
pub const HIDDEN_ROWS: usize = 4;

/// Anchor row for new pieces — high enough that every rotation fits in the buffer.
pub const SPAWN_Y: i32 = ROWS as i32 + 1;

#[derive(Clone, Copy, Debug)]
pub struct ActivePiece {
    pub kind: PieceKind,
    pub rotation: u8,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum RunPhase {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource, Clone)]
pub struct Board {
    pub cells: [[Option<PieceKind>; COLS]; ROWS],
    pub active: Option<ActivePiece>,
    pub next: PieceKind,
    pub next_family: PieceFamily,
    pub phase: RunPhase,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[None; COLS]; ROWS],
            active: None,
            next: PieceKind::T,
            next_family: PieceFamily::Fork,
            phase: RunPhase::Playing,
        }
    }
}

impl Board {
    fn in_hidden_buffer(y: i32) -> bool {
        y >= ROWS as i32 && y < ROWS as i32 + HIDDEN_ROWS as i32
    }

    pub fn occupied(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= COLS as i32 {
            return true;
        }
        if y < 0 || Self::in_hidden_buffer(y) {
            return false;
        }
        if y >= ROWS as i32 {
            return true;
        }
        self.cells[y as usize][x as usize].is_some()
    }

    pub fn fits(&self, piece: &ActivePiece) -> bool {
        for (dx, dy) in cells(piece.kind, piece.rotation) {
            let x = piece.x + dx;
            let y = piece.y + dy;
            if x < 0 || x >= COLS as i32 || y < 0 {
                return false;
            }
            if y >= ROWS as i32 + HIDDEN_ROWS as i32 {
                return false;
            }
            if Self::in_hidden_buffer(y) {
                continue;
            }
            if self.cells[y as usize][x as usize].is_some() {
                return false;
            }
        }
        true
    }

    pub fn lock_piece(&mut self, piece: &ActivePiece) {
        for (dx, dy) in cells(piece.kind, piece.rotation) {
            let x = piece.x + dx;
            let y = piece.y + dy;
            if y >= 0 && y < ROWS as i32 && x >= 0 && x < COLS as i32 {
                self.cells[y as usize][x as usize] = Some(piece.kind);
            }
        }
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut cleared = 0u32;
        let mut row = (ROWS as i32) - 1;
        while row >= 0 {
            if self.cells[row as usize].iter().all(|c| c.is_some()) {
                cleared += 1;
                for r in (1..=row as usize).rev() {
                    self.cells[r] = self.cells[r - 1];
                }
                self.cells[0] = [None; COLS];
            } else {
                row -= 1;
            }
        }
        cleared
    }

    /// Color at board coordinate including active piece overlay.
    pub fn display_color(&self, col: usize, row: usize) -> Option<Color> {
        if let Some(active) = &self.active {
            for (dx, dy) in cells(active.kind, active.rotation) {
                let x = active.x + dx;
                let y = active.y + dy;
                if x as usize == col && y as usize == row && (0..ROWS as i32).contains(&y) {
                    return Some(active.kind.color());
                }
            }
        }
        self.cells[row][col].map(|k| k.color())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::PieceKind;

    #[test]
    fn every_piece_spawns_in_buffer() {
        let board = Board::default();
        for kind in [
            PieceKind::I,
            PieceKind::O,
            PieceKind::T,
            PieceKind::S,
            PieceKind::Z,
            PieceKind::J,
            PieceKind::L,
        ] {
            for rotation in 0..4u8 {
                let piece = ActivePiece {
                    kind,
                    rotation,
                    x: 3,
                    y: SPAWN_Y,
                };
                assert!(
                    board.fits(&piece),
                    "{kind:?} r{rotation} must spawn at y={SPAWN_Y}"
                );
            }
        }
    }
}
