use matasano_rs;

const PLAINTEXT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const CIPHERTEXT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    let bytes = matasano_rs::hex_to_bytes(PLAINTEXT).unwrap();
    let result_ciphertext = matasano_rs::base64::encode(&bytes);
    println!("Result Ciphertext  : {}", result_ciphertext);
    println!("Expected Ciphertext: {}", CIPHERTEXT);
    assert_eq!(result_ciphertext, CIPHERTEXT);
    println!("\x1b[32mCORRECT\x1b[0m");
}
