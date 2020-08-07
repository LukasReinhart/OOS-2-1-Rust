use std::sync::Mutex;
use rand;

pub struct WorldMap {
    width: usize,
    height: usize,
    points_left: Mutex<usize>,
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
            points_left: Mutex::new(0),
            fields: Mutex::new(fields),
        }
    }

    /// Translates X/Y coordinates to an index in the 'fields' vector.
    fn position_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    /// Sets all fields to a random value between (including) 1 to 'max_field_Score' each.
    pub fn randomize_fields(&self, max_field_score: usize) {
        if let Ok(fields) = self.fields.lock() {
            let mut fields = fields;
            if let Ok(points_left) = self.points_left.lock() {
                let mut points_left = points_left;

                for i in 0..fields.len() {
                    let new_points = (rand::random::<usize>() % max_field_score) + 1;
                    *points_left += new_points - fields[i];
                    fields[i] = new_points;
                }
            }
        }
    }

    /// Lowers score at the given coordinates by 1, to a minimum of 0, and returns 1 if successful.
    pub fn deduct_score_at(&self, x: usize, y: usize) -> usize {
        let mut harvested = 0;

        if let Ok(fields) = self.fields.lock() {
            let mut fields = fields;
            let idx = self.position_index(x, y);
            if fields[idx] > 0 {
                if let Ok(points_left) = self.points_left.lock() {
                    let mut points_left = points_left;

                    *points_left -= 1;
                    fields[idx] -= 1;
                    harvested += 1;
                }
            }
        }

        harvested
    }

    /// Returns an internal counter of points remaining on the fields.
    pub fn points_left(&self) -> usize {
        if let Ok(points_left) = self.points_left.lock() {
            *points_left
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
    fn worldmap_total_score() {
        let map = WorldMap::new(5, 5);
        map.randomize_fields(5);
        assert_ne!(map.points_left(), 0);
    }

    #[test]
    fn worldmap_basic_deduction() {
        let map = WorldMap::new(5, 5);
        map.randomize_fields(5);
        assert_eq!(map.deduct_score_at(2, 2), 1);
    }
}