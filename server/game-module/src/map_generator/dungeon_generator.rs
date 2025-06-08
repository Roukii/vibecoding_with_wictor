use spacetimedb::rand::rngs::StdRng;
use spacetimedb::rand::{Rng, SeedableRng};

use crate::map_generator::room::Room;
use crate::map_generator::room_manager::RoomManager;
use crate::map_generator::types::{Position, TileType};

pub struct DungeonGenerator {
    pub width: usize,
    pub height: usize,
    pub room_width: usize,
    pub room_height: usize,
    pub central_room_multiplier: usize,
    pub map: Vec<Vec<u8>>,
    pub rooms: Vec<Room>,
    pub room_grid: Vec<Vec<Option<usize>>>, // Grid indicating which room occupies each cell
    pub spawn_points: Vec<Position>,        // List of possible spawn points at map edges
    pub rng: StdRng,
    pub room_manager: RoomManager,
}

impl DungeonGenerator {
    pub fn new(
        rooms_width: usize,  // Number of rooms horizontally
        rooms_height: usize, // Number of rooms vertically
        room_width: usize,
        room_height: usize,
        central_room_multiplier: usize,
        seed: u64,
    ) -> Self {
        // Calculate total map size based on number of rooms with shared walls
        let room_width = room_width.max(20); // Ensure minimum room size of 20
        let room_height = room_height.max(20); // Ensure minimum room size of 20
                                               // Each room shares a wall with adjacent rooms, so we subtract overlapping walls
        let width = room_width + (rooms_width - 1) * (room_width - 1);
        let height = room_height + (rooms_height - 1) * (room_height - 1);

        let map = vec![vec![TileType::Wall as u8; width]; height];
        let room_grid = vec![vec![None; rooms_width]; rooms_height];

        DungeonGenerator {
            width,
            height,
            room_width,
            room_height,
            central_room_multiplier,
            map,
            rooms: Vec::new(),
            room_grid,
            spawn_points: Vec::new(),
            rng: StdRng::seed_from_u64(seed),
            room_manager: RoomManager::for_dungeons(),
        }
    }

    pub fn generate(&mut self) -> Vec<Vec<u8>> {
        self.place_rooms();
        self.render_map(); // Render rooms first
        self.connect_rooms(); // Then place doors on top
        self.generate_spawn_points(); // Generate spawn points at map edges
        self.map.clone()
    }

    /// Set a specific central room template to use
    pub fn set_central_room_template(&mut self, template_name: &str) -> Result<(), String> {
        self.room_manager.set_central_room(template_name)
    }

    fn place_rooms(&mut self) {
        self.place_central_room();
        let (edge_positions, interior_positions) = self.collect_available_positions();
        self.place_edge_spawn_rooms(&edge_positions);
        self.place_interior_rooms(&interior_positions);
    }

    fn place_central_room(&mut self) {
        let grid_width = self.room_grid[0].len(); // Number of rooms horizontally
        let grid_height = self.room_grid.len(); // Number of rooms vertically

        // Place central room using a central room template
        let central_grid_x = (grid_width - self.central_room_multiplier) / 2;
        let central_grid_y = (grid_height - self.central_room_multiplier) / 2;
        // Calculate position with shared walls: each room overlaps by 1 tile
        let central_x = central_grid_x * (self.room_width - 1);
        let central_y = central_grid_y * (self.room_height - 1);

        // Try to get a central room template (either set specific one or random)
        if let Some(central_template) = self.room_manager.get_central_template(&mut self.rng) {
            // Central room should be the size of 4 regular rooms (2x2 arrangement) with shared walls
            let central_width = self.room_width * 2 - 1; // -1 for shared wall in the middle
            let central_height = self.room_height * 2 - 1; // -1 for shared wall in the middle
            if let Ok(central_room) = self.room_manager.create_room_from_template_with_min_size(
                central_template,
                central_x,
                central_y,
                central_width,
                central_height,
                &mut self.rng,
            ) {
                self.rooms.push(central_room);
            } else {
                // Fallback to simple central room if template creation fails
                log::info!(
                    "Failed to create central room from template: {}",
                    central_template.name
                );
                let central_room =
                    Room::new(central_x, central_y, central_width, central_height, true);
                self.rooms.push(central_room);
            }
        } else {
            // Fallback to simple central room if no template available
            // Central room should be the size of 4 regular rooms (2x2 arrangement) with shared walls
            let central_width = self.room_width * 2 - 1; // -1 for shared wall in the middle
            let central_height = self.room_height * 2 - 1; // -1 for shared wall in the middle
            let central_room = Room::new(central_x, central_y, central_width, central_height, true);
            self.rooms.push(central_room);
        }

        // Mark central room area in grid
        for dy in 0..self.central_room_multiplier {
            for dx in 0..self.central_room_multiplier {
                if central_grid_y + dy < grid_height && central_grid_x + dx < grid_width {
                    self.room_grid[central_grid_y + dy][central_grid_x + dx] = Some(0);
                }
            }
        }
    }

    fn collect_available_positions(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let grid_width = self.room_grid[0].len();
        let grid_height = self.room_grid.len();

        let mut edge_positions = Vec::new();
        let mut interior_positions = Vec::new();

        for grid_y in 0..grid_height {
            for grid_x in 0..grid_width {
                if self.room_grid[grid_y][grid_x].is_none() {
                    // Check if this position is on the edge of the grid
                    let is_edge = grid_x == 0
                        || grid_x == grid_width - 1
                        || grid_y == 0
                        || grid_y == grid_height - 1;

                    if is_edge {
                        edge_positions.push((grid_x, grid_y));
                    } else {
                        interior_positions.push((grid_x, grid_y));
                    }
                }
            }
        }

        (edge_positions, interior_positions)
    }

    fn place_edge_spawn_rooms(&mut self, edge_positions: &[(usize, usize)]) {
        let mut room_count = 1; // Start from 1 since central room is 0

        // Place spawn rooms at all edge positions
        for &(grid_x, grid_y) in edge_positions {
            // Calculate position with shared walls: each room overlaps by 1 tile
            let x = grid_x * (self.room_width - 1);
            let y = grid_y * (self.room_height - 1);

            // Force use of spawn room template
            let mut room = if let Some(spawn_template) =
                self.room_manager.get_random_template_by_type(
                    &mut self.rng,
                    crate::map_generator::room_templates::RoomType::Spawn,
                ) {
                if let Ok(room) = self.room_manager.create_room_from_template_with_min_size(
                    spawn_template,
                    x,
                    y,
                    self.room_width,
                    self.room_height,
                    &mut self.rng,
                ) {
                    // Check if the room fits within bounds
                    if x + room.width <= self.width && y + room.height <= self.height {
                        room
                    } else {
                        // Fallback to simple room if template doesn't fit
                        Room::new(x, y, self.room_width, self.room_height, false)
                    }
                } else {
                    // Fallback to simple room if template creation fails
                    Room::new(x, y, self.room_width, self.room_height, false)
                }
            } else {
                // Fallback to simple room if no spawn template available
                Room::new(x, y, self.room_width, self.room_height, false)
            };

            // Ensure this room is marked as a spawn room regardless of how it was created
            room.room_type = crate::map_generator::room_templates::RoomType::Spawn;

            self.rooms.push(room);
            self.room_grid[grid_y][grid_x] = Some(room_count);
            room_count += 1;
        }
    }

    fn place_interior_rooms(&mut self, interior_positions: &[(usize, usize)]) {
        let mut room_count = self.rooms.len(); // Continue from where edge rooms left off

        // Shuffle interior positions for random placement
        let mut shuffled_positions = interior_positions.to_vec();
        for i in (1..shuffled_positions.len()).rev() {
            let j = self.rng.gen_range(0..=i);
            shuffled_positions.swap(i, j);
        }

        // Place non-spawn rooms in interior positions
        for &(grid_x, grid_y) in &shuffled_positions {
            // Calculate position with shared walls: each room overlaps by 1 tile
            let x = grid_x * (self.room_width - 1);
            let y = grid_y * (self.room_height - 1);

            // Use any non-spawn room template - try multiple times to avoid spawn rooms
            let mut room = None;
            for _attempt in 0..10 {
                if let Some(template) = self.room_manager.get_random_template(&mut self.rng, false)
                {
                    // Skip spawn room templates for interior positions
                    if template.room_type == crate::map_generator::room_templates::RoomType::Spawn {
                        continue;
                    }

                    if let Ok(created_room) =
                        self.room_manager.create_room_from_template_with_min_size(
                            template,
                            x,
                            y,
                            self.room_width,
                            self.room_height,
                            &mut self.rng,
                        )
                    {
                        // Check if the room fits within bounds
                        if x + created_room.width <= self.width
                            && y + created_room.height <= self.height
                        {
                            room = Some(created_room);
                            break;
                        }
                    }
                }
            }

            // If no suitable template found, fallback to simple room
            let final_room =
                room.unwrap_or_else(|| Room::new(x, y, self.room_width, self.room_height, false));

            self.rooms.push(final_room);
            self.room_grid[grid_y][grid_x] = Some(room_count);
            room_count += 1;
        }
    }

    fn connect_rooms(&mut self) {
        // Connect ALL neighboring rooms, not just minimum spanning tree
        let mut all_connections = Vec::new();

        // First, find all potential connections
        for (i, room1) in self.rooms.iter().enumerate() {
            for (j, room2) in self.rooms.iter().enumerate() {
                if i >= j {
                    continue;
                }

                let conn_points = self.find_connection_points(room1, room2);
                if !conn_points.is_empty() {
                    all_connections.push((i, j, conn_points));
                }
            }
        }

        // Then create all connections
        for (i, j, conn_points) in all_connections {
            self.create_connection(i, j, &conn_points);
        }
    }

    fn find_connection_points(&self, room1: &Room, room2: &Room) -> Vec<Position> {
        let mut connections = Vec::new();

        // Check if rooms are adjacent
        let room1_connections = room1.get_global_connections();
        let room2_connections = room2.get_global_connections();

        for pos1 in &room1_connections {
            for pos2 in &room2_connections {
                // Check if connection points are adjacent
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

    fn create_connection(
        &mut self,
        _room1_idx: usize,
        _room2_idx: usize,
        conn_points: &[Position],
    ) {
        // Connect ALL adjacent connection points (every pair represents one door)
        // conn_points come in pairs: [pos1, pos2, pos3, pos4, ...] where (pos1, pos2) is one connection
        for i in (0..conn_points.len()).step_by(2) {
            if let (Some(pos1), Some(pos2)) = (conn_points.get(i), conn_points.get(i + 1)) {
                // Always create doors for full connectivity
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

    fn render_map(&mut self) {
        // Clear map
        for row in &mut self.map {
            for cell in row {
                *cell = TileType::Wall as u8;
            }
        }

        // Render all rooms
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

    fn generate_spawn_points(&mut self) {
        self.spawn_points.clear();

        // Collect spawn points from spawn rooms (marked with 'S' in templates)
        for room in &self.rooms {
            if room.room_type == crate::map_generator::room_templates::RoomType::Spawn {
                // Get all spawn points from this spawn room
                let global_spawn_points = room.get_global_spawn_points();

                self.spawn_points.extend(global_spawn_points);
            }
        }

        // If no spawn points were found from templates, fall back to edge detection
        if self.spawn_points.is_empty() {
            log::warn!(
                "No spawn points found in spawn room templates, falling back to edge detection"
            );

            for room in &self.rooms {
                if room.is_central {
                    continue; // Skip central rooms for spawn points
                }

                // Check if room touches any edge of the map
                let touches_left = room.position.x == 0;
                let touches_right = room.position.x + room.width >= self.width;
                let touches_top = room.position.y == 0;
                let touches_bottom = room.position.y + room.height >= self.height;

                if touches_left || touches_right || touches_top || touches_bottom {
                    // Find floor tiles in this edge room
                    for (row_idx, row) in room.tiles.iter().enumerate() {
                        for (col_idx, &tile) in row.iter().enumerate() {
                            if tile == TileType::Floor {
                                let global_x = room.position.x + col_idx;
                                let global_y = room.position.y + row_idx;

                                // Check if this floor tile is actually on the edge
                                let is_edge = (global_x == 0 && touches_left)
                                    || (global_x == self.width - 1 && touches_right)
                                    || (global_y == 0 && touches_top)
                                    || (global_y == self.height - 1 && touches_bottom);

                                // Or if it's close to the edge (within 2 tiles)
                                let is_near_edge = global_x < 2
                                    || global_x >= self.width - 2
                                    || global_y < 2
                                    || global_y >= self.height - 2;

                                if is_edge || is_near_edge {
                                    self.spawn_points.push(Position {
                                        x: global_x,
                                        y: global_y,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // If still no spawn points found, add some default ones
        if self.spawn_points.is_empty() {
            // Add corners as fallback spawn points
            let margin = 2;
            self.spawn_points.extend_from_slice(&[
                Position {
                    x: margin,
                    y: margin,
                },
                Position {
                    x: self.width - margin - 1,
                    y: margin,
                },
                Position {
                    x: margin,
                    y: self.height - margin - 1,
                },
                Position {
                    x: self.width - margin - 1,
                    y: self.height - margin - 1,
                },
            ]);
        }
    }

    pub fn get_central_room_position(&self) -> Option<Position> {
        self.rooms
            .iter()
            .find(|room| room.is_central)
            .map(|room| Position {
                x: room.position.x + room.width / 2,
                y: room.position.y + room.height / 2,
            })
    }

    /// Get all possible spawn points at the edges of the map
    pub fn get_spawn_points(&self) -> &Vec<Position> {
        &self.spawn_points
    }

    /// Get a random spawn point from the available spawn points
    pub fn get_random_spawn_point(&self) -> Option<Position> {
        if self.spawn_points.is_empty() {
            return None;
        }

        // Create a temporary RNG with a seed based on the current state
        let mut temp_rng = StdRng::seed_from_u64(self.spawn_points.len() as u64);
        let index = temp_rng.gen_range(0..self.spawn_points.len());
        self.spawn_points.get(index).copied()
    }

    /// Get the best spawn point (closest to an edge)
    pub fn get_best_spawn_point(&self) -> Option<Position> {
        if self.spawn_points.is_empty() {
            return None;
        }

        // Find the spawn point closest to any edge
        self.spawn_points
            .iter()
            .min_by_key(|pos| {
                let dist_to_left = pos.x;
                let dist_to_right = self.width.saturating_sub(pos.x + 1);
                let dist_to_top = pos.y;
                let dist_to_bottom = self.height.saturating_sub(pos.y + 1);

                dist_to_left
                    .min(dist_to_right)
                    .min(dist_to_top)
                    .min(dist_to_bottom)
            })
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_rooms_are_spawn_rooms() {
        // Create a small 5x5 room grid dungeon
        let mut generator = DungeonGenerator::new(5, 5, 20, 20, 2, 12345);

        // Generate the dungeon
        generator.generate();

        // Check that edge rooms are spawn rooms
        let grid_width = generator.room_grid[0].len();
        let grid_height = generator.room_grid.len();

        // Count spawn rooms on edges vs interior
        let mut edge_spawn_count = 0;
        let mut total_edge_rooms = 0;
        let mut total_spawn_points_from_all_spawn_rooms = 0;

        // First, count spawn points from ALL spawn rooms (to match generate_spawn_points logic)
        for room in &generator.rooms {
            if room.room_type == crate::map_generator::room_templates::RoomType::Spawn {
                total_spawn_points_from_all_spawn_rooms += room.spawn_points.len();
            }
        }

        for (room_idx, room) in generator.rooms.iter().enumerate() {
            if room_idx == 0 {
                // Skip central room (index 0)
                continue;
            }

            // Find this room's grid position
            let mut room_grid_pos = None;
            for grid_y in 0..grid_height {
                for grid_x in 0..grid_width {
                    if let Some(idx) = generator.room_grid[grid_y][grid_x] {
                        if idx == room_idx {
                            room_grid_pos = Some((grid_x, grid_y));
                            break;
                        }
                    }
                }
                if room_grid_pos.is_some() {
                    break;
                }
            }

            if let Some((grid_x, grid_y)) = room_grid_pos {
                let is_edge = grid_x == 0
                    || grid_x == grid_width - 1
                    || grid_y == 0
                    || grid_y == grid_height - 1;

                if is_edge {
                    total_edge_rooms += 1;
                    if room.room_type == crate::map_generator::room_templates::RoomType::Spawn {
                        edge_spawn_count += 1;

                    }
                } else {
                    // This is an interior room - check if it's wrongly marked as spawn
                    if room.room_type == crate::map_generator::room_templates::RoomType::Spawn {
                    }
                }
            }
        }

        // Assert that all edge rooms are spawn rooms
        assert_eq!(
            edge_spawn_count, total_edge_rooms,
            "All edge rooms should be spawn rooms, but only {} out of {} are spawn rooms",
            edge_spawn_count, total_edge_rooms
        );

        // Assert that we have spawn points from templates
        assert!(
            total_spawn_points_from_all_spawn_rooms > 0,
            "Spawn rooms should have spawn points marked with 'S'"
        );
        assert_eq!(
            generator.spawn_points.len(),
            total_spawn_points_from_all_spawn_rooms,
            "Generator should have {} spawn points from templates, but has {}",
            total_spawn_points_from_all_spawn_rooms,
            generator.spawn_points.len()
        );
    }
}
