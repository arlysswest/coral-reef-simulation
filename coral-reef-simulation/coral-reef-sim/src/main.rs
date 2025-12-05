//CHANGES TO ADD

// FINSIHED !!
// 1. Game over = coral cover = 0%
// redirects to 2 options: either quit or restart -> done
// 2. problems should only occur once after each tool is applied and once at the beggining -> done

//NEEDS IMROVEMENT:
// 3. work on improving coral reef display - right now its just color blocks I wnat it to display actual corals 

//WHAT TO FOCUS ON LATER:
// 1. add map feature
// 2. get rid of warnings !!
// 3. finsh README.md

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

#[derive(Resource)]
struct ReefStats {
    coral: i32,
    algae: i32,
    ph: f32,
    temp: f32,
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
struct CoralSprite;

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
        .add_systems(Startup, setup_start_screen)
        .add_systems(Update, start_button_system.run_if(in_state(AppState::StartMenu)))
        .add_systems(OnEnter(AppState::InGame), setup_game_ui)
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(
            Update,
            (
                tool_button_system,
                problem_timer_system,
                update_stats_ui_system,
                update_coral_display,
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

    commands
        .spawn(NodeBundle {
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
        })
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "Help restore the coral reef!",
                TextStyle {
                    font: Default::default(),
                    font_size: 42.0,
                    color: Color::BLACK,
                },
            ));

            root
                .spawn((
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
    root_nodes: Query<Entity, With<Node>>,
) {
    for interaction in &mut interaction_q {
        if *interaction == Interaction::Pressed {
            // Clear start screen UI
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
    _asset_server: Res<AssetServer>,
    mut stats: ResMut<ReefStats>,
    mut state: ResMut<GameState>,
) {
    // Reset stats and state each time we enter InGame
    stats.coral = 35;
    stats.algae = 10;
    stats.ph = 8.1;
    stats.temp = 27.0;

    state.turn = 0;
    state.message = "Simulation Started! Use tools to restore the reef.".into();

    timer.0.reset();
    timer.0.pause();

    commands.spawn(Camera2dBundle::default());

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
            root
                .spawn(NodeBundle {
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
                    row
                        .spawn((
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
                    row
                        .spawn(NodeBundle {
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
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgb(
                                        0.05, 0.2, 0.25,
                                    )),
                                    ..default()
                                })
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

                            // MESSAGES
                            sidebar
                                .spawn(NodeBundle {
                                    style: Style {
                                        height: Val::Percent(30.0),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgb(
                                        0.05, 0.15, 0.35,
                                    )),
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
                                                        background_color:
                                                            BackgroundColor(Color::srgb(
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

            root
                .spawn((
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

            root
                .spawn((
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
) {
    for (interaction, mut color, tool) in &mut interaction_q {
        match *interaction {
            Interaction::Pressed => {
                apply_tool(tool.kind, &mut stats, &mut state);

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

fn apply_tool(kind: ToolKind, stats: &mut ReefStats, state: &mut GameState) {
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
//              CLAMP STATS LOGIC
// ─────────────────────────────────────────────
fn clamp_stats(stats: &mut ReefStats) {
    stats.coral = stats.coral.clamp(0, 100);
    stats.algae = stats.algae.clamp(0, 100);
    stats.ph = stats.ph.clamp(0.0, 14.0);
    stats.temp = stats.temp.clamp(0.0, 40.0);
}

// ─────────────────────────────────────────────
//            UPDATE CORAL DISPLAY
// ─────────────────────────────────────────────
fn update_coral_display(
    stats: Res<ReefStats>,
    reef_query: Query<Entity, With<ReefView>>,
    coral_query: Query<Entity, With<CoralSprite>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !stats.is_changed() {
        return;
    }

    // Remove old corals
    for coral in coral_query.iter() {
        commands.entity(coral).despawn_recursive();
    }

    // Number of healthy corals = coral cover %
    let healthy = stats.coral.clamp(0, 100);
    let texture = asset_server.load("coral-transparent.png");

    let reef_entity = reef_query.single();
    let mut rng = rand::thread_rng();

    commands.entity(reef_entity).with_children(|reef| {
        for _ in 0..healthy {
            let left = rng.gen_range(0.0..92.0);
            let bottom = rng.gen_range(0.0..92.0);

            reef.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(38.0),
                        height: Val::Px(38.0),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(left),
                        bottom: Val::Percent(bottom),
                        ..default()
                    },
                    image: UiImage::new(texture.clone()),
                    // Let the sprite's transparency show through
                    background_color: BackgroundColor(Color::NONE),
                    ..default()
                },
                CoralSprite,
            ));
        }
    });
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
