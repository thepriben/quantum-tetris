//! Neon Tetris UI — board grid + HUD.

use crate::board::{Board, COLS, ROWS};
use crate::game_state::GameRun;
use crate::pieces::PieceKind;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;

pub(crate) const BG: Color = Color::srgb(0.04, 0.07, 0.14);
const GRID: Color = Color::srgba(0.15, 0.22, 0.38, 0.55);
const PANEL: Color = Color::srgba(0.08, 0.12, 0.22, 0.92);

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
pub(crate) struct HudBits;
#[derive(Component)]
pub(crate) struct HudEvent;
#[derive(Component)]
pub(crate) struct HudHint;

pub fn setup_ui(mut commands: Commands, run: Res<GameRun>) {
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
                    column_gap: Val::Px(3.0),
                    ..default()
                })
                .with_children(|line| {
                    for col in 0..COLS {
                        line.spawn((
                            GridCell { col, row },
                            Node {
                                width: Val::Px(26.0),
                                height: Val::Px(26.0),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(GRID),
                        ));
                    }
                });
            }
        });
}

fn spawn_side_panel(parent: &mut ChildSpawnerCommands, run: &GameRun) {
    parent
        .spawn(panel_node())
        .with_children(|p| {
            p.spawn((
                HudMode,
                Text::new(mode_label(run.is_quantum)),
                text_style(18.0, mode_color(run.is_quantum)),
            ));
            p.spawn((
                HudScore,
                Text::new("Score 0"),
                text_style(22.0, Color::srgb(0.9, 0.95, 1.0)),
            ));
            p.spawn((
                HudLines,
                Text::new("Lines 0 · Lv 1"),
                text_style(16.0, Color::srgb(0.7, 0.85, 0.95)),
            ));
            p.spawn((
                Text::new("Next"),
                text_style(14.0, Color::srgb(0.6, 0.75, 0.9)),
            ));
            p.spawn((
                HudNext,
                Text::new("T"),
                text_style(32.0, PieceKind::T.color()),
            ));
            p.spawn((
                HudBits,
                Text::new("[---] —%"),
                text_style(14.0, Color::srgb(0.55, 0.9, 0.75)),
            ));
            p.spawn((
                Text::new("Controls"),
                text_style(13.0, Color::srgb(0.55, 0.65, 0.8)),
            ));
            p.spawn((
                Text::new("← →  move\n↑     rotate\n↓     soft drop\nSpace observe"),
                text_style(13.0, Color::srgb(0.75, 0.85, 0.95)),
            ));
            p.spawn((
                HudHint,
                Text::new(&run.hint),
                text_style(14.0, Color::srgb(0.65, 0.82, 0.95)),
            ));
            p.spawn((
                HudEvent,
                Text::new(&run.last_event),
                text_style(13.0, Color::srgb(0.98, 0.82, 0.45)),
            ));
        });
}

pub fn refresh_ui(
    board: Res<Board>,
    run: Res<GameRun>,
    mut cells: Query<(&GridCell, &mut BackgroundColor)>,
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

    for mut t in texts.p0().iter_mut() {
        **t = mode_label(run.is_quantum).into();
    }
    for mut t in texts.p1().iter_mut() {
        **t = format!("Score {}", run.score);
    }
    for mut t in texts.p2().iter_mut() {
        **t = format!("Lines {} · Lv {}", run.lines, run.level);
    }
    for mut t in texts.p3().iter_mut() {
        **t = next_label(board.next).into();
    }
    for mut t in texts.p4().iter_mut() {
        **t = if run.last_bits.is_empty() {
            "—".into()
        } else {
            format!("[{}] {:.0}%", run.last_bits, run.last_confidence)
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
    (TextFont { font_size: size, ..default() }, TextColor(color))
}

fn mode_label(q: bool) -> &'static str {
    if q {
        "QIP · quantum"
    } else {
        "Classic · uniform"
    }
}

fn mode_color(q: bool) -> Color {
    if q {
        Color::srgb(0.55, 0.85, 1.0)
    } else {
        Color::srgb(0.85, 0.75, 0.55)
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
