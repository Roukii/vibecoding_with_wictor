use spacetimedb::rand::rngs::StdRng;
use spacetimedb::rand::{Rng, SeedableRng};

use crate::map_generator::room::Room;
use crate::map_generator::room_manager::RoomManager;
use crate::map_generator::room_templates::{town_templates::*, RoomTemplate};
use crate::map_generator::types::{Position, TileType};

/// A town generator that creates custom maps from room templates.
///
/// This generator has been simplified to work with a single room template instead of
/// the previous grid-based approach. It's designed to create towns from pre-defined
/// templates without needing to specify size or other parameters.
///
/// # Examples
///
/// ```rust
/// // Create a town using the default town square template
/// let mut town_gen = TownGenerator::new();
/// let map = town_gen.generate();
///
/// // Create a town from a custom template
/// let mut town_gen = TownGenerator::from_template(&custom_template);
/// let map = town_gen.generate_from_custom_template(&custom_template);
/// ```
pub struct TownGenerator {
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<u8>>,
    pub room: Option<Room>,
    pub spawn_points: Vec<Position>,
    pub rng: StdRng,
    pub room_manager: RoomManager,
}

impl TownGenerator {
    /// Create a new town generator from a room template
    pub fn from_template(template: &RoomTemplate) -> Self {
        Self::from_template_with_seed(template, 0)
    }

    /// Create a new town generator from a room template with a specific seed
    pub fn from_template_with_seed(template: &RoomTemplate, seed: u64) -> Self {
        // Parse the template to get dimensions
        let parsed = RoomManager::parse_room_template(template).expect("Invalid template");

        let width = parsed.width;
        let height = parsed.height;
        let map = vec![vec![TileType::Wall as u8; width]; height];

        // Create room manager for towns
        let room_manager = RoomManager::for_towns();

        TownGenerator {
            width,
            height,
            map,
            room: None,
            spawn_points: Vec::new(),
            rng: StdRng::seed_from_u64(seed),
            room_manager,
        }
    }

    /// Create a town generator using the default town square template
    pub fn new() -> Self {
        if let Some(town_square) = ALL_TOWN_TEMPLATES.iter().find(|t| t.name == "town_square") {
            Self::from_template(town_square)
        } else {
            // Fallback if town square template not found
            Self::from_template(&TOWN_SQUARE)
        }
    }

    /// Create a town generator using the default town square template with a seed
    pub fn with_seed(seed: u64) -> Self {
        if let Some(town_square) = ALL_TOWN_TEMPLATES.iter().find(|t| t.name == "town_square") {
            Self::from_template_with_seed(town_square, seed)
        } else {
            // Fallback if town square template not found
            Self::from_template_with_seed(&TOWN_SQUARE, seed)
        }
    }

    /// Generate the map from the template
    pub fn generate(&mut self) -> Vec<Vec<u8>> {
        self.generate_from_template();
        self.generate_spawn_points();
        self.map.clone()
    }

    /// Generate the map from a specific template
    pub fn generate_from_template(&mut self) -> Vec<Vec<u8>> {
        // Use the town square template by default
        if let Some(town_square) = ALL_TOWN_TEMPLATES.iter().find(|t| t.name == "town_square") {
            if let Ok(room) =
                self.room_manager
                    .create_room_from_template(town_square, 0, 0, &mut self.rng)
            {
                self.room = Some(room);
                self.render_map();
            }
        }
        self.map.clone()
    }

    /// Generate the map from a custom template
    pub fn generate_from_custom_template(&mut self, template: &RoomTemplate) -> Vec<Vec<u8>> {
        // Parse the template to update dimensions if needed
        if let Ok(parsed) = RoomManager::parse_room_template(template) {
            // Resize map if necessary
            if parsed.width != self.width || parsed.height != self.height {
                self.width = parsed.width;
                self.height = parsed.height;
                self.map = vec![vec![TileType::Wall as u8; self.width]; self.height];
            }

            // Create room from template
            if let Ok(room) =
                self.room_manager
                    .create_room_from_template(template, 0, 0, &mut self.rng)
            {
                self.room = Some(room);
                self.render_map();
                self.generate_spawn_points();
            }
        }
        self.map.clone()
    }

    fn render_map(&mut self) {
        // Initialize map with wall tiles
        for row in &mut self.map {
            for cell in row {
                *cell = TileType::Wall as u8;
            }
        }

        // Render the room onto the map
        if let Some(room) = &self.room {
            for (row_idx, row) in room.tiles.iter().enumerate() {
                for (col_idx, &tile) in row.iter().enumerate() {
                    let global_x = room.position.x + col_idx;
                    let global_y = room.position.y + row_idx;

                    if global_x < self.width && global_y < self.height {
                        self.map[global_y][global_x] = tile as u8;
                    }
                }
            }
        }
    }

    fn generate_spawn_points(&mut self) {
        self.spawn_points.clear();

        // Generate spawn points from the room
        if let Some(room) = &self.room {
            // Add spawn points around the center of the room
            let center_x = room.position.x + room.width / 2;
            let center_y = room.position.y + room.height / 2;

            // Create a ring of spawn points around the center
            let spawn_radius = 5;
            for angle in 0..8 {
                let radians = (angle as f64) * std::f64::consts::PI / 4.0;
                let spawn_x = center_x + (spawn_radius as f64 * radians.cos()) as usize;
                let spawn_y = center_y + (spawn_radius as f64 * radians.sin()) as usize;

                // Ensure spawn point is walkable
                if spawn_x < self.width && spawn_y < self.height {
                    if self.map[spawn_y][spawn_x] == TileType::Floor as u8 {
                        self.spawn_points.push(Position {
                            x: spawn_x,
                            y: spawn_y,
                        });
                    }
                }
            }

            // If no good spawn points found, add center as fallback
            if self.spawn_points.is_empty() {
                self.spawn_points.push(Position {
                    x: center_x,
                    y: center_y,
                });
            }
        }
    }

    /// Get all possible spawn points in the town
    pub fn get_spawn_points(&self) -> &Vec<Position> {
        &self.spawn_points
    }

    /// Get a random spawn point from the available spawn points
    pub fn get_random_spawn_point(&mut self) -> Option<Position> {
        if self.spawn_points.is_empty() {
            return None;
        }

        let index = self.rng.gen_range(0..self.spawn_points.len());
        self.spawn_points.get(index).copied()
    }

    /// Get the primary spawn point (center of the room)
    pub fn get_primary_spawn_point(&self) -> Option<Position> {
        if let Some(room) = &self.room {
            Some(Position {
                x: room.position.x + room.width / 2,
                y: room.position.y + room.height / 2,
            })
        } else {
            None
        }
    }
}
