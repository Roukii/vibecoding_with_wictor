use crate::tables::{message, Message};
use spacetimedb::{reducer, ReducerContext, Table};

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
