use crate::map_generator::{
    DungeonParams, GenerationParams, Generator, MapType, TileType, TownParams,
};

/// Example demonstrating the new Generator usage
pub fn demonstrate_generator_usage() {
    println!("=== Generator Usage Examples ===\n");

    // Example 1: Simple dungeon generation
    println!("1. Creating a dungeon using the Generator:");
    match Generator::generate_dungeon(
        "Example Dungeon".to_string(),
        12345,
        4,  // rooms width
        4,  // rooms height
        25, // room width
        25, // room height
    ) {
        Ok(result) => {
            println!(
                "✓ Dungeon '{}' generated successfully! Size: {}x{}, {} rooms, {} spawn points",
                result.name,
                result.width,
                result.height,
                result.metadata.room_count,
                result.spawn_points.len()
            );
            println!(
                "  Primary spawn: ({}, {}), Generation time: {}ms",
                result.spawn_position.x,
                result.spawn_position.y,
                result.metadata.generation_time_ms.unwrap_or(0)
            );
            if !result.metadata.special_features.is_empty() {
                println!("  Special features: {:?}", result.metadata.special_features);
            }
        }
        Err(e) => println!("✗ Failed to generate dungeon: {}", e),
    }

    println!();

    // Example 2: Town generation
    println!("2. Creating a town using the Generator:");
    match Generator::generate_town(
        "Example Town".to_string(),
        54321,
        3,     // town size (3x3 grid)
        30,    // room width
        30,    // room height
        false, // is starting town
    ) {
        Ok(result) => {
            println!(
                "✓ Town '{}' generated successfully! Size: {}x{}, {} areas, {} spawn points",
                result.name,
                result.width,
                result.height,
                result.metadata.room_count,
                result.spawn_points.len()
            );
            println!(
                "  Primary spawn: ({}, {}), Generation time: {}ms",
                result.spawn_position.x,
                result.spawn_position.y,
                result.metadata.generation_time_ms.unwrap_or(0)
            );
            if !result.metadata.special_features.is_empty() {
                println!("  Special features: {:?}", result.metadata.special_features);
            }
        }
        Err(e) => println!("✗ Failed to generate town: {}", e),
    }

    println!();

    // Example 3: Advanced generation with custom parameters
    println!("3. Advanced generation with custom parameters:");
    let params = GenerationParams {
        dungeon: DungeonParams {
            rooms_width: 5,
            rooms_height: 5,
            room_width: 20,
            room_height: 20,
            central_room_multiplier: 3,
            central_room_template: Some("throne_room".to_string()),
        },
        ..Default::default()
    };

    match Generator::generate_map(
        MapType::Dungeon,
        "Throne Room Dungeon".to_string(),
        99999,
        params,
    ) {
        Ok(result) => {
            println!(
                "✓ Advanced dungeon '{}' generated! Size: {}x{}, {} rooms",
                result.name, result.width, result.height, result.metadata.room_count
            );
            println!(
                "  Primary spawn: ({}, {}), Generation time: {}ms",
                result.spawn_position.x,
                result.spawn_position.y,
                result.metadata.generation_time_ms.unwrap_or(0)
            );
            if !result.metadata.special_features.is_empty() {
                println!("  Special features: {:?}", result.metadata.special_features);
            }
        }
        Err(e) => println!("✗ Failed to generate advanced dungeon: {}", e),
    }

    println!();

    // Example 4: Starting town generation
    println!("4. Creating a starting town:");
    let town_params = GenerationParams {
        town: TownParams {
            town_size: 3,
            room_width: 35,
            room_height: 35,
            is_starting_town: true,
        },
        ..Default::default()
    };

    match Generator::generate_map(MapType::Town, "Starting Town".to_string(), 42, town_params) {
        Ok(result) => {
            println!(
                "✓ Starting town '{}' generated! Size: {}x{}, {} areas",
                result.name, result.width, result.height, result.metadata.room_count
            );
            println!(
                "  Primary spawn: ({}, {}), Is starting town: {}",
                result.spawn_position.x, result.spawn_position.y, result.is_starting_town
            );
        }
        Err(e) => println!("✗ Failed to generate starting town: {}", e),
    }

    println!();

    // Example 5: Wilderness generation (placeholder)
    println!("5. Attempting wilderness generation:");
    match Generator::generate_map(
        MapType::Wilderness,
        "Test Wilderness".to_string(),
        77777,
        GenerationParams::default(),
    ) {
        Ok(result) => {
            println!(
                "✓ Wilderness '{}' generated! Size: {}x{}",
                result.name, result.width, result.height
            );
        }
        Err(e) => println!("✗ Wilderness generation failed: {}", e),
    }

    println!("\n=== End of Examples ===");
}

/// Helper function to print a small section of a map for visualization
pub fn print_map_section(
    tiles: &[u8],
    map_width: usize,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
) {
    println!(
        "Map section ({}x{}) starting at ({}, {}):",
        width, height, start_x, start_y
    );

    for y in start_y..(start_y + height) {
        for x in start_x..(start_x + width) {
            if x < map_width {
                let index = y * map_width + x;
                if index < tiles.len() {
                    let ch = match TileType::from(tiles[index]) {
                        TileType::Wall => '#',
                        TileType::Floor => '.',
                        TileType::Door => 'D',
                    };
                    print!("{}", ch);
                } else {
                    print!(" ");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

/// Compare the old and new approaches
pub fn compare_approaches() {
    println!("=== Comparing Old vs New Approaches ===\n");

    // Old approach (direct DungeonGenerator usage)
    println!("Old approach - Direct DungeonGenerator:");
    let mut old_gen = crate::map_generator::DungeonGenerator::new(3, 3, 25, 25, 2);
    let old_map = old_gen.generate();
    println!(
        "✓ Direct dungeon generated! Size: {}x{}",
        old_map.len(),
        old_map[0].len()
    );

    println!();

    // New approach (Encapsulated Generator)
    println!("New approach - Encapsulated Generator:");
    match Generator::generate_dungeon(
        "Comparison Dungeon".to_string(),
        0,  // same seed for comparison
        3,  // rooms width
        3,  // rooms height
        25, // room width
        25, // room height
    ) {
        Ok(new_result) => {
            println!(
                "✓ Encapsulated generator dungeon created! Size: {}x{}",
                new_result.width, new_result.height
            );

            // Both should produce similar results
            if old_map.len() == new_result.height && old_map[0].len() == new_result.width {
                println!("✓ Both approaches produce maps of the same size");
            }

            println!(
                "✓ New approach provides additional metadata: {} rooms, {} spawn points, generation time: {}ms",
                new_result.metadata.room_count,
                new_result.spawn_points.len(),
                new_result.metadata.generation_time_ms.unwrap_or(0)
            );
        }
        Err(e) => println!("✗ Encapsulated generator failed: {}", e),
    }

    println!("\n=== Benefits of New Approach ===");
    println!("• Encapsulated interface - only generation functions exposed");
    println!("• Comprehensive result structure with metadata");
    println!("• Unified interface for all map types");
    println!("• Better error handling");
    println!("• Performance timing included");
    println!("• Extensible for future map types");

    println!("\n=== End of Comparison ===");
}
