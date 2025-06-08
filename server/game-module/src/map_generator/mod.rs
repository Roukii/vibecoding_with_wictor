pub mod dungeon_generator;
pub mod generator;
pub mod room;
pub mod room_manager;
pub mod room_templates;
pub mod town_generator;
pub mod types;
pub mod utils;

// Re-export the main public API
pub use generator::{
    DungeonParams, GenerationParams, Generator, MapGenerationResult, MapMetadata, MapType,
    TownParams, WildernessParams,
};
pub use types::{Position, TileType};

// Internal API for advanced usage
pub use room::Room;
pub use room_manager::RoomManager;
pub use room_templates::{RoomTemplate, DUNGEON_TEMPLATES, TOWN_TEMPLATES};
pub use town_generator::TownGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_generator::{dungeon_generator::DungeonGenerator, room_manager::RoomType};
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
        let mut generator = DungeonGenerator::new(6, 6, 20, 20, 2, 10); // 2x2 rooms, each 20x20
        let map = generator.generate();

        print_map(&map);

        //Basic sanity checks
        assert_eq!(map.len(), 115);
        assert_eq!(map[0].len(), 115);

        //Should have at least a central room
        assert!(!generator.rooms.is_empty());
        assert!(generator.rooms.iter().any(|room| room.is_central));
    }

    #[test]
    fn test_template_room_creation() {
        // Test that rooms created from templates work properly
        let room_manager = RoomManager::for_dungeons();
        let mut rng = spacetimedb::rand::rngs::StdRng::seed_from_u64(42);

        // Try to create a room of a specific type that we know exists
        let combat_room = Room::random_from_type(&room_manager, 0, 0, RoomType::Combat, &mut rng)
            .expect("Should be able to create combat room from templates");
        assert!(combat_room.width >= 20);
        assert!(combat_room.height >= 20);
        assert_eq!(combat_room.get_room_type(), RoomType::Combat);

        // Test weighted random selection (non-central)
        let random_room = Room::random_from_templates(&room_manager, 0, 0, false, &mut rng)
            .expect("Should be able to create random non-central room from templates");
        assert!(random_room.width >= 20);
        assert!(random_room.height >= 20);
        // Should not be central or boss type for non-central selection
        assert!(!matches!(
            random_room.get_room_type(),
            RoomType::Central
        ));
    }

    #[test]
    fn test_town_generation() {
        // Test town generation using template-based approach
        let mut town_generator = TownGenerator::new();
        let map = town_generator.generate();

        // Basic sanity checks
        assert!(!map.is_empty(), "Town map should not be empty");
        assert!(!map[0].is_empty(), "Town map rows should not be empty");

        // Should have the correct dimensions (30x30 from town square template)
        assert_eq!(map.len(), 30, "Town height should be correct");
        assert_eq!(map[0].len(), 30, "Town width should be correct");

        // Should have generated a room
        assert!(town_generator.room.is_some(), "Town should have a room");

        // Should have a central room (town square)
        if let Some(room) = &town_generator.room {
            assert!(
                room.is_central,
                "Town should have a central area (town square)"
            );
        }

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

        for row in &map {
            for &tile in row {
                match TileType::from(tile) {
                    TileType::Floor => has_floor = true,
                    TileType::Door => {} // Doors are expected but not required for this test
                    TileType::Wall => {}
                }
            }
        }

        assert!(has_floor, "Town should have walkable floor areas");

        // println!("Generated town map:");
        // print_map(&map);
    }

    #[test]
    fn test_town_templates() {
        // Test that all town templates are valid and can be parsed
        for template in TOWN_TEMPLATES {
            let room_manager = RoomManager::for_towns();
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

        //println!("All {} town templates are valid!", ALL_TOWN_TEMPLATES.len());
    }

    #[test]
    fn test_town_spawn_points() {
        // Test spawn point generation specifically
        let mut town_generator = TownGenerator::new();
        let _map = town_generator.generate();

        let spawn_points = town_generator.get_spawn_points();
        assert!(!spawn_points.is_empty(), "Should have spawn points");

        // Test getting a random spawn point
        let mut town_generator_mut = TownGenerator::new();
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
    fn test_town_structure() {
        // Test that the town square template has proper structure
        let mut town_generator = TownGenerator::new();
        let map = town_generator.generate();

        //println!("Testing town structure:");
        //print_map(&map);

        // Verify that the map contains different tile types indicating structure
        let mut wall_count = 0;
        let mut floor_count = 0;
        let mut connection_count = 0;

        for row in &map {
            for &tile in row {
                match TileType::from(tile) {
                    TileType::Wall => wall_count += 1,
                    TileType::Floor => floor_count += 1,
                    TileType::Door => connection_count += 1,
                }
            }
        }

        assert!(
            wall_count > 0,
            "Town should have walls (building structure)"
        );
        assert!(floor_count > 0, "Town should have floors (walkable areas)");

        // Check that the town square has reasonable structure (mix of walls and floors)
        let total_tiles = wall_count + floor_count + connection_count;
        let floor_ratio = floor_count as f32 / total_tiles as f32;
        assert!(
            floor_ratio > 0.1 && floor_ratio < 0.9,
            "Town should have a reasonable balance of floors and walls, got floor ratio: {}",
            floor_ratio
        );

        // Verify the template has the expected 30x30 dimensions
        assert_eq!(map.len(), 30, "Town should be 30 tiles high");
        assert_eq!(map[0].len(), 30, "Town should be 30 tiles wide");
    }
}
