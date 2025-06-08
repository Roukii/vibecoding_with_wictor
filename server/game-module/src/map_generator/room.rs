use crate::map_generator::room_manager::RoomManager;
use crate::map_generator::room_templates::RoomTemplate;
use crate::map_generator::types::{Position, TileType};
use spacetimedb::rand::Rng;

#[derive(Debug, Clone)]
pub struct Room {
    pub position: Position,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
    pub connections: Vec<Position>, // Potential connection points (relative to room)
    pub is_central: bool,
    pub template_name: Option<String>, // Track which template was used
}

impl Room {
    /// Create a room from a template using RoomManager
    pub fn from_template<R: Rng>(
        room_manager: &RoomManager,
        template: &RoomTemplate,
        x: usize,
        y: usize,
        rng: &mut R,
    ) -> Result<Self, String> {
        room_manager.create_room_from_template(template, x, y, rng)
    }

    /// Create a room from a template by name
    pub fn from_template_name<R: Rng>(
        room_manager: &RoomManager,
        template_name: &str,
        x: usize,
        y: usize,
        rng: &mut R,
    ) -> Result<Self, String> {
        let template = room_manager
            .get_template_by_name(template_name)
            .ok_or_else(|| format!("Template '{}' not found", template_name))?;

        let mut room = room_manager.create_room_from_template(template, x, y, rng)?;
        room.template_name = Some(template_name.to_string());
        Ok(room)
    }

    /// Create a random room from available templates
    pub fn random_from_templates<R: Rng>(
        room_manager: &RoomManager,
        x: usize,
        y: usize,
        prefer_central: bool,
        rng: &mut R,
    ) -> Result<Self, String> {
        let template = room_manager
            .get_random_template(rng, prefer_central)
            .ok_or_else(|| "No suitable templates found".to_string())?;

        let mut room = room_manager.create_room_from_template(template, x, y, rng)?;
        room.template_name = Some(template.name.to_string());
        Ok(room)
    }

    /// Legacy constructor - create a room procedurally (for backward compatibility)
    /// Enforces minimum room size of 20x20
    pub fn new(x: usize, y: usize, width: usize, height: usize, is_central: bool) -> Self {
        // Enforce minimum room size of 20x20
        let width = width.max(20);
        let height = height.max(20);

        let mut tiles = vec![vec![TileType::Wall; width]; height];
        let mut connections = Vec::new();

        // Fill interior with floor
        for row in 1..height - 1 {
            for col in 1..width - 1 {
                tiles[row][col] = TileType::Floor;
            }
        }

        // Add potential connection points on walls (middle of each side)
        if !is_central {
            // Top wall
            if height > 2 {
                connections.push(Position { x: width / 2, y: 0 });
            }
            // Bottom wall
            if height > 2 {
                connections.push(Position {
                    x: width / 2,
                    y: height - 1,
                });
            }
            // Left wall
            if width > 2 {
                connections.push(Position {
                    x: 0,
                    y: height / 2,
                });
            }
            // Right wall
            if width > 2 {
                connections.push(Position {
                    x: width - 1,
                    y: height / 2,
                });
            }
        } else {
            // Central room has more connection points
            for i in 1..width - 1 {
                if i % 2 == 0 {
                    connections.push(Position { x: i, y: 0 }); // Top
                    connections.push(Position {
                        x: i,
                        y: height - 1,
                    }); // Bottom
                }
            }
            for i in 1..height - 1 {
                if i % 2 == 0 {
                    connections.push(Position { x: 0, y: i }); // Left
                    connections.push(Position { x: width - 1, y: i }); // Right
                }
            }
        }

        Room {
            position: Position { x, y },
            width,
            height,
            tiles,
            connections,
            is_central,
            template_name: None,
        }
    }

    pub fn get_global_connections(&self) -> Vec<Position> {
        self.connections
            .iter()
            .map(|conn| Position {
                x: self.position.x + conn.x,
                y: self.position.y + conn.y,
            })
            .collect()
    }

    /// Place a door at a specific connection point
    pub fn add_door(&mut self, connection_index: usize) -> Result<(), String> {
        if connection_index >= self.connections.len() {
            return Err("Connection index out of bounds".to_string());
        }

        let conn = self.connections[connection_index];
        if conn.x >= self.width || conn.y >= self.height {
            return Err("Connection point outside room bounds".to_string());
        }

        self.tiles[conn.y][conn.x] = TileType::Door;
        Ok(())
    }

    /// Get the template name used to create this room (if any)
    pub fn get_template_name(&self) -> Option<&str> {
        self.template_name.as_deref()
    }
}
