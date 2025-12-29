use redis::{AsyncCommands, Client, Commands};
use crate::event::ChatEvent;

pub fn publish_event(  // Note: remove .async if not needed
    client: &Client,
    event: &ChatEvent,
) -> redis::RedisResult<usize> {  
    let mut c = client.get_connection()?; 
    let payload = serde_json::to_string(event).unwrap();
    let count: usize = c.publish("chat:events", payload)?;
    Ok(count)
}
