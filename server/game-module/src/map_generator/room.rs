use crate::map_generator::room_manager::RoomManager;
use crate::map_generator::room_templates::{RoomTemplate, RoomType};
use crate::map_generator::types::{Position, TileType};
use spacetimedb::rand::Rng;

#[derive(Debug, Clone)]
pub struct Room {
    pub position: Position,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
    pub connections: Vec<Position>, // Potential connection points (relative to room)
    pub spawn_points: Vec<Position>, // Spawn points marked in template (relative to room)
    pub is_central: bool,
    pub room_type: RoomType,
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

    /// Create a random room of a specific type
    pub fn random_from_type<R: Rng>(
        room_manager: &RoomManager,
        x: usize,
        y: usize,
        room_type: RoomType,
        rng: &mut R,
    ) -> Result<Self, String> {
        let template = room_manager
            .get_random_template_by_type(rng, room_type)
            .ok_or_else(|| format!("No templates found for room type {:?}", room_type))?;

        let mut room = room_manager.create_room_from_template(template, x, y, rng)?;
        room.template_name = Some(template.name.to_string());
        Ok(room)
    }

    /// Simple constructor for internal generator use only
    /// This is kept for backward compatibility with existing generators
    pub(crate) fn new(x: usize, y: usize, width: usize, height: usize, is_central: bool) -> Self {
        crate::map_generator::utils::RoomUtils::create_room(x, y, width, height, is_central)
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

    /// Get spawn points in global coordinates
    pub fn get_global_spawn_points(&self) -> Vec<Position> {
        self.spawn_points
            .iter()
            .map(|spawn| Position {
                x: self.position.x + spawn.x,
                y: self.position.y + spawn.y,
            })
            .collect()
    }

    /// Get the template name used to create this room (if any)
    pub fn get_template_name(&self) -> Option<&str> {
        self.template_name.as_deref()
    }

    /// Get the room type
    pub fn get_room_type(&self) -> RoomType {
        self.room_type
    }
}
