use fhtw_roboter_wettsammeln::WorldMap;

#[test]
fn worldmap_total_score() {
    let map = WorldMap::new(5, 5);
    map.randomize_fields(5);
    assert_ne!(map.points_left(), 0);
}
