use matasano_rs::bytes_to_hex;

const PLAINTEXT: &[u8] =
    b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
const CIPHERTEXT: &str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

fn main() {
    let xored_plaintext = b"ICE"
        .iter()
        .cycle()
        .zip(PLAINTEXT)
        .map(|(key, c)| key ^ c)
        .collect::<Vec<_>>();

    let ciphertext = bytes_to_hex(&xored_plaintext);

    println!("Result Ciphertext : {}", ciphertext);
    println!("Expected Ciphertext: {}", CIPHERTEXT);
    assert_eq!(ciphertext, CIPHERTEXT);
    println!("\x1b[32mCORRECT\x1b[0m");
}
