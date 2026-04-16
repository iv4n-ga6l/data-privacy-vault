use aws_sdk_kms::{Client, Error};
use std::env;

pub async fn get_encryption_key() -> Result<Vec<u8>, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let key_id = env::var("KMS_KEY_ID").expect("KMS_KEY_ID must be set");

    let response = client
        .generate_data_key()
        .key_id(key_id)
        .key_spec("AES_256")
        .send()
        .await?;

    Ok(response.plaintext.unwrap().into_inner())
}
