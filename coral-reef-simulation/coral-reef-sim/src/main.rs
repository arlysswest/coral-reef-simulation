//CURRENT PROGRESS ON VISUAL VERSION

use bevy::prelude::*;
use std::time::Duration;
use std::io;

#[derive(Resource)]
struct ProblemTimer(Timer);

#[derive(Resource)]
struct GameState {
    score: i32,
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

// Marker components for UI text
#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct MessageText;

//Main Logic
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
        .insert_resource(ProblemTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .insert_resource(GameState {
            score: 0,
            turn: 0,
            message: "Welcome to Coral Reef Sim!".to_string(),
        })
        .insert_resource(ReefStats {
            coral: 10,
            algae: 5,
            ph: 8.1,
            temp: 27.5,
        })
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (button_system, problem_timer_system, update_stats_ui_system))
        .run();
}

// UI setup
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.1, 0.2, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            // Start Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.),
                            height: Val::Px(60.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(10.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.9, 0.8, 0.2)),
                        ..default()
                    },
                    Name::new("StartButton"),
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: Default::default(),
                            font_size: 36.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            // Stats Text
            parent.spawn((
                TextBundle::from_section(
                    "Stats: Coral = 10, Algae = 5, pH = 8.1, Temp = 27.5°C",
                    TextStyle {
                        font: Default::default(),
                        font_size: 36.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                }),
                StatsText,
            ));

            // Message Text
            parent.spawn((
                TextBundle::from_section(
                    "Welcome to Coral Reef Sim!",
                    TextStyle {
                        font: Default::default(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                }),
                MessageText,
            ));
        });
}

//Button logic
fn button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<GameState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.score += 1;
                state.message = format!("You helped the reef! Score = {}", state.score);
                *color = BackgroundColor(Color::srgb(0.2, 0.8, 0.2));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.9, 0.9, 0.5));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.9, 0.8, 0.2));
            }
        }
    }
}

// Problem timer system
fn problem_timer_system(
    time: Res<Time>,
    mut timer: ResMut<ProblemTimer>,
    mut stats: ResMut<ReefStats>,
    mut state: ResMut<GameState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        stats.coral -= 1;
        stats.algae += 1;
        state.turn += 1;
        state.message = format!(
            "Turn {} complete! Coral: {}, Algae: {}, pH: {:.2}",
            state.turn, stats.coral, stats.algae, stats.ph
        );
    }
}

// Update UI text each frame
fn update_stats_ui_system(
        state: Res<GameState>,
        stats: Res<ReefStats>,
        mut stats_text: Query<&mut Text, (With<StatsText>, Without<MessageText>)>,
        mut message_text: Query<&mut Text, (With<MessageText>, Without<StatsText>)>,
    
) {
    if let Ok(mut text) = stats_text.get_single_mut() {
        text.sections[0].value = format!(
            "Turn {} | Coral: {} | Algae: {} | pH: {:.2} | Temp: {:.1}°C",
            state.turn, stats.coral, stats.algae, stats.ph, stats.temp
        );
    }

    if let Ok(mut msg) = message_text.get_single_mut() {
        msg.sections[0].value = state.message.clone();
    }
}

//unneeded?
/*
fn print_stats(coral: i32, algae: i32, temp: f32, ph: f32) {
    println!(" Coral cover: {}%", coral);
    println!(" Algae cover: {}%", algae);
    println!(" Water temp : {:.1} °C", temp);
    println!(" pH         : {:.2}", ph);
}
*/

//KEEP THIS FUNCTION
/*
fn apply_tool(choice: u32, coral: &mut i32, algae: &mut i32, ph: &mut f32) {
    match choice {
        1 => { *coral += 5; println!("Applied Artificial substrates / 3D printed modules!"); }
        2 => { *coral += 4; *algae -= 2; println!("Applied Coral gardening!"); }
        3 => { *coral += 6; *algae -= 3; println!("Applied Micro-fragmentation!"); }
        4 => { *coral += 3; *algae -= 3; *ph += 0.05; println!("Applied Removing pollution!"); }
        _ => println!("Invalid tool."),
    }
    if *coral > 100 { *coral = 100; }
    if *algae < 0 { *algae = 0; }
}
*/

/*
//KEEP THIS FUNCTION
fn apply_problem(problem: u32, coral: &mut i32, algae: &mut i32, temp: &mut f32, ph: &mut f32) {
    match problem {
        1 => { println!("Pollution!");                  *coral -= 4; *algae += 5; *ph -= 0.05; }
        2 => { println!("Invasive species!");           *coral -= 5; *algae += 4; }
        3 => { println!("CO2 emissions rising!");       *coral -= 3; *algae += 3; *temp += 1.0; *ph -= 0.03; }
        4 => { println!("physical damage from storm!"); *coral -= 6; *algae += 2; }
        5 => { println!("Overfishing!");                *coral -= 4; *algae += 3; }
        _ => {}
    }
    if *coral < 0 { *coral = 0; }
    if *algae > 100 { *algae = 100; }
}
*/

/*
//KEEP THIS FUNCTION
fn more_info() {
    loop {
        println!("Do you want more information about a problem or a tool?");
        println!("1: I want more information about a problem");
        println!("2: I want more information about a tool");
        println!("3: What statistics should the reef have");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read");

        let option: u32 = match option.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input, please enter 1, 2 or 3.\n");
                continue;
            }
        };

        match option {
            1 => {
                println!("\n--- PROBLEMS ---");

                println!("POLLUTION:");
                println!("WHAT THIS DOES TO THE CORAL REEF: Pollution from runoff, plastics, and chemicals clouds the water and blocks sunlight, reduces oxygen, and poisons coral organisms. It also encourages algae growth that competes with coral.");
                println!("AFFECTS ON STATISTICS: Coral cover: -5%, algea cover: +5%, ph: -0.05, temperature = no change/n");
                
                println!("INVASIVE SPECIES:");
                println!("WHAT THIS DOES TO THE CORAL REEF: Non-native species, like crown-of-thorns starfish or lionfish, eat coral or outcompete local fish that keep algae in check, destabilizing the reef ecosystem.");
                println!("AFFECTS ON STATISTICS: Coral cover: -5%, algea cover: +4%, ph = no change, temperature = no change/n");
                
                println!("CO2 EMISSIONS RISING:");
                println!("WHAT THIS DOES TO THE CORAL REEF: Rising CO2 causes ocean acidification (lowering pH) and warmer waters, leading to coral bleaching and slower skeleton growth.");
                println!("AFFECTS ON STATISTICS: Coral cover: -3%, Algae cover: +3%, pH: -0.03, Temperature: +1.0°C\n");
                
                println!("PHYSICAL DAMAGE FROM STORM:");
                println!("WHAT THIS DOES TO THE CORAL REEF: Strong waves from storms break coral structures and scatter fragments. Recovery takes years without restoration help.");
                println!("AFFECTS ON STATISTICS: Coral cover: -6%, Algae cover: +2%, pH: no change, Temperature: no change\n ");

                println!("OVERFISHING:");
                println!("WHAT THIS DOES TO THE CORAL REEF: Removing too many fish—especially herbivores—causes algae to grow unchecked, reducing coral health and biodiversity.");
                println!("AFFECTS ON STATISTICS: Coral cover: -4%, Algae cover: +3%, pH: no change, Temperature: no change\n");
            }
            2 => {
                println!("\n--- TOOLS / RESTORATION METHODS ---");

                println!("ARTICIFIAL SUBSTRATES/3D PRINTED MODEL:");
                println!("WHAT THIS IS: Man-made reef structures designed to provide stable surfaces for coral larvae to attach to and grow on.");
                println!("HOW THIS HELPS THE REEF: Replaces lost habitat and encourages new coral growth by giving larvae safe, suitable surfaces. ");
                println!("AFFECTS ON STATISTICS: Coral cover: +5%, Algae cover: no change, pH: no change\n/n");

                println!("CORAL GARDENING:");
                println!("WHAT THIS IS: Growing coral fragments in underwater nurseries and replanting them onto damaged reefs.");
                println!("HOW THIS HELPS THE REEF: Boosts coral recovery and biodiversity by restoring live coral cover faster.");
                println!("AFFECTS ON STATISTICS: Coral cover: +4%, Algae cover: -2%, pH: no change\n");

                println!("MICRO-FRAGMENTATION:");
                println!("WHAT THIS IS: Cutting coral into small fragments to speed up their healing and growth once reattached to the reef.");
                println!("HOW THIS HELPS THE REEF: Rapidly increases coral cover and allows corals to fuse together, forming larger colonies faster.");
                println!("AFFECTS ON STATISTICS: Coral cover: +6%, Algae cover: -3%, pH: no change\n");


                println!("REMOVING POLUTION");
                println!("WHAT THIS IS: Physically cleaning debris and reducing pollutants in the reef area.");
                println!("HOW THIS HELPS THE REEF: Improves water quality, increases light penetration, and helps coral recover.");
                println!("AFFECTS ON STATISTICS: Coral cover: +3%, Algae cover: -3%, pH: +0.05\n");
            }
            3 => {
                println!("\n--- REEF STATISTICS ---");

                println!("UNHEALTHY REEF:");
                println!(" Coral cover: below 20%");
                println!(" Algae cover: above 60%");
                println!(" pH: below 7.8");
                println!(" Temperature: above 30°C");
                println!(" Condition: Reef struggling, low biodiversity, frequent bleaching events.\n");

                println!("HEALTHY REEF:");
                println!(" Coral cover: 40–60%");
                println!(" Algae cover: 10–25%");
                println!(" pH: 8.0–8.2");
                println!(" Temperature: 26–28°C");
                println!(" Condition: Balanced ecosystem with strong coral growth and fish diversity.\n");

                println!("GAME OVER:");
                println!(" Coral cover: 0%");
                println!(" Algae cover: 100%");
                println!(" pH: below 7.7");
                println!(" Temperature: above 32°C");
                println!(" Condition: Coral death, ecosystem collapse, no recovery possible without intervention.\n");
                
            }
            _ => {
                println!("Invalid input, please enter 1, 2 or 3.\n");
                continue;
            }
           
        }
        break;//exit the loop

        

    }
}
*/