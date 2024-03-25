use matasano_rs::{bytes_to_hex, hex_to_bytes};

const PLAINTEXT: &str = "1c0111001f010100061a024b53535009181c";
const KEY: &str = "686974207468652062756c6c277320657965";
const CIPHERTEXT: &str = "746865206b696420646f6e277420706c6179";

fn main() {
    let plaintext = hex_to_bytes(PLAINTEXT).unwrap();
    let cipher = hex_to_bytes(KEY).unwrap();

    let bytes_result = plaintext
        .iter()
        .zip(cipher.iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<_>>();

    let result_ciphertext: String = bytes_to_hex(&bytes_result);

    println!("Result Ciphertext : {}", result_ciphertext);
    println!("Expected Ciphertext: {}", CIPHERTEXT);
    assert_eq!(result_ciphertext, CIPHERTEXT);
    println!("\x1b[32mCORRECT\x1b[0m");
}
