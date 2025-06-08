use crate::tables::{map, Map, MapType};
use crate::types::Vec2;
use game_module::map_generator;
use spacetimedb::{reducer, ReducerContext, Table};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    // Called when the module is initially published
    // Generate a starting town where all players will spawn
    let town_result = map_generator::Generator::generate_town(
        "Starting Town".to_string(),
        42,   // Fixed seed for consistent starting town
        3,    // 3x3 town grid
        30,   // room width
        30,   // room height
        true, // is starting town
    )
    .map_err(|e| format!("Failed to generate starting town: {}", e));

    let town_result = match town_result {
        Ok(result) => result,
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };

    // Convert spawn position and points to Vec2
    let spawn_position = Vec2 {
        x: town_result.spawn_position.x as f64,
        y: town_result.spawn_position.y as f64,
    };

    let spawn_points: Vec<Vec2> = town_result
        .spawn_points
        .iter()
        .map(|pos| Vec2 {
            x: pos.x as f64,
            y: pos.y as f64,
        })
        .collect();

    // Create and store the starting town
    let town = Map {
        id: 0, // auto_inc will handle this
        name: town_result.name,
        map_type: crate::tables::MapType::Town,
        width: town_result.width as u64,
        height: town_result.height as u64,
        tiles: town_result.tiles,
        spawn_position,
        spawn_points,
        is_starting_town: town_result.is_starting_town,
        entity_ids: Vec::new(), // Initially no entities
        created_at: ctx.timestamp,
    };

    ctx.db.map().insert(town);

    // Also generate a small dungeon for exploration
    let dungeon_result = map_generator::Generator::generate_dungeon(
        "Exploration Dungeon".to_string(),
        123, // Different seed for dungeon
        3,   // 3x3 grid of rooms
        3,   // rooms height
        20,  // room width
        20,  // room height
    )
    .map_err(|e| format!("Failed to generate dungeon: {}", e));

    let dungeon_result = match dungeon_result {
        Ok(result) => result,
        Err(e) => {
            log::error!("{}", e);
            return;
        }
    };

    // Convert spawn position and points to Vec2
    let dungeon_spawn_position = Vec2 {
        x: dungeon_result.spawn_position.x as f64,
        y: dungeon_result.spawn_position.y as f64,
    };

    let dungeon_spawn_points: Vec<Vec2> = dungeon_result
        .spawn_points
        .iter()
        .map(|pos| Vec2 {
            x: pos.x as f64,
            y: pos.y as f64,
        })
        .collect();

    let dungeon = Map {
        id: 0,
        name: dungeon_result.name,
        map_type: crate::tables::MapType::Dungeon,
        width: dungeon_result.width as u64,
        height: dungeon_result.height as u64,
        tiles: dungeon_result.tiles,
        spawn_position: dungeon_spawn_position,
        spawn_points: dungeon_spawn_points,
        is_starting_town: false,
        entity_ids: Vec::new(), // Initially no entities
        created_at: ctx.timestamp,
    };

    ctx.db.map().insert(dungeon);

    log::info!(
        "Starting town generated: {} areas, size {}x{}, {} spawn points (seed: {}, generation time: {}ms)",
        town_result.metadata.room_count,
        town_result.width,
        town_result.height,
        town_result.spawn_points.len(),
        town_result.metadata.seed,
        town_result.metadata.generation_time_ms.unwrap_or(0)
    );
    log::info!(
        "Exploration dungeon generated: {} rooms, size {}x{} (seed: {}, generation time: {}ms)",
        dungeon_result.metadata.room_count,
        dungeon_result.width,
        dungeon_result.height,
        dungeon_result.metadata.seed,
        dungeon_result.metadata.generation_time_ms.unwrap_or(0)
    );

    // Initialize the tick system to run continuously
    match crate::tick::initialize_tick_system(ctx) {
        Ok(()) => log::info!("Tick system initialized successfully"),
        Err(e) => log::error!("Failed to initialize tick system: {}", e),
    }
}

#[reducer]
pub fn get_latest_dungeon(ctx: &ReducerContext) -> Result<(), String> {
    // Find the most recently created dungeon
    if let Some(dungeon) = ctx
        .db
        .map()
        .iter()
        .filter(|m| m.map_type == MapType::Dungeon)
        .max_by_key(|d| d.created_at)
    {
        log::info!(
            "Latest dungeon: '{}' ({}x{}) created at {:?}",
            dungeon.name,
            dungeon.width,
            dungeon.height,
            dungeon.created_at
        );
        Ok(())
    } else {
        Err("No dungeons found. Generate one first!".to_string())
    }
}

#[reducer]
pub fn get_starting_town(ctx: &ReducerContext) -> Result<(), String> {
    // Find the starting town
    if let Some(town) = ctx
        .db
        .map()
        .iter()
        .filter(|m| m.map_type == MapType::Town)
        .find(|t| t.is_starting_town)
    {
        log::info!(
            "Starting town: '{}' ({}x{}) with {} spawn points, created at {:?}",
            town.name,
            town.width,
            town.height,
            town.spawn_points.len(),
            town.created_at
        );
        Ok(())
    } else {
        Err("No starting town found!".to_string())
    }
}
