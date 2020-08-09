use fhtw_roboter_wettsammeln;
use fhtw_roboter_wettsammeln::WorldMap;
use fhtw_roboter_wettsammeln::robots::{Robot, RandomBot, NearsightBot};
use std::io;
use std::error::Error;
use std::str::FromStr;
use std::thread;
use std::sync::Arc;
use std::time::SystemTime;


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

    let start_time = SystemTime::now();

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

    let mut best_robot = &results[0];
    for robot in &results {
        if robot.0 > best_robot.0 {
            best_robot = &robot;
        }
    }

    println!("Best Robot was {}", best_robot.1);

    Ok(())
}
