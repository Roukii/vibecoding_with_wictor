use crate::map_generator::room_templates::RoomTemplate;
use crate::map_generator::room_templates::{DUNGEON_TEMPLATES, TOWN_TEMPLATES};
use crate::map_generator::types::{Position, TileType};
use spacetimedb::rand::Rng;
use std::collections::HashMap;

// Re-export RoomType for external use
pub use crate::map_generator::room_templates::RoomType;

#[derive(Debug, Clone)]
pub struct ParsedRoom {
    pub name: String,
    pub room_type: RoomType,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
    pub connections: Vec<Position>,
    pub spawn_points: Vec<Position>,
    pub is_central: bool,
}

/// Configuration for room type weights
#[derive(Debug, Clone)]
pub struct RoomTypeWeights {
    pub weights: HashMap<RoomType, u32>,
}

impl RoomTypeWeights {
    /// Create default weights for town generation
    pub fn default_town() -> Self {
        let mut weights = HashMap::new();

        // Use the default weights from RoomType enum
        for &room_type in &[RoomType::Town] {
            weights.insert(room_type, room_type.default_dungeon_weight());
        }

        Self { weights }
    }

    /// Create default weights for dungeon generation
    pub fn default_dungeon() -> Self {
        let mut weights = HashMap::new();

        // Use the default weights from RoomType enum
        for &room_type in &[
            RoomType::Combat,
            RoomType::Treasure,
            RoomType::Central,
            RoomType::Rest,
            RoomType::Spawn,
        ] {
            weights.insert(room_type, room_type.default_dungeon_weight());
        }

        Self { weights }
    }

    /// Create custom weights
    pub fn custom(weights: HashMap<RoomType, u32>) -> Self {
        Self { weights }
    }

    /// Check if these weights are primarily for dungeon generation
    pub fn is_dungeon_weights(&self) -> bool {
        // Check if we have any dungeon-specific types (excluding Central which is shared)
        self.weights.keys().any(|room_type| {
            matches!(
                room_type,
                RoomType::Combat | RoomType::Treasure | RoomType::Rest | RoomType::Spawn
            )
        })
    }
}

pub struct RoomManager {
    templates: Vec<RoomTemplate>,
    central_room_template_name: Option<String>,
    room_type_weights: RoomTypeWeights,
}

impl RoomManager {
    /// Create a room manager with specific room type weights
    /// Automatically selects appropriate template collection based on weights
    pub fn with_weights(weights: RoomTypeWeights) -> Self {
        let templates = if weights.is_dungeon_weights() {
            DUNGEON_TEMPLATES.to_vec()
        } else {
            TOWN_TEMPLATES.to_vec()
        };

        RoomManager {
            templates,
            central_room_template_name: None,
            room_type_weights: weights,
        }
    }

    /// Create a room manager for dungeons
    pub fn for_dungeons() -> Self {
        let templates = DUNGEON_TEMPLATES.to_vec();

        RoomManager {
            templates,
            central_room_template_name: None,
            room_type_weights: RoomTypeWeights::default_dungeon(),
        }
    }

    /// Create a room manager for towns
    pub fn for_towns() -> Self {
        let templates = TOWN_TEMPLATES.to_vec();

        RoomManager {
            templates,
            central_room_template_name: None,
            room_type_weights: RoomTypeWeights::default_town(),
        }
    }

    pub fn parse_room_template(template: &RoomTemplate) -> Result<ParsedRoom, String> {
        let lines: Vec<&str> = template.template.trim().lines().collect();

        if lines.is_empty() {
            return Err("Empty template".to_string());
        }

        let height = lines.len();
        let width = lines[0].len();

        // Validate that all lines have the same width
        for line in &lines {
            if line.len() != width {
                return Err(format!(
                    "Inconsistent line width in template '{}'",
                    template.name
                ));
            }
        }

        let mut tiles = vec![vec![TileType::Wall; width]; height];
        let mut connections = Vec::new();
        let mut spawn_points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                tiles[y][x] = match ch {
                    '#' => TileType::Wall,
                    '.' => TileType::Floor,
                    'D' => TileType::Door,
                    'C' => {
                        // Connection points are parsed from template and stored as wall tiles
                        connections.push(Position { x, y });
                        TileType::Wall
                    }
                    'T' => TileType::Floor, // Throne or special floor tile
                    ' ' => TileType::Wall,  // Spaces are treated as walls
                    'S' => {
                        // S represents a spawn point (which is a floor tile)
                        spawn_points.push(Position { x, y });
                        TileType::Floor
                    }
                    _ => {
                        return Err(format!(
                            "Invalid character '{}' in template '{}'",
                            ch, template.name
                        ))
                    }
                };
            }
        }

        Ok(ParsedRoom {
            name: template.name.to_string(),
            room_type: template.room_type,
            width,
            height,
            tiles,
            connections, // Use parsed connections from template instead of manual connection_points
            spawn_points,
            is_central: template.is_central,
        })
    }

    /// Select a random room type based on configured weights
    pub fn select_room_type<R: Rng>(&self, rng: &mut R, prefer_central: bool) -> Option<RoomType> {
        if prefer_central {
            // For central rooms, always return Central type
            return Some(RoomType::Central);
        }

        // For regular rooms, exclude Central and Boss types
        let eligible_weights: Vec<(RoomType, u32)> = self
            .room_type_weights
            .weights
            .iter()
            .filter(|(&room_type, _)| !matches!(room_type, RoomType::Central))
            .map(|(&room_type, &weight)| (room_type, weight))
            .collect();

        let total_weight: u32 = eligible_weights.iter().map(|(_, weight)| weight).sum();
        if total_weight == 0 {
            return None;
        }

        let mut target = rng.gen_range(0..total_weight);

        for (room_type, weight) in eligible_weights {
            if weight > 0 && target < weight {
                return Some(room_type);
            }
            target = target.saturating_sub(weight);
        }

        // Fallback - return first available non-central type
        self.room_type_weights
            .weights
            .keys()
            .find(|&room_type| !matches!(room_type, RoomType::Central))
            .copied()
    }

    /// Get a random template of a specific room type
    pub fn get_random_template_by_type<R: Rng>(
        &self,
        rng: &mut R,
        room_type: RoomType,
    ) -> Option<&RoomTemplate> {
        let candidates: Vec<&RoomTemplate> = self
            .templates
            .iter()
            .filter(|t| t.room_type == room_type)
            .collect();

        if candidates.is_empty() {
            return None;
        }

        let total_weight: u32 = candidates.iter().map(|t| t.weight).sum();
        if total_weight == 0 {
            return candidates.first().copied();
        }

        let mut target = rng.gen_range(0..total_weight);

        for template in &candidates {
            if target < template.weight {
                return Some(template);
            }
            target -= template.weight;
        }

        candidates.last().copied()
    }

    /// Get a random room template based on room type weights
    pub fn get_weighted_random_template<R: Rng>(
        &self,
        rng: &mut R,
        prefer_central: bool,
    ) -> Option<&RoomTemplate> {
        // First, select a room type based on weights
        let selected_type = self.select_room_type(rng, prefer_central)?;

        // Then, select a template of that type
        self.get_random_template_by_type(rng, selected_type)
    }

    pub fn get_random_template<R: Rng>(
        &self,
        rng: &mut R,
        prefer_central: bool,
    ) -> Option<&RoomTemplate> {
        // Use the new weighted selection method
        self.get_weighted_random_template(rng, prefer_central)
    }

    pub fn get_template_by_name(&self, name: &str) -> Option<&RoomTemplate> {
        self.templates.iter().find(|t| t.name == name)
    }

    /// Set a specific central room template to use
    pub fn set_central_room(&mut self, template_name: &str) -> Result<(), String> {
        let _template = self
            .templates
            .iter()
            .find(|t| t.name == template_name && t.is_central)
            .ok_or_else(|| format!("Central room template '{}' not found", template_name))?;

        // Store a reference to the template from current templates
        self.central_room_template_name = Some(template_name.to_string());

        Ok(())
    }

    /// Get a central room template - either the specifically set one or a random one
    pub fn get_central_template<R: Rng>(&self, rng: &mut R) -> Option<&RoomTemplate> {
        if let Some(central_template_name) = self.central_room_template_name.as_ref() {
            self.get_template_by_name(central_template_name)
        } else {
            self.get_random_template_by_type(rng, RoomType::Central)
        }
    }

    pub fn create_room_from_template<R: Rng>(
        &self,
        template: &RoomTemplate,
        x: usize,
        y: usize,
        _rng: &mut R,
    ) -> Result<crate::map_generator::room::Room, String> {
        let parsed = Self::parse_room_template(template)?;

        Ok(crate::map_generator::room::Room {
            position: Position { x, y },
            width: parsed.width,
            height: parsed.height,
            tiles: parsed.tiles,
            connections: parsed.connections,
            spawn_points: parsed.spawn_points,
            is_central: parsed.is_central,
            room_type: parsed.room_type,
            template_name: Some(template.name.to_string()),
        })
    }

    pub fn create_room_from_template_with_min_size<R: Rng>(
        &self,
        template: &RoomTemplate,
        x: usize,
        y: usize,
        min_width: usize,
        min_height: usize,
        _rng: &mut R,
    ) -> Result<crate::map_generator::room::Room, String> {
        let parsed = Self::parse_room_template(template)?;

        // Use the larger of template size or minimum size
        let final_width = parsed.width.max(min_width);
        let final_height = parsed.height.max(min_height);

        // If we need to scale up, create a new tile grid and adjust connections
        let (final_tiles, final_connections) =
            if final_width > parsed.width || final_height > parsed.height {
                // Create a larger room with walls
                let mut new_tiles = vec![vec![TileType::Wall; final_width]; final_height];

                // Copy the original template in the center
                let offset_x = (final_width - parsed.width) / 2;
                let offset_y = (final_height - parsed.height) / 2;

                for (y, row) in parsed.tiles.iter().enumerate() {
                    for (x, &tile) in row.iter().enumerate() {
                        if offset_y + y < final_height && offset_x + x < final_width {
                            new_tiles[offset_y + y][offset_x + x] = tile;
                        }
                    }
                }

                // Adjust connection points for the new offset
                let adjusted_connections = parsed
                    .connections
                    .iter()
                    .map(|pos| Position {
                        x: pos.x + offset_x,
                        y: pos.y + offset_y,
                    })
                    .collect();

                (new_tiles, adjusted_connections)
            } else {
                (parsed.tiles, parsed.connections.clone())
            };

        Ok(crate::map_generator::room::Room {
            position: Position { x, y },
            width: final_width,
            height: final_height,
            tiles: final_tiles,
            connections: final_connections,
            spawn_points: parsed.spawn_points,
            is_central: parsed.is_central,
            room_type: parsed.room_type,
            template_name: Some(template.name.to_string()),
        })
    }
}

impl Default for RoomManager {
    fn default() -> Self {
        Self::for_towns()
    }
}
