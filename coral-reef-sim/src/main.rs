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
    let mut coral: i32 = 35;
    let mut algae: i32 = 10;
    let mut temp: f32 = 27.0;
    let mut ph: f32 = 8.1;

    println!("WELCOME TO THE CORAL REEF SIMULATION!\n");

    let mut turn = 1;
    let max_turns = 20;

    loop {
        println!("\n=== TURN {} ===", turn);

        // (1) Display current stats
        print_stats(coral, algae, temp, ph);

        // (2) Ask user if they want to apply tool or quit
        println!("\nWould you like to use a tool or quit?");
        println!("0: Quit");
        println!("1: Artificial substrates / 3D printed modules");
        println!("2: Coral gardening");
        println!("3: Micro-fragmentation");
        println!("4: Removing pollution");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        let choice: u32 = match choice.trim().parse() {
            Ok(n) => n,
            Err(_) => { println!("Invalid input"); continue; }
        };

        // (3) Quit path
        if choice == 0 {
            println!("Simulation ended by user.");
            break;
        }

        // (4) apply tool
        apply_tool(choice, &mut coral, &mut algae, &mut ph);
        println!("\nAFTER TOOL:");
        print_stats(coral, algae, temp, ph);
        println!("Great job! You helped the reef.");

        // (5) random problem
        let problem = rand::thread_rng().gen_range(1..=5);
        println!("\nA problem has occurred!");
        apply_problem(problem, &mut coral, &mut algae, &mut temp, &mut ph);

        println!("\nAFTER PROBLEM:");
        print_stats(coral, algae, temp, ph);

        // (6) check end
        if coral <= 0 {
            println!("\nThe coral reef has collapsed. GAME OVER.");
            break;
        }
        if algae >= 100 {
            println!("\nThe reef is overrun by algae. GAME OVER.");
            break;
        }
        if turn >= max_turns {
            println!("\nMaximum turns reached. Simulation ended.");
            break;
        }

        turn += 1;
    }
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
