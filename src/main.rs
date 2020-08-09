// FOREWORD
// 
// Hi! I assume you have no idea what Rust is.
// Rust is a programming language, somewhat different from C++.
//
// Pointer access is inferred by the compiler.
// To guarantee this happens safely, there are "ownership" rules:
// * Only one variable can own a value
// * Ownership can be transferred "moved" by the owner
// * Immutable (Read-Only) references are allowed (using &)
// * Only one mutable (Read-Write) reference can exist at a time
// 
// Allocated memory gets freed automatically.
// However, this does not use a garbage collector.
// Instead, the compiler inserts the necessary code.
// This requires clear "lifetimes" of values.
// Lifetimes across threads can be very tricky.
//
// Because of several caveats regarding Inheritance,
// Rust does not have objects in the way C++ does.
// However, it has structs which can have methods attached.
// Also, structs can implement traits (shared method signatures).
//
// While C++ also allows for automated testing,
// Cargo (Rusts standard project manager) has it built-in.
// This project has several Unit Tests in the respective files.
// Further Integration Tests can be put into the "tests" dir.
// Note that the test modules only compile for test builds.
//
// There are more things to watch out for.
// Here is an online book to read into:
// <https://doc.rust-lang.org/book/>
// Working with Rust sure has been fun so far.
// Performance is roughly on-par with C++, with some variance.
//
// -LG

// Dependencies
use fhtw_roboter_wettsammeln;
use fhtw_roboter_wettsammeln::WorldMap;
use fhtw_roboter_wettsammeln::robots::{Robot, RandomBot, NearsightBot};
use std::io;
use std::error::Error;
use std::str::FromStr;
use std::thread;
use std::sync::Arc;
use std::time::SystemTime;

/// Fetches stdin and tries to parse it to type T.
fn get_user_input<T>(fancyname: &str) -> T
where T: FromStr {
    loop {
        let mut raw_input = String::new();
        
        println!("{}: ", fancyname);
        match io::stdin().read_line(&mut raw_input) {
            Err(_) => continue,
            _ => (),
        }
        
        match raw_input.trim().parse::<T>() {
            Ok(val) => return val,
            Err(_) => continue,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    //TODO restrict input to >0 somehow (custom type? closure arg?)
    let map_size: usize = get_user_input("Map Size");
    let max_field_score: usize = get_user_input("Max Field Score");
    let amount_randombots: usize = get_user_input("Amount RandomBots");
    let amount_nearsightbots: usize = get_user_input("Amount NearsightBots");

    let map = WorldMap::new(map_size, map_size);
    map.randomize_fields(max_field_score);

    // Map must be wrapped in an "atomic reference counter" (="arc") to be used across threads safely.
    let map = Arc::new(map);

    let mut randombots = Vec::with_capacity(amount_randombots);
    let mut nearsightbots = Vec::with_capacity(amount_randombots);

    for i in 0..amount_randombots {
        let mut new_robot = RandomBot::new(i, Arc::clone(&map));
        new_robot.randomize_position();
        randombots.push(new_robot);
    }
    for i in 0..amount_nearsightbots {
        let mut new_robot = NearsightBot::new(i, Arc::clone(&map));
        new_robot.randomize_position();
        nearsightbots.push(new_robot);
    }
    
    let mut threads = Vec::with_capacity(amount_randombots + amount_nearsightbots);
    let mut results = Vec::with_capacity(amount_randombots + amount_nearsightbots);

    // Start the benchmark timer!
    let start_time = SystemTime::now();

    // Due to Rusts ownership and lifetime rules, we cannot access the bot in the main thread and another one.
    // The new thread could pass the bot back when it joins, but we only really care about the score.
    for mut robot in randombots.pop() {
        let new_thread = thread::spawn( move || {
            robot.run();
            (robot.score(), robot.to_string())
        } );
        threads.push(new_thread);
    }
    for mut robot in nearsightbots.pop() {
        let new_thread = thread::spawn( move || {
            robot.run();
            (robot.score(), robot.to_string())
        } );
        threads.push(new_thread);
    }
    // Now that every thread has been spawned, we can wait for them to join back.
    for new_thread in threads {
        match new_thread.join() {
            Ok(thing) => results.push(thing),
            _ => continue
        }
    }

    match start_time.elapsed() {
        Ok(duration) => println!("The robots worked for {} ms.", duration.as_millis()),
        Err(_) => println!("Could not determine duration."),
    }

    // Evaluate the results...
    let mut best_robot = &results[0];
    for robot in &results {
        if robot.0 > best_robot.0 {
            best_robot = &robot;
        }
    }

    println!("Best Robot was {}", best_robot.1);

    // No errors, woohoo!
    Ok(())
}
