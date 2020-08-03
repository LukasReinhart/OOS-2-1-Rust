use fhtw_roboter_wettsammeln::WorldMap;

#[test]
fn worldmap_basic_deduction() {
    let map = WorldMap::new(5, 5);
    map.randomize_fields(5);
    assert_eq!(map.deduct_score_at(2, 2), 1);
}
