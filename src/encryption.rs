use aes::Aes256;
use aes::cipher::{BlockCipher, KeyInit, generic_array::GenericArray};
use std::str;
use crate::kms::get_encryption_key;

pub fn encrypt(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encryption_key = get_encryption_key()?;
    let key = GenericArray::from_slice(&encryption_key);
    let cipher = Aes256::new(key);

    let mut block = GenericArray::clone_from_slice(data.as_bytes());
    cipher.encrypt_block(&mut block);

    Ok(base64::encode(block))
}

pub fn decrypt(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encryption_key = get_encryption_key()?;
    let key = GenericArray::from_slice(&encryption_key);
    let cipher = Aes256::new(key);

    let mut block = GenericArray::clone_from_slice(&base64::decode(data)?);
    cipher.decrypt_block(&mut block);

    Ok(str::from_utf8(&block)?.to_string())
}
