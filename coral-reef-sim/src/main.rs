//NOTES WAYS TO IMPROVE
// 1. make each different problem affect the stats differently in a way that makes sense for each indiidual problem
// 2. make each different tool affect the stats differently
// 3. have different problems occur randomly
// 4. use a loop that looks like:
        //  1. show starting stats
        // 2. ask user what tool to use
        // 3. user selects tool
        // 4. tool updates stats
        // 5. output updated stats
        // 6. output congradulaatpry message
        // 7. have a random problem occur
        // 8. output a message abot the problem
        // 9. have the problem affect the statistics
        // 10. repeat steps 2-9
// eventually add visuals (second least important)
// add map feature (least important)

//PROBLEMS:
//Pollution 
    //coral cover: dec
    //algea cover: inc
    //temp (c): no change
    //ph: dec

//Invasive species
    //coral cover: dec
    //algea cover: inc
    //temp (c): no change
    //ph: no change

//Co2 emissions
    //coral cover: dec
    //algea cover: inc
    //temp (c): inc
    //ph: dec

//Storm or physical damage  - decrease coral cover
    //coral cover: dec
    //algea cover: inc
    //temp (c): no change
    //ph: no change

//Over fishing
    //coral cover: dec
    //algea cover: inc
    //temp (c): no change
    //ph: no change

//TOOLS:
//artificial substrates / 3D printed modules
    //coral cover: inc
    //algea cover: no change
    //temp (c): no change
    //ph: no change

//coral gardening
    //coral cover: inc
    //algea cover: dec
    //temp (c): no change
    //ph: no change

//Micro-fragmentation
    //coral cover: inc
    //algea cover: dec
    //temp (c): no change
    //ph: no change

//Removing pollution
    //coral cover: inc
    //algea cover: dec
    //temp (c): no change
    //ph: inc


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


