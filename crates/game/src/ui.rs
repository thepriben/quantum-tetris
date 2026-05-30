//! Neon Tetris UI — board grid + HUD.

use crate::board::{Board, COLS, ROWS};
use crate::game_state::GameRun;
use crate::pieces::PieceKind;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;

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
pub(crate) struct HudMode;
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
pub(crate) struct HudEvent;
#[derive(Component)]
pub(crate) struct HudHint;

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
                column_gap: Val::Px(28.0),
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
        p.spawn((
            Text::new("QUANTUM TETRIS"),
            text_style(20.0, Color::srgb(0.45, 0.95, 1.0)),
        ));
        p.spawn((
            HudMode,
            Text::new(mode_label(run.is_quantum)),
            text_style(16.0, mode_color(run.is_quantum)),
        ));
        p.spawn((
            HudScore,
            Text::new("SCORE 0"),
            text_style(26.0, Color::srgb(1.0, 0.92, 0.35)),
        ));
        p.spawn((
            HudLines,
            Text::new("LINES 0 · LV 1"),
            text_style(15.0, Color::srgb(0.7, 0.85, 0.95)),
        ));
        spawn_arcade_controls(p);
        p.spawn((
            Text::new("NEXT · TELEPORTER"),
            text_style(13.0, Color::srgb(0.6, 0.75, 0.9)),
        ));
        spawn_next_preview(p);
        p.spawn((
            HudNext,
            Text::new("Next Fork T"),
            text_style(14.0, Color::srgb(0.55, 0.65, 0.8)),
        ));
        p.spawn((
            Text::new("Bell → family"),
            text_style(12.0, Color::srgb(0.55, 0.65, 0.8)),
        ));
        p.spawn((
            Text::new("00 Line · 01 Block · 10 Fork · 11 Corner"),
            text_style(11.0, Color::srgb(0.65, 0.78, 0.92)),
        ));
        p.spawn((
            HudBits,
            Text::new("[---] —%"),
            text_style(14.0, Color::srgb(0.55, 0.9, 0.75)),
        ));
        p.spawn((
            HudHint,
            Text::new(&run.hint),
            text_style(13.0, Color::srgb(0.98, 0.75, 0.45)),
        ));
        p.spawn((
            HudEvent,
            Text::new(&run.last_event),
            text_style(12.0, Color::srgb(0.98, 0.82, 0.45)),
        ));
    });
}

fn spawn_arcade_controls(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.12, 0.06, 0.22, 0.95)),
            BorderColor::all(Color::srgb(0.85, 0.35, 0.95)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("INSERT COIN TO PLAY"),
                text_style(11.0, Color::srgb(0.95, 0.55, 0.75)),
            ));
            arcade_row(panel, "←  →", "MOVE", Color::srgb(0.35, 0.75, 1.0));
            arcade_row(panel, "↑", "ROTATE", Color::srgb(1.0, 0.85, 0.25));
            arcade_row(panel, "↓", "FASTER!", Color::srgb(1.0, 0.55, 0.35));
            arcade_row(panel, "SPACE", "OBSERVE!", Color::srgb(0.55, 1.0, 0.65));
        });
}

fn arcade_row(parent: &mut ChildSpawnerCommands, key: &str, action: &str, key_color: Color) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.05, 0.08, 0.16, 0.9)),
                BorderColor::all(key_color),
            ))
            .with_children(|key_box| {
                key_box.spawn((Text::new(key), text_style(14.0, key_color)));
            });
            row.spawn((
                Text::new(action),
                text_style(13.0, Color::srgb(0.92, 0.94, 1.0)),
            ));
        });
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

fn preview_color(kind: PieceKind, col: usize, row: usize) -> Option<Color> {
    for (pc, pr) in crate::pieces::preview_shape(kind) {
        if pc == col && pr == row {
            return Some(kind.color());
        }
    }
    None
}

#[allow(clippy::type_complexity)]
pub(crate) fn refresh_ui(
    board: Res<Board>,
    run: Res<GameRun>,
    mut cells: Query<(&GridCell, &mut BackgroundColor)>,
    mut preview: Query<(&NextPreviewCell, &mut BackgroundColor), Without<GridCell>>,
    mut texts: ParamSet<(
        Query<&mut Text, With<HudMode>>,
        Query<&mut Text, With<HudScore>>,
        Query<&mut Text, With<HudLines>>,
        Query<&mut Text, With<HudNext>>,
        Query<&mut Text, With<HudBits>>,
        Query<&mut Text, With<HudHint>>,
        Query<&mut Text, With<HudEvent>>,
    )>,
) {
    for (cell, mut bg) in &mut cells {
        if let Some(c) = board.display_color(cell.col, cell.row) {
            *bg = BackgroundColor(c);
        } else {
            *bg = BackgroundColor(GRID);
        }
    }

    for (cell, mut bg) in &mut preview {
        if let Some(c) = preview_color(board.next, cell.col, cell.row) {
            *bg = BackgroundColor(c);
        } else {
            *bg = BackgroundColor(GRID);
        }
    }

    for mut t in texts.p0().iter_mut() {
        **t = mode_label(run.is_quantum).into();
    }
    for mut t in texts.p1().iter_mut() {
        **t = format!("SCORE {}", run.score);
    }
    for mut t in texts.p2().iter_mut() {
        **t = format!(
            "LINES {} · LV {} · {:.2}s",
            run.lines, run.level, run.drop_interval
        );
    }
    for mut t in texts.p3().iter_mut() {
        **t = format!(
            "Next {} {}",
            board.next_family.label(),
            next_label(board.next)
        );
    }
    for mut t in texts.p4().iter_mut() {
        **t = if run.last_bits.is_empty() {
            "—".into()
        } else {
            format!(
                "tele [{bits}] {fam} {conf:.0}%",
                bits = run.last_bits,
                fam = run.active_family.label(),
                conf = run.last_confidence
            )
        };
    }
    for mut t in texts.p5().iter_mut() {
        **t = run.hint.clone();
    }
    for mut t in texts.p6().iter_mut() {
        **t = run.last_event.clone();
    }
}

fn panel_node() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(16.0)),
            min_width: Val::Px(200.0),
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

fn mode_label(qiskit: bool) -> &'static str {
    if qiskit {
        "QISKIT · BORN RULE"
    } else {
        "CLASSIC · ARCADE"
    }
}

fn mode_color(qiskit: bool) -> Color {
    if qiskit {
        Color::srgb(0.55, 0.85, 1.0)
    } else {
        Color::srgb(0.95, 0.75, 0.35)
    }
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
