use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

// === Resources ===

#[derive(Resource)]
struct ProblemTimer(Timer);

#[derive(Resource)]
struct GameState {
    turn: u32,
    message: String, // ONLY messages here (no stats)
}

#[derive(Resource)]
struct ReefStats {
    coral: i32,
    algae: i32,
    ph: f32,
    temp: f32,
}

// === UI Markers ===

#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct MessageText;

#[derive(Component)]
struct MapPanel;

#[derive(Component)]
struct ReefView;

#[derive(Clone, Copy)]
enum ToolKind {
    ArtificialSubstrates,
    CoralGardening,
    MicroFragmentation,
    RemovingPollution,
}

#[derive(Component)]
struct ToolButton {
    kind: ToolKind,
}

#[derive(Component)]
struct QuitButton;

// === MAIN ===

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Coral Reef Simulator".into(),
                resolution: (900., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ProblemTimer(Timer::new(
            Duration::from_secs(5),
            TimerMode::Repeating,
        )))
        .insert_resource(GameState {
            turn: 0,
            message: "Welcome! Use tools to restore the reef.".to_string(),
        })
        .insert_resource(ReefStats {
            coral: 35,
            algae: 10,
            ph: 8.1,
            temp: 27.0,
        })
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            (
                quit_button_system,
                tool_button_system,
                problem_timer_system,
                update_stats_ui_system,
            ),
        )
        .run();
}

// === UI SETUP ===

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Root vertical layout
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.0, 0.1, 0.25)),
            ..default()
        })
        .with_children(|root| {
            // === Top Bar (Quit button) ===
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
                ..default()
            })
            .with_children(|bar| {
                bar.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(60.0),
                            height: Val::Px(28.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::right(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.8, 0.3, 0.3)),
                        ..default()
                    },
                    QuitButton,
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: Default::default(),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            });

            // === Main Content (reef + sidebar) ===
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(80.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|main_row| {
                // LEFT — Reef View
                main_row
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_grow: 3.0,
                                margin: UiRect::all(Val::Px(8.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.1, 0.25, 0.5)),
                            ..default()
                        },
                        ReefView,
                    ))
                    .with_children(|reef| {
                        reef.spawn(TextBundle::from_section(
                            "Reef View (visuals coming soon)",
                            TextStyle {
                                font: Default::default(),
                                font_size: 22.0,
                                color: Color::WHITE,
                            },
                        ));
                    });

                // RIGHT — Sidebar
                main_row
                    .spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            margin: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|sidebar| {
                        // === MAP (TOP RIGHT) ===
                        sidebar
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        height: Val::Percent(25.0),
                                        padding: UiRect::all(Val::Px(8.0)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgb(
                                        0.05, 0.2, 0.25,
                                    )),
                                    ..default()
                                },
                                MapPanel,
                            ))
                            .with_children(|map| {
                                map.spawn(TextBundle::from_section(
                                    "Map (Future Feature)",
                                    TextStyle {
                                        font: Default::default(),
                                        font_size: 18.0,
                                        color: Color::WHITE,
                                    },
                                ));
                            });

                        // === MESSAGES (Under Map) ===
                        sidebar
                            .spawn(NodeBundle {
                                style: Style {
                                    height: Val::Percent(30.0),
                                    padding: UiRect::all(Val::Px(8.0)),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::srgb(
                                    0.05, 0.15, 0.35,
                                )),
                                ..default()
                            })
                            .with_children(|msg_panel| {
                                msg_panel.spawn(TextBundle::from_section(
                                    "Messages",
                                    TextStyle {
                                        font: Default::default(),
                                        font_size: 22.0,
                                        color: Color::srgb(0.8, 0.9, 1.0),
                                    },
                                ));

                                msg_panel.spawn((
                                    TextBundle::from_section(
                                        "Welcome! Use tools to improve the reef.",
                                        TextStyle {
                                            font: Default::default(),
                                            font_size: 18.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::top(Val::Px(6.0)),
                                        ..default()
                                    }),
                                    MessageText,
                                ));
                            });

                        // === TOOLS (Bottom right) ===
                        sidebar
                            .spawn(NodeBundle {
                                style: Style {
                                    height: Val::Percent(45.0),
                                    padding: UiRect::all(Val::Px(8.0)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(6.0),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::srgb(
                                    0.06, 0.18, 0.28,
                                )),
                                ..default()
                            })
                            .with_children(|tools| {
                                tools.spawn(TextBundle::from_section(
                                    "Tools",
                                    TextStyle {
                                        font: Default::default(),
                                        font_size: 22.0,
                                        color: Color::srgb(0.8, 0.9, 1.0),
                                    },
                                ));

                                let spawn_tool = |label: &str,
                                                  kind: ToolKind,
                                                  parent: &mut ChildBuilder| {
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    height: Val::Px(32.0),
                                                    width: Val::Percent(100.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                background_color: BackgroundColor(Color::srgb(
                                                    0.9, 0.8, 0.3,
                                                )),
                                                ..default()
                                            },
                                            ToolButton { kind },
                                        ))
                                        .with_children(|b| {
                                            b.spawn(TextBundle::from_section(
                                                label,
                                                TextStyle {
                                                    font: Default::default(),
                                                    font_size: 16.0,
                                                    color: Color::BLACK,
                                                },
                                            ));
                                        });
                                };

                                spawn_tool(
                                    "Artificial substrates / 3D modules",
                                    ToolKind::ArtificialSubstrates,
                                    tools,
                                );
                                spawn_tool(
                                    "Coral gardening",
                                    ToolKind::CoralGardening,
                                    tools,
                                );
                                spawn_tool(
                                    "Micro-fragmentation",
                                    ToolKind::MicroFragmentation,
                                    tools,
                                );
                                spawn_tool(
                                    "Removing pollution",
                                    ToolKind::RemovingPollution,
                                    tools,
                                );
                            });
                    });
            });

            // === Bottom Stats Bar ===
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.02, 0.05, 0.12)),
                ..default()
            })
            .with_children(|stats_row| {
                stats_row.spawn((
                    TextBundle::from_section(
                        "Stats loading...",
                        TextStyle {
                            font: Default::default(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    StatsText,
                ));
            });
        });
}

// === QUIT BUTTON SYSTEM ===

fn quit_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.6, 0.2, 0.2));
                app_exit_events.send(AppExit::Success);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.9, 0.4, 0.4));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.8, 0.3, 0.3));
            }
        }
    }
}

// === TOOL BUTTONS ===

fn tool_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ToolButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut stats: ResMut<ReefStats>,
    mut state: ResMut<GameState>,
) {
    for (interaction, mut color, tool_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                apply_tool(tool_button.kind, &mut stats, &mut state);
                *color = BackgroundColor(Color::srgb(0.3, 0.9, 0.4));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.95, 0.9, 0.6));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.9, 0.8, 0.3));
            }
        }
    }
}

fn apply_tool(kind: ToolKind, stats: &mut ReefStats, state: &mut GameState) {
    let msg = match kind {
        ToolKind::ArtificialSubstrates => {
            stats.coral += 5;
            "Applied artificial substrates!"
        }
        ToolKind::CoralGardening => {
            stats.coral += 4;
            stats.algae -= 2;
            "Applied coral gardening!"
        }
        ToolKind::MicroFragmentation => {
            stats.coral += 6;
            stats.algae -= 3;
            "Applied micro-fragmentation!"
        }
        ToolKind::RemovingPollution => {
            stats.coral += 3;
            stats.algae -= 3;
            stats.ph += 0.05;
            "Pollution removed!"
        }
    };

    clamp_stats(stats);
    state.message = msg.to_string(); // ✔ ONLY message, no stats
}

// === PROBLEM TIMER ===

fn problem_timer_system(
    time: Res<Time>,
    mut timer: ResMut<ProblemTimer>,
    mut stats: ResMut<ReefStats>,
    mut state: ResMut<GameState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let problem = rng.gen_range(1..=5);

        let msg = match problem {
            1 => {
                stats.coral -= 4;
                stats.algae += 5;
                stats.ph -= 0.05;
                "Pollution occurred!"
            }
            2 => {
                stats.coral -= 5;
                stats.algae += 4;
                "Invasive species appeared!"
            }
            3 => {
                stats.coral -= 3;
                stats.algae += 3;
                stats.temp += 1.0;
                stats.ph -= 0.03;
                "CO₂ emissions increased!"
            }
            4 => {
                stats.coral -= 6;
                stats.algae += 2;
                "Storm damage occurred!"
            }
            5 => {
                stats.coral -= 4;
                stats.algae += 3;
                "Overfishing event!"
            }
            _ => "Unknown problem",
        };

        clamp_stats(&mut stats);

        state.turn += 1;
        state.message = msg.to_string(); // ✔ messages ONLY
    }
}

// === STAT CLAMP ===

fn clamp_stats(stats: &mut ReefStats) {
    stats.coral = stats.coral.clamp(0, 100);
    stats.algae = stats.algae.clamp(0, 100);
    stats.ph = stats.ph.clamp(0.0, 14.0);
    stats.temp = stats.temp.clamp(0.0, 40.0);
}

// === UI UPDATE ===
fn update_stats_ui_system(
    state: Res<GameState>,
    stats: Res<ReefStats>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<StatsText>>,
        Query<&mut Text, With<MessageText>>,
    )>,
) {
    // Update stats bar
    if let Ok(mut text) = text_queries.p0().get_single_mut() {
        text.sections[0].value = format!(
            "Water pH: {:.2} | Temp: {:.1}°C | Coral: {}% | Algae: {}% | Turn {}",
            stats.ph, stats.temp, stats.coral, stats.algae, state.turn
        );
    }

    // Update message panel
    if let Ok(mut text) = text_queries.p1().get_single_mut() {
        text.sections[0].value = state.message.clone();
    }
}
