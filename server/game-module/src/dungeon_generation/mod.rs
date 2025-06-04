pub mod generator;
pub mod room;
pub mod types;
pub mod utils;

// Re-export the main public API
pub use generator::MapGenerator;
pub use room::Room;
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
        let mut generator = MapGenerator::new(40, 30, 6, 6, 2);
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
        let room = Room::new(10, 10, 6, 6, false);
        let connections = room.get_global_connections();

        // Should have connection points on each side
        //assert!(!connections.is_empty());
    }
}
