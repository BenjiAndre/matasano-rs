use matasano_rs::aes;
use openssl::rand::rand_bytes;
use rand::{thread_rng, Rng};

fn encryption_oracle(plaintext: &[u8]) -> (u8, Vec<u8>) {
    let mut rng = thread_rng();

    let prefix_count = rng.gen_range(5..=10);
    let suffix_count = rng.gen_range(5..=10);

    let mut prefix = vec![0u8; prefix_count];
    let mut suffix = vec![0u8; suffix_count];
    let mut key = vec![0u8; 16];

    rand_bytes(&mut prefix).unwrap();
    rand_bytes(&mut suffix).unwrap();
    rand_bytes(&mut key).unwrap();

    let mut data = Vec::with_capacity(prefix.len() + plaintext.len() + suffix.len());
    data.extend(prefix);
    data.extend_from_slice(plaintext);
    data.extend(suffix);

    if rng.gen_range(0..=1) == 1 {
        (1, aes::cbc_encrypt(&data, &key).unwrap())
    } else {
        (0, aes::ecb_encrypt(&data, &key).unwrap())
    }
}

fn main() {
    let plaintext = [0; 69];
    for _ in 0..10000 {
        let (mode, ciphertext) = encryption_oracle(&plaintext);
        let guess = if ciphertext[16..32] == ciphertext[32..48] {
            0
        } else {
            1
        };
        assert!(guess == mode);
    }
    println!("\x1b[32mCORRECT\x1b[0m");
}
