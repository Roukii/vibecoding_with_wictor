pub mod generator;
pub mod room;
pub mod room_manager;
pub mod room_templates;
pub mod template_example;
pub mod types;
pub mod utils;

// Re-export the main public API
pub use generator::MapGenerator;
pub use room::Room;
pub use room_manager::RoomManager;
pub use room_templates::{RoomTemplate, ALL_TEMPLATES};
pub use template_example::{demonstrate_template_system, TemplateMapGenerator};
pub use types::{Position, TileType};
pub use utils::{generate_example_map, run_example};

#[cfg(test)]
mod tests {
    use super::*;

    pub fn print_map(map: &[Vec<u8>]) {
        for row in map {
            for &cell in row {
                let ch = match TileType::from(cell) {
                    TileType::Wall => '#',
                    TileType::Floor => '.',
                    TileType::Door => 'D',
                };
                print!("{}", ch);
            }
            println!("");
        }
    }
    #[test]
    fn test_map_generation() {
        let mut generator = MapGenerator::new(5, 5, 20, 20, 2); // 2x2 rooms, each 20x20
        let map = generator.generate();

        print_map(&map);

        // Basic sanity checks
        //assert_eq!(map.len(), 30);
        //assert_eq!(map[0].len(), 40);

        // Should have at least a central room
        //assert!(!generator.rooms.is_empty());
        //assert!(generator.rooms.iter().any(|room| room.is_central));
    }

    #[test]
    fn test_room_connections() {
        let room = Room::new(10, 10, 25, 25, false); // Updated to use larger room size
        let _connections = room.get_global_connections();

        // Should have connection points on each side
        //assert!(!connections.is_empty());

        // Verify minimum size is enforced
        assert!(room.width >= 20);
        assert!(room.height >= 20);
    }

    #[test]
    fn test_minimum_room_size_enforcement() {
        // Test that Room::new enforces minimum size of 20x20
        let small_room = Room::new(0, 0, 5, 10, false);
        assert_eq!(small_room.width, 20);
        assert_eq!(small_room.height, 20);

        let adequate_room = Room::new(0, 0, 25, 30, false);
        assert_eq!(adequate_room.width, 25);
        assert_eq!(adequate_room.height, 30);
    }
}
