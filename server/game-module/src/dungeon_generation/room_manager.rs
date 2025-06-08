use crate::dungeon_generation::room_templates::RoomTemplate;
use crate::dungeon_generation::types::{Position, TileType};
use crate::dungeon_generation::ALL_TEMPLATES;
use spacetimedb::rand::Rng;

#[derive(Debug, Clone)]
pub struct ParsedRoom {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
    pub connections: Vec<Position>,
    pub is_central: bool,
}

pub struct RoomManager {
    templates: Vec<RoomTemplate>,
    central_room: Option<&'static RoomTemplate>,
}

impl RoomManager {
    pub fn new() -> Self {
        let templates = ALL_TEMPLATES.to_vec();

        RoomManager {
            templates,
            central_room: None,
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
            width,
            height,
            tiles,
            connections, // Use parsed connections from template instead of manual connection_points
            is_central: template.is_central,
        })
    }

    pub fn get_random_template<R: Rng>(
        &self,
        rng: &mut R,
        prefer_central: bool,
    ) -> Option<&RoomTemplate> {
        let candidates: Vec<&RoomTemplate> = if prefer_central {
            self.templates.iter().filter(|t| t.is_central).collect()
        } else {
            self.templates.iter().filter(|t| !t.is_central).collect()
        };

        if candidates.is_empty() {
            return None;
        }

        let total_weight: u32 = candidates.iter().map(|t| t.weight).sum();
        let mut target = rng.gen_range(0..total_weight);

        for template in &candidates {
            if target < template.weight {
                return Some(template);
            }
            target -= template.weight;
        }

        candidates.last().copied()
    }

    pub fn get_template_by_name(&self, name: &str) -> Option<&RoomTemplate> {
        self.templates.iter().find(|t| t.name == name)
    }

    pub fn get_all_templates(&self) -> &[RoomTemplate] {
        &self.templates
    }

    /// Set a specific central room template to use
    pub fn set_central_room(&mut self, template_name: &str) -> Result<(), String> {
        let _template = self
            .templates
            .iter()
            .find(|t| t.name == template_name && t.is_central)
            .ok_or_else(|| format!("Central room template '{}' not found", template_name))?;

        // Store a reference to the template from ALL_TEMPLATES
        self.central_room = ALL_TEMPLATES.iter().find(|t| t.name == template_name);

        Ok(())
    }

    /// Get the currently set central room template
    pub fn get_central_room(&self) -> Option<&RoomTemplate> {
        self.central_room
    }

    /// Clear the central room template (use random selection)
    pub fn clear_central_room(&mut self) {
        self.central_room = None;
    }

    /// Get a central room template - either the specifically set one or a random one
    pub fn get_central_template<R: Rng>(&self, rng: &mut R) -> Option<&RoomTemplate> {
        if let Some(central_template) = self.central_room {
            Some(central_template)
        } else {
            self.get_random_template(rng, true)
        }
    }

    pub fn create_room_from_template<R: Rng>(
        &self,
        template: &RoomTemplate,
        x: usize,
        y: usize,
        _rng: &mut R,
    ) -> Result<crate::dungeon_generation::room::Room, String> {
        let parsed = Self::parse_room_template(template)?;

        Ok(crate::dungeon_generation::room::Room {
            position: Position { x, y },
            width: parsed.width,
            height: parsed.height,
            tiles: parsed.tiles,
            connections: parsed.connections,
            is_central: parsed.is_central,
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
    ) -> Result<crate::dungeon_generation::room::Room, String> {
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

        Ok(crate::dungeon_generation::room::Room {
            position: Position { x, y },
            width: final_width,
            height: final_height,
            tiles: final_tiles,
            connections: final_connections,
            is_central: parsed.is_central,
            template_name: Some(template.name.to_string()),
        })
    }
}

impl Default for RoomManager {
    fn default() -> Self {
        Self::new()
    }
}
