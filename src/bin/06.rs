use itertools::Itertools;
use matasano_rs::{base64, char_freq};

const INPUT: &str = include_str!("../../inputs/06.in");

fn hamming_distance(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter()
        .zip(s2)
        .fold(0, |acc, (&b1, &b2)| acc + (b1 ^ b2).count_ones() as usize)
}

fn find_keysize(s: &[u8]) -> usize {
    (2..=40)
        .min_by_key(|&size| {
            let blocks = s.chunks(size).take(4).collect::<Vec<&[u8]>>();

            blocks
                .iter()
                .tuple_combinations()
                .map(|(b1, b2)| hamming_distance(b1, b2))
                .sum::<usize>()
                / size
        })
        .unwrap()
}

fn crack_single_byte_xor(bytes: &[u8]) -> u8 {
    (0u8..=255)
        .max_by_key(|key| bytes.iter().map(|&b| char_freq(b ^ key)).sum::<u64>())
        .unwrap()
}

fn main() {
    let ciphertext = base64::decode(&INPUT.lines().join(""));
    let keysize = find_keysize(&ciphertext);
    let key = (0..keysize)
        .map(|i| {
            let block = (0..ciphertext.len() / keysize)
                .map(|n| ciphertext[n * keysize + i])
                .collect::<Vec<_>>();
            crack_single_byte_xor(&block)
        })
        .collect::<Vec<_>>();

    let plaintext = key
        .iter()
        .cycle()
        .zip(ciphertext)
        .map(|(key, c)| key ^ c)
        .collect::<Vec<_>>();

    println!("{}", String::from_utf8(plaintext).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(37, hamming_distance(b"this is a test", b"wokka wokka!!!"));
    }
}
