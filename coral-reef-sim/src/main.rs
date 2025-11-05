fn main() {
    let mut coral: i32 = 35;
    let mut algae: i32 = 10;
    let mut temp: i32 = 27;
    let mut ph: f32 = 8.1;

    println!("INITIAL STATS");
    print_stats(coral, algae, temp, ph);

    // apply 1 tool
    apply_tool(&mut coral, &mut algae, &mut ph);
    println!("\nAFTER 1 TOOL:");
    print_stats(coral, algae, temp, ph);

    // apply 1 problem
    apply_problem(&mut coral, &mut algae, &mut temp, &mut ph);
    println!("\nAFTER 1 PROBLEM:");
    print_stats(coral, algae, temp, ph);
}

fn print_stats(coral: i32, algae: i32, temp: i32, ph: f32) {
    println!(" coral cover: {}%", coral);
    println!(" algae cover: {}%", algae);
    println!(" water temp : {} °C", temp);
    println!(" pH         : {:.2}", ph);
}

fn apply_tool(coral: &mut i32, algae: &mut i32, ph: &mut f32) {
    *coral += 2;
    *algae -= 2;
    *ph    += 0.03;

    if *algae < 0 { *algae = 0; }
    if *coral > 100 { *coral = 100; }
}

fn apply_problem(coral: &mut i32, algae: &mut i32, temp: &mut i32, ph: &mut f32) {
    *coral -= 3;
    *algae += 3;
    *temp  += 1;
    *ph    -= 0.05;

    if *coral < 0 { *coral = 0; }
    if *algae > 100 { *algae = 100; }
}

