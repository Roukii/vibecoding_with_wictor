pub mod generator;
pub mod room;
pub mod room_manager;
pub mod room_templates;
pub mod template_example;
pub mod town_generator;
pub mod types;
pub mod utils;

// Re-export the main public API
pub use generator::MapGenerator;
pub use room::Room;
pub use room_manager::RoomManager;
pub use room_templates::{RoomTemplate, ALL_TEMPLATES, ALL_TOWN_TEMPLATES};
pub use template_example::{demonstrate_template_system, TemplateMapGenerator};
pub use town_generator::TownGenerator;
pub use types::{Position, TileType};
pub use utils::{generate_example_map, run_example};

#[cfg(test)]
mod tests {
    use super::*;
    use spacetimedb::rand::SeedableRng;

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
        let mut generator = MapGenerator::new(6, 6, 20, 20, 2); // 2x2 rooms, each 20x20
        let map = generator.generate();

        //print_map(&map);

        //Basic sanity checks
        assert_eq!(map.len(), 30);
        assert_eq!(map[0].len(), 40);

        //Should have at least a central room
        assert!(!generator.rooms.is_empty());
        assert!(generator.rooms.iter().any(|room| room.is_central));
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

    #[test]
    fn test_town_generation() {
        // Test town generation with 3x3 grid, 30x30 areas
        let mut town_generator = TownGenerator::new(3, 30, 30);
        let map = town_generator.generate();

        // Basic sanity checks
        assert!(!map.is_empty(), "Town map should not be empty");
        assert!(!map[0].is_empty(), "Town map rows should not be empty");

        // Should have the correct dimensions
        // 3 areas of 30 tiles each with 2 shared walls = 30 + 29 + 29 = 88
        let expected_size = 30 + (3 - 1) * (30 - 1); // 88
        assert_eq!(map.len(), expected_size, "Town height should be correct");
        assert_eq!(map[0].len(), expected_size, "Town width should be correct");

        // Should have generated rooms (town areas)
        assert!(
            !town_generator.rooms.is_empty(),
            "Town should have rooms/areas"
        );

        // Should have a central room (town square)
        assert!(
            town_generator.rooms.iter().any(|room| room.is_central),
            "Town should have a central area (town square)"
        );

        // Should have spawn points
        assert!(
            !town_generator.get_spawn_points().is_empty(),
            "Town should have spawn points"
        );

        // Should have a primary spawn point
        assert!(
            town_generator.get_primary_spawn_point().is_some(),
            "Town should have a primary spawn point"
        );

        // Check that spawn points are within map bounds
        for spawn_point in town_generator.get_spawn_points() {
            assert!(
                spawn_point.x < town_generator.width,
                "Spawn point x should be within map bounds"
            );
            assert!(
                spawn_point.y < town_generator.height,
                "Spawn point y should be within map bounds"
            );
        }

        // Verify that the map contains walkable areas (floors and doors)
        let mut has_floor = false;
        let mut has_door = false;

        for row in &map {
            for &tile in row {
                match TileType::from(tile) {
                    TileType::Floor => has_floor = true,
                    TileType::Door => has_door = true,
                    TileType::Wall => {}
                }
            }
        }

        assert!(has_floor, "Town should have walkable floor areas");
        assert!(has_door, "Town should have doors connecting areas");


        // println!("Generated town map:");
        // print_map(&map);
    }

    #[test]
    fn test_town_templates() {
        // Test that all town templates are valid and can be parsed
        for template in ALL_TOWN_TEMPLATES {
            let room_manager = RoomManager::new();
            let parsed_result = RoomManager::parse_room_template(template);

            assert!(
                parsed_result.is_ok(),
                "Town template '{}' should be parseable",
                template.name
            );

            let parsed = parsed_result.unwrap();
            assert!(!parsed.name.is_empty(), "Template should have a name");
            assert!(parsed.width > 0, "Template should have valid width");
            assert!(parsed.height > 0, "Template should have valid height");
            assert!(!parsed.tiles.is_empty(), "Template should have tiles");

            // Test creating a room from the template
            let room_result = room_manager.create_room_from_template(
                template,
                0,
                0,
                &mut spacetimedb::rand::rngs::StdRng::seed_from_u64(42),
            );

            assert!(
                room_result.is_ok(),
                "Should be able to create room from template '{}'",
                template.name
            );
        }

        println!("All {} town templates are valid!", ALL_TOWN_TEMPLATES.len());
    }

    #[test]
    fn test_town_spawn_points() {
        // Test spawn point generation specifically
        let mut town_generator = TownGenerator::new(3, 30, 30);
        let _map = town_generator.generate();

        let spawn_points = town_generator.get_spawn_points();
        assert!(!spawn_points.is_empty(), "Should have spawn points");

        // Test getting a random spawn point
        let mut town_generator_mut = TownGenerator::new(3, 30, 30);
        let _map = town_generator_mut.generate();

        let random_spawn = town_generator_mut.get_random_spawn_point();
        assert!(
            random_spawn.is_some(),
            "Should be able to get a random spawn point"
        );

        // Test primary spawn point
        let primary_spawn = town_generator.get_primary_spawn_point();
        assert!(primary_spawn.is_some(), "Should have a primary spawn point");

        if let Some(primary) = primary_spawn {
            assert!(
                primary.x < town_generator.width && primary.y < town_generator.height,
                "Primary spawn point should be within map bounds"
            );
        }
    }

    #[test]
    fn test_town_has_streets() {
        // Test that the town has open streets (floor tiles) between buildings
        let mut town_generator = TownGenerator::new(3, 30, 30);
        let map = town_generator.generate();

        println!("Testing town streets:");
        print_map(&map);

        // Check for streets (floor tiles) between building areas
        // Sample some areas that should be streets
        let mut street_tiles_found = 0;
        let mut total_samples = 0;

        // Sample the middle areas between buildings (should be streets)
        for y in [15, 44, 73] {
            // These are approximate middle points between 30x30 building areas
            for x in [15, 44, 73] {
                if y < map.len() && x < map[0].len() {
                    total_samples += 1;
                    if map[y][x] == TileType::Floor as u8 {
                        street_tiles_found += 1;
                    }
                }
            }
        }

        // We should have at least some street tiles in the sampled areas
        assert!(
            street_tiles_found > 0,
            "Town should have street tiles (floors) between buildings, found {} street tiles out of {} samples",
            street_tiles_found,
            total_samples
        );

        // Also verify that not everything is just walls or just floors
        let mut wall_count = 0;
        let mut floor_count = 0;
        let mut door_count = 0;

        for row in &map {
            for &tile in row {
                match TileType::from(tile) {
                    TileType::Wall => wall_count += 1,
                    TileType::Floor => floor_count += 1,
                    TileType::Door => door_count += 1,
                }
            }
        }

        assert!(
            wall_count > 0,
            "Town should have walls (building structure)"
        );
        assert!(
            floor_count > 0,
            "Town should have floors (streets and interiors)"
        );
        assert!(
            door_count > 0,
            "Town should have doors (building entrances)"
        );
    }
}
