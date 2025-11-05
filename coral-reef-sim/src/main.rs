// Change temp to a float instead of int
//make the loop follow desired patttern:
//Display current statistics
//Ask user if they would like to apply one of the tools or quit 
//-> if they choose to quit end the simulation
//-> if they choose a tool continue to next step
//Adjust statistics based off selected tool
//Output updates statistics
//Output a congratulatory message
//Have program select a random problem to occur
//Update statistics based off random problem
//Repeat steps 1-7


use rand::Rng; // Make sure Rng trait is in scope
use std::io;

fn main() {
    let mut coral: i32 = 35;
    let mut algae: i32 = 10;
    let mut temp: i32 = 27;
    let mut ph: f32 = 8.1;

    println!("WELCOME TO THE CORAL REEF SIMULATION!\n");
    print_stats(coral, algae, temp, ph);

    let mut turn = 1;
    let max_turns = 20;

    loop {
        println!("\n=== TURN {} ===", turn);

        // Ask user to pick a tool
        println!("\nChoose a tool to apply:");
        println!("1: Artificial substrates / 3D printed modules");
        println!("2: Coral gardening");
        println!("3: Micro-fragmentation");
        println!("4: Removing pollution");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Invalid input, try again."); 
                continue; 
            }
        };

        apply_tool(choice, &mut coral, &mut algae, &mut ph);
        println!("\nAFTER TOOL:");
        print_stats(coral, algae, temp, ph);
        println!("Great job! You helped the reef.");

        // Random problem occurs
        let problem = rand::thread_rng().gen_range(1..=5);
        match problem {
            1 => {
                println!("\nProblem: Pollution has occurred!");
                apply_problem(problem, &mut coral, &mut algae, &mut temp, &mut ph);
            }
            2 => {
                println!("\nProblem: Invasive species attack!");
                apply_problem(problem, &mut coral, &mut algae, &mut temp, &mut ph);
            }
            3 => {
                println!("\nProblem: CO2 emissions rising!");
                apply_problem(problem, &mut coral, &mut algae, &mut temp, &mut ph);
            }
            4 => {
                println!("\nProblem: Storm or physical damage!");
                apply_problem(problem, &mut coral, &mut algae, &mut temp, &mut ph);
            }
            5 => {
                println!("\nProblem: Overfishing detected!");
                apply_problem(problem, &mut coral, &mut algae, &mut temp, &mut ph);
            }
            _ => {}
        }

        println!("\nAFTER PROBLEM:");
        print_stats(coral, algae, temp, ph);

        // Check end conditions
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

// Print current stats
fn print_stats(coral: i32, algae: i32, temp: i32, ph: f32) {
    println!(" Coral cover: {}%", coral);
    println!(" Algae cover: {}%", algae);
    println!(" Water temp : {} °C", temp);
    println!(" pH         : {:.2}", ph);
}

// Apply a tool chosen by user
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

// Apply a random problem
fn apply_problem(problem: u32, coral: &mut i32, algae: &mut i32, temp: &mut i32, ph: &mut f32) {
    match problem {
        1 => { *coral -= 4; *algae += 5; *ph -= 0.05; } // Pollution
        2 => { *coral -= 5; *algae += 4; } // Invasive species
        3 => { *coral -= 3; *algae += 3; *temp += 1; *ph -= 0.03; } // CO2 emissions
        4 => { *coral -= 6; *algae += 2; } // Storm
        5 => { *coral -= 4; *algae += 3; } // Overfishing
        _ => {}
    }
    if *coral < 0 { *coral = 0; }
    if *algae > 100 { *algae = 100; }
}
