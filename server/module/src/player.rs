use spacetimedb::{reducer, ReducerContext, Table};

use crate::entity::move_entity;
use crate::tables::{dungeon, player, town};

// Helper function to check if a tile is walkable (Floor=1 or Door=2)
fn is_tile_walkable(tiles: &[u8], x: usize, y: usize, width: u64, height: u64) -> bool {
    if x >= width as usize || y >= height as usize {
        return false;
    }
    let index = y * (width as usize) + x;
    match tiles.get(index) {
        Some(1) | Some(2) => true, // Floor or Door
        _ => false,                // Wall or out of bounds
    }
}

#[reducer]
pub fn move_player(ctx: &ReducerContext, x: f64, y: f64) -> Result<(), String> {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        // If player has an entity, use entity-based movement
        if let Some(entity_id) = player.entity_id {
            return move_entity(ctx, entity_id, x, y);
        }

        // Fallback to legacy movement for players without entities
        let mut player = player;

        // First check if there's a starting town to validate movement against
        if let Some(town) = ctx.db.town().iter().find(|t| t.is_starting_town) {
            let tile_x = x as usize;
            let tile_y = y as usize;
            // Validate that the position is within bounds and walkable
            if !is_tile_walkable(&town.tiles, tile_x, tile_y, town.width, town.height) {
                return Err(
                    "Cannot move to that position - blocked by wall or out of bounds".to_string(),
                );
            }
        } else if let Some(dungeon) = ctx.db.dungeon().iter().max_by_key(|d| d.created_at) {
            // Fallback to dungeon validation if no town exists
            let tile_x = x as usize;
            let tile_y = y as usize;

            if !is_tile_walkable(
                &dungeon.tiles,
                tile_x,
                tile_y,
                dungeon.width,
                dungeon.height,
            ) {
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
