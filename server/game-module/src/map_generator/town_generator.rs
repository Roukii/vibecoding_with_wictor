use spacetimedb::rand::rngs::StdRng;
use spacetimedb::rand::{Rng, SeedableRng};

use crate::map_generator::room::Room;
use crate::map_generator::room_manager::RoomManager;
use crate::map_generator::room_templates::town_templates::*;
use crate::map_generator::types::{Position, TileType};

pub struct TownGenerator {
    pub width: usize,
    pub height: usize,
    pub room_width: usize,
    pub room_height: usize,
    pub map: Vec<Vec<u8>>,
    pub rooms: Vec<Room>,
    pub spawn_points: Vec<Position>, // Multiple spawn points around town square
    pub rng: StdRng,
    pub room_manager: RoomManager,
}

impl TownGenerator {
    pub fn new(town_size: usize, room_width: usize, room_height: usize) -> Self {
        Self::with_seed(town_size, room_width, room_height, 0)
    }

    pub fn with_seed(town_size: usize, room_width: usize, room_height: usize, seed: u64) -> Self {
        // Calculate total map size - town is square
        let room_width = room_width.max(20); // Ensure minimum room size
        let room_height = room_height.max(20);

        // Each room shares a wall with adjacent rooms
        let width = room_width + (town_size - 1) * (room_width - 1);
        let height = room_height + (town_size - 1) * (room_height - 1);

        let map = vec![vec![TileType::Wall as u8; width]; height];

        // Create room manager - we'll use it with the existing template system
        let room_manager = RoomManager::new();

        TownGenerator {
            width,
            height,
            room_width,
            room_height,
            map,
            rooms: Vec::new(),
            spawn_points: Vec::new(),
            rng: StdRng::seed_from_u64(seed),
            room_manager,
        }
    }

    pub fn generate(&mut self) -> Vec<Vec<u8>> {
        self.place_town_areas();
        self.render_map();
        self.connect_areas();
        self.generate_spawn_points();
        self.map.clone()
    }

    fn place_town_areas(&mut self) {
        let town_size = 3; // 3x3 town grid

        // Place town square in the center
        let center_x = town_size / 2;
        let center_y = town_size / 2;
        let center_pos_x = center_x * (self.room_width - 1);
        let center_pos_y = center_y * (self.room_height - 1);

        // Create town square using template
        if let Some(square_template) = ALL_TOWN_TEMPLATES.iter().find(|t| t.name == "town_square") {
            if let Ok(square_room) = self.room_manager.create_room_from_template(
                square_template,
                center_pos_x,
                center_pos_y,
                &mut self.rng,
            ) {
                self.rooms.push(square_room);
            } else {
                // Fallback to simple room
                let square_room = Room::new(
                    center_pos_x,
                    center_pos_y,
                    self.room_width,
                    self.room_height,
                    true,
                );
                self.rooms.push(square_room);
            }
        }

        // Define town layout with specific buildings
        let town_layout = [
            ["residential", "blacksmith", "residential"],
            ["market", "town_square", "general_store"],
            ["town_gate", "tavern", "town_gate"],
        ];

        // Place other buildings around the town square
        for (row, row_buildings) in town_layout.iter().enumerate() {
            for (col, &building_type) in row_buildings.iter().enumerate() {
                if building_type == "town_square" {
                    continue; // Already placed
                }

                let pos_x = col * (self.room_width - 1);
                let pos_y = row * (self.room_height - 1);

                // Create room based on building type - use town templates
                let room = if let Some(template) =
                    ALL_TOWN_TEMPLATES.iter().find(|t| t.name == building_type)
                {
                    if let Ok(room) = self.room_manager.create_room_from_template(
                        template,
                        pos_x,
                        pos_y,
                        &mut self.rng,
                    ) {
                        room
                    } else {
                        // Fallback to simple room
                        Room::new(pos_x, pos_y, self.room_width, self.room_height, false)
                    }
                } else {
                    // Fallback to simple room
                    Room::new(pos_x, pos_y, self.room_width, self.room_height, false)
                };

                self.rooms.push(room);
            }
        }
    }

    fn render_map(&mut self) {
        // Initialize map with floor tiles (open streets/corridors between buildings)
        for row in &mut self.map {
            for cell in row {
                *cell = TileType::Floor as u8;
            }
        }

        // Render all rooms (buildings) on top of the street grid
        for room in &self.rooms {
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

    fn connect_areas(&mut self) {
        // Collect all connection points first to avoid borrowing issues
        let mut all_connections = Vec::new();

        for (i, room1) in self.rooms.iter().enumerate() {
            for (j, room2) in self.rooms.iter().enumerate() {
                if i >= j {
                    continue;
                }

                let conn_points = self.find_connection_points(room1, room2);
                if !conn_points.is_empty() {
                    all_connections.push(conn_points);
                }
            }
        }

        // Now create all connections
        for conn_points in all_connections {
            self.create_connection(&conn_points);
        }
    }

    fn find_connection_points(&self, room1: &Room, room2: &Room) -> Vec<Position> {
        let mut connections = Vec::new();
        let room1_connections = room1.get_global_connections();
        let room2_connections = room2.get_global_connections();

        for pos1 in &room1_connections {
            for pos2 in &room2_connections {
                let dx = pos1.x as i32 - pos2.x as i32;
                let dy = pos1.y as i32 - pos2.y as i32;

                if (dx.abs() == 1 && dy == 0) || (dx == 0 && dy.abs() == 1) {
                    connections.push(*pos1);
                    connections.push(*pos2);
                }
            }
        }

        connections
    }

    fn create_connection(&mut self, conn_points: &[Position]) {
        // Connect adjacent connection points
        for i in (0..conn_points.len()).step_by(2) {
            if let (Some(pos1), Some(pos2)) = (conn_points.get(i), conn_points.get(i + 1)) {
                self.set_tile(pos1.x, pos1.y, TileType::Door);
                self.set_tile(pos2.x, pos2.y, TileType::Door);
            }
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, tile_type: TileType) {
        if x < self.width && y < self.height {
            self.map[y][x] = tile_type as u8;
        }
    }

    fn generate_spawn_points(&mut self) {
        self.spawn_points.clear();

        // Find the town square (central room)
        if let Some(town_square) = self.rooms.iter().find(|room| room.is_central) {
            // Add spawn points around the town square
            let center_x = town_square.position.x + town_square.width / 2;
            let center_y = town_square.position.y + town_square.height / 2;

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

    /// Get the primary spawn point (center of town square)
    pub fn get_primary_spawn_point(&self) -> Option<Position> {
        if let Some(town_square) = self.rooms.iter().find(|room| room.is_central) {
            Some(Position {
                x: town_square.position.x + town_square.width / 2,
                y: town_square.position.y + town_square.height / 2,
            })
        } else {
            None
        }
    }
}
