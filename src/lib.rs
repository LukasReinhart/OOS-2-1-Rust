use std::fmt;

pub struct WorldMap {
    points_left: usize,
    width: usize,
    height: usize,
}

impl WorldMap {
    pub fn new(width: usize, height: usize) -> Self {
        WorldMap {
            width,
            height,
            points_left: 0,
        }
    }

    pub fn randomize_fields(&self, max_field_score: usize) {

    }

    pub fn points_left(&self) -> usize {
        self.points_left
    }
}

pub mod robots {
    use super::*;
    pub struct RandomBot {
        id: usize,
        score: usize,
        //map: &WorldMap,
    }

    impl RandomBot {
        pub fn new(id: usize, map: &WorldMap) -> Self {
            RandomBot {
                id,
                score: 0,
                //map,
            }
        }

        pub fn score(&self) -> usize {
            self.score
        }
    }

    impl fmt::Display for RandomBot {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RandomBot #{} (score: {})", self.id, self.score)
        }
    }
}