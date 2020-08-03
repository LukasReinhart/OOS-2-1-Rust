use fhtw_roboter_wettsammeln;
use fhtw_roboter_wettsammeln::WorldMap;
use fhtw_roboter_wettsammeln::robots::RandomBot;
use std::io;
use std::error::Error;
use std::str::FromStr;
use std::thread;

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
    let amount_robots: usize = get_user_input("Amount Robots");

    let map = WorldMap::new(map_size, map_size);
    map.randomize_fields(max_field_score);

    let mut robots = Vec::with_capacity(amount_robots);

    for i in 0..amount_robots {
        let new_robot = RandomBot::new(i, &map);
        robots.push(new_robot);
    }

    while map.points_left() > 0 {
        thread::yield_now(); //TODO consider switching to Condvar
    }

    let mut best_robot = &robots[0];
    for robot in &robots {
        if robot.score() > best_robot.score() {
            best_robot = &robot;
        }
    }

    println!("Best Robot was {}", best_robot);

    Ok(())
}
