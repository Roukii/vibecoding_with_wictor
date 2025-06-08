use crate::types::Vec2;
use spacetimedb::{table, Identity, Timestamp};

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub position: Vec2,
}

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    pub identity: Identity,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub online: bool,
}

#[table(name = message, public)]
pub struct Message {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub sender: Identity,
    pub sent: Timestamp,
    pub text: String,
}

#[table(name = dungeon, public)]
pub struct Dungeon {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub tiles: Vec<u8>, // Flattened 2D array: tiles[y * width + x] = tile_type (0=Wall, 1=Floor, 2=Door)
    pub spawn_position: Vec2, // Primary spawn position (best spawn point)
    pub spawn_points: Vec<Vec2>, // All possible spawn points at edges
    pub created_at: Timestamp,
}

#[table(name = town, public)]
pub struct Town {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub tiles: Vec<u8>, // Flattened 2D array: tiles[y * width + x] = tile_type (0=Wall, 1=Floor, 2=Door)
    pub spawn_position: Vec2, // Primary spawn position (center of town square)
    pub spawn_points: Vec<Vec2>, // Multiple spawn points around town square
    pub is_starting_town: bool, // Whether this is the main starting town
    pub created_at: Timestamp,
}
