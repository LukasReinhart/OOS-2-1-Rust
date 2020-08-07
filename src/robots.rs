use std::sync::Arc;
use std::fmt;
use std::thread;
use std::ops::{Deref, DerefMut};

use crate::WorldMap;

pub struct RobotCore {
    id: usize,

    score: usize,
    map: Arc<WorldMap>,
    x: usize,
    y: usize,
}

impl RobotCore {
    pub fn new(id: usize, map: Arc<WorldMap>) -> Self {
        RobotCore {
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

    /// Programs and unleashes the robot, sending it on an uncontrollable rampage through its world map until all score is gone.
    pub fn run(&mut self, step_fn: fn(&mut Self)) {
        while self.map.points_left() > 0 {
            step_fn(self);
            
            self.score += self.map.deduct_score_at(self.x, self.y);
            // give others a chance?
            thread::yield_now();
        }
    }

}


pub struct RandomBot(RobotCore);

impl Deref for RandomBot {
    type Target = RobotCore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RandomBot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for RandomBot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RandomBot #{} (score: {})", self.id, self.score)
    }
}

impl RandomBot {
    pub fn new(id: usize, map: Arc<WorldMap>) -> Self {
        RandomBot(
            RobotCore::new(id, map)
        )
    }

    /// Step in random (within world map bounds) direction.
    fn step_fn(robot: &mut RobotCore) {
        let dir = rand::random::<u8>() % 4;

        for i in 0..4 {
            let dir = (dir + i) % 4;

            if dir == 0 && robot.x > 0 {
                robot.x -= 1;
                return;
            }
            else if dir == 1 && robot.y > 0 {
                robot.y -= 1;
                return;
            }
            else if dir == 2 && robot.x < robot.map.width() - 1 {
                robot.x += 1;
                return;
            }
            else if dir == 3 && robot.y < robot.map.height() - 1 {
                robot.y += 1;
                return;
            }
        }
    }

    pub fn step(&mut self) {
        Self::step_fn(&mut self.0);
    }

    pub fn run(mut self) -> (usize, String) {
        self.0.run(Self::step_fn);
        (self.score, self.to_string())
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