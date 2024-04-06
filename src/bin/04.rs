use matasano_rs::hex_to_bytes;

const INPUT: &str = include_str!("../../inputs/04.in");

fn char_freq(c: u8) -> u64 {
    match c.to_ascii_lowercase() {
        b'a' => 8000,
        b'b' => 1600,
        b'c' => 3000,
        b'd' => 4400,
        b'e' => 12000,
        b'f' => 2500,
        b'g' => 1700,
        b'h' => 6400,
        b'i' => 8000,
        b'j' => 400,
        b'k' => 800,
        b'l' => 4000,
        b'm' => 3000,
        b'n' => 8000,
        b'o' => 8000,
        b'p' => 1700,
        b'q' => 500,
        b'r' => 6200,
        b's' => 8000,
        b't' => 9000,
        b'u' => 3400,
        b'v' => 1200,
        b'w' => 2000,
        b'x' => 400,
        b'y' => 2000,
        b'z' => 200,
        b' ' => 15000,
        b'\'' => 100,
        _ => 0,
    }
}

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
