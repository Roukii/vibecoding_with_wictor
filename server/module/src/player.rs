use spacetimedb::{reducer, ReducerContext, Table};

#[reducer]
pub fn move_player(ctx: &ReducerContext, x: f64, y: f64) -> Result<(), String> {
    if let Some(mut player) = ctx.db.player().identity().find(ctx.sender) {
        // First check if there's a starting town to validate movement against
        if let Some(town) = ctx.db.town().iter().find(|t| t.is_starting_town) {
            let tile_x = x as usize;
            let tile_y = y as usize;

            // Validate that the position is within bounds and walkable
            if !town.is_walkable(tile_x, tile_y) {
                return Err(
                    "Cannot move to that position - blocked by wall or out of bounds".to_string(),
                );
            }
        } else if let Some(dungeon) = ctx.db.dungeon().iter().max_by_key(|d| d.created_at) {
            // Fallback to dungeon validation if no town exists
            let tile_x = x as usize;
            let tile_y = y as usize;

            if !dungeon.is_walkable(tile_x, tile_y) {
                return Err(
                    "Cannot move to that position - blocked by wall or out of bounds".to_string(),
                );
            }
        }

        player.position.x = x;
        player.position.y = y;
        ctx.db.player().identity().update(player);
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}