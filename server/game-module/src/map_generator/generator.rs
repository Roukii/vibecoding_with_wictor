use crate::map_generator::dungeon_generator::DungeonGenerator;
use crate::map_generator::town_generator::TownGenerator;
use crate::map_generator::types::Position;

/// Enum representing different types of maps that can be generated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapType {
    Dungeon,
    Town,
    Wilderness,
    Instance,
}

/// Parameters for dungeon generation
#[derive(Debug, Clone)]
pub struct DungeonParams {
    pub rooms_width: usize,
    pub rooms_height: usize,
    pub room_width: usize,
    pub room_height: usize,
    pub central_room_multiplier: usize,
    pub central_room_template: Option<String>,
}

impl Default for DungeonParams {
    fn default() -> Self {
        Self {
            rooms_width: 3,
            rooms_height: 3,
            room_width: 20,
            room_height: 20,
            central_room_multiplier: 2,
            central_room_template: None,
        }
    }
}

/// Parameters for town generation
#[derive(Debug, Clone)]
pub struct TownParams {
    pub town_size: usize,
    pub room_width: usize,
    pub room_height: usize,
    pub is_starting_town: bool,
}

impl Default for TownParams {
    fn default() -> Self {
        Self {
            town_size: 3,
            room_width: 30,
            room_height: 30,
            is_starting_town: false,
        }
    }
}

/// Parameters for wilderness generation (placeholder for future implementation)
#[derive(Debug, Clone)]
pub struct WildernessParams {
    pub width: usize,
    pub height: usize,
    pub biome: String,
}

impl Default for WildernessParams {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            biome: "forest".to_string(),
        }
    }
}

/// Comprehensive result of map generation
#[derive(Debug, Clone)]
pub struct MapGenerationResult {
    pub map_type: MapType,
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>, // Flattened 2D array
    pub spawn_position: Position,
    pub spawn_points: Vec<Position>,
    pub is_starting_town: bool,
    pub metadata: MapMetadata,
}

/// Additional metadata about the generated map
#[derive(Debug, Clone)]
pub struct MapMetadata {
    pub room_count: usize,
    pub seed: u64,
    pub generation_time_ms: Option<u64>,
    pub special_features: Vec<String>,
}

/// Main generator that handles all map generation
pub struct Generator;

impl Generator {
    /// Generate a map of the specified type with the given parameters
    pub fn generate_map(
        map_type: MapType,
        name: String,
        seed: u64,
        params: GenerationParams,
    ) -> Result<MapGenerationResult, String> {
        let start_time = std::time::Instant::now();

        match map_type {
            MapType::Dungeon => Self::generate_dungeon_map(name, seed, params.dungeon),
            MapType::Town => Self::generate_town_map(name, seed, params.town),
            MapType::Wilderness => Self::generate_wilderness_map(name, seed, params.wilderness),
            MapType::Instance => Err("Instance generation not yet implemented".to_string()),
        }
        .map(|mut result| {
            result.metadata.generation_time_ms = Some(start_time.elapsed().as_millis() as u64);
            result
        })
    }

    /// Convenience method for quick dungeon generation
    pub fn generate_dungeon(
        name: String,
        seed: u64,
        rooms_width: usize,
        rooms_height: usize,
        room_width: usize,
        room_height: usize,
    ) -> Result<MapGenerationResult, String> {
        let params = GenerationParams {
            dungeon: DungeonParams {
                rooms_width,
                rooms_height,
                room_width,
                room_height,
                ..Default::default()
            },
            ..Default::default()
        };
        Self::generate_map(MapType::Dungeon, name, seed, params)
    }

    /// Convenience method for quick town generation
    pub fn generate_town(
        name: String,
        seed: u64,
        town_size: usize,
        room_width: usize,
        room_height: usize,
        is_starting_town: bool,
    ) -> Result<MapGenerationResult, String> {
        let params = GenerationParams {
            town: TownParams {
                town_size,
                room_width,
                room_height,
                is_starting_town,
            },
            ..Default::default()
        };
        Self::generate_map(MapType::Town, name, seed, params)
    }

    /// Generate a dungeon map
    fn generate_dungeon_map(
        name: String,
        seed: u64,
        params: DungeonParams,
    ) -> Result<MapGenerationResult, String> {
        let mut dungeon_gen = DungeonGenerator::with_seed(
            params.rooms_width,
            params.rooms_height,
            params.room_width,
            params.room_height,
            params.central_room_multiplier,
            seed,
        );

        // Set central room template if specified
        if let Some(template) = &params.central_room_template {
            dungeon_gen.set_central_room_template(template)?;
        }

        let map = dungeon_gen.generate();
        let spawn_position = dungeon_gen.get_best_spawn_point().unwrap_or(Position {
            x: dungeon_gen.width / 2,
            y: dungeon_gen.height / 2,
        });
        let spawn_points = dungeon_gen.get_spawn_points().clone();

        // Flatten the 2D map into 1D
        let tiles: Vec<u8> = map.into_iter().flatten().collect();

        let mut special_features = Vec::new();
        if params.central_room_template.is_some() {
            special_features.push("Central Room Template".to_string());
        }
        if let Some(central_pos) = dungeon_gen.get_central_room_position() {
            special_features.push(format!(
                "Central Room at ({}, {})",
                central_pos.x, central_pos.y
            ));
        }

        Ok(MapGenerationResult {
            map_type: MapType::Dungeon,
            name,
            width: dungeon_gen.width,
            height: dungeon_gen.height,
            tiles,
            spawn_position,
            spawn_points,
            is_starting_town: false,
            metadata: MapMetadata {
                room_count: dungeon_gen.rooms.len(),
                seed,
                generation_time_ms: None, // Will be set by caller
                special_features,
            },
        })
    }

    /// Generate a town map
    fn generate_town_map(
        name: String,
        seed: u64,
        params: TownParams,
    ) -> Result<MapGenerationResult, String> {
        let mut town_gen = TownGenerator::with_seed(
            params.town_size,
            params.room_width,
            params.room_height,
            seed,
        );

        let map = town_gen.generate();
        let spawn_position = town_gen.get_primary_spawn_point().unwrap_or(Position {
            x: town_gen.width / 2,
            y: town_gen.height / 2,
        });
        let spawn_points = town_gen.get_spawn_points().clone();

        // Flatten the 2D map into 1D
        let tiles: Vec<u8> = map.into_iter().flatten().collect();

        let mut special_features = Vec::new();
        if params.is_starting_town {
            special_features.push("Starting Town".to_string());
        }
        special_features.push("Town Square".to_string());

        Ok(MapGenerationResult {
            map_type: MapType::Town,
            name,
            width: town_gen.width,
            height: town_gen.height,
            tiles,
            spawn_position,
            spawn_points,
            is_starting_town: params.is_starting_town,
            metadata: MapMetadata {
                room_count: town_gen.rooms.len(),
                seed,
                generation_time_ms: None, // Will be set by caller
                special_features,
            },
        })
    }

    /// Generate a wilderness map (placeholder)
    fn generate_wilderness_map(
        name: String,
        seed: u64,
        params: WildernessParams,
    ) -> Result<MapGenerationResult, String> {
        // Placeholder implementation - generates a simple open area
        let total_tiles = params.width * params.height;
        let tiles = vec![1u8; total_tiles]; // All floor tiles

        let spawn_position = Position {
            x: params.width / 2,
            y: params.height / 2,
        };

        // Generate spawn points around the edges
        let mut spawn_points = Vec::new();
        let edge_spacing = 10;
        for i in (0..params.width).step_by(edge_spacing) {
            spawn_points.push(Position { x: i, y: 0 });
            spawn_points.push(Position {
                x: i,
                y: params.height - 1,
            });
        }
        for i in (0..params.height).step_by(edge_spacing) {
            spawn_points.push(Position { x: 0, y: i });
            spawn_points.push(Position {
                x: params.width - 1,
                y: i,
            });
        }

        Ok(MapGenerationResult {
            map_type: MapType::Wilderness,
            name,
            width: params.width,
            height: params.height,
            tiles,
            spawn_position,
            spawn_points,
            is_starting_town: false,
            metadata: MapMetadata {
                room_count: 0,
                seed,
                generation_time_ms: None,
                special_features: vec![format!("Biome: {}", params.biome)],
            },
        })
    }
}

/// Container for all generation parameters
#[derive(Debug, Clone)]
pub struct GenerationParams {
    pub dungeon: DungeonParams,
    pub town: TownParams,
    pub wilderness: WildernessParams,
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            dungeon: DungeonParams::default(),
            town: TownParams::default(),
            wilderness: WildernessParams::default(),
        }
    }
}
