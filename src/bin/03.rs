use matasano_rs::hex_to_bytes;

const CIPHERTEXT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

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
