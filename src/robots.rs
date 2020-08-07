use std::sync::Arc;
use std::fmt;
use std::thread;

use crate::WorldMap;

pub struct RandomBot {
    id: usize,

    score: usize,
    map: Arc<WorldMap>,
    x: usize,
    y: usize,
}

impl RandomBot {
    pub fn new(id: usize, map: Arc<WorldMap>) -> Self {
        RandomBot {
            id,
            score: 0,
            map,
            x: 0,
            y: 0,
        }
    }

    /// Returns the amount of score points collected by the robot.
    pub fn score(&self) -> usize {
        self.score
    }

    // Teleports the robot to a random position (within world map bounds).
    pub fn randomize_position(&mut self) {
        self.x = rand::random::<usize>() % self.map.width();
        self.y = rand::random::<usize>() % self.map.height();
    }

    /// Step in random (within world map bounds) direction.
    fn step(&mut self) {
        let dir = rand::random::<u8>() % 4;

        for i in 0..4 {
            let dir = (dir + i) % 4;

            if dir == 0 && self.x > 0 {
                self.x -= 1;
                return;
            }
            else if dir == 1 && self.y > 0 {
                self.y -= 1;
                return;
            }
            else if dir == 2 && self.x < self.map.width() - 1 {
                self.x += 1;
                return;
            }
            else if dir == 3 && self.y < self.map.height() - 1 {
                self.y += 1;
                return;
            }
        }
    }

    /// Unleashes the robot, letting it go on an uncontrollable rampage through its world map until all score is gone.
    pub fn run(mut self) -> (usize, String) {
        while self.map.points_left() > 0 {
            self.step();
            
            self.score += self.map.deduct_score_at(self.x, self.y);
            // give others a chance?
            thread::yield_now();
        }
        (self.score, self.to_string())
    }

}

impl fmt::Display for RandomBot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RandomBot #{} (score: {})", self.id, self.score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_move_updown() {
        let map = Arc::new(WorldMap::new(1, 2));
        let mut new_robot = RandomBot::new(0, Arc::clone(&map));

        new_robot.step();
        assert_eq!(new_robot.x, 0);
        assert_eq!(new_robot.y, 1);

        new_robot.step();
        assert_eq!(new_robot.x, 0);
        assert_eq!(new_robot.y, 0);
    }

    #[test]
    fn can_move_leftright() {
        let map = Arc::new(WorldMap::new(2, 1));
        let mut new_robot = RandomBot::new(0, Arc::clone(&map));

        new_robot.step();
        assert_eq!(new_robot.x, 1);
        assert_eq!(new_robot.y, 0);

        new_robot.step();
        assert_eq!(new_robot.x, 0);
        assert_eq!(new_robot.y, 0);
    }
}