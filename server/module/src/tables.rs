use crate::types::Vec2;
use spacetimedb::{table, Identity, Timestamp};

#[derive(spacetimedb::SpacetimeType, Clone, Debug, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Npc,
    Monster,
    Summoned,
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
    pub entity_id: Option<u64>,
    pub current_map_id: Option<u64>, // The map the player is currently in
}

#[table(name = player_offline, public)]
pub struct PlayerOffline {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub entity_id: Option<u64>,
    pub current_map_id: Option<u64>, // The map the player was in when they went offline
    pub last_seen: Timestamp,
}

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    pub identity: Identity,
    pub name: Option<String>,
    pub online: bool,
}

#[table(name = game_info, public)]
pub struct GameInfo {
    #[primary_key]
    pub id: u64, // Using a constant ID (e.g., 1) for singleton pattern
    pub starting_town_map_id: u64,
    pub updated_at: Timestamp,
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

#[derive(spacetimedb::SpacetimeType, Clone, Debug, PartialEq, Eq)]
pub enum MapType {
    Dungeon,
    Town,
    Wilderness,
    Instance,
}

#[table(name = map, public)]
pub struct Map {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub map_type: MapType,
    pub width: u64,
    pub height: u64,
    pub tiles: Vec<u8>, // Flattened 2D array: tiles[y * width + x] = tile_type (0=Wall, 1=Floor, 2=Door)
    pub spawn_position: Vec2, // Primary spawn position
    pub spawn_points: Vec<Vec2>, // All possible spawn points
    pub is_starting_town: bool, // Whether this is the main starting town (only relevant for towns)
    pub entity_ids: Vec<u64>, // List of entity IDs in this map
    pub created_at: Timestamp,
}
