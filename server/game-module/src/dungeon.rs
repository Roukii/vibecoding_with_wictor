// Type definitions that match the SpacetimeDB table structures
#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Dungeon {
    pub width: u64,
    pub height: u64,
    pub tiles: Vec<u8>, // Flattened 2D array: tiles[y * width + x] = tile_type (0=Wall, 1=Floor, 2=Door)
    pub spawn_position: Vec2, // Primary spawn position (best spawn point)
    pub spawn_points: Vec<Vec2>, // All possible spawn points at edges
}

#[derive(Clone, Debug)]
pub struct Town {
    pub width: u64,
    pub height: u64,
    pub tiles: Vec<u8>, // Flattened 2D array: tiles[y * width + x] = tile_type (0=Wall, 1=Floor, 2=Door)
    pub spawn_position: Vec2, // Primary spawn position (center of town square)
    pub spawn_points: Vec<Vec2>, // Multiple spawn points around town square
}

impl Dungeon {
    /// Get tile type at the given coordinates
    pub fn get_tile(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width as usize || y >= self.height as usize {
            return None;
        }
        let index = y * (self.width as usize) + x;
        self.tiles.get(index).copied()
    }

    /// Check if a position is walkable (Floor or Door)
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        match self.get_tile(x, y) {
            Some(1) | Some(2) => true, // Floor or Door
            _ => false,                // Wall or out of bounds
        }
    }

    /// Get all valid spawn points at the edges of the map
    pub fn get_spawn_positions(&self) -> &Vec<Vec2> {
        &self.spawn_points
    }

    /// Get valid spawn positions (floor tiles near the spawn position) - improved implementation
    pub fn get_spawn_positions_near_primary(&self) -> Vec<Vec2> {
        let mut positions = Vec::new();
        let spawn_x = self.spawn_position.x as usize;
        let spawn_y = self.spawn_position.y as usize;

        // Check a larger area around the spawn position (5x5 grid)
        let radius = 2;
        for dy in 0..=(radius * 2) {
            for dx in 0..=(radius * 2) {
                let x = spawn_x.saturating_sub(radius).saturating_add(dx);
                let y = spawn_y.saturating_sub(radius).saturating_add(dy);

                if self.is_walkable(x, y) {
                    positions.push(Vec2 {
                        x: x as f64,
                        y: y as f64,
                    });
                }
            }
        }

        // If no positions found near spawn, return the spawn itself
        if positions.is_empty() {
            positions.push(self.spawn_position.clone());
        }

        positions
    }

    /// Get a random spawn point from the available spawn points with seed-based selection
    pub fn get_random_spawn_point(&self, seed: usize) -> Option<Vec2> {
        if self.spawn_points.is_empty() {
            return Some(self.spawn_position.clone());
        }

        // Use a simple linear congruential generator for deterministic randomness
        let a = 1664525u64;
        let c = 1013904223u64;
        let m = 2u64.pow(32);

        let random_value = ((a.wrapping_mul(seed as u64).wrapping_add(c)) % m) as usize;
        let index = random_value % self.spawn_points.len();

        self.spawn_points.get(index).cloned()
    }
}

impl Town {
    /// Get tile type at the given coordinates
    pub fn get_tile(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width as usize || y >= self.height as usize {
            return None;
        }
        let index = y * (self.width as usize) + x;
        self.tiles.get(index).copied()
    }

    /// Check if a position is walkable (Floor or Door)
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        match self.get_tile(x, y) {
            Some(1) | Some(2) => true, // Floor or Door
            _ => false,                // Wall or out of bounds
        }
    }

    /// Get all valid spawn points in the town
    pub fn get_spawn_positions(&self) -> &Vec<Vec2> {
        &self.spawn_points
    }

    /// Get a random spawn point from the available spawn points with seed-based selection
    pub fn get_random_spawn_point(&self, seed: usize) -> Option<Vec2> {
        if self.spawn_points.is_empty() {
            return Some(self.spawn_position.clone());
        }

        // Use a simple linear congruential generator for deterministic randomness
        let a = 1664525u64;
        let c = 1013904223u64;
        let m = 2u64.pow(32);

        let random_value = ((a.wrapping_mul(seed as u64).wrapping_add(c)) % m) as usize;
        let index = random_value % self.spawn_points.len();

        self.spawn_points.get(index).cloned()
    }
}
