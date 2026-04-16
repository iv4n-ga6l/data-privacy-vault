use aes::Aes256;
use aes::cipher::{BlockCipher, KeyInit, generic_array::GenericArray};
use std::str;

const ENCRYPTION_KEY: &[u8; 32] = b"anexampleverysecurekey1234567890";

pub fn encrypt(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = GenericArray::from_slice(ENCRYPTION_KEY);
    let cipher = Aes256::new(key);

    let mut block = GenericArray::clone_from_slice(data.as_bytes());
    cipher.encrypt_block(&mut block);

    Ok(base64::encode(block))
}

pub fn decrypt(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = GenericArray::from_slice(ENCRYPTION_KEY);
    let cipher = Aes256::new(key);

    let mut block = GenericArray::clone_from_slice(&base64::decode(data)?);
    cipher.decrypt_block(&mut block);

    Ok(str::from_utf8(&block)?.to_string())
}