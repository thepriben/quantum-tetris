//! Tetris UI — board grid + HUD.

use crate::board::{Board, COLS, ROWS};
use crate::config::QuantumSession;
use crate::game_state::GameRun;
use crate::i18n::{self, Locale};
use crate::pieces::PieceKind;
use crate::tetris;
use bevy::ecs::query::Or;
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
pub(crate) struct ModeBtn(pub BackendKind);
#[derive(Component)]
pub(crate) struct ModeClassicLabel;
#[derive(Component)]
pub(crate) struct ModeRustqipLabel;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub(crate) struct ModeQiskitLabel;
#[cfg(not(target_arch = "wasm32"))]
#[derive(Component)]
pub(crate) struct LangToggleBtn;
#[derive(Component)]
pub(crate) struct LangToggleLabel;
#[derive(Component)]
pub(crate) struct HudCircuitTitle;
#[derive(Component)]
pub(crate) struct HudCircuit;
#[derive(Component)]
pub(crate) struct HudEvent;
#[derive(Component)]
pub(crate) struct HintMoveLabel;
#[derive(Component)]
pub(crate) struct HintRotateLabel;
#[derive(Component)]
pub(crate) struct HintFasterLabel;
#[derive(Component)]
pub(crate) struct HintDropLabel;

pub(crate) fn setup_ui(mut commands: Commands, run: Res<GameRun>, locale: Res<Locale>) {
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
            spawn_side_panel(root, &run, *locale);
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

fn spawn_side_panel(parent: &mut ChildSpawnerCommands, run: &GameRun, locale: Locale) {
    parent.spawn(panel_node()).with_children(|p| {
        #[cfg(not(target_arch = "wasm32"))]
        spawn_lang_row(p, locale);
        spawn_mode_row(p, run.backend_kind, locale);
        spawn_controls_hint(p, locale);
        p.spawn((
            HudScore,
            Text::new("0"),
            text_style(28.0, Color::srgb(1.0, 0.92, 0.35)),
        ));
        p.spawn((
            HudLines,
            Text::new(i18n::lines_level(locale, 0, 1)),
            text_style(14.0, Color::srgb(0.7, 0.85, 0.95)),
        ));
        spawn_next_preview(p);
        p.spawn((
            HudNext,
            Text::new(i18n::next_piece(locale, "T")),
            text_style(13.0, Color::srgb(0.65, 0.78, 0.92)),
        ));
        p.spawn((
            HudEvent,
            Text::new("—"),
            text_style(12.0, Color::srgb(0.72, 0.82, 0.92)),
        ));
        p.spawn((
            HudBits,
            Text::new("—"),
            text_style(13.0, Color::srgb(0.55, 0.9, 0.75)),
        ));
        p.spawn((
            HudCircuitTitle,
            Text::new(i18n::circuit_heading(locale)),
            text_style(11.0, Color::srgb(0.55, 0.75, 0.95)),
        ));
        p.spawn((
            HudCircuit,
            Text::new("—"),
            text_style_multiline(10.0, Color::srgb(0.62, 0.78, 0.88)),
        ));
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_lang_row(parent: &mut ChildSpawnerCommands, locale: Locale) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                LangToggleBtn,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.08, 0.14, 0.24, 0.95)),
                BorderColor::all(Color::srgb(0.35, 0.5, 0.68)),
            ))
            .with_children(|btn| {
                btn.spawn((
                    LangToggleLabel,
                    Text::new(locale.toggle_label()),
                    text_style(11.0, Color::srgb(0.75, 0.88, 1.0)),
                ));
            });
        });
}

fn spawn_mode_row(parent: &mut ChildSpawnerCommands, active: BackendKind, locale: Locale) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            column_gap: Val::Px(6.0),
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|row| {
            spawn_mode_button(
                row,
                ModeBtn(BackendKind::Classic),
                ModeClassicLabel,
                i18n::mode_classic(locale),
                active == BackendKind::Classic,
            );
            spawn_mode_button(
                row,
                ModeBtn(BackendKind::Quantum),
                ModeRustqipLabel,
                i18n::mode_rustqip(locale),
                active == BackendKind::Quantum,
            );
            #[cfg(not(target_arch = "wasm32"))]
            spawn_mode_button(
                row,
                ModeBtn(BackendKind::Qiskit),
                ModeQiskitLabel,
                i18n::mode_qiskit(locale),
                active == BackendKind::Qiskit,
            );
        });
}

fn spawn_mode_button(
    parent: &mut ChildSpawnerCommands,
    mode: ModeBtn,
    label_marker: impl Component,
    label: &str,
    selected: bool,
) {
    let (bg, border, text_c) = mode_button_colors(selected);
    parent
        .spawn((
            mode,
            Button,
            Node {
                width: Val::Px(76.0),
                height: Val::Px(36.0),
                min_width: Val::Px(76.0),
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
            btn.spawn((
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                label_marker,
                Text::new(label),
                text_style(12.0, text_c),
            ));
        });
}

fn spawn_controls_hint(parent: &mut ChildSpawnerCommands, locale: Locale) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            column_gap: Val::Px(5.0),
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|row| {
            spawn_hint_chip(row, "LR", i18n::hint_move(locale), HintMoveLabel);
            spawn_hint_chip(row, "^", i18n::hint_rotate(locale), HintRotateLabel);
            spawn_hint_chip(row, "v", i18n::hint_faster(locale), HintFasterLabel);
            spawn_hint_chip(row, "Sp", i18n::hint_drop(locale), HintDropLabel);
        });
}

fn spawn_hint_chip(
    parent: &mut ChildSpawnerCommands,
    key: &str,
    action: &str,
    label_marker: impl Component,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|chip| {
            chip.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(5.0), Val::Px(2.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.16, 0.26, 0.98)),
                BorderColor::all(Color::srgb(0.4, 0.58, 0.82)),
            ))
            .with_children(|k| {
                k.spawn((
                    Text::new(key),
                    text_style(10.0, Color::srgb(1.0, 0.82, 0.4)),
                ));
            });
            chip.spawn((
                label_marker,
                Text::new(action),
                text_style(10.0, Color::srgb(0.78, 0.88, 0.98)),
            ));
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

#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::type_complexity)]
pub(crate) fn handle_lang_button(
    mut locale: ResMut<Locale>,
    btn: Query<&Interaction, (Changed<Interaction>, With<LangToggleBtn>, With<Button>)>,
) {
    if btn.iter().any(|i| *i == Interaction::Pressed) {
        *locale = locale.toggle();
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn handle_mode_buttons(
    mut session: ResMut<QuantumSession>,
    locale: Res<Locale>,
    mut board: ResMut<Board>,
    mut run: ResMut<GameRun>,
    buttons: Query<(&ModeBtn, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (mode, interaction) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        apply_mode(&mut session, &mut board, &mut run, mode.0, *locale);
    }
}

fn apply_mode(
    session: &mut QuantumSession,
    board: &mut Board,
    run: &mut GameRun,
    kind: BackendKind,
    locale: Locale,
) {
    if !session.switch_to(kind) {
        return;
    }
    tetris::restart_game(session, board, run, locale);
}

#[allow(clippy::type_complexity)]
pub(crate) fn refresh_ui(
    board: Res<Board>,
    run: Res<GameRun>,
    locale: Res<Locale>,
    mut bg_queries: ParamSet<(
        Query<(&GridCell, &mut BackgroundColor)>,
        Query<(&NextPreviewCell, &mut BackgroundColor), Without<GridCell>>,
    )>,
    mut mode_buttons: Query<(&ModeBtn, &mut BackgroundColor, &mut BorderColor), With<Button>>,
    mut texts: Query<
        (
            &mut Text,
            Has<HudScore>,
            Has<HudLines>,
            Has<HudNext>,
            Has<HudBits>,
            Has<HudEvent>,
            Has<HudCircuitTitle>,
            Has<HudCircuit>,
            Has<ModeClassicLabel>,
            Has<ModeRustqipLabel>,
            Has<HintMoveLabel>,
            Has<HintRotateLabel>,
            Has<HintFasterLabel>,
            Has<HintDropLabel>,
            Has<LangToggleLabel>,
        ),
        Or<(
            With<HudScore>,
            With<HudLines>,
            With<HudNext>,
            With<HudBits>,
            With<HudEvent>,
            With<HudCircuitTitle>,
            With<HudCircuit>,
            With<ModeClassicLabel>,
            With<ModeRustqipLabel>,
            With<HintMoveLabel>,
            With<HintRotateLabel>,
            With<HintFasterLabel>,
            With<HintDropLabel>,
            With<LangToggleLabel>,
        )>,
    >,
    #[cfg(not(target_arch = "wasm32"))] mut qiskit_labels: Query<
        &mut Text,
        (
            With<ModeQiskitLabel>,
            Without<ModeClassicLabel>,
            Without<ModeRustqipLabel>,
        ),
    >,
) {
    for (cell, mut bg) in bg_queries.p0().iter_mut() {
        *bg = BackgroundColor(board.display_color(cell.col, cell.row).unwrap_or(GRID));
    }

    for (cell, mut bg) in bg_queries.p1().iter_mut() {
        *bg = BackgroundColor(preview_color(board.next, cell.col, cell.row).unwrap_or(GRID));
    }

    for (mode, mut bg, mut border) in mode_buttons.iter_mut() {
        let (c, b, _) = mode_button_colors(mode.0 == run.backend_kind);
        *bg = BackgroundColor(c);
        *border = BorderColor::all(b);
    }

    #[cfg(not(target_arch = "wasm32"))]
    for mut label in qiskit_labels.iter_mut() {
        **label = i18n::mode_qiskit(*locale).into();
    }

    for (
        mut t,
        score,
        lines,
        next,
        bits,
        event,
        circ_title,
        circ,
        classic,
        rustqip,
        move_l,
        rotate_l,
        faster_l,
        drop_l,
        lang,
    ) in texts.iter_mut()
    {
        if score {
            **t = run.score.to_string();
        } else if lines {
            **t = i18n::lines_level(*locale, run.lines, run.level);
        } else if next {
            **t = i18n::next_piece(*locale, next_label(board.next));
        } else if bits {
            **t = if run.last_bits.is_empty() {
                "—".into()
            } else if run.is_quantum {
                format!("[{}] {:.0}%", run.last_bits, run.last_confidence)
            } else {
                format!("[{}]", run.last_bits)
            };
        } else if event {
            **t = if run.last_event.is_empty() {
                "—".into()
            } else {
                run.last_event.clone()
            };
        } else if circ_title {
            **t = i18n::circuit_heading(*locale).into();
        } else if circ {
            **t = i18n::circuit_explain(*locale, run.last_moment).into();
        } else if classic {
            **t = i18n::mode_classic(*locale).into();
        } else if rustqip {
            **t = i18n::mode_rustqip(*locale).into();
        } else if move_l {
            **t = i18n::hint_move(*locale).into();
        } else if rotate_l {
            **t = i18n::hint_rotate(*locale).into();
        } else if faster_l {
            **t = i18n::hint_faster(*locale).into();
        } else if drop_l {
            **t = i18n::hint_drop(*locale).into();
        } else if lang {
            **t = locale.toggle_label().into();
        }
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
            min_width: Val::Px(260.0),
            max_width: Val::Px(260.0),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(PANEL),
        BorderColor::all(Color::srgb(0.3, 0.45, 0.65)),
    )
}

fn text_style_multiline(size: f32, color: Color) -> impl Bundle {
    (
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
        TextLayout {
            linebreak: LineBreak::WordBoundary,
            ..default()
        },
    )
}

fn text_style(size: f32, color: Color) -> impl Bundle {
    (
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
        TextLayout {
            linebreak: LineBreak::NoWrap,
            ..default()
        },
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
