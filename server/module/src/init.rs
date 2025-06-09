use crate::tables::{game_info, map, GameInfo, Map, MapType};
use crate::types::Vec2;
use game_module::map_generator;
use spacetimedb::{reducer, ReducerContext, Table};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    // Called when the module is initially published
    log::info!("Initializing game world...");

    // Generate the starting town and get its ID
    let starting_town_id = match generate_starting_town(ctx) {
        Some(id) => id,
        None => {
            log::error!("Failed to generate starting town");
            return;
        }
    };

    // Generate exploration dungeon
    generate_exploration_dungeon(ctx);

    // Initialize game info with the starting town
    if let Err(e) = initialize_game_info(ctx, starting_town_id) {
        log::error!("Failed to initialize game info: {}", e);
        return;
    }

    // Initialize other game systems
    initialize_game_systems(ctx);

    log::info!("Game world initialization complete!");
}

/// Initialize game info with starting town
fn initialize_game_info(ctx: &ReducerContext, starting_town_map_id: u64) -> Result<(), String> {
    // Check if game info already exists
    if ctx.db.game_info().id().find(1).is_some() {
        log::info!("Game info already initialized");
        return Ok(());
    }

    // Create the game info record
    let game_info = GameInfo {
        id: 1, // Singleton pattern - always use ID 1
        starting_town_map_id,
        updated_at: ctx.timestamp,
    };

    ctx.db.game_info().insert(game_info);
    log::info!(
        "Game info initialized with starting town map ID: {}",
        starting_town_map_id
    );
    Ok(())
}

/// Generate the starting town and return its ID
fn generate_starting_town(ctx: &ReducerContext) -> Option<u64> {
    let town_result = map_generator::Generator::generate_town(
        "Starting Town".to_string(),
        42,   // Fixed seed for consistent starting town
        3,    // 3x3 town grid
        20,   // room width
        20,   // room height
        true, // is starting town
    )
    .map_err(|e| format!("Failed to generate starting town: {}", e));

    let town_result = match town_result {
        Ok(result) => result,
        Err(e) => {
            log::error!("{}", e);
            return None;
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

    // Insert and find the town to get its actual ID
    ctx.db.map().insert(town);

    // Find the town we just inserted to get its ID
    let town_id = ctx
        .db
        .map()
        .iter()
        .filter(|m| m.map_type == MapType::Town && m.is_starting_town)
        .max_by_key(|t| t.created_at)
        .map(|t| t.id);

    log::info!(
        "Starting town generated: {} areas, size {}x{}, {} spawn points (seed: {}, generation time: {}ms)",
        town_result.metadata.room_count,
        town_result.width,
        town_result.height,
        town_result.spawn_points.len(),
        town_result.metadata.seed,
        town_result.metadata.generation_time_ms.unwrap_or(0)
    );

    town_id
}

/// Generate exploration dungeon
fn generate_exploration_dungeon(ctx: &ReducerContext) {
    let dungeon_result = map_generator::Generator::generate_dungeon(
        "Exploration Dungeon".to_string(),
        123, // Different seed for dungeon
        10,  // 3x3 grid of rooms
        10,  // rooms height
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
        "Exploration dungeon generated: {} rooms, size {}x{} (seed: {}, generation time: {}ms)",
        dungeon_result.metadata.room_count,
        dungeon_result.width,
        dungeon_result.height,
        dungeon_result.metadata.seed,
        dungeon_result.metadata.generation_time_ms.unwrap_or(0)
    );
}

/// Initialize game systems
fn initialize_game_systems(ctx: &ReducerContext) {
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