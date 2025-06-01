
use spacetimedb::{table, reducer, Table, ReducerContext, Identity, Timestamp};

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    avatar_url: Option<String>,
    online: bool,
}

#[table(name = message, public)]
pub struct Message {
    #[primary_key]
    #[auto_inc]
    id: u64,
    sender: Identity,
    sent: Timestamp,
    text: String,
}

#[reducer]
/// Clients invoke this reducer to set their user names.
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User { name: Some(name), ..user });
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
#[reducer]
/// Clients invoke this reducer to set their avatar URL.
pub fn set_avatar(ctx: &ReducerContext, avatar_url: String) -> Result<(), String> {
    let avatar_url = validate_avatar_url(avatar_url)?;
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User { avatar_url: Some(avatar_url), ..user });
        Ok(())
    } else {
        Err("Cannot set avatar for unknown user".to_string())
    }
}

/// Takes an avatar URL and checks if it's acceptable.
fn validate_avatar_url(avatar_url: String) -> Result<String, String> {
    if avatar_url.is_empty() {
        Err("Avatar URL must not be empty".to_string())
    } else if avatar_url.len() > 500 {
        Err("Avatar URL too long".to_string())
    } else if !avatar_url.starts_with("http://") && !avatar_url.starts_with("https://") {
        Err("Avatar URL must be a valid HTTP or HTTPS URL".to_string())
    } else {
        Ok(avatar_url)
    }
}

#[reducer]
/// Clients invoke this reducer to send messages.
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    let text = validate_message(text)?;
    log::info!("{}", text);
    ctx.db.message().insert(Message {
        id: 0, // auto_inc will handle this
        sender: ctx.sender,
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}

/// Takes a message's text and checks if it's acceptable to send.
fn validate_message(text: String) -> Result<String, String> {
    if text.is_empty() {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(text)
    }
}

#[reducer]
/// Clients invoke this reducer to delete their own messages.
pub fn delete_message(ctx: &ReducerContext, message_id: u64) -> Result<(), String> {
    if let Some(message) = ctx.db.message().id().find(message_id) {
        // Check if the sender is the owner of the message
        if message.sender == ctx.sender {
            ctx.db.message().id().delete(message_id);
            log::info!("Message {} deleted by user {:?}", message_id, ctx.sender);
            Ok(())
        } else {
            Err("You can only delete your own messages".to_string())
        }
    } else {
        Err("Message not found".to_string())
    }
}

#[reducer(client_connected)]
// Called when a client connects to a SpacetimeDB database server
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        // If this is a returning user, i.e. we already have a `User` with this `Identity`,
        // set `online: true`, but leave `name` and `identity` unchanged.
        ctx.db.user().identity().update(User { online: true, ..user });
    } else {
        // If this is a new user, create a `User` row for the `Identity`,
        // which is online, but hasn't set a name or avatar.
        ctx.db.user().insert(User {
            name: None,
            avatar_url: None,
            identity: ctx.sender,
            online: true,
        });
    }
}

#[reducer(client_disconnected)]
// Called when a client disconnects from SpacetimeDB database server
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User { online: false, ..user });
    } else {
        // This branch should be unreachable,
        // as it doesn't make sense for a client to disconnect without connecting first.
        log::warn!("Disconnect event for unknown user with identity {:?}", ctx.sender);
    }
}