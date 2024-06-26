use itertools::Itertools;

pub mod aes;
pub mod base64;

pub fn hex_to_u8(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some((c as u8) - b'0'),
        'a'..='f' => Some((c as u8) - b'a' + 10),
        _ => None,
    }
}

pub fn hex_to_bytes(hex: &str) -> Option<Vec<u8>> {
    hex.chars()
        .tuples()
        .map(|(c1, c2)| {
            let d1 = hex_to_u8(c1)?;
            let d2 = hex_to_u8(c2)?;
            Some(d1 * 16 + d2)
        })
        .collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .flat_map(|&b| [b >> 4, b & 0xf])
        .map(|h| match h {
            0..=9 => (h + b'0') as char,
            10..=16 => (h - 10 + b'a') as char,
            _ => unreachable!(),
        })
        .collect()
}

pub fn char_freq(c: u8) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        // Test cases
        let test_cases: Vec<(&str, Option<Vec<u8>>)> = vec![
            ("", Some(vec![])),                                 // Empty input
            ("48656c6c6f", Some(vec![72, 101, 108, 108, 111])), // "Hello"
            ("deadbeef", Some(vec![222, 173, 190, 239])),       // Some bytes
            ("gg", None),                                       // Invalid hex characters
        ];

        // Run tests
        for (input, expected_output) in test_cases {
            let output = hex_to_bytes(input);
            assert_eq!(output, expected_output);
            println!("Input: {:?} => Bytes: {:?}", input, output);
        }
    }

    #[test]
    fn test_bytes_to_hex() {
        // Test cases
        let test_cases: Vec<(&[u8], &str)> = vec![
            (&[], ""),                                 // Empty input
            (&[72, 101, 108, 108, 111], "48656c6c6f"), // "Hello"
            (&[222, 173, 190, 239], "deadbeef"),       // Some bytes
            (&[15], "0f"),                             // Single byte
            (&[0, 255, 16, 31], "00ff101f"),           // Some bytes
        ];

        // Run tests
        for (input, expected_output) in test_cases {
            let output = bytes_to_hex(input);
            assert_eq!(output, expected_output);
            println!("Bytes: {:?} => Hex: {:?}", input, output);
        }
    }
}
