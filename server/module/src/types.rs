use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
