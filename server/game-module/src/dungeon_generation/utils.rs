use crate::dungeon_generation::generator::MapGenerator;

// Example usage
pub fn generate_example_map() {
    let mut generator = MapGenerator::with_seed(3, 2, 20, 20, 2, 12345); // 3x2 grid of rooms
    let _map = generator.generate();

    log::info!("Generated Map:");

    if let Some(spawn) = generator.get_spawn_room() {
        log::info!("Player spawn: ({}, {})", spawn.x, spawn.y);
    }

    if let Some(center) = generator.get_central_room_position() {
        log::info!("Central room: ({}, {})", center.x, center.y);
    }
}

// Main function logic as a utility function
pub fn run_example() {
    // Create a 60×40 map with 8×8 rooms, central room is 2×2 rooms in size
    let mut generator = MapGenerator::new(3, 2, 20, 20, 2); // 3x2 grid of rooms
    let _map = generator.generate();

    // Get spawn and goal positions
    let spawn = generator.get_spawn_room();
    let center = generator.get_central_room_position();

    log::info!("Spawn: {:?}", spawn);
    log::info!("Center: {:?}", center);
}
