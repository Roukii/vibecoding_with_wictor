use crate::dungeon_generation::types::{TileType, Position};

#[derive(Debug, Clone)]
pub struct Room {
    pub position: Position,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
    pub connections: Vec<Position>, // Potential connection points (relative to room)
    pub is_central: bool,
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize, is_central: bool) -> Self {
        let mut tiles = vec![vec![TileType::Wall; width]; height];
        let mut connections = Vec::new();

        // Fill interior with floor
        for row in 1..height-1 {
            for col in 1..width-1 {
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
                connections.push(Position { x: width / 2, y: height - 1 });
            }
            // Left wall
            if width > 2 {
                connections.push(Position { x: 0, y: height / 2 });
            }
            // Right wall
            if width > 2 {
                connections.push(Position { x: width - 1, y: height / 2 });
            }
        } else {
            // Central room has more connection points
            for i in 1..width-1 {
                if i % 2 == 0 {
                    connections.push(Position { x: i, y: 0 }); // Top
                    connections.push(Position { x: i, y: height - 1 }); // Bottom
                }
            }
            for i in 1..height-1 {
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
        }
    }

    pub fn get_global_connections(&self) -> Vec<Position> {
        self.connections.iter().map(|conn| Position {
            x: self.position.x + conn.x,
            y: self.position.y + conn.y,
        }).collect()
    }
} 