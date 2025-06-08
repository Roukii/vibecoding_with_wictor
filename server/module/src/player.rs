use spacetimedb::{reducer, ReducerContext};

use crate::entity::move_entity;
use crate::tables::player;

#[reducer]
pub fn move_player(ctx: &ReducerContext, x: f64, y: f64) -> Result<(), String> {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        // Move the player's entity
        if let Some(entity_id) = player.entity_id {
            move_entity(ctx, entity_id, x, y)
        } else {
            Err("Player has no associated entity".to_string())
        }
    } else {
        Err("Player not found".to_string())
    }
}
