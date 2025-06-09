use crate::tables::{
    entity, game_info, map, player, player_offline, user, Entity, EntityType, GameInfo, Player,
    PlayerOffline, User,
};
use spacetimedb::{reducer, ReducerContext, Table};

#[reducer]
/// Clients invoke this reducer to set their user names.
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        // Update the user record with the name
        ctx.db.user().identity().update(User {
            name: Some(name.clone()),
            ..user
        });

        // Create or update the player record
        // Check if player exists in either online or offline table
        if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
            // Update existing online player name
            ctx.db.player().identity().update(Player { name, ..player });
        } else if let Some(offline_player) = ctx.db.player_offline().identity().find(ctx.sender) {
            // Update existing offline player name
            ctx.db.player_offline().identity().update(PlayerOffline {
                name,
                ..offline_player
            });
        } else {
            // Create new player record - determine if user is currently online
            let is_user_online = ctx
                .db
                .user()
                .identity()
                .find(ctx.sender)
                .map(|u| u.online)
                .unwrap_or(false);

            if is_user_online {
                // User is online, create in Player table
                ctx.db.player().insert(Player {
                    identity: ctx.sender,
                    name,
                    entity_id: None,
                    current_map_id: None,
                });
            } else {
                // User is offline, create in PlayerOffline table
                ctx.db.player_offline().insert(PlayerOffline {
                    identity: ctx.sender,
                    name,
                    entity_id: None,
                    current_map_id: None,
                    last_seen: ctx.timestamp,
                });
            }
        }

        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

#[reducer(client_connected)]
// Called when a client connects to a SpacetimeDB database server
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        // If this is a returning user, i.e. we already have a `User` with this `Identity`,
        // set `online: true`, but leave `name` and `identity` unchanged.
        ctx.db.user().identity().update(User {
            online: true,
            ..user
        });
    } else {
        // If this is a new user, create a `User` row for the `Identity`,
        // which is online, but hasn't set a name.
        ctx.db.user().insert(User {
            name: None,
            identity: ctx.sender,
            online: true,
        });
    }

    // Move player from offline to online table if they exist
    if let Some(offline_player) = ctx.db.player_offline().identity().find(ctx.sender) {
        // Move player from PlayerOffline to Player table
        ctx.db.player().insert(Player {
            identity: offline_player.identity,
            name: offline_player.name,
            entity_id: offline_player.entity_id,
            current_map_id: offline_player.current_map_id,
        });
        // Remove from PlayerOffline table
        ctx.db.player_offline().identity().delete(ctx.sender);
    }
}

#[reducer(client_disconnected)]
// Called when a client disconnects from SpacetimeDB database server
pub fn identity_disconnected(ctx: &ReducerContext) {
    let now = ctx.timestamp;

    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: false,
            ..user
        });
    } else {
        // This branch should be unreachable,
        // as it doesn't make sense for a client to disconnect without connecting first.
        log::warn!(
            "Disconnect event for unknown user with identity {:?}",
            ctx.sender
        );
    }

    // Move player from online to offline table if they exist
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        // Move player from Player to PlayerOffline table
        ctx.db.player_offline().insert(PlayerOffline {
            identity: player.identity,
            name: player.name,
            entity_id: player.entity_id,
            current_map_id: player.current_map_id,
            last_seen: now,
        });
        // Remove from Player table
        ctx.db.player().identity().delete(ctx.sender);
    }
}

#[reducer]
/// Initialize game info with starting town map ID
pub fn initialize_game_info(ctx: &ReducerContext, starting_town_map_id: u64) -> Result<(), String> {
    // Check if game info already exists
    if ctx.db.game_info().id().find(1).is_some() {
        return Err("Game info already initialized".to_string());
    }

    // Verify the starting town map exists
    if ctx.db.map().id().find(starting_town_map_id).is_none() {
        return Err("Starting town map does not exist".to_string());
    }

    // Create game info record
    ctx.db.game_info().insert(GameInfo {
        id: 1, // Singleton pattern - always use ID 1
        starting_town_map_id,
        updated_at: ctx.timestamp,
    });

    log::info!(
        "Game info initialized with starting town map ID: {}",
        starting_town_map_id
    );
    Ok(())
}

#[reducer]
/// Spawn a player entity in the starting town
pub fn spawn_player_entity(ctx: &ReducerContext) -> Result<(), String> {
    // Check if user has a name set
    let user = ctx
        .db
        .user()
        .identity()
        .find(ctx.sender)
        .ok_or("User not found")?;

    if user.name.is_none() {
        return Err("Player must set a name before spawning".to_string());
    }

    // Check if player is online
    let player = ctx
        .db
        .player()
        .identity()
        .find(ctx.sender)
        .ok_or("Player not online or not found")?;

    // Check if player already has an entity
    if player.entity_id.is_some() {
        return Err("Player already has an entity".to_string());
    }

    // Get game info to find starting town
    let game_info = ctx
        .db
        .game_info()
        .id()
        .find(1)
        .ok_or("Game info not initialized")?;

    // Get the starting town map
    let mut starting_town = ctx
        .db
        .map()
        .id()
        .find(game_info.starting_town_map_id)
        .ok_or("Starting town map not found")?;

    // Create the player entity
    let spawn_position = if !starting_town.spawn_points.is_empty() {
        starting_town.spawn_points[0].clone()
    } else {
        starting_town.spawn_position.clone()
    };

    let new_entity = Entity {
        id: 0, // Will be auto-incremented
        entity_type: EntityType::Player,
        position: spawn_position,
        direction: 0.0, // Facing east
        owner_identity: Some(ctx.sender),
        created_at: ctx.timestamp,
    };

    ctx.db.entity().insert(new_entity);

    // Find the inserted entity to get its actual ID
    let entity_id = ctx
        .db
        .entity()
        .iter()
        .filter(|e| e.owner_identity == Some(ctx.sender) && e.entity_type == EntityType::Player)
        .last()
        .map(|e| e.id)
        .ok_or("Failed to retrieve created entity")?;

    // Add entity to the starting town's entity list
    starting_town.entity_ids.push(entity_id);
    ctx.db.map().id().update(starting_town);

    // Update player with entity ID and current map
    ctx.db.player().identity().update(Player {
        entity_id: Some(entity_id),
        current_map_id: Some(game_info.starting_town_map_id),
        ..player
    });

    log::info!(
        "Player {} spawned entity {} in starting town (map {})",
        user.name.unwrap_or("Unknown".to_string()),
        entity_id,
        game_info.starting_town_map_id
    );

    Ok(())
}