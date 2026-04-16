use aws_sdk_kms::Client;
use aws_sdk_kms::types::DataKeySpec;
use std::env;
use std::error::Error;
use std::io;

const LOCAL_KEY_ENV: &str = "LOCAL_ENCRYPTION_KEY";
const DEFAULT_LOCAL_KEY: &str = "data-privacy-vault-local-key-32";

fn local_encryption_key() -> Vec<u8> {
    let source = env::var(LOCAL_KEY_ENV).unwrap_or_else(|_| DEFAULT_LOCAL_KEY.to_string());
    let mut key = [0u8; 32];
    let bytes = source.as_bytes();
    let len = bytes.len().min(key.len());
    key[..len].copy_from_slice(&bytes[..len]);
    key.to_vec()
}

pub async fn get_encryption_key() -> Result<Vec<u8>, Box<dyn Error>> {
    let Ok(key_id) = env::var("KMS_KEY_ID") else {
        return Ok(local_encryption_key());
    };

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let response = client
        .generate_data_key()
        .key_id(key_id)
        .key_spec(DataKeySpec::Aes256)
        .send()
        .await?;

    response
        .plaintext
        .map(|blob| blob.into_inner())
        .ok_or_else(|| io::Error::other("KMS response did not include plaintext data key").into())
}
