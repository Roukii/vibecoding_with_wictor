use crate::map_generator::{room::Room, room_manager::RoomManager, types::TileType};
use spacetimedb::rand::{rngs::StdRng, SeedableRng};

/// Example function demonstrating how to use the new room template system
pub fn demonstrate_template_system() {
    // Create a room manager (loads all templates)
    let room_manager = RoomManager::new();

    // Create a random number generator
    let mut rng = StdRng::seed_from_u64(42);

    println!("Available room templates:");
    for template in room_manager.get_all_templates() {
        println!(
            "- {} (weight: {}, central: {})",
            template.name, template.weight, template.is_central
        );
    }

    // Create a room from a specific template
    match Room::from_template_name(&room_manager, "small_square", 10, 10, &mut rng) {
        Ok(room) => {
            println!("\nCreated room from 'small_square' template:");
            print_room(&room);
        }
        Err(e) => println!("Error creating room: {}", e),
    }

    // Create a random regular room
    match Room::random_from_templates(&room_manager, 20, 20, false, &mut rng) {
        Ok(room) => {
            println!(
                "\nCreated random regular room (template: {}):",
                room.get_template_name().unwrap_or("unknown")
            );
            print_room(&room);
        }
        Err(e) => println!("Error creating random room: {}", e),
    }

    // Create a random central room
    match Room::random_from_templates(&room_manager, 30, 30, true, &mut rng) {
        Ok(room) => {
            println!(
                "\nCreated random central room (template: {}):",
                room.get_template_name().unwrap_or("unknown")
            );
            print_room(&room);
        }
        Err(e) => println!("Error creating central room: {}", e),
    }
}

/// Helper function to print a room's layout
pub fn print_room(room: &Room) {
    println!(
        "Room at ({}, {}) - {}x{}",
        room.position.x, room.position.y, room.width, room.height
    );

    for (y, row) in room.tiles.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            let ch = match tile {
                TileType::Wall => '#',
                TileType::Floor => '.',
                TileType::Door => 'D',
            };

            // Highlight connection points
            let is_connection = room
                .connections
                .iter()
                .any(|conn| conn.x == x && conn.y == y);
            if is_connection && tile == TileType::Wall {
                print!("C");
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }

    println!("Connection points: {:?}", room.connections);
    println!();
}

/// Example of how to integrate templates into the existing MapGenerator
pub fn create_template_based_map_generator() -> TemplateMapGenerator {
    TemplateMapGenerator::new(80, 60)
}

pub struct TemplateMapGenerator {
    pub width: usize,
    pub height: usize,
    pub room_manager: RoomManager,
    pub rooms: Vec<Room>,
    pub rng: StdRng,
}

impl TemplateMapGenerator {
    pub fn new(width: usize, height: usize) -> Self {
        Self::with_seed(width, height, 42)
    }

    pub fn with_seed(width: usize, height: usize, seed: u64) -> Self {
        TemplateMapGenerator {
            width,
            height,
            room_manager: RoomManager::new(),
            rooms: Vec::new(),
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate(&mut self) -> Vec<Vec<u8>> {
        self.place_template_rooms();
        self.render_to_map()
    }

    fn place_template_rooms(&mut self) {
        // Place a central room first
        if let Ok(central_room) = Room::random_from_templates(
            &self.room_manager,
            self.width / 2 - 5,
            self.height / 2 - 4,
            true,
            &mut self.rng,
        ) {
            self.rooms.push(central_room);
        }

        // Place some regular rooms around it
        let positions = vec![
            (10, 10),
            (50, 10),
            (10, 40),
            (50, 40),
            (30, 5),
            (5, 25),
            (60, 25),
            (30, 50),
        ];

        for (x, y) in positions {
            if let Ok(room) =
                Room::random_from_templates(&self.room_manager, x, y, false, &mut self.rng)
            {
                self.rooms.push(room);
            }
        }
    }

    fn render_to_map(&self) -> Vec<Vec<u8>> {
        let mut map = vec![vec![TileType::Wall as u8; self.width]; self.height];

        for room in &self.rooms {
            for (room_y, row) in room.tiles.iter().enumerate() {
                for (room_x, &tile) in row.iter().enumerate() {
                    let map_x = room.position.x + room_x;
                    let map_y = room.position.y + room_y;

                    if map_x < self.width && map_y < self.height {
                        map[map_y][map_x] = tile as u8;
                    }
                }
            }
        }

        map
    }
}
