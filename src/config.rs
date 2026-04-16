use redis::{Client, Connection};
use std::env;

pub fn get_redis_connection() -> redis::RedisResult<Connection> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = Client::open(redis_url)?;
    client.get_connection()
}