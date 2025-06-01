use spacetimedb::{Identity, ReducerContext, Table, Timestamp};

#[spacetimedb::table(name = player, public)]
pub struct Player {
    identity: Identity,
    name: String,
    entity_id: Option<u32>,
    online: bool,
}

#[spacetimedb::table(name = entity, public)]
pub struct Entity {
    id: u32,
    x: f32,
    y: f32,
}

#[spacetimedb::table(name = message, public)]
pub struct Message {
    sender: Identity,
    sent: Timestamp,
    text: String,
}

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // Called when the module is initially published
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    let identity = ctx.identity();
    let mut players = ctx.db.player().iter().filter(|p| p.identity == identity);

    if let Some(existing_player) = players.next() {
        // Update existing player's online status
        let updated_player = Player {
            identity,
            name: existing_player.name.clone(),
            entity_id: existing_player.entity_id,
            online: true,
        };
        ctx.db.player().insert(updated_player);
        log::info!("Existing player reconnected: {:?}", identity);
    } else {
        // Create new player
        let default_name = format!("Player_{}", ctx.db.player().iter().count() + 1);
        ctx.db.player().insert(Player {
            identity,
            name: default_name.clone(),
            entity_id: None,
            online: true,
        });
        log::info!("New player connected with identity: {:?}", identity);
    }
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    let identity = ctx.identity();
    let mut players = ctx.db.player().iter().filter(|p| p.identity == identity);

    if let Some(player) = players.next() {
        let updated_player = Player {
            identity,
            name: player.name.clone(),
            entity_id: player.entity_id,
            online: false,
        };
        ctx.db.player().insert(updated_player);
        log::info!("Player disconnected: {:?}", identity);
    }
}

#[spacetimedb::reducer]
pub fn update_player_name(ctx: &ReducerContext, new_name: String) {
    let identity = ctx.identity();
    let mut players = ctx.db.player().iter().filter(|p| p.identity == identity);

    if let Some(player) = players.next() {
        let updated_player = Player {
            identity,
            name: new_name.clone(),
            entity_id: player.entity_id,
            online: player.online,
        };
        ctx.db.player().insert(updated_player);
        log::info!("Player {:?} renamed to {}", identity, new_name);
    } else {
        log::warn!("Player with identity {:?} not found", identity);
    }
}

#[spacetimedb::reducer]
pub fn spawn_player_entity(ctx: &ReducerContext, x: f32, y: f32) {
    let identity = ctx.identity();
    let mut players = ctx.db.player().iter().filter(|p| p.identity == identity);

    if let Some(player) = players.next() {
        // Generate a new entity ID
        let entity_id = (ctx.db.entity().iter().count() as u32) + 1;

        // Create the entity
        ctx.db.entity().insert(Entity {
            id: entity_id,
            x,
            y,
        });

        // Update the player with the entity reference
        let updated_player = Player {
            identity,
            name: player.name.clone(),
            entity_id: Some(entity_id),
            online: player.online,
        };
        ctx.db.player().insert(updated_player);

        log::info!("Created entity {} for player {:?}", entity_id, identity);
    } else {
        log::warn!("Player with identity {:?} not found", identity);
    }
}

#[spacetimedb::reducer]
pub fn send_message(ctx: &ReducerContext, message: String) {
    let identity = ctx.identity();
    let mut players = ctx.db.player().iter().filter(|p| p.identity == identity);

    if let Some(_player) = players.next() {
        ctx.db.message().insert(Message {
            sender: identity,
            sent: ctx.timestamp,
            text: message,
        });
    } else {
        log::warn!(
            "Message not sent: Player with identity {:?} not found",
            identity
        );
    }
}

#[spacetimedb::reducer]
pub fn move_entity(ctx: &ReducerContext, new_x: f32, new_y: f32) {
    let identity = ctx.identity();
    let player = ctx.db.player().iter().find(|p| p.identity == identity);

    if let Some(player) = player {
        if let Some(entity_id) = player.entity_id {
            let mut entities = ctx.db.entity().iter().filter(|e| e.id == entity_id);
            if let Some(entity) = entities.next() {
                let updated_entity = Entity {
                    id: entity.id,
                    x: new_x,
                    y: new_y,
                };
                ctx.db.entity().insert(updated_entity);
                log::info!(
                    "Entity {} moved to position ({}, {})",
                    entity_id,
                    new_x,
                    new_y
                );
            }
        } else {
            log::warn!("Player {:?} has no entity assigned", identity);
        }
    } else {
        log::warn!("Player with identity {:?} not found", identity);
    }
}
