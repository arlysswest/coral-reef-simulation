//PROGRAM IS CURRENTLY MEETING BASIC EXPECTATIONS

//WAYS IT COULD BE IMPROVED
// 1. make a visual version
// 2. add a map function in the visual version
// 3. add a more info option to explain why things are occuring and what the tools are
// 4. add pauses so that less text comes out at once making it more digestable
// 5. need to add additional documentation

use rand::Rng;
use std::io;

fn main() {
    let coral: i32 = 35;
    let algae: i32 = 10;
    let temp: f32 = 27.0;
    let ph: f32 = 8.1;

    println!("WELCOME TO THE CORAL REEF SIMULATION!\n");

    simulate_turn(1, 20, coral, algae, temp, ph);
}

fn simulate_turn(
    turn: u32,
    max_turns: u32,
    mut coral: i32,
    mut algae: i32,
    mut temp: f32,
    mut ph: f32,
) {
    println!("\n=== TURN {} ===", turn);
    print_stats(coral, algae, temp, ph);

    println!("\nWould you like to use a tool or quit?");
    println!("0: Quit");
    println!("1: Artificial substrates / 3D printed modules");
    println!("2: Coral gardening");
    println!("3: Micro-fragmentation");
    println!("4: Removing pollution");
    println!("5: I want more information");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read");
    let choice: u32 = match choice.trim().parse() {
        Ok(n) => n,
        Err(_) => { println!("Invalid input"); return simulate_turn(turn, max_turns, coral, algae, temp, ph); }
    };

    if choice == 0 {
        println!("Simulation ended by user.");
        return;
    }

    if choice == 5 {
        more_info();
        return simulate_turn(turn, max_turns, coral, algae, temp, ph);
    }

    apply_tool(choice, &mut coral, &mut algae, &mut ph);
    println!("\nAFTER TOOL:");
    print_stats(coral, algae, temp, ph);
    println!("Great job! You helped the reef.");

    let problem = rand::thread_rng().gen_range(1..=5);
    println!("\nA problem has occurred!");
    apply_problem(problem, &mut coral, &mut algae, &mut temp, &mut ph);

    println!("\nAFTER PROBLEM:");
    print_stats(coral, algae, temp, ph);

    if coral <= 0 {
        println!("\nThe coral reef has collapsed. GAME OVER.");
        return;
    }
    if algae >= 100 {
        println!("\nThe reef is overrun by algae. GAME OVER.");
        return;
    }
    if turn >= max_turns {
        println!("\nMaximum turns reached. Simulation ended.");
        return;
    }

    // *** THE RECURSION ***
    simulate_turn(turn + 1, max_turns, coral, algae, temp, ph);
}


fn print_stats(coral: i32, algae: i32, temp: f32, ph: f32) {
    println!(" Coral cover: {}%", coral);
    println!(" Algae cover: {}%", algae);
    println!(" Water temp : {:.1} °C", temp);
    println!(" pH         : {:.2}", ph);
}

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
            break; //exit the loop
        }

        

    }
}
