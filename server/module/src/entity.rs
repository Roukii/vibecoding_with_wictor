use crate::tables::{entity, player, Entity, EntityType};
use crate::types::Vec2;
use spacetimedb::{reducer, ReducerContext, Table};

#[reducer]
pub fn create_player_entity(ctx: &ReducerContext) -> Result<(), String> {
    // Check if player already has an entity
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        if player.entity_id.is_some() {
            return Err("Player already has an entity".to_string());
        }
    } else {
        return Err("Player not found".to_string());
    }

    // Create entity for the player
    let entity = Entity {
        id: 0, // auto_inc will handle this
        entity_type: EntityType::Player,
        position: Vec2 { x: 0.0, y: 0.0 }, // Will be set when player joins a dungeon/town
        direction: 0.0,                    // Facing east initially
        owner_identity: Some(ctx.sender),
        created_at: ctx.timestamp,
    };

    let entity_id = ctx.db.entity().insert(entity).id;

    // Update player with entity_id
    if let Some(mut player) = ctx.db.player().identity().find(ctx.sender) {
        player.entity_id = Some(entity_id);
        ctx.db.player().identity().update(player);
    }

    Ok(())
}

pub fn move_entity(ctx: &ReducerContext, entity_id: u64, x: f64, y: f64) -> Result<(), String> {
    if let Some(mut entity) = ctx.db.entity().id().find(entity_id) {
        // Verify the entity belongs to the calling player
        if entity.owner_identity != Some(ctx.sender) {
            return Err("You don't own this entity".to_string());
        }

        // Position validation would need to be handled differently
        // Since entities no longer store location references, validation logic
        // should be implemented at a higher level or through other means

        // Update entity position
        entity.position.x = x;
        entity.position.y = y;
        ctx.db.entity().id().update(entity);

        Ok(())
    } else {
        Err("Entity not found".to_string())
    }
}
