#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall = 0,
    Floor = 1,
    Door = 2,
}

impl From<u8> for TileType {
    fn from(value: u8) -> Self {
        match value {
            0 => TileType::Wall,
            1 => TileType::Floor,
            2 => TileType::Door,
            _ => TileType::Wall,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
} 