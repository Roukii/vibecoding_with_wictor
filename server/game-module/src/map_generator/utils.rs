use crate::map_generator::room_templates::RoomType;
use crate::map_generator::types::{Position, TileType};

/// Utility functions for room creation
pub struct RoomUtils;

impl RoomUtils {
    /// Simple constructor for internal generator use only
    /// This is kept for backward compatibility with existing generators
    pub(crate) fn create_room(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        is_central: bool,
    ) -> crate::map_generator::room::Room {
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

        // Assign a default room type based on whether it's central
        let room_type = if is_central {
            RoomType::Central
        } else {
            RoomType::Combat // Default fallback
        };

        crate::map_generator::room::Room {
            position: Position { x, y },
            width,
            height,
            tiles,
            connections,
            spawn_points: Vec::new(), // No spawn points for programmatically created rooms
            is_central,
            room_type,
            template_name: None,
        }
    }
}
