const BASE64_ENCODING_TABLE: [u8; 64] =
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_char_value(c: char) -> Result<usize, &'static str> {
    match c {
        'A'..='Z' => Ok(c as usize - 'A' as usize),
        'a'..='z' => Ok(c as usize - 'a' as usize + 26),
        '0'..='9' => Ok(c as usize - '0' as usize + 52),
        '+' => Ok(62),
        '/' => Ok(63),
        '=' => Ok(0), // padding character
        _ => Err("Invalid character in base64 string"),
    }
}

pub fn encode(plaintext: &[u8]) -> String {
    if plaintext.len() == 0 {
        return String::new();
    }

    let n_chars = (plaintext.len() - 1) / 3 * 4 + 4;
    let mut raw_ciphertext = vec![0u8; n_chars];
    let (mut i, mut j) = (0, 0);

    while i < plaintext.len() {
        let octet1 = *plaintext.get(i + 0).unwrap_or(&0) as u32;
        let octet2 = *plaintext.get(i + 1).unwrap_or(&0) as u32;
        let octet3 = *plaintext.get(i + 2).unwrap_or(&0) as u32;
        i += 3;

        let sextets = ((octet1 << 0x10) + (octet2 << 0x08) + octet3) as usize;
        raw_ciphertext[j + 0] = BASE64_ENCODING_TABLE[(sextets >> 0x12) & 63];
        raw_ciphertext[j + 1] = BASE64_ENCODING_TABLE[(sextets >> 0x0c) & 63];
        raw_ciphertext[j + 2] = BASE64_ENCODING_TABLE[(sextets >> 0x06) & 63];
        raw_ciphertext[j + 3] = BASE64_ENCODING_TABLE[(sextets >> 0x00) & 63];
        j += 4;
    }

    for i in 0..([0, 2, 1][plaintext.len() % 3]) {
        raw_ciphertext[n_chars - 1 - i] = b'=';
    }

    String::from_utf8(raw_ciphertext).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"foo"), "Zm9v");
        assert_eq!(encode(b"foobar"), "Zm9vYmFy");
        assert_eq!(encode(b"rustlings"), "cnVzdGxpbmdz");
        assert_eq!(encode(b"f"), "Zg==");
        assert_eq!(encode(b"fo"), "Zm8=");
    }
}
