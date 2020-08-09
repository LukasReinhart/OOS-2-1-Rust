use fhtw_roboter_wettsammeln::WorldMap;
use fhtw_roboter_wettsammeln::robots::{Robot, RandomBot, NearsightBot};
use std::sync::Arc;

#[test]
fn randombot_score() {
    let map = Arc::new(WorldMap::new(1, 1));
    map.randomize_fields(1);
    assert_eq!(map.points_left(), 1);

    let mut new_robot = RandomBot::new(0, Arc::clone(&map));
    new_robot.run();
    assert_eq!(map.points_left(), 0);
    assert_eq!(new_robot.score(), 1);
}

#[test]
fn nearsightbot_score() {
    let map = Arc::new(WorldMap::new(1, 1));
    map.randomize_fields(1);
    assert_eq!(map.points_left(), 1);

    let mut new_robot = NearsightBot::new(0, Arc::clone(&map));
    new_robot.run();
    assert_eq!(map.points_left(), 0);
    assert_eq!(new_robot.score(), 1);
}