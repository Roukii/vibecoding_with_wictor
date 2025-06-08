#[derive(Debug, Clone)]
pub struct RoomTemplate {
    pub name: &'static str,
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
    weight: 5,
    is_central: false,
    template: "
#########CC#########
#..................#
#..................#
#..................#
#..................#
#..................#
#..................#
#..................#
C..................C
C..................C
#..................#
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

pub const ALL_TEMPLATES: &[RoomTemplate] = &[
    BASIC_ROOM,
    // Include central room templates
    super::central_rooms::CENTRAL_HALL,
    super::central_rooms::CENTRAL_CHAMBER,
    super::central_rooms::CENTRAL_COURTYARD,
    super::central_rooms::CENTRAL_THRONE_ROOM,
];
