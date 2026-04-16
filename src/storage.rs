use redis::{Commands, Connection, RedisResult};
use crate::encryption::{encrypt, decrypt};
use crate::config::get_redis_connection;

const REDIS_KEY_PREFIX: &str = "data_privacy_vault:";

pub async fn store_tokenized_data(token: String, original_value: String) {
    let encrypted_value = encrypt(&original_value).await.expect("Failed to encrypt data");
    let key = format!("{}{}", REDIS_KEY_PREFIX, token);

    let mut conn = get_redis_connection().expect("Failed to connect to Redis");
    let _: RedisResult<()> = conn.set(key, encrypted_value);
}

pub async fn retrieve_original_data(token: &str) -> Option<String> {
    let key = format!("{}{}", REDIS_KEY_PREFIX, token);

    let mut conn = get_redis_connection().expect("Failed to connect to Redis");
    if let Ok(encrypted_value) = conn.get::<_, String>(key) {
        return decrypt(&encrypted_value).await.ok();
    }

    None
}