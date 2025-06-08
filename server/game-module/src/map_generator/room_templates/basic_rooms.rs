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

pub const SECONDARY_ROOM: RoomTemplate = RoomTemplate {
    name: "basic_room",
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
pub const ALL_TEMPLATES: &[RoomTemplate] = &[
    BASIC_ROOM,
    SECONDARY_ROOM,
];
