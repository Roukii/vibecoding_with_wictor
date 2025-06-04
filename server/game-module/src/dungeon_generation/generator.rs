use spacetimedb::rand::rngs::StdRng;
use spacetimedb::rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::dungeon_generation::room::Room;
use crate::dungeon_generation::types::{Position, TileType};

pub struct MapGenerator {
    pub width: usize,
    pub height: usize,
    pub room_width: usize,
    pub room_height: usize,
    pub central_room_multiplier: usize,
    pub map: Vec<Vec<u8>>,
    pub rooms: Vec<Room>,
    pub room_grid: Vec<Vec<Option<usize>>>, // Grid indicating which room occupies each cell
    pub rng: StdRng,
}

impl MapGenerator {
    pub fn new(
        width: usize,
        height: usize,
        room_width: usize,
        room_height: usize,
        central_room_multiplier: usize,
    ) -> Self {
        Self::with_seed(
            width,
            height,
            room_width,
            room_height,
            central_room_multiplier,
            0,
        )
    }

    pub fn with_seed(
        width: usize,
        height: usize,
        room_width: usize,
        room_height: usize,
        central_room_multiplier: usize,
        seed: u64,
    ) -> Self {
        let map = vec![vec![TileType::Wall as u8; width]; height];
        let grid_width = width / room_width;
        let grid_height = height / room_height;
        let room_grid = vec![vec![None; grid_width]; grid_height];

        MapGenerator {
            width,
            height,
            room_width,
            room_height,
            central_room_multiplier,
            map,
            rooms: Vec::new(),
            room_grid,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate(&mut self) -> Vec<Vec<u8>> {
        self.place_rooms();
        self.connect_rooms();
        self.render_map();
        self.map.clone()
    }

    fn place_rooms(&mut self) {
        let grid_width = self.width / self.room_width;
        let grid_height = self.height / self.room_height;

        // Place central room
        let central_size = self.room_width * self.central_room_multiplier;
        let central_grid_x = (grid_width - self.central_room_multiplier) / 2;
        let central_grid_y = (grid_height - self.central_room_multiplier) / 2;
        let central_x = central_grid_x * self.room_width;
        let central_y = central_grid_y * self.room_height;

        let central_room = Room::new(central_x, central_y, central_size, central_size, true);
        self.rooms.push(central_room);

        // Mark central room area in grid
        for dy in 0..self.central_room_multiplier {
            for dx in 0..self.central_room_multiplier {
                if central_grid_y + dy < grid_height && central_grid_x + dx < grid_width {
                    self.room_grid[central_grid_y + dy][central_grid_x + dx] = Some(0);
                }
            }
        }

        // Place other rooms
        let mut room_count = 1;
        let target_rooms = (grid_width * grid_height) / 3; // Fill about 1/3 of available space

        for _ in 0..target_rooms {
            let mut attempts = 0;
            while attempts < 50 {
                let grid_x = self.rng.gen_range(0..grid_width);
                let grid_y = self.rng.gen_range(0..grid_height);

                if self.room_grid[grid_y][grid_x].is_none() {
                    let x = grid_x * self.room_width;
                    let y = grid_y * self.room_height;
                    let room = Room::new(x, y, self.room_width, self.room_height, false);
                    self.rooms.push(room);
                    self.room_grid[grid_y][grid_x] = Some(room_count);
                    room_count += 1;
                    break;
                }
                attempts += 1;
            }
        }
    }

    fn connect_rooms(&mut self) {
        let mut connections: HashMap<(usize, usize), Vec<Position>> = HashMap::new();

        // Find potential connections between adjacent rooms
        for (i, room1) in self.rooms.iter().enumerate() {
            for (j, room2) in self.rooms.iter().enumerate() {
                if i >= j {
                    continue;
                }

                let conn_points = self.find_connection_points(room1, room2);
                if !conn_points.is_empty() {
                    connections.insert((i, j), conn_points);
                }
            }
        }

        // Ensure all rooms are connected (minimum spanning tree approach)
        let mut connected = HashSet::new();
        let mut to_connect = VecDeque::new();

        // Start with room 0 (central room)
        connected.insert(0);
        to_connect.push_back(0);

        while !to_connect.is_empty() {
            let current = to_connect.pop_front().unwrap();

            // Find all rooms that can connect to current room
            for ((i, j), conn_points) in &connections {
                let (room_a, room_b) = if *i == current {
                    (*i, *j)
                } else if *j == current {
                    (*j, *i)
                } else {
                    continue;
                };

                if connected.contains(&room_a) && !connected.contains(&room_b) {
                    // Connect these rooms
                    self.create_connection(room_a, room_b, conn_points);
                    connected.insert(room_b);
                    to_connect.push_back(room_b);
                } else if !connected.contains(&room_a) && connected.contains(&room_b) {
                    // Connect these rooms
                    self.create_connection(room_a, room_b, conn_points);
                    connected.insert(room_a);
                    to_connect.push_back(room_a);
                }
            }
        }

        // Add some extra connections for more interesting layouts
        for ((i, j), conn_points) in &connections {
            if connected.contains(i) && connected.contains(j) && self.rng.gen_bool(0.3) {
                self.create_connection(*i, *j, conn_points);
            }
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
        if conn_points.len() >= 2 {
            // If multiple connection points, randomly choose 1-2 connections
            let num_connections = if conn_points.len() > 4 && self.rng.gen_bool(0.4) {
                2
            } else {
                1
            };

            for _ in 0..num_connections {
                if let Some(pos) =
                    conn_points.get(self.rng.gen_range(0..conn_points.len().min(2)) * 2)
                {
                    // Mark connection points as doors or open
                    if let Some(adjacent_pos) =
                        conn_points.get(self.rng.gen_range(0..conn_points.len().min(2)) * 2 + 1)
                    {
                        if num_connections == 1 {
                            self.set_tile(pos.x, pos.y, TileType::Door);
                            self.set_tile(adjacent_pos.x, adjacent_pos.y, TileType::Door);
                        } else {
                            self.set_tile(pos.x, pos.y, TileType::Floor);
                            self.set_tile(adjacent_pos.x, adjacent_pos.y, TileType::Floor);
                        }
                    }
                }
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

    pub fn get_spawn_room(&self) -> Option<Position> {
        // Find an edge room (not central room)
        let edge_rooms: Vec<&Room> = self
            .rooms
            .iter()
            .filter(|room| !room.is_central)
            .filter(|room| {
                // Check if room is on the edge of the map
                room.position.x == 0
                    || room.position.y == 0
                    || room.position.x + room.width >= self.width - self.room_width
                    || room.position.y + room.height >= self.height - self.room_height
            })
            .collect();

        if !edge_rooms.is_empty() {
            // Create a temporary RNG with a seed based on the current state
            let mut temp_rng = StdRng::seed_from_u64(self.rooms.len() as u64);
            if let Some(room) = edge_rooms.get(temp_rng.gen_range(0..edge_rooms.len())) {
                return Some(Position {
                    x: room.position.x + room.width / 2,
                    y: room.position.y + room.height / 2,
                });
            }
        }

        // Fallback to any non-central room
        self.rooms
            .iter()
            .find(|room| !room.is_central)
            .map(|room| Position {
                x: room.position.x + room.width / 2,
                y: room.position.y + room.height / 2,
            })
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
}
