use fhtw_roboter_wettsammeln::WorldMap;
use fhtw_roboter_wettsammeln::robots::RandomBot;
use std::sync::Arc;

#[test]
fn robot_score() {
    let map = Arc::new(WorldMap::new(1, 1));
    map.randomize_fields(1);
    assert_eq!(map.points_left(), 1);

    let new_robot = RandomBot::new(0, Arc::clone(&map));
    let result = new_robot.run();
    assert_eq!(map.points_left(), 0);
    assert_eq!(result.0, 1);
}
