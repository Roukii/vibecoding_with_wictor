pub mod basic_rooms;
pub mod central_rooms;
pub mod town_templates;

pub use basic_rooms::*;
pub use central_rooms::*;
pub use town_templates::*;

/// Collection of all dungeon-related room templates
pub const DUNGEON_TEMPLATES: &[RoomTemplate] = &[
    // Basic room templates
    basic_rooms::BASIC_ROOM,
    basic_rooms::SECONDARY_ROOM,
    basic_rooms::COMBAT_ROOM,
    basic_rooms::TREASURE_ROOM,
    // Spawn room templates
    basic_rooms::SPAWN_ROOM_BASIC,
    basic_rooms::SPAWN_ROOM_SAFE,
    basic_rooms::SPAWN_ROOM_ENTRANCE,
    basic_rooms::SPAWN_ROOM_OUTPOST,
    // Central room templates
    central_rooms::CENTRAL_HALL,
    central_rooms::CENTRAL_CHAMBER,
    central_rooms::CENTRAL_COURTYARD,
    central_rooms::CENTRAL_THRONE_ROOM,
    central_rooms::CENTRAL_GREAT_HALL,
];

/// Collection of all town-related room templates
pub const TOWN_TEMPLATES: &[RoomTemplate] = &[town_templates::TOWN_SQUARE];
