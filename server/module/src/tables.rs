use crate::types::Vec2;
use spacetimedb::{table, Identity, Timestamp};

#[derive(spacetimedb::SpacetimeType, Clone, Debug, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Npc,
    Monster,
    Item,
}

#[table(name = entity, public)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub entity_type: EntityType,
    pub position: Vec2,
    pub direction: f64, // Direction in radians (0 = east, π/2 = north, π = west, 3π/2 = south)
    pub owner_identity: Option<Identity>, // For player entities, this is the player's identity
    pub created_at: Timestamp,
}

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub position: Vec2, // Keep for backward compatibility, but entity position should be primary
    pub entity_id: Option<u64>, // Reference to the player's entity
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
    pub entity_ids: Vec<u64>, // List of entity IDs in this dungeon
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
    pub entity_ids: Vec<u64>, // List of entity IDs in this town
    pub created_at: Timestamp,
}

// GameTick table is defined in tick.rs due to scheduled reducer requirements
