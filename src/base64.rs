const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn decode(s: &str) -> Vec<u8> {
    let mut result = Vec::new();

    for ch in s.chars().filter(|&c| !c.is_whitespace() && c != '=') {
        let value = BASE64_CHARS.iter().position(|&x| x == ch as u8).unwrap();
        result.push(value as u8);
    }

    result.truncate(result.len());

    let mut decoded_bytes = Vec::new();
    for chunk in result.chunks(4) {
        let mut value = (chunk[0] << 2) + (chunk[1] >> 4);
        decoded_bytes.push(value);

        if chunk.len() > 2 {
            value = ((chunk[1] & 0x0F) << 4) + (chunk[2] >> 2);
            decoded_bytes.push(value);
        }

        if chunk.len() > 3 {
            value = ((chunk[2] & 0x03) << 6) + chunk[3];
            decoded_bytes.push(value);
        }
    }

    decoded_bytes
}

pub fn encode(plaintext: &[u8]) -> String {
    let mut result = String::new();

    for chunk in plaintext.chunks(3) {
        let mut value = (chunk[0] & 0xFC) >> 2;
        result.push(BASE64_CHARS[value as usize] as char);

        value = ((chunk[0] & 0x03) << 4) + ((chunk.get(1).unwrap_or(&0) & 0xF0) >> 4);
        result.push(BASE64_CHARS[value as usize] as char);

        if let Some(&_second) = chunk.get(1) {
            value = ((chunk[1] & 0x0F) << 2) + ((chunk.get(2).unwrap_or(&0) & 0xC0) >> 6);
            result.push(BASE64_CHARS[value as usize] as char);

            if let Some(&_third) = chunk.get(2) {
                value = chunk[2] & 0x3F;
                result.push(BASE64_CHARS[value as usize] as char);
            } else {
                result.push('=');
            }
        } else {
            result.push_str("==");
        }
    }

    result
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

    #[test]
    fn test_base64_decode() {
        assert_eq!(decode(""), b"");
        assert_eq!(decode("Zm9v"), b"foo");
        assert_eq!(decode("Zm9vYmFy"), b"foobar");
        assert_eq!(decode("cnVzdGxpbmdz"), b"rustlings");
        assert_eq!(decode("Zg=="), b"f");
        assert_eq!(decode("Zm8="), b"fo");
    }
}
