use matasano_rs::{char_freq, hex_to_bytes};

const CIPHERTEXT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn main() {
    let bytes = hex_to_bytes(CIPHERTEXT).unwrap();

    let (max_score, raw_plaintext) = (0u8..=255)
        .map(|key| {
            let candidate = bytes.iter().map(|&byte| byte ^ key).collect::<Vec<_>>();
            let candidate_score = candidate.iter().copied().map(char_freq).sum::<u64>();
            (candidate_score, candidate)
        })
        .max_by_key(|(score, _)| *score)
        .unwrap();

    let plaintext = String::from_utf8(raw_plaintext).unwrap();

    println!("Max Score: {}", max_score);
    println!("Decoded: {:?}", plaintext);
}
