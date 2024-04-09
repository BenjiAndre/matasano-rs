use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/08.in");

fn main() {
    let (number, ciphertext) = INPUT
        .lines()
        .find_position(|text| {
            text.as_bytes()
                .chunks(32)
                .tuple_combinations()
                .any(|(b1, b2)| b1 == b2)
        })
        .unwrap();

    let matching_encryption = ciphertext
        .as_bytes()
        .chunks(32)
        .tuple_combinations()
        .find(|(b1, b2)| b1 == b2)
        .unwrap()
        .0
        .to_vec();

    println!("The ciphertext n = {} was encrypted using ECB", number);
    for block in ciphertext.as_bytes().chunks(32) {
        if block == matching_encryption {
            println!(
                "\x1b[32m{}\x1b[0m",
                String::from_utf8(block.to_vec()).unwrap()
            );
        } else {
            println!("{}", String::from_utf8(block.to_vec()).unwrap());
        }
    }
}
