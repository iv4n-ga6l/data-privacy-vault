use aes::Aes256;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit, generic_array::GenericArray};
use std::str;
use crate::kms::get_encryption_key;
use base64::{Engine as _, engine::general_purpose};

pub async fn encrypt(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encryption_key = get_encryption_key().await?;
    let key = GenericArray::from_slice(&encryption_key);
    let cipher = Aes256::new(key);

    let mut block = [0u8; 16];
    let data_bytes = data.as_bytes();
    let len = data_bytes.len().min(16);
    block[..len].copy_from_slice(&data_bytes[..len]);
    
    let mut block = GenericArray::from(block);
    cipher.encrypt_block(&mut block);

    Ok(general_purpose::STANDARD.encode(block))
}

pub async fn decrypt(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encryption_key = get_encryption_key().await?;
    let key = GenericArray::from_slice(&encryption_key);
    let cipher = Aes256::new(key);

    let decoded = general_purpose::STANDARD.decode(data)?;
    let mut block = GenericArray::clone_from_slice(&decoded);
    cipher.decrypt_block(&mut block);

    let decrypted_str = str::from_utf8(&block)?.trim_matches('\0').to_string();
    Ok(decrypted_str)
}
