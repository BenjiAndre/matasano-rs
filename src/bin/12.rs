use matasano_rs::{aes, base64};
use openssl::rand::rand_bytes;

const SOMETHING: &str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK
";

fn special_encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut new_plaintext = plaintext.to_vec();
    new_plaintext.extend(base64::decode(SOMETHING));
    aes::ecb_encrypt(&new_plaintext, key).unwrap()
}

fn guess_block_size(key: &[u8]) -> usize {
    let first_length = special_encrypt(b"X", key).len();
    for i in 2..=255 {
        let plaintext = vec![b'X'; i];
        let new_length = special_encrypt(&plaintext, key).len();
        if new_length > first_length {
            return new_length - first_length;
        }
    }
    println!("fuck.");
    0
}

fn decrypt(block_size: usize, key: &[u8]) -> String {
    let mut plaintext = vec![];
    let size = block_size - 1;
    loop {
        let padding_size = size - plaintext.len() % block_size;
        let mut guess = vec![b'X'; padding_size];
        let expected = special_encrypt(&guess, key);

        guess.extend(&plaintext);
        guess.push(0);

        let l = guess.len() - 1;
        let found_byte = (0u8..=0xff)
            .find(|&byte| {
                guess[l] = byte;
                special_encrypt(&guess, key)[l - size..=l] == expected[l - size..=l]
            })
            .unwrap_or(0);

        if found_byte == 0 {
            break;
        }

        plaintext.push(found_byte)
    }

    String::from_utf8(plaintext).unwrap()
}

fn main() {
    let mut key = vec![0u8; 16];
    rand_bytes(&mut key).unwrap();

    let block_size = guess_block_size(&key);
    let ecb_check = special_encrypt(&[b'X'; 32], &key);
    assert_eq!(
        ecb_check[0..block_size],
        ecb_check[block_size..(2 * block_size)]
    );

    println!("{}", decrypt(block_size, &key));
}
