use matasano_rs::base64;

const INPUT: &str = include_str!("../../inputs/06.in");

fn hamming_distance(s1: &str, s2: &str) -> usize {
    s1.as_bytes()
        .iter()
        .zip(s2.as_bytes())
        .fold(0, |acc, (&b1, &b2)| acc + (b1 ^ b2).count_ones() as usize)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(37, hamming_distance("this is a test", "wokka wokka!!!"));
    }
}
