//! Neon Tetris UI — board grid + minimal HUD.

use crate::board::{Board, COLS, ROWS};
use crate::config::QuantumSession;
use crate::game_state::GameRun;
use crate::pieces::PieceKind;
use crate::tetris;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;
use quantum_tetris_quantum::BackendKind;

pub(crate) const BG: Color = Color::srgb(0.04, 0.07, 0.14);
const GRID: Color = Color::srgba(0.15, 0.22, 0.38, 0.55);
const GRID_BORDER: Color = Color::srgba(0.08, 0.12, 0.22, 0.9);
const PANEL: Color = Color::srgba(0.08, 0.12, 0.22, 0.92);
const CELL_PX: f32 = 26.0;
const CELL_GAP: f32 = 2.0;

#[derive(Component)]
pub(crate) struct HudRoot;
#[derive(Component)]
pub(crate) struct GridCell {
    pub col: usize,
    pub row: usize,
}
#[derive(Component)]
pub(crate) struct HudScore;
#[derive(Component)]
pub(crate) struct HudLines;
#[derive(Component)]
pub(crate) struct HudNext;
#[derive(Component)]
pub(crate) struct NextPreviewCell {
    pub col: usize,
    pub row: usize,
}
#[derive(Component)]
pub(crate) struct HudBits;
#[derive(Component)]
pub(crate) struct ModeClassicBtn;
#[derive(Component)]
pub(crate) struct ModeQuantumBtn;

pub(crate) fn setup_ui(mut commands: Commands, run: Res<GameRun>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(24.0),
                ..default()
            },
        ))
        .with_children(|root| {
            spawn_side_panel(root, &run);
            spawn_grid(root);
        });
}

fn spawn_grid(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(CELL_GAP),
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.1, 0.2, 0.95)),
            BorderColor::all(Color::srgb(0.35, 0.55, 0.85)),
        ))
        .with_children(|wrap| {
            for row in (0..ROWS).rev() {
                wrap.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(CELL_GAP),
                    width: Val::Px(COLS as f32 * CELL_PX + (COLS - 1) as f32 * CELL_GAP),
                    height: Val::Px(CELL_PX),
                    ..default()
                })
                .with_children(|line| {
                    for col in 0..COLS {
                        line.spawn(grid_cell_bundle(col, row));
                    }
                });
            }
        });
}

fn grid_cell_bundle(col: usize, row: usize) -> impl Bundle {
    (
        GridCell { col, row },
        Node {
            width: Val::Px(CELL_PX),
            height: Val::Px(CELL_PX),
            min_width: Val::Px(CELL_PX),
            min_height: Val::Px(CELL_PX),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(GRID),
        BorderColor::all(GRID_BORDER),
    )
}

fn spawn_side_panel(parent: &mut ChildSpawnerCommands, run: &GameRun) {
    parent.spawn(panel_node()).with_children(|p| {
        spawn_mode_row(p, run.is_quantum);
        p.spawn((
            Text::new("← → move   ↑ rotate   ↓ faster   Space drop"),
            text_style(12.0, Color::srgb(0.75, 0.85, 0.95)),
        ));
        p.spawn((
            HudScore,
            Text::new("0"),
            text_style(28.0, Color::srgb(1.0, 0.92, 0.35)),
        ));
        p.spawn((
            HudLines,
            Text::new("lines 0 · lv 1"),
            text_style(14.0, Color::srgb(0.7, 0.85, 0.95)),
        ));
        spawn_next_preview(p);
        p.spawn((
            HudNext,
            Text::new("next T"),
            text_style(13.0, Color::srgb(0.65, 0.78, 0.92)),
        ));
        p.spawn((
            HudBits,
            Text::new("—"),
            text_style(13.0, Color::srgb(0.55, 0.9, 0.75)),
        ));
    });
}

fn spawn_mode_row(parent: &mut ChildSpawnerCommands, quantum: bool) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|row| {
            spawn_mode_button(row, ModeClassicBtn, "CLASSIC", !quantum);
            spawn_mode_button(row, ModeQuantumBtn, "QUANTUM", quantum);
        });
}

fn spawn_mode_button<M: Component>(
    parent: &mut ChildSpawnerCommands,
    marker: M,
    label: &str,
    selected: bool,
) {
    let (bg, border, text_c) = mode_button_colors(selected);
    parent
        .spawn((
            marker,
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(36.0),
                min_width: Val::Px(100.0),
                min_height: Val::Px(36.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(bg),
            BorderColor::all(border),
        ))
        .with_children(|btn| {
            btn.spawn((Text::new(label), text_style(13.0, text_c)));
        });
}

fn mode_button_colors(selected: bool) -> (Color, Color, Color) {
    if selected {
        (
            Color::srgb(0.15, 0.35, 0.55),
            Color::srgb(0.45, 0.85, 1.0),
            Color::srgb(0.95, 0.98, 1.0),
        )
    } else {
        (
            Color::srgb(0.06, 0.1, 0.18),
            Color::srgb(0.3, 0.4, 0.5),
            Color::srgb(0.65, 0.72, 0.82),
        )
    }
}

const PREVIEW: usize = 4;
const PREVIEW_CELL: f32 = 18.0;

fn spawn_next_preview(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                padding: UiRect::all(Val::Px(6.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.08, 0.16, 0.95)),
            BorderColor::all(Color::srgb(0.25, 0.4, 0.6)),
        ))
        .with_children(|wrap| {
            for row in 0..PREVIEW {
                wrap.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(2.0),
                    height: Val::Px(PREVIEW_CELL),
                    ..default()
                })
                .with_children(|line| {
                    for col in 0..PREVIEW {
                        line.spawn((
                            NextPreviewCell { col, row },
                            Node {
                                width: Val::Px(PREVIEW_CELL),
                                height: Val::Px(PREVIEW_CELL),
                                min_width: Val::Px(PREVIEW_CELL),
                                min_height: Val::Px(PREVIEW_CELL),
                                border_radius: BorderRadius::all(Val::Px(3.0)),
                                ..default()
                            },
                            BackgroundColor(GRID),
                        ));
                    }
                });
            }
        });
}

#[allow(clippy::type_complexity)]
pub(crate) fn handle_mode_buttons(
    mut session: ResMut<QuantumSession>,
    mut board: ResMut<Board>,
    mut run: ResMut<GameRun>,
    mut buttons: ParamSet<(
        Query<&Interaction, (Changed<Interaction>, With<ModeClassicBtn>, With<Button>)>,
        Query<&Interaction, (Changed<Interaction>, With<ModeQuantumBtn>, With<Button>)>,
    )>,
) {
    let pick_classic = buttons.p0().iter().any(|i| *i == Interaction::Pressed);
    let pick_quantum = buttons.p1().iter().any(|i| *i == Interaction::Pressed);

    if pick_classic {
        apply_mode(&mut session, &mut board, &mut run, BackendKind::Classic);
    } else if pick_quantum {
        apply_mode(&mut session, &mut board, &mut run, BackendKind::Quantum);
    }
}

fn apply_mode(
    session: &mut QuantumSession,
    board: &mut Board,
    run: &mut GameRun,
    kind: BackendKind,
) {
    if !session.switch_to(kind) {
        return;
    }
    tetris::restart_game(session, board, run);
}

#[allow(clippy::type_complexity)]
pub(crate) fn refresh_ui(
    board: Res<Board>,
    run: Res<GameRun>,
    mut cells: Query<(&GridCell, &mut BackgroundColor)>,
    mut preview: Query<(&NextPreviewCell, &mut BackgroundColor), Without<GridCell>>,
    mut classic_btn: Query<
        (&mut BackgroundColor, &mut BorderColor),
        (With<ModeClassicBtn>, Without<ModeQuantumBtn>),
    >,
    mut quantum_btn: Query<
        (&mut BackgroundColor, &mut BorderColor),
        (With<ModeQuantumBtn>, Without<ModeClassicBtn>),
    >,
    mut texts: ParamSet<(
        Query<&mut Text, With<HudScore>>,
        Query<&mut Text, With<HudLines>>,
        Query<&mut Text, With<HudNext>>,
        Query<&mut Text, With<HudBits>>,
    )>,
) {
    for (cell, mut bg) in &mut cells {
        *bg = BackgroundColor(board.display_color(cell.col, cell.row).unwrap_or(GRID));
    }

    for (cell, mut bg) in &mut preview {
        *bg = BackgroundColor(preview_color(board.next, cell.col, cell.row).unwrap_or(GRID));
    }

    let quantum = run.is_quantum;
    if let Ok((mut bg, mut border)) = classic_btn.single_mut() {
        let (c, b, _) = mode_button_colors(!quantum);
        *bg = BackgroundColor(c);
        *border = BorderColor::all(b);
    }
    if let Ok((mut bg, mut border)) = quantum_btn.single_mut() {
        let (c, b, _) = mode_button_colors(quantum);
        *bg = BackgroundColor(c);
        *border = BorderColor::all(b);
    }

    for mut t in texts.p0().iter_mut() {
        **t = run.score.to_string();
    }
    for mut t in texts.p1().iter_mut() {
        **t = format!("lines {} · lv {}", run.lines, run.level);
    }
    for mut t in texts.p2().iter_mut() {
        **t = format!("next {}", next_label(board.next));
    }
    for mut t in texts.p3().iter_mut() {
        **t = if run.last_bits.is_empty() {
            "—".into()
        } else if run.is_quantum {
            format!("[{}] {:.0}%", run.last_bits, run.last_confidence)
        } else {
            format!("[{}]", run.last_bits)
        };
    }
}

fn preview_color(kind: PieceKind, col: usize, row: usize) -> Option<Color> {
    for (pc, pr) in crate::pieces::preview_shape(kind) {
        if pc == col && pr == row {
            return Some(kind.color());
        }
    }
    None
}

fn panel_node() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            padding: UiRect::all(Val::Px(14.0)),
            min_width: Val::Px(220.0),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(PANEL),
        BorderColor::all(Color::srgb(0.3, 0.45, 0.65)),
    )
}

fn text_style(size: f32, color: Color) -> impl Bundle {
    (
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
    )
}

fn next_label(k: PieceKind) -> &'static str {
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
