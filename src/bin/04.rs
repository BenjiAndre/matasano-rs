use matasano_rs::{char_freq, hex_to_bytes};

const INPUT: &str = include_str!("../../inputs/04.in");

fn crack_single_byte_xor(s: &str) -> (u64, Vec<u8>) {
    let bytes = hex_to_bytes(s).unwrap();

    let (score, decoded) = (0..=255)
        .map(|key| {
            let candidate = bytes.iter().map(|&bytes| bytes ^ key).collect::<Vec<_>>();
            let candidate_score = candidate.iter().copied().map(char_freq).sum::<u64>();
            (candidate_score, candidate)
        })
        .max_by_key(|(score, _)| *score)
        .unwrap();

    (score, decoded)
}

fn main() {
    let (max_score, decoded) = INPUT
        .lines()
        .map(crack_single_byte_xor)
        .max_by_key(|(score, _)| *score)
        .unwrap();

    let result = String::from_utf8(decoded).expect("Failed to convert to String");
    println!("Max Score: {}", max_score);
    println!("Decoded: {:?}", result);
}
