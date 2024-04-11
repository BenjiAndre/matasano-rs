use itertools::Itertools;
use matasano_rs::{aes, base64};

const CIPHERTEXT: &str = include_str!("../../inputs/10.in");
const KEY: &[u8] = b"YELLOW SUBMARINE";
const IV: &[u8] = &[0u8; 16];

fn manual_cbc_decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    assert!(
        ciphertext.len() % 16 == 0,
        "Ciphertext size must be a multiple of 16."
    );
    let mut decrypted = Vec::with_capacity(ciphertext.len());
    let mut previous_block = iv;

    for block in ciphertext.chunks_exact(16) {
        let mut decrypted_block = aes::ecb_decrypt(block, key).unwrap();

        for (i, byte) in decrypted_block.iter_mut().enumerate() {
            *byte ^= previous_block[i];
        }

        previous_block = block;

        decrypted.extend_from_slice(&decrypted_block);
    }
    decrypted
}

fn main() {
    let ciphertext = base64::decode(&CIPHERTEXT.lines().join(""));
    let plaintext = manual_cbc_decrypt(&ciphertext, KEY, IV);
    println!("{}", String::from_utf8(plaintext).unwrap());
}
