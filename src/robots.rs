use std::sync::Arc;
use std::fmt;
use std::thread;

use crate::WorldPosition;
use crate::WorldMap;


pub struct RandomBot {
    id: usize,
    score: usize,
    pos: WorldPosition,
    map: Arc<WorldMap>,
}

impl fmt::Display for RandomBot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RandomBot #{} (score: {})", self.id, self.score)
    }
}

impl RandomBot {
    pub fn new(id: usize, map: Arc<WorldMap>) -> Self {
        Self {
            id,
            score: 0,
            pos: WorldPosition::new(&map, 0, 0),
            map,
        }
    }

    pub fn randomize_position(&mut self) {
        self.pos.randomize();
    }

    /// Returns the amount of score points collected by the robot.
    pub fn score(&self) -> usize {
        self.score
    }

    /// Step in random (within world map bounds) direction.
    fn step(&mut self) {
        if let Some(new_pos) = try_step(&self.pos, &self.map, false) {
            self.pos = new_pos;
        }
    }

    /// Programs and unleashes the robot, sending it on an uncontrollable rampage through its world map until all score is gone.
    pub fn run(&mut self) -> (usize, String) {
        while self.map.points_left() > 0 {
            // move
            self.step();

            // attack
            self.score += self.map.deduct_score_at(&self.pos);
            
            // end of turn
            thread::yield_now();
        }
        (self.score, self.to_string())
    }
}


pub struct NearsightBot {
    id: usize,
    score: usize,
    pos: WorldPosition,
    map: Arc<WorldMap>,
}

impl fmt::Display for NearsightBot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NearsightBot #{} (score: {})", self.id, self.score)
    }
}

impl NearsightBot {
    pub fn new(id: usize, map: Arc<WorldMap>) -> Self {
        Self {
            id,
            score: 0,
            pos: WorldPosition::new(&map, 0, 0),
            map,
        }
    }


    pub fn randomize_position(&mut self) {
        self.pos.randomize();
    }

    /// Returns the amount of score points collected by the robot.
    pub fn score(&self) -> usize {
        self.score
    }

    /// Step in random (within world map bounds) direction.
    fn step(&mut self) {
        // Go to score if possible
        if let Some(new_pos) = try_step(&self.pos, &self.map, true) {
            self.pos = new_pos;
        }
        // Go elsewhere to find score farther away
        else if let Some(new_pos) = try_step(&self.pos, &self.map, false) {
            self.pos = new_pos;
        }

        self.score += self.map.deduct_score_at(&self.pos);
    }

    /// Programs and unleashes the robot, sending it on an uncontrollable rampage through its world map until all score is gone.
    pub fn run(&mut self) -> (usize, String) {
        while self.map.points_left() > 0 {
            // move
            self.step();

            // attack
            self.score += self.map.deduct_score_at(&self.pos);
            
            // end of turn
            thread::yield_now();
        }
        (self.score, self.to_string())
    }
}


fn try_step(current_pos: &WorldPosition, map: &WorldMap, check_score: bool) -> Option<WorldPosition> {
    let dir = rand::random::<u8>() % 4;

    for i in 0..4 {
        let dir = (dir + i) % 4;

        //TODO elegant way to move the "in-bounds" check to the WorldPosition implementation?
        match dir {
            0 => {
                let x = current_pos.x();
                if x > 0 {
                    let mut new_pos = current_pos.clone();
                    new_pos.set_x( new_pos.x() - 1 );
                    if !check_score || map.points_at(&new_pos) > 0 {
                        return Some(new_pos);
                    }
                }
            },
            1 => {
                let y = current_pos.y();
                if y > 0 {
                    let mut new_pos = current_pos.clone();
                    new_pos.set_y( new_pos.y() - 1 );
                    if !check_score || map.points_at(&new_pos) > 0 {
                        return Some(new_pos);
                    }
                }
            },
            2 => {
                let x = current_pos.x();
                if x < map.width() - 1 {
                    let mut new_pos = current_pos.clone();
                    new_pos.set_x( new_pos.x() + 1 );
                    if !check_score || map.points_at(&new_pos) > 0 {
                        return Some(new_pos);
                    }
                }
            },
            _ => {
                let y = current_pos.y();
                if y < map.height() - 1 {
                    let mut new_pos = current_pos.clone();
                    new_pos.set_y( new_pos.y() + 1 );
                    if !check_score || map.points_at(&new_pos) > 0 {
                        return Some(new_pos);
                    }
                }
            },
        }
    }
    None
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randombot_can_move_updown() {
        let map = Arc::new(WorldMap::new(1, 2));
        let mut new_robot = RandomBot::new(0, Arc::clone(&map));

        new_robot.step();
        assert_eq!(new_robot.pos.x(), 0);
        assert_eq!(new_robot.pos.y(), 1);

        new_robot.step();
        assert_eq!(new_robot.pos.x(), 0);
        assert_eq!(new_robot.pos.y(), 0);
    }

    #[test]
    fn randombot_can_move_leftright() {
        let map = Arc::new(WorldMap::new(2, 1));
        let mut new_robot = RandomBot::new(0, Arc::clone(&map));

        new_robot.step();
        assert_eq!(new_robot.pos.x(), 1);
        assert_eq!(new_robot.pos.y(), 0);

        new_robot.step();
        assert_eq!(new_robot.pos.x(), 0);
        assert_eq!(new_robot.pos.y(), 0);
    }

    #[test]
    fn nearsightbot_can_move_updown() {
        let map = Arc::new(WorldMap::new(1, 2));
        let mut new_robot = NearsightBot::new(0, Arc::clone(&map));

        new_robot.step();
        assert_eq!(new_robot.pos.x(), 0);
        assert_eq!(new_robot.pos.y(), 1);

        new_robot.step();
        assert_eq!(new_robot.pos.x(), 0);
        assert_eq!(new_robot.pos.y(), 0);
    }

    #[test]
    fn nearsightbot_can_move_leftright() {
        let map = Arc::new(WorldMap::new(2, 1));
        let mut new_robot = NearsightBot::new(0, Arc::clone(&map));

        new_robot.step();
        assert_eq!(new_robot.pos.x(), 1);
        assert_eq!(new_robot.pos.y(), 0);

        new_robot.step();
        assert_eq!(new_robot.pos.x(), 0);
        assert_eq!(new_robot.pos.y(), 0);
    }
}