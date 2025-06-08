#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomType {
    Combat,
    Treasure,
    Central,
    Rest,
    Spawn,
    Town,
}

impl RoomType {
    /// Get the default weight for this room type in dungeons
    pub fn default_dungeon_weight(&self) -> u32 {
        match self {
            RoomType::Combat => 40,
            RoomType::Treasure => 15,
            RoomType::Rest => 12,
            RoomType::Spawn => 10,
            _ => 0, // Town types not used in dungeons
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoomTemplate {
    pub name: &'static str,
    pub room_type: RoomType,
    pub weight: u32,
    pub template: &'static str,
    pub is_central: bool,
}

// Legend:
// # = Wall
// . = Floor
// D = Door
// C = Connection point (will become door when connected)

pub const BASIC_ROOM: RoomTemplate = RoomTemplate {
    name: "basic_room",
    room_type: RoomType::Spawn,
    weight: 5,
    is_central: false,
    template: "
#########CC#########
#..................#
#..................#
#..................#
#..................#
#......S...........#
#..................#
#..................#
C..................C
C..................C
#..................#
#..................#
#......S...........#
#..................#
#..................#
#..................#
#..................#
#..................#
#..................#
#########CC#########",
};

pub const SECONDARY_ROOM: RoomTemplate = RoomTemplate {
    name: "divided_room",
    room_type: RoomType::Rest,
    weight: 5,
    is_central: false,
    template: "
#########CC#########
#..................#
#..................#
#..................#
#########DD#########
#......#...........#
#......#...........#
#......#...........#
C......#...........C
C......#...........C
###D##########D#####
#..................#
#..................#
#..................#
#..................#
#..................#
#..................#
#..................#
#..................#
#########CC#########",
};

pub const COMBAT_ROOM: RoomTemplate = RoomTemplate {
    name: "combat_chamber",
    room_type: RoomType::Combat,
    weight: 8,
    is_central: false,
    template: "
#########CC#########
#..................#
#..................#
#.....######.......#
#.....#....#.......#
#.....#....#.......#
#.....#....#.......#
#.....######.......#
C..................C
C..................C
#..................#
#.....######.......#
#.....#....#.......#
#.....#....#.......#
#.....#....#.......#
#.....######.......#
#..................#
#..................#
#..................#
#########CC#########",
};

pub const TREASURE_ROOM: RoomTemplate = RoomTemplate {
    name: "treasure_vault",
    room_type: RoomType::Treasure,
    weight: 3,
    is_central: false,
    template: "
#########CC#########
#..................#
#..................#
#.....########.....#
#.....#......#.....#
#.....#......#.....#
#.....#......#.....#
#.....#......#.....#
C.....D......D.....C
C.....D......D.....C
#.....#......#.....#
#.....#......#.....#
#.....#......#.....#
#.....#......#.....#
#.....########.....#
#..................#
#..................#
#..................#
#..................#
#########CC#########",
};

pub const SPAWN_ROOM_BASIC: RoomTemplate = RoomTemplate {
    name: "spawn_room_basic",
    room_type: RoomType::Spawn,
    weight: 8,
    is_central: false,
    template: "
#########CC#########
#..................#
#..................#
#..................#
#..................#
#.....S.......S....#
#..................#
#..................#
C..................C
C..................C
#..................#
#..................#
#.....S.......S....#
#..................#
#..................#
#..................#
#..................#
#..................#
#..................#
#########CC#########",
};

pub const SPAWN_ROOM_SAFE: RoomTemplate = RoomTemplate {
    name: "spawn_room_safe",
    room_type: RoomType::Spawn,
    weight: 6,
    is_central: false,
    template: "
#########CC#########
#..................#
#..................#
#....##########....#
#....#........#....#
#....#....S...#....#
#....#........#....#
#....#........#....#
C....#........#....C
C....#........#....C
#....#........#....#
#....#........#....#
#....#....S...#....#
#....#........#....#
#....##########....#
#..................#
#..................#
#..................#
#..................#
#########CC#########",
};

pub const SPAWN_ROOM_ENTRANCE: RoomTemplate = RoomTemplate {
    name: "spawn_room_entrance",
    room_type: RoomType::Spawn,
    weight: 10,
    is_central: false,
    template: "
#########CC#########
#..................#
#..................#
#..................#
#......######......#
#......#....#......#
#......#.S..#......#
#......#....#......#
C......D....D......C
C......D....D......C
#......#....#......#
#......#..S.#......#
#......#....#......#
#......######......#
#..................#
#..................#
#..................#
#..................#
#..................#
#########CC#########",
};

pub const SPAWN_ROOM_OUTPOST: RoomTemplate = RoomTemplate {
    name: "spawn_room_outpost",
    room_type: RoomType::Spawn,
    weight: 7,
    is_central: false,
    template: "
#########CC#########
#..................#
#...###........###.#
#...#..........#..#
#...#....S.....#..#
#..................#
#..................#
#.........S........#
C..................C
C..................C
#.........S........#
#..................#
#..................#
#...#....S.....#..#
#...#..........#..#
#...###........###.#
#..................#
#..................#
#..................#
#########CC#########",
};

/// All available spawn room templates
pub const ALL_SPAWN_TEMPLATES: &[RoomTemplate] = &[
    BASIC_ROOM, // This was already a spawn room
    SPAWN_ROOM_BASIC,
    SPAWN_ROOM_SAFE,
    SPAWN_ROOM_ENTRANCE,
    SPAWN_ROOM_OUTPOST,
];
