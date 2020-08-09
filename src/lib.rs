use std::sync::Mutex;
use rand;
use rand::Rng;

/// Represents a bounded position within a 'WorldMap'.
pub struct WorldPosition {
    x: usize,
    y: usize,
    map_width: usize,
    map_height: usize,
}

impl WorldPosition {
    pub fn new(map: &WorldMap, x: usize, y: usize) -> Self {
        WorldPosition {
            x,
            y,
            map_width: map.width(),
            map_height: map.height(),
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn set_x(&mut self, x: usize) {
        if x < self.map_width {
            self.x = x;
        } else {
            self.x = self.map_width;
        }
    }
    pub fn set_y(&mut self, y: usize) {
        if y < self.map_width {
            self.y = y;
        } else {
            self.y = self.map_width;
        }
    }

    /// Translates X/Y coordinates to an index in the 'fields' vector.
    pub fn to_index(&self) -> usize {
        self.x + self.y * self.map_width
    }

    /// Sets to a random position (within world map bounds).
    pub fn randomize(&mut self) {
        self.x = rand::random::<usize>() % self.map_width;
        self.y = rand::random::<usize>() % self.map_height;
    }
}

impl Clone for WorldPosition {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            map_width: self.map_width,
            map_height: self.map_height,
        }
    }
}

/// Thread-safe rectangular area for bots to collect score in.
pub struct WorldMap {
    width: usize,
    height: usize,
    score_left: Mutex<usize>,
    fields: Mutex<Vec<usize>>,
}

impl WorldMap {
    /// Creates a new, empty world map of given dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        let mut fields = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            fields.push(0);
        }
        WorldMap {
            width,
            height,
            score_left: Mutex::new(0),
            fields: Mutex::new(fields),
        }
    }

    /// Sets all fields to a random value between (including) 1 to 'max_field_score' each.
    pub fn randomize_fields(&self, max_field_score: usize) {
        if let Ok(fields) = self.fields.lock() {
            let mut fields = fields;
            if let Ok(score_left) = self.score_left.lock() {
                let mut score_left = score_left;

                let mut rng = rand::thread_rng();
                for i in 0..fields.len() {
                    let new_score = (rng.gen::<usize>() % max_field_score) + 1;
                    *score_left += new_score - fields[i];
                    fields[i] = new_score;
                }
            }
        }
    }

    /// Lowers score at the given coordinates by 1, to a minimum of 0, and returns 1 if successful.
    pub fn deduct_score_at(&self, pos: &WorldPosition) -> usize {
        let mut harvested = 0;

        if let Ok(fields) = self.fields.lock() {
            let mut fields = fields;
            let idx = pos.to_index();
            if fields[idx] > 0 {
                if let Ok(score_left) = self.score_left.lock() {
                    let mut score_left = score_left;

                    *score_left -= 1;
                    fields[idx] -= 1;
                    harvested += 1;
                }
            }
        }

        harvested
    }

    /// Returns the amount of score at the given position.
    pub fn score_at(&self, pos: &WorldPosition) -> usize {
        if let Ok(fields) = self.fields.lock() {
            let idx = pos.to_index();
            fields[idx]
        } else {
            0
        }
    }

    /// Returns an internal counter of score remaining on the fields.
    pub fn score_left(&self) -> usize {
        if let Ok(score_left) = self.score_left.lock() {
            *score_left
        } else {
            0
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

pub mod robots;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worldpos_getset_x_y() {
        let map = WorldMap::new(5, 5);
        let mut pos = WorldPosition::new(&map, 2, 3);
        assert_eq!(pos.x(), 2);
        assert_eq!(pos.y(), 3);
        pos.set_x(1);
        pos.set_y(4);
        assert_eq!(pos.x(), 1);
        assert_eq!(pos.y(), 4);
    }

    #[test]
    fn worldpos_bounded_set_x_y() {
        let map = WorldMap::new(5, 5);
        let mut pos = WorldPosition::new(&map, 2, 3);
        pos.set_x(8);
        pos.set_y(8);
        assert_ne!(pos.x(), 8);
        assert_ne!(pos.y(), 8);
    }
    
    #[test]
    fn worldmap_total_score() {
        let map = WorldMap::new(5, 5);
        map.randomize_fields(5);
        assert_ne!(map.score_left(), 0);
    }

    #[test]
    fn worldmap_score_at() {
        let map = WorldMap::new(5, 5);
        map.randomize_fields(5);
        let pos = WorldPosition::new(&map, 2, 2);
        assert_ne!(map.score_at(&pos), 0);
    }

    #[test]
    fn worldmap_basic_deduction() {
        let map = WorldMap::new(5, 5);
        map.randomize_fields(5);
        let pos = WorldPosition::new(&map, 2, 2);
        assert_eq!(map.deduct_score_at(&pos), 1);
    }

    #[test]
    fn worldmap_score_at_deduction() {
        let map = WorldMap::new(5, 5);
        map.randomize_fields(5);
        let pos = WorldPosition::new(&map, 2, 2);
        let score_before = map.score_at(&pos);
        map.deduct_score_at(&pos);
        let score_after = map.score_at(&pos);
        assert_eq!(score_before, score_after + 1);
    }
}