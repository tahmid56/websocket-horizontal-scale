use redis::{AsyncCommands, Client};

pub async fn try_acquire_lock(
    client: &Client,
    key: &str,
    ttl_secs: usize,
) -> redis::RedisResult<bool> {
    let mut conn = client.get_multiplexed_async_connection().await?;

    let result: Option<String> = redis::cmd("SET")
        .arg(key)
        .arg("locked")
        .arg("NX")
        .arg("EX")
        .arg(ttl_secs)
        .query_async(&mut conn)
        .await?;

    Ok(result.is_some())
}

pub async fn release_lock(
    client: &Client,
    key: &str,
) -> redis::RedisResult<()> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let _: () = conn.del(key).await?;
    Ok(())
}

