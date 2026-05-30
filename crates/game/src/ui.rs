//! Graphical HUD — submarine run.

use crate::game_state::{GameRun, RunState, ENERGY_GOAL, RUN_DURATION_SECS};
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;

const CYAN: Color = Color::srgb(0.35, 0.95, 0.92);
const GOLD: Color = Color::srgb(0.98, 0.82, 0.35);
const PANEL: Color = Color::srgba(0.08, 0.2, 0.28, 0.9);

#[derive(Component)]
pub(crate) struct HudRoot;
#[derive(Component)]
pub(crate) struct HudTimerFill;
#[derive(Component)]
pub(crate) struct HudTimerLabel;
#[derive(Component)]
pub(crate) struct HudEnergyDot(pub u8);
#[derive(Component)]
pub(crate) struct HudHint;
#[derive(Component)]
pub(crate) struct HudCurrentArrow;
#[derive(Component)]
pub(crate) struct HudModeBadge;
#[derive(Component)]
pub(crate) struct HudConfidence;
#[derive(Component)]
pub(crate) struct HudEvent;

pub(crate) fn setup_hud(commands: &mut Commands, run: &GameRun) {
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            ZIndex(100),
        ))
        .with_children(|root| {
            spawn_top(root, run);
            spawn_bottom(root, run);
        });
}

pub(crate) fn refresh_hud(
    run: Res<GameRun>,
    mut timer_fill: Query<&mut Node, With<HudTimerFill>>,
    mut texts: ParamSet<(
        Query<&mut Text, With<HudTimerLabel>>,
        Query<&mut Text, With<HudHint>>,
        Query<&mut Text, With<HudCurrentArrow>>,
        Query<&mut Text, With<HudModeBadge>>,
        Query<&mut Text, With<HudConfidence>>,
        Query<&mut Text, With<HudEvent>>,
    )>,
    mut dots: Query<(&HudEnergyDot, &mut BackgroundColor)>,
) {
    let pct = (run.time_remaining / RUN_DURATION_SECS).clamp(0.0, 1.0) * 100.0;
    for mut n in &mut timer_fill {
        n.width = Val::Percent(pct);
    }
    let m = (run.time_remaining / 60.0).floor() as u32;
    let s = (run.time_remaining % 60.0).floor() as u32;
    for mut t in texts.p0().iter_mut() {
        **t = format!("{m:02}:{s:02}");
    }
    for (dot, mut bg) in &mut dots {
        bg.0 = if dot.0 < run.energy {
            CYAN
        } else {
            Color::srgba(0.3, 0.5, 0.55, 0.4)
        };
    }
    for mut t in texts.p1().iter_mut() {
        **t = run.hint.clone();
    }
    for mut t in texts.p2().iter_mut() {
        **t = current_arrow(&run.quantum_current);
    }
    for mut t in texts.p3().iter_mut() {
        **t = if run.is_quantum {
            "QIP (quantum)".into()
        } else {
            "CLASSIC (uniform)".into()
        };
    }
    for mut t in texts.p4().iter_mut() {
        **t = if run.last_confidence > 0.0 {
            format!("{:.0}%", run.last_confidence)
        } else {
            "—".into()
        };
    }
    for mut t in texts.p5().iter_mut() {
        **t = match run.state {
            RunState::Won => "You win!".into(),
            RunState::LostTime => "Time's up".into(),
            RunState::Playing => {
                if run.last_event.is_empty() {
                    format!("coh {:.0}", run.coherence)
                } else if run.last_bits.is_empty() {
                    run.last_event.clone()
                } else {
                    format!(
                        "[{}] {:.0}% — {}",
                        run.last_bits, run.last_confidence, run.last_event
                    )
                }
            }
        };
    }
}

fn spawn_top(parent: &mut ChildSpawnerCommands, run: &GameRun) {
    parent.spawn(panel()).with_children(|bar| {
        bar.spawn((
            HudModeBadge,
            Text::new(if run.is_quantum {
                "QIP (quantum)"
            } else {
                "CLASSIC (uniform)"
            }),
            label_color(
                16.0,
                if run.is_quantum {
                    Color::srgb(0.55, 0.85, 1.0)
                } else {
                    Color::srgb(0.85, 0.75, 0.55)
                },
            ),
        ));
        bar.spawn(Node {
            flex_grow: 1.0,
            ..default()
        });
        bar.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|c| {
            c.spawn((HudTimerLabel, Text::new("02:00"), label(18.0)));
            c.spawn((
                Node {
                    width: Val::Px(110.0),
                    height: Val::Px(8.0),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.4, 0.5, 0.5)),
            ))
            .with_children(|t| {
                t.spawn((
                    HudTimerFill,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(CYAN),
                ));
            });
        });
        bar.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(5.0),
            margin: UiRect::left(Val::Px(12.0)),
            ..default()
        })
        .with_children(|row| {
            for i in 0..ENERGY_GOAL {
                row.spawn((
                    HudEnergyDot(i),
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.5, 0.55, 0.4)),
                ));
            }
        });
    });
}

fn spawn_bottom(parent: &mut ChildSpawnerCommands, run: &GameRun) {
    parent.spawn(panel()).with_children(|p| {
        p.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        })
        .with_children(|row| {
            row.spawn((Text::new("Drive"), label(14.0)));
            for k in ["↑", "↓", "←", "→"] {
                key_badge(row, k);
            }
            key_badge(row, "Space");
            row.spawn((Text::new("act"), label(14.0)));
        });
        p.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((Text::new("Current"), label(14.0)));
            row.spawn((
                HudCurrentArrow,
                Text::new(current_arrow(&run.quantum_current)),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
            ));
            row.spawn((Text::new(run.current_label.clone()), label(14.0)));
            row.spawn((
                HudConfidence,
                Text::new("—"),
                label_color(14.0, Color::srgb(0.65, 0.9, 0.75)),
            ));
        });
        p.spawn((HudHint, Text::new(&run.hint), label(15.0)));
        p.spawn((HudEvent, Text::new(""), label_color(14.0, GOLD)));
    });
}

fn current_arrow(v: &Vec3) -> String {
    if v.length_squared() < 0.05 {
        return "·".into();
    }
    if v.z < -0.45 {
        "↑".into()
    } else if v.z > 0.45 {
        "↓".into()
    } else if v.x > 0.45 {
        "→".into()
    } else if v.x < -0.45 {
        "←".into()
    } else {
        "◎".into()
    }
}

fn panel() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            padding: UiRect::axes(Val::Px(12.0), Val::Px(10.0)),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(PANEL),
        BorderColor::all(Color::srgb(0.35, 0.55, 0.65)),
    )
}

fn label(size: f32) -> impl Bundle {
    label_color(size, Color::srgb(0.85, 0.95, 1.0))
}

fn label_color(size: f32, color: Color) -> impl Bundle {
    (
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
    )
}

fn key_badge(parent: &mut ChildSpawnerCommands, k: &str) {
    parent.spawn((
        Node {
            min_width: Val::Px(28.0),
            height: Val::Px(28.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.35, 0.45, 0.9)),
        BorderColor::all(Color::srgb(0.45, 0.7, 0.8)),
        Text::new(k),
        label(15.0),
    ));
}
