use itertools::Itertools;
use matasano_rs::base64::decode;
use openssl::symm::{decrypt, Cipher};

const CIPHERTEXT: &str = include_str!("../../inputs/07.in");
const KEY: &[u8] = b"YELLOW SUBMARINE";

fn main() {
    let bytes = decode(&CIPHERTEXT.lines().join(""));
    let plaintext = decrypt(Cipher::aes_128_ecb(), KEY, None, &bytes).unwrap();
    println!("{}", String::from_utf8(plaintext).unwrap())
}
