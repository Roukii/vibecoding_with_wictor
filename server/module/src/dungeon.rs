use crate::tables::{dungeon, player, town, Dungeon, Town};
use crate::types::Vec2;
use game_module::map_generator;
use spacetimedb::{reducer, ReducerContext, Table};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    // Called when the module is initially published
    // Generate a starting town where all players will spawn
    let mut town_generator = map_generator::TownGenerator::new(3, 30, 30); // 3x3 town grid, each area 30x30
    let map = town_generator.generate();

    // Get spawn position from the town generator (center of town square)
    let spawn_position = if let Some(spawn) = town_generator.get_primary_spawn_point() {
        Vec2 {
            x: spawn.x as f64,
            y: spawn.y as f64,
        }
    } else {
        // Default spawn at center of map
        Vec2 {
            x: (town_generator.width / 2) as f64,
            y: (town_generator.height / 2) as f64,
        }
    };

    // Flatten the 2D map into a 1D vector for storage
    let tiles: Vec<u8> = map.into_iter().flatten().collect();

    // Convert all spawn points to Vec2
    let spawn_points: Vec<Vec2> = town_generator
        .get_spawn_points()
        .iter()
        .map(|pos| Vec2 {
            x: pos.x as f64,
            y: pos.y as f64,
        })
        .collect();

    // Create and store the starting town
    let town = Town {
        id: 0, // auto_inc will handle this
        name: "Starting Town".to_string(),
        width: town_generator.width as u64,
        height: town_generator.height as u64,
        tiles,
        spawn_position,
        spawn_points,
        is_starting_town: true,
        created_at: ctx.timestamp,
    };

    ctx.db.town().insert(town);

    // Also generate a small dungeon for exploration
    let mut dungeon_generator = map_generator::MapGenerator::new(3, 3, 20, 20, 1); // 3x3 grid of rooms, each 20x20
    let dungeon_map = dungeon_generator.generate();

    let dungeon_spawn_position = if let Some(spawn) = dungeon_generator.get_best_spawn_point() {
        Vec2 {
            x: spawn.x as f64,
            y: spawn.y as f64,
        }
    } else {
        Vec2 {
            x: (dungeon_generator.width / 2) as f64,
            y: (dungeon_generator.height / 2) as f64,
        }
    };

    let dungeon_tiles: Vec<u8> = dungeon_map.into_iter().flatten().collect();
    let dungeon_spawn_points: Vec<Vec2> = dungeon_generator
        .get_spawn_points()
        .iter()
        .map(|pos| Vec2 {
            x: pos.x as f64,
            y: pos.y as f64,
        })
        .collect();

    let dungeon = Dungeon {
        id: 0,
        name: "Exploration Dungeon".to_string(),
        width: dungeon_generator.width as u64,
        height: dungeon_generator.height as u64,
        tiles: dungeon_tiles,
        spawn_position: dungeon_spawn_position,
        spawn_points: dungeon_spawn_points,
        created_at: ctx.timestamp,
    };

    ctx.db.dungeon().insert(dungeon);

    log::info!(
        "Starting town generated: {} areas, size {}x{}, {} spawn points",
        town_generator.rooms.len(),
        town_generator.width,
        town_generator.height,
        town_generator.get_spawn_points().len()
    );
    log::info!(
        "Exploration dungeon generated: {} rooms, size {}x{}",
        dungeon_generator.rooms.len(),
        dungeon_generator.width,
        dungeon_generator.height
    );
}


#[reducer]
pub fn get_latest_dungeon(ctx: &ReducerContext) -> Result<(), String> {
    // Find the most recently created dungeon
    if let Some(dungeon) = ctx.db.dungeon().iter().max_by_key(|d| d.created_at) {
        log::info!(
            "Latest dungeon: '{}' ({}x{}) created at {:?}",
            dungeon.name,
            dungeon.width,
            dungeon.height,
            dungeon.created_at
        );
        Ok(())
    } else {
        Err("No dungeons found. Generate one first!".to_string())
    }
}

#[reducer]
pub fn get_starting_town(ctx: &ReducerContext) -> Result<(), String> {
    // Find the starting town
    if let Some(town) = ctx.db.town().iter().find(|t| t.is_starting_town) {
        log::info!(
            "Starting town: '{}' ({}x{}) with {} spawn points, created at {:?}",
            town.name,
            town.width,
            town.height,
            town.spawn_points.len(),
            town.created_at
        );
        Ok(())
    } else {
        Err("No starting town found!".to_string())
    }
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

    /// Get valid spawn positions (floor tiles near the spawn position) - legacy method
    pub fn get_spawn_positions_near_primary(&self) -> Vec<Vec2> {
        let mut positions = Vec::new();
        let spawn_x = self.spawn_position.x as usize;
        let spawn_y = self.spawn_position.y as usize;

        // Check a 5x5 area around the spawn position
        for dy in 0..5 {
            for dx in 0..5 {
                let x = spawn_x.saturating_sub(2).saturating_add(dx);
                let y = spawn_y.saturating_sub(2).saturating_add(dy);

                if self.is_walkable(x, y) {
                    positions.push(Vec2 {
                        x: x as f64,
                        y: y as f64,
                    });
                }
            }
        }

        positions
    }

    /// Get a random spawn point from the available spawn points
    pub fn get_random_spawn_point(&self) -> Option<Vec2> {
        if self.spawn_points.is_empty() {
            return Some(self.spawn_position);
        }

        // Simple random selection based on the number of spawn points
        let index = self.spawn_points.len() % self.spawn_points.len().max(1);
        self.spawn_points.get(index).copied()
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

    /// Get a random spawn point from the available spawn points
    pub fn get_random_spawn_point(&self) -> Option<Vec2> {
        if self.spawn_points.is_empty() {
            return Some(self.spawn_position);
        }

        // Simple random selection based on the number of spawn points
        let index = self.spawn_points.len() % self.spawn_points.len().max(1);
        self.spawn_points.get(index).copied()
    }
}
