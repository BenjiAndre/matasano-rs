use openssl::rand::rand_bytes;
use openssl::symm::{Cipher, Crypter, Mode};
use std::error::Error;

fn pkcs7(data: &[u8], block_size: usize) -> Vec<u8> {
    if data.len() % 16 == 0 {
        return data.to_vec();
    }
    let padding_size = block_size - (data.len() % block_size);
    let mut padded_data = Vec::from(data);
    for _ in 0..padding_size {
        padded_data.push(padding_size as u8);
    }
    padded_data
}

fn unpad(data: &[u8]) -> Vec<u8> {
    let padding_size = *data.last().unwrap() as usize;
    if padding_size > data.len() {
        return data.to_vec();
    }
    let padding_start = data.len() - padding_size;
    if data[padding_start..]
        .iter()
        .all(|&x| x as usize == padding_size)
    {
        data[..padding_start].to_vec()
    } else {
        data.to_vec()
    }
}

pub fn ecb_decrypt(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Cipher::aes_128_ecb();

    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, None)?;
    crypter.pad(false);

    let mut output = vec![0; ciphertext.len() + cipher.block_size()];
    let count = crypter.update(ciphertext, &mut output)?;
    output.truncate(count);
    Ok(unpad(&output))
}

pub fn ecb_encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Cipher::aes_128_ecb();
    let padded_data = &pkcs7(plaintext, cipher.block_size());

    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, None)?;
    crypter.pad(false);

    let mut output = vec![0; padded_data.len() + cipher.block_size()];
    let count = crypter.update(padded_data, &mut output)?;
    output.truncate(count);
    Ok(output)
}

pub fn cbc_decrypt(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Cipher::aes_128_cbc();
    let iv_size = cipher.block_size();

    let (iv, ciphertext) = ciphertext.split_at(iv_size);

    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;
    crypter.pad(false);

    let mut output = vec![0; ciphertext.len() + cipher.block_size()];
    let count = crypter.update(ciphertext, &mut output)?;
    let rest = crypter.finalize(&mut output[count..])?;
    output.truncate(count + rest);
    Ok(unpad(&output))
}

pub fn cbc_encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Cipher::aes_128_cbc();
    let mut iv = vec![0; cipher.block_size()];
    rand_bytes(&mut iv)?;

    let padded_data = pkcs7(plaintext, cipher.block_size());

    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(&iv))?;
    crypter.pad(false);

    let mut output = vec![0; padded_data.len() + cipher.block_size()];
    let count = crypter.update(&padded_data, &mut output)?;
    let rest = crypter.finalize(&mut output[count..])?;
    output.truncate(count + rest);

    let mut final_output = iv;
    final_output.extend(output);

    Ok(final_output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::rand::rand_bytes;

    #[test]
    fn test_aes_ecb_encrypt_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let data = b"Hello, world!";

        let encrypted = ecb_encrypt(data, key).expect("Encryption failed");
        let decrypted = ecb_decrypt(&encrypted, key).expect("Decryption failed");

        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_aes_cbc_encrypt_decrypt() {
        let key = b"YELLOW SUBMARINE";
        let data = b"Hello, world!";

        let encrypted = cbc_encrypt(data, key).expect("Encryption failed");
        let decrypted = cbc_decrypt(&encrypted, key).expect("Decryption failed");

        assert_eq!(decrypted, data);
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
