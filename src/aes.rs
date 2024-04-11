use openssl::symm::{Cipher, Crypter, Mode};
use std::error::Error;

fn aes(block: &[u8], key: &[u8], cipher: Cipher, mode: Mode) -> Result<Vec<u8>, Box<dyn Error>> {
    if block.len() % 16 != 0 {
        return Err("Block size must be a multiple of 16.".into());
    }

    let mut crypter = Crypter::new(cipher, mode, key, None)?;
    crypter.pad(false);

    let mut output = vec![0; block.len() + cipher.block_size()];
    let count = crypter.update(block, &mut output)?;
    output.truncate(count);
    Ok(output)
}

pub fn ecb_decrypt(block: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    aes(block, key, Cipher::aes_128_ecb(), Mode::Decrypt)
}

pub fn ecb_encrypt(block: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    aes(block, key, Cipher::aes_128_ecb(), Mode::Encrypt)
}

pub fn cbc_decrypt(block: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    aes(block, key, Cipher::aes_128_cbc(), Mode::Decrypt)
}

pub fn cbc_encrypt(block: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    aes(block, key, Cipher::aes_128_cbc(), Mode::Encrypt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::rand::rand_bytes;

    #[test]
    fn test_aes_ecb_encrypt_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let data = b"Hello, world!   ";

        let encrypted = ecb_encrypt(data, key).expect("Encryption failed");
        let decrypted = ecb_decrypt(&encrypted, key).expect("Decryption failed");

        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_aes_cbc_encrypt_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let data = b"Hello, world!   ";

        let encrypted = cbc_encrypt(data, key).expect("Encryption failed");
        let decrypted = cbc_decrypt(&encrypted, key).expect("Decryption failed");

        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_invalid_block_size() {
        let key = b"YELLOW SUBMARINE";
        let data = b"Too short";

        let result = ecb_encrypt(data, key);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_keys_produce_different_results() {
        let key1 = b"YELLOW SUBMARINE";
        let mut key2 = [0u8; 16];
        rand_bytes(&mut key2).expect("Failed to generate random key");
        let data = b"Hello, world!   ";

        let encrypted1 = ecb_encrypt(data, key1).expect("Encryption with key1 failed");
        let encrypted2 = ecb_encrypt(data, &key2).expect("Encryption with key2 failed");

        assert_ne!(encrypted1, encrypted2);
    }
}
