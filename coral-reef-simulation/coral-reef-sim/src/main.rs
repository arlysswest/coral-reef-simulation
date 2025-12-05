//WHAT TO WORK ON TO FINSIH THE PROJECT !!
// 2. fix repo file organization issue

// NOTE:
//" WARN bevy_winit::state: Skipped event Destroyed for unknown winit Window Id WindowId(140343576157488)"
// -> issue with macos / bevy and can only  be stopped by muting the warning.
// -> It does not mean anything is wrong and does not need to be fixed.

use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

// ─────────────────────────────────────────────
//                APP STATE
// ─────────────────────────────────────────────
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    StartMenu,
    InGame,
    GameOver,
}

// ─────────────────────────────────────────────
//                RESOURCES
// ─────────────────────────────────────────────
#[derive(Resource)]
struct ProblemTimer(Timer);

#[derive(Resource)]
struct GameState {
    turn: u32,
    message: String,
}

#[derive(Resource, Clone, Copy)]
struct ReefStats {
    coral: i32,
    algae: i32,
    ph: f32,
    temp: f32,
}

#[derive(Clone, Copy)]
struct CellStats {
    coral: i32,
    algae: i32,
    ph: f32,
    temp: f32,
}

#[derive(Resource)]
struct MapState {
    cells: [[CellStats; 3]; 3],
    active_x: usize,
    active_y: usize,
}

impl MapState {
    fn new() -> Self {
        let base = CellStats {
            coral: 35,
            algae: 10,
            ph: 8.1,
            temp: 27.0,
        };
        Self {
            cells: [[base; 3]; 3],
            active_x: 0,
            active_y: 0,
        }
    }
}

// ─────────────────────────────────────────────
//                UI MARKERS
// ─────────────────────────────────────────────
#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct MessageText;

#[derive(Component)]
struct ReefView;

#[derive(Component)]
struct Coral {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct ToolButton {
    kind: ToolKind,
}

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct StartScreenRoot;

// Map-related components
#[derive(Component)]
struct MapMiniRoot;

#[derive(Component)]
struct MapMiniCell {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct MapOverlayRoot;

#[derive(Component)]
struct MapOverlayCell {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct MapCloseButton;

#[derive(Clone, Copy)]
enum ToolKind {
    ArtificialSubstrates,
    CoralGardening,
    MicroFragmentation,
    RemovingPollution,
}

// ─────────────────────────────────────────────
//               MAIN ENTRYPOINT
// ─────────────────────────────────────────────
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
        .insert_state(AppState::StartMenu)
        .insert_resource(ProblemTimer(Timer::new(
            Duration::from_secs(3),
            TimerMode::Once, // fire once per cycle (start or tool use)
        )))
        .insert_resource(GameState {
            turn: 0,
            message: "Welcome! Use tools to restore the reef.".into(),
        })
        .insert_resource(ReefStats {
            coral: 35,
            algae: 10,
            ph: 8.1,
            temp: 27.0,
        })
        .insert_resource(MapState::new())
        .add_systems(Startup, setup_start_screen)
        .add_systems(
            Update,
            start_button_system.run_if(in_state(AppState::StartMenu)),
        )
        .add_systems(OnEnter(AppState::InGame), setup_game_ui)
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(
            Update,
            (
                tool_button_system,
                problem_timer_system,
                update_stats_ui_system,
                update_coral_display,
                map_expand_system,
                map_close_system,
                map_overlay_cell_system,
                update_map_colors_system,
            )
                .run_if(in_state(AppState::InGame)),
        )
        // Quit works in any state that has a QuitButton
        .add_systems(Update, quit_button_system)
        // Restart only on GameOver screen
        .add_systems(
            Update,
            restart_button_system.run_if(in_state(AppState::GameOver)),
        )
        .run();
}

// ─────────────────────────────────────────────
//             START SCREEN SETUP
// ─────────────────────────────────────────────

fn setup_start_screen(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Tag root node with StartScreenRoot
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.0, 0.4, 0.7)),
                ..default()
            },
            StartScreenRoot,
        ))
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "Help restore the coral reef!",
                TextStyle {
                    font: Default::default(),
                    font_size: 42.0,
                    color: Color::BLACK,
                },
            ));

            root.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(25.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(1.0, 0.45, 0.25)),
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                },
                StartButton,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "START",
                    TextStyle {
                        font: Default::default(),
                        font_size: 32.0,
                        color: Color::BLACK,
                    },
                ));
            });
        });
}

fn start_button_system(
    mut interaction_q: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    root_nodes: Query<Entity, With<StartScreenRoot>>,
) {
    for interaction in &mut interaction_q {
        if *interaction == Interaction::Pressed {
            // Delete ONLY the start screen UI
            for entity in &root_nodes {
                commands.entity(entity).despawn_recursive();
            }
            next_state.set(AppState::InGame);
        }
    }
}

// ─────────────────────────────────────────────
//             IN-GAME UI SETUP
// ─────────────────────────────────────────────
fn setup_game_ui(
    mut commands: Commands,
    mut timer: ResMut<ProblemTimer>,
    mut stats: ResMut<ReefStats>,
    mut state: ResMut<GameState>,
    mut map_state: ResMut<MapState>,
) {
    // Reset map + stats each time we enter InGame
    let base = CellStats {
        coral: 35,
        algae: 10,
        ph: 8.1,
        temp: 27.0,
    };
    map_state.cells = [[base; 3]; 3];
    map_state.active_x = 0;
    map_state.active_y = 0;

    // Current cell stats
    let active = map_state.cells[0][0];
    stats.coral = active.coral;
    stats.algae = active.algae;
    stats.ph = active.ph;
    stats.temp = active.temp;

    state.turn = 0;
    state.message = "Simulation Started! Use tools to restore the reef.".into();

    timer.0.reset();
    timer.0.pause();

    // NOTE: no extra Camera2dBundle here – we reuse the start-screen camera

    // ROOT
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.0, 0.1, 0.25)),
            ..default()
        })
        .with_children(|root| {
            // TOP BAR
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Row,
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

            // MAIN ROW
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(80.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|row| {
                //
                // LEFT: REEF VIEW
                //
                row.spawn((
                    NodeBundle {
                        style: Style {
                            flex_grow: 3.0,
                            margin: UiRect::all(Val::Px(10.0)),
                            position_type: PositionType::Relative,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::FlexStart,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.0, 0.4, 0.7)),
                        ..default()
                    },
                    ReefView,
                ));

                //
                // RIGHT COLUMN
                //
                row.spawn(NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|sidebar| {
                    // MAP BOX
                    sidebar
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(25.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(6.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.05, 0.2, 0.25)),
                            ..default()
                        })
                        .with_children(|map_box| {
                            map_box.spawn(TextBundle::from_section(
                                "Map (click to expand)",
                                TextStyle {
                                    font: Default::default(),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            ));

                            // Mini map button (3x3)
                            map_box
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(80.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::srgb(
                                            0.02, 0.08, 0.15,
                                        )),
                                        ..default()
                                    },
                                    MapMiniRoot,
                                ))
                                .with_children(|mini| {
                                    for y in 0..3 {
                                        mini.spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(33.33),
                                                flex_direction: FlexDirection::Row,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::NONE),
                                            ..default()
                                        })
                                        .with_children(
                                            move |row_node| {
                                                for x in 0..3 {
                                                    row_node.spawn((
                                                        NodeBundle {
                                                            style: Style {
                                                                width: Val::Percent(33.33),
                                                                height: Val::Percent(100.0),
                                                                margin: UiRect::all(Val::Px(1.0)),
                                                                border: UiRect::all(Val::Px(3.0)),
                                                                ..default()
                                                            },
                                                            background_color: BackgroundColor(
                                                                Color::srgb(0.0, 0.4, 0.7),
                                                            ), // will be updated
                                                            border_color: BorderColor(Color::BLACK),
                                                            ..default()
                                                        },
                                                        MapMiniCell { x, y },
                                                    ));
                                                }
                                            },
                                        );
                                    }
                                });
                        });

                    // MESSAGES
                    sidebar
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(30.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.05, 0.15, 0.35)),
                            ..default()
                        })
                        .with_children(|msgs| {
                            msgs.spawn(TextBundle::from_section(
                                "Messages",
                                TextStyle {
                                    font: Default::default(),
                                    font_size: 22.0,
                                    color: Color::srgb(0.8, 0.9, 1.0),
                                },
                            ));
                            msgs.spawn((
                                TextBundle::from_section(
                                    "Simulation Started!",
                                    TextStyle {
                                        font: Default::default(),
                                        font_size: 18.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                MessageText,
                            ));
                        });

                    // TOOLS
                    sidebar
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(45.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(6.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.06, 0.18, 0.28)),
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

                            let spawn_tool =
                                |label: &str, kind: ToolKind, parent: &mut ChildBuilder| {
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
                            spawn_tool("Coral gardening", ToolKind::CoralGardening, tools);
                            spawn_tool("Micro-fragmentation", ToolKind::MicroFragmentation, tools);
                            spawn_tool("Removing pollution", ToolKind::RemovingPollution, tools);
                        });
                });
            });

            // BOTTOM STATS BAR
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.02, 0.05, 0.12)),
                ..default()
            })
            .with_children(|stats_node| {
                stats_node.spawn((
                    TextBundle::from_section(
                        "Water pH: 0 | Temp: 0 | Coral: 0 | Algae: 0 | Turn 0",
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

    // Start the first "beginning" problem cycle
    timer.0.unpause();
}

// ─────────────────────────────────────────────
//             GAME OVER SCREEN SETUP
// ─────────────────────────────────────────────
fn setup_game_over(mut commands: Commands, nodes: Query<Entity, With<Node>>) {
    // Clear any existing UI nodes
    for entity in &nodes {
        commands.entity(entity).despawn_recursive();
    }

    // Reuse cameras that already exist; just spawn new UI
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.05, 0.0, 0.1)),
            ..default()
        })
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "GAME OVER\nThe reef has collapsed.",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));

            root.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(220.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.3, 0.9, 0.4)),
                    ..default()
                },
                RestartButton,
            ))
            .with_children(|b| {
                b.spawn(TextBundle::from_section(
                    "RESTART",
                    TextStyle {
                        font: Default::default(),
                        font_size: 28.,
                        color: Color::BLACK,
                    },
                ));
            });

            root.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(220.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.9, 0.3, 0.3)),
                    ..default()
                },
                QuitButton,
            ))
            .with_children(|b| {
                b.spawn(TextBundle::from_section(
                    "QUIT",
                    TextStyle {
                        font: Default::default(),
                        font_size: 28.,
                        color: Color::WHITE,
                    },
                ));
            });
        });
}

// ─────────────────────────────────────────────
//             RESTART BUTTON LOGIC
// ─────────────────────────────────────────────
fn restart_button_system(
    mut interaction_q: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    nodes: Query<Entity, With<Node>>,
) {
    for interaction in &mut interaction_q {
        if *interaction == Interaction::Pressed {
            // Clear Game Over UI
            for entity in &nodes {
                commands.entity(entity).despawn_recursive();
            }
            // Go back to InGame, which will reset stats and UI
            next_state.set(AppState::InGame);
        }
    }
}

// ─────────────────────────────────────────────
//             QUIT BUTTON LOGIC
// ─────────────────────────────────────────────
fn quit_button_system(
    mut interaction_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color) in &mut interaction_q {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.6, 0.2, 0.2));
                exit.send(AppExit::Success);
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

// ─────────────────────────────────────────────
//              TOOLS LOGIC
// ─────────────────────────────────────────────
fn tool_button_system(
    mut interaction_q: Query<
        (&Interaction, &mut BackgroundColor, &ToolButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut stats: ResMut<ReefStats>,
    mut state: ResMut<GameState>,
    mut timer: ResMut<ProblemTimer>,
    mut map_state: ResMut<MapState>,
) {
    for (interaction, mut color, tool) in &mut interaction_q {
        match *interaction {
            Interaction::Pressed => {
                apply_tool(tool.kind, &mut stats, &mut state, &mut map_state);

                // After using a tool, schedule exactly one new problem
                timer.0.reset();
                timer.0.unpause();

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

fn apply_tool(
    kind: ToolKind,
    stats: &mut ReefStats,
    state: &mut GameState,
    map_state: &mut MapState,
) {
    state.message = match kind {
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
    }
    .into();

    clamp_stats(stats);
    sync_active_cell(stats, map_state);
}

// ─────────────────────────────────────────────
//              RANDOM PROBLEMS
// ─────────────────────────────────────────────
fn problem_timer_system(
    time: Res<Time>,
    mut timer: ResMut<ProblemTimer>,
    mut stats: ResMut<ReefStats>,
    mut state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut map_state: ResMut<MapState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let p = rng.gen_range(1..=5);

        state.message = match p {
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
        }
        .to_string();

        clamp_stats(&mut stats);
        state.turn += 1;

        // Sync map cell with new stats
        sync_active_cell(&stats, &mut map_state);

        // Stop this cycle (we only want one problem per timer run)
        timer.0.pause();

        // Check for game over (coral = 0%)
        if stats.coral <= 0 {
            stats.coral = 0;
            state.message = " The reef has collapsed. Game over.".into();
            next_state.set(AppState::GameOver);
        }
    }
}

// ─────────────────────────────────────────────
//              CLAMP & SYNC LOGIC
// ─────────────────────────────────────────────
fn clamp_stats(stats: &mut ReefStats) {
    stats.coral = stats.coral.clamp(0, 100);
    stats.algae = stats.algae.clamp(0, 100);
    stats.ph = stats.ph.clamp(0.0, 14.0);
    stats.temp = stats.temp.clamp(0.0, 40.0);
}

fn sync_active_cell(stats: &ReefStats, map_state: &mut MapState) {
    let cell = &mut map_state.cells[map_state.active_y][map_state.active_x];
    cell.coral = stats.coral;
    cell.algae = stats.algae;
    cell.ph = stats.ph;
    cell.temp = stats.temp;
}

// ─────────────────────────────────────────────
//            UPDATE CORAL DISPLAY
// ─────────────────────────────────────────────
fn update_coral_display(
    stats: Res<ReefStats>,
    reef_query: Query<Entity, With<ReefView>>,
    mut coral_query: Query<(Entity, &Coral, &mut Style)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !stats.is_changed() {
        return;
    }

    let target = stats.coral.clamp(0, 100) as usize;

    let Ok(reef) = reef_query.get_single() else {
        return; // no reef exists right now → do nothing
    };

    // Extract existing coral data (entity, x, y)
    let current: Vec<(Entity, f32, f32)> = coral_query
        .iter_mut()
        .map(|(e, coral, _style)| (e, coral.x, coral.y))
        .collect();

    let current_count = current.len();

    // -------------------------------
    // CASE 1: Need to REMOVE corals
    // -------------------------------
    if current_count > target {
        let to_remove = current_count - target;
        for (entity, _, _) in current.into_iter().take(to_remove) {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    // -------------------------------
    // CASE 2: Need to ADD corals
    // -------------------------------
    if current_count < target {
        let to_add = target - current_count;
        let texture = asset_server.load("coral-transparent.png");
        let mut rng = rand::thread_rng();

        commands.entity(reef).with_children(|reef| {
            for _ in 0..to_add {
                let x = rng.gen_range(0.0..92.0);
                let y = rng.gen_range(0.0..92.0);

                reef.spawn((
                    ImageBundle {
                        style: Style {
                            width: Val::Px(38.0),
                            height: Val::Px(38.0),
                            position_type: PositionType::Absolute,
                            left: Val::Percent(x),
                            bottom: Val::Percent(y),
                            ..default()
                        },
                        image: UiImage::new(texture.clone()),
                        background_color: BackgroundColor(Color::NONE),
                        ..default()
                    },
                    Coral { x, y },
                ));
            }
        });
    }

    // -------------------------------
    // CORALS STAY IN SAME POSITIONS
    // -------------------------------
    for (_, coral, mut style) in coral_query.iter_mut() {
        style.left = Val::Percent(coral.x);
        style.bottom = Val::Percent(coral.y);
    }
}

// ─────────────────────────────────────────────
//             UPDATE STATS TEXT
// ─────────────────────────────────────────────
fn update_stats_ui_system(
    state: Res<GameState>,
    stats: Res<ReefStats>,
    mut sets: ParamSet<(
        Query<&mut Text, With<StatsText>>,
        Query<&mut Text, With<MessageText>>,
    )>,
) {
    if let Ok(mut txt) = sets.p0().get_single_mut() {
        txt.sections[0].value = format!(
            "Water pH: {:.2} | Temp: {:.1}°C | Coral: {}% | Algae: {}% | Turn {}",
            stats.ph, stats.temp, stats.coral, stats.algae, state.turn
        );
    }

    if let Ok(mut msg) = sets.p1().get_single_mut() {
        msg.sections[0].value = state.message.clone();
    }
}

// ─────────────────────────────────────────────
//                MAP COLOR HELPER
// ─────────────────────────────────────────────
fn color_for_cell(map: &MapState, x: usize, y: usize) -> Color {
    let stats = map.cells[y][x];
    if stats.coral >= 60 {
        // healthy = orange
        Color::srgb(1.0, 0.45, 0.25)
    } else if stats.coral <= 20 {
        // unhealthy = white
        Color::WHITE
    } else {
        // typical = sea blue
        Color::srgb(0.0, 0.4, 0.7)
    }
}

// ─────────────────────────────────────────────
//             UPDATE MAP COLORS
// ─────────────────────────────────────────────
fn update_map_colors_system(
    map_state: Res<MapState>,
    mut sets: ParamSet<(
        Query<(&MapMiniCell, &mut BackgroundColor)>,
        Query<(&MapOverlayCell, &mut BackgroundColor)>,
    )>,
) {
    // Mini-map colors
    for (cell, mut color) in sets.p0().iter_mut() {
        *color = BackgroundColor(color_for_cell(&map_state, cell.x, cell.y));
    }

    // Overlay colors
    for (cell, mut color) in sets.p1().iter_mut() {
        *color = BackgroundColor(color_for_cell(&map_state, cell.x, cell.y));
    }
}

// ─────────────────────────────────────────────
//             MAP EXPAND / CLOSE / CLICK
// ─────────────────────────────────────────────
fn map_expand_system(
    mut interaction_q: Query<&Interaction, (Changed<Interaction>, With<MapMiniRoot>)>,
    overlay_q: Query<Entity, With<MapOverlayRoot>>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_q {
        if *interaction == Interaction::Pressed {
            // Only spawn if there isn't already an overlay
            if overlay_q.is_empty() {
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                            ..default()
                        },
                        MapOverlayRoot,
                    ))
                    .with_children(|root| {
                        // Close button in top-left
                        root.spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Px(10.0),
                                top: Val::Px(10.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::NONE),
                            ..default()
                        })
                        .with_children(|close_container| {
                            close_container
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(32.0),
                                            height: Val::Px(32.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::srgb(
                                            0.9, 0.3, 0.3,
                                        )),
                                        ..default()
                                    },
                                    MapCloseButton,
                                ))
                                .with_children(|b| {
                                    b.spawn(TextBundle::from_section(
                                        "X",
                                        TextStyle {
                                            font: Default::default(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                        });

                        // Big 3x3 grid
                        root.spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(300.0),
                                height: Val::Px(300.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.02, 0.05, 0.12)),
                            ..default()
                        })
                        .with_children(|grid| {
                            for y in 0..3 {
                                grid.spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(33.33),
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::NONE),
                                    ..default()
                                })
                                .with_children(move |row_node| {
                                    for x in 0..3 {
                                        row_node.spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Percent(33.33),
                                                    height: Val::Percent(100.0),
                                                    margin: UiRect::all(Val::Px(2.0)),
                                                    border: UiRect::all(Val::Px(3.0)),
                                                    ..default()
                                                },
                                                background_color: BackgroundColor(Color::srgb(
                                                    0.0, 0.4, 0.7,
                                                )), // updated by system
                                                border_color: BorderColor(Color::BLACK),
                                                ..default()
                                            },
                                            MapOverlayCell { x, y },
                                        ));
                                    }
                                });
                            }
                        });
                    });
            }
        }
    }
}

fn map_close_system(
    mut interaction_q: Query<&Interaction, (Changed<Interaction>, With<MapCloseButton>)>,
    overlay_q: Query<Entity, With<MapOverlayRoot>>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_q {
        if *interaction == Interaction::Pressed {
            for e in &overlay_q {
                commands.entity(e).despawn_recursive();
            }
        }
    }
}

fn map_overlay_cell_system(
    mut interaction_q: Query<(&Interaction, &MapOverlayCell), Changed<Interaction>>,
    mut map_state: ResMut<MapState>,
    mut stats: ResMut<ReefStats>,
) {
    for (interaction, cell) in &mut interaction_q {
        if *interaction == Interaction::Pressed {
            // Store old indices first
            let old_x = map_state.active_x;
            let old_y = map_state.active_y;

            // Save current stats into old active cell (scoped borrow)
            {
                let old_cell = &mut map_state.cells[old_y][old_x];
                old_cell.coral = stats.coral;
                old_cell.algae = stats.algae;
                old_cell.ph = stats.ph;
                old_cell.temp = stats.temp;
            }

            // Switch active cell
            map_state.active_x = cell.x;
            map_state.active_y = cell.y;

            // Load new cell stats into ReefStats
            let new_x = map_state.active_x;
            let new_y = map_state.active_y;
            let new_stats = map_state.cells[new_y][new_x];
            stats.coral = new_stats.coral;
            stats.algae = new_stats.algae;
            stats.ph = new_stats.ph;
            stats.temp = new_stats.temp;
        }
    }
}

// ─────────────────────────────────────────────
//                 UNIT TESTS
// ─────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------
    // TEST 1 — clamp_stats() keeps values in range
    // -----------------------------------------
    #[test]
    fn test_clamp_stats() {
        let mut stats = ReefStats {
            coral: 150, // too high
            algae: -20, // too low
            ph: 20.0,   // too high
            temp: -5.0, // too low
        };

        clamp_stats(&mut stats);

        assert_eq!(stats.coral, 100);
        assert_eq!(stats.algae, 0);
        assert_eq!(stats.ph, 14.0);
        assert_eq!(stats.temp, 0.0);
    }

    // -----------------------------------------
    // TEST 2 — sync_active_cell() writes ReefStats into MapState
    // -----------------------------------------
    #[test]
    fn test_sync_active_cell() {
        let mut map_state = MapState::new();
        map_state.active_x = 1;
        map_state.active_y = 2;

        let stats = ReefStats {
            coral: 55,
            algae: 12,
            ph: 8.3,
            temp: 27.5,
        };

        sync_active_cell(&stats, &mut map_state);

        let cell = map_state.cells[2][1];
        assert_eq!(cell.coral, 55);
        assert_eq!(cell.algae, 12);
        assert!((cell.ph - 8.3).abs() < 0.0001);
        assert!((cell.temp - 27.5).abs() < 0.0001);
    }

    // -----------------------------------------
    // TEST 3 — color_for_cell() returns correct colors
    // -----------------------------------------
    #[test]
    fn test_color_for_cell() {
        let mut map = MapState::new();

        // healthy cell
        map.cells[0][0].coral = 75;
        let c = color_for_cell(&map, 0, 0);
        assert_eq!(c, Color::srgb(1.0, 0.45, 0.25)); // orange

        // unhealthy cell
        map.cells[1][1].coral = 10;
        let c2 = color_for_cell(&map, 1, 1);
        assert_eq!(c2, Color::WHITE);

        // medium cell (default blue)
        map.cells[2][2].coral = 40;
        let c3 = color_for_cell(&map, 2, 2);
        assert_eq!(c3, Color::srgb(0.0, 0.4, 0.7));
    }
}
