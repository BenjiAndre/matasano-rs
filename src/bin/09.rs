fn pkcs7(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_size = block_size - (data.len() % block_size);
    let mut padded_data = Vec::from(data);
    for _ in 0..padding_size {
        padded_data.push(padding_size as u8);
    }
    padded_data
}

fn main() {
    let padded_data = pkcs7(b"YELLOW SUBMARINE", 20);
    let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";
    assert_eq!(padded_data, expected);
    println!("\x1b[32mCORRECT\x1b[0m");
}
