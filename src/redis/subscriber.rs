use redis::{Client, AsyncCommands};
use futures_util::StreamExt;
use crate::{event::ChatEvent, events::handler::handle_event, state::SharedState};

pub async fn start_subscriber(
    client: Client,
    state: SharedState,
) -> redis::RedisResult<()> {
    let mut pubsub = client.get_async_pubsub().await?;

    pubsub.subscribe("chat:events").await?;

    let mut stream = pubsub.on_message();

    while let Some(msg) = stream.next().await {
        let payload: String = msg.get_payload()?;

        let event: ChatEvent = match serde_json::from_str(&payload) {
            Ok(e) => e,
            Err(_) => continue,
        };

        handle_event(event, &state).await;
    }

    Ok(())
}
